//! Finds a registered demo's direct source definition without rewriting it.

use rustify_docs::diagnostic_text::{code_span, literal};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use syn::ext::IdentExt;
use syn::spanned::Spanned;
use syn::{Attribute, Expr, Item, ItemFn, ItemMod, Lit, Meta, PathArguments, Token};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct DemoSource {
    pub path: String,
    pub function: String,
    pub file: String,
    pub start_line: usize,
    /// SHA-256 of the entire original file, including its BOM and line endings.
    pub digest: String,
}

/// One generation's file and AST cache. Start a new resolver for new inputs.
pub(super) struct Resolver {
    project_root: PathBuf,
    library_root: PathBuf,
    files: BTreeMap<PathBuf, Rc<SourceFile>>,
    resolved: BTreeMap<Vec<String>, DemoSource>,
    snapshot: Option<BTreeMap<PathBuf, Arc<[u8]>>>,
}

struct SourceFile {
    path: String,
    text: String,
    syntax: syn::File,
    parse_offset: usize,
    digest: String,
}

struct ModuleDirs {
    normal: PathBuf,
    explicit: PathBuf,
}

impl Resolver {
    pub(super) fn new(project_root: &Path, library_root: &Path) -> Result<Self, String> {
        let project_root = fs::canonicalize(project_root)
            .map_err(|error| format!("cannot open the source project boundary: {error}"))?;
        if !project_root.is_dir() {
            return Err("the source project boundary must be a directory".to_string());
        }
        let library_root = if library_root.is_absolute() {
            library_root.to_path_buf()
        } else {
            project_root.join(library_root)
        };
        let resolver = Self {
            project_root,
            library_root,
            files: BTreeMap::new(),
            resolved: BTreeMap::new(),
            snapshot: None,
        };
        resolver.canonical_source(&resolver.library_root)?;
        Ok(resolver)
    }

    /// Uses the bytes captured before Cargo runs, rather than rereading files
    /// into an older compiled generation.
    pub(super) fn from_snapshot(
        project_root: &Path,
        library_root: &Path,
        files: &BTreeMap<PathBuf, Arc<[u8]>>,
    ) -> Result<Self, String> {
        let mut resolver = Self::new(project_root, library_root)?;
        resolver.snapshot = Some(files.clone());
        Ok(resolver)
    }

    /// Files reached by direct module lookup, including sources outside the
    /// Cargo package directory but still inside the project boundary.
    pub(super) fn captured_files(&self) -> BTreeMap<PathBuf, Arc<[u8]>> {
        self.files
            .iter()
            .map(|(path, source)| (path.clone(), Arc::from(source.text.as_bytes())))
            .collect()
    }

    pub(super) fn resolve(&mut self, symbol: &str) -> Result<DemoSource, String> {
        let parts = symbol_parts(symbol)?;
        if let Some(source) = self.resolved.get(&parts) {
            return Ok(source.clone());
        }
        let library_root = self.library_root.clone();
        let source = self.load(&library_root)?;
        let directory = library_root
            .parent()
            .ok_or("the Cargo library source has no parent directory")?
            .to_path_buf();
        let resolved = self.walk(
            source.clone(),
            &source.syntax.items,
            ModuleDirs {
                normal: directory.clone(),
                explicit: directory,
            },
            &parts,
        )?;
        self.resolved.insert(parts, resolved.clone());
        Ok(resolved)
    }

    fn canonical_source(&self, path: &Path) -> Result<PathBuf, String> {
        let canonical = fs::canonicalize(path)
            .map_err(|error| format!("cannot open a Rust source file inside the project: {error}; register a direct source function"))?;
        if !canonical.starts_with(&self.project_root) {
            return Err("Rust source crosses the project boundary; move the demo into the project and register a direct source function".to_string());
        }
        if !canonical.is_file() {
            return Err(
                "the Rust source must be a file; register a direct source function".to_string(),
            );
        }
        Ok(canonical)
    }

    fn load(&mut self, path: &Path) -> Result<Rc<SourceFile>, String> {
        let canonical = self.canonical_source(path)?;
        if let Some(source) = self.files.get(&canonical) {
            return Ok(source.clone());
        }
        let relative = canonical
            .strip_prefix(&self.project_root)
            .map_err(|_| "Rust source crosses the project boundary".to_string())?;
        let path = relative
            .to_str()
            .ok_or("Rust source paths must be UTF-8")?
            .to_string();
        let text = match &self.snapshot {
            Some(files) => match files.get(&canonical) {
                Some(bytes) => std::str::from_utf8(bytes)
                    .map_err(|error| {
                        format!("{}: cannot read UTF-8 Rust source: {error}", literal(&path))
                    })?
                    .to_string(),
                None => fs::read_to_string(&canonical).map_err(|error| {
                    format!("{}: cannot read UTF-8 Rust source: {error}", literal(&path))
                })?,
            },
            None => fs::read_to_string(&canonical).map_err(|error| {
                format!("{}: cannot read UTF-8 Rust source: {error}", literal(&path))
            })?,
        };
        let syntax = syn::parse_file(&text)
            .map_err(|error| format!("{}: cannot parse Rust source: {error}", literal(&path)))?;
        // syn removes the BOM and shebang before parsing. Restore their byte
        // offset so spans still address the original, unmodified file.
        let parse_offset = usize::from(text.starts_with('\u{feff}')) * '\u{feff}'.len_utf8()
            + syntax.shebang.as_ref().map_or(0, String::len);
        let digest = format!("{:x}", Sha256::digest(text.as_bytes()));
        let source = Rc::new(SourceFile {
            path,
            text,
            syntax,
            parse_offset,
            digest,
        });
        self.files.insert(canonical, source.clone());
        Ok(source)
    }

    fn walk(
        &mut self,
        source: Rc<SourceFile>,
        items: &[Item],
        dirs: ModuleDirs,
        parts: &[String],
    ) -> Result<DemoSource, String> {
        let name = &parts[0];
        if parts.len() == 1 {
            let functions: Vec<_> = items
                .iter()
                .filter_map(|item| match item {
                    Item::Fn(function) if function.sig.ident.unraw() == name => Some(function),
                    _ => None,
                })
                .collect();
            return match functions.as_slice() {
                [function] => source.function(function),
                [] => Err(format!("{}: cannot locate a direct free function named {}; re-exported, generated and external functions need a direct source function path", literal(&source.path), code_span(name))),
                _ => Err(format!("{}: ambiguous function {} has multiple definitions, including possible cfg alternatives; register a uniquely named direct source function", literal(&source.path), code_span(name))),
            };
        }

        let modules: Vec<_> = items
            .iter()
            .filter_map(|item| match item {
                Item::Mod(module) if module.ident.unraw() == name => Some(module),
                _ => None,
            })
            .collect();
        let module = match modules.as_slice() {
            [module] => *module,
            [] => return Err(format!("{}: cannot locate source module {}; aliases, external crates and methods need a direct source function path", literal(&source.path), code_span(name))),
            _ => return Err(format!("{}: ambiguous module {} has multiple definitions; register a uniquely named direct source function", literal(&source.path), code_span(name))),
        };
        let explicit = module_path(&module.attrs)
            .map_err(|error| format!("{}: {error}", literal(&source.path)))?;
        if let Some((_, items)) = &module.content {
            let directory =
                explicit.map_or_else(|| dirs.normal.join(name), |path| dirs.explicit.join(path));
            return self.walk(
                source.clone(),
                items,
                ModuleDirs {
                    normal: directory.clone(),
                    explicit: directory,
                },
                &parts[1..],
            );
        }

        let explicit_file = explicit.is_some();
        let path = self.module_file(module, &dirs, explicit)?;
        let nested = self.load(&path)?;
        let directory = path
            .parent()
            .ok_or("the source module has no parent directory")?
            .to_path_buf();
        // An explicit #[path] owns its containing directory, unlike the
        // implicit foo.rs layout whose children live in foo/.
        let normal = if explicit_file || path.file_name().is_some_and(|name| name == "mod.rs") {
            directory.clone()
        } else {
            directory.join(
                path.file_stem()
                    .ok_or("the source module has no file stem")?,
            )
        };
        self.walk(
            nested.clone(),
            &nested.syntax.items,
            ModuleDirs {
                normal,
                explicit: directory,
            },
            &parts[1..],
        )
    }

    fn module_file(
        &self,
        module: &ItemMod,
        dirs: &ModuleDirs,
        explicit: Option<PathBuf>,
    ) -> Result<PathBuf, String> {
        if let Some(path) = explicit {
            return Ok(dirs.explicit.join(path));
        }
        let name = module.ident.unraw().to_string();
        let plain = dirs.normal.join(format!("{name}.rs"));
        let directory = dirs.normal.join(&name).join("mod.rs");
        match (plain.is_file(), directory.is_file()) {
            (true, false) => Ok(plain),
            (false, true) => Ok(directory),
            (true, true) => {
                let plain = self.canonical_source(&plain)?;
                let directory = self.canonical_source(&directory)?;
                let display = |path: &Path| path.strip_prefix(&self.project_root).ok().and_then(Path::to_str).unwrap_or("a project module").to_string();
                Err(format!("ambiguous module {} has both {} and {}; keep one source file and register a direct source function", code_span(&name), code_span(&display(&plain)), code_span(&display(&directory))))
            }
            (false, false) => Err(format!("cannot find the Rust source for module {}; check its .rs or mod.rs declaration and register a direct source function", code_span(&name))),
        }
    }
}

impl SourceFile {
    fn function(&self, function: &ItemFn) -> Result<DemoSource, String> {
        let range = function.span().byte_range();
        let mut start = range.start + self.parse_offset;
        let mut end = range.end + self.parse_offset;
        if self.text.get(start..end).is_none() {
            return Err(format!(
                "{}: source spans cannot locate the original function bytes",
                literal(&self.path)
            ));
        }
        let mut line_start = self.text[..start].rfind('\n').map_or(0, |at| at + 1);
        if line_start == 0 && self.text.starts_with('\u{feff}') {
            line_start = '\u{feff}'.len_utf8();
        }
        if self.text[line_start..start]
            .bytes()
            .all(|byte| matches!(byte, b' ' | b'\t'))
        {
            start = line_start;
        }
        let tail = &self.text[end..];
        let spaces = tail
            .bytes()
            .take_while(|byte| matches!(byte, b' ' | b'\t'))
            .count();
        let ending = &tail[spaces..];
        if ending.starts_with("\r\n") {
            end += spaces + 2;
        } else if ending.starts_with('\n') {
            end += spaces + 1;
        } else if ending.is_empty() {
            end += spaces;
        }
        Ok(DemoSource {
            path: self.path.clone(),
            function: self.text[start..end].to_string(),
            file: self.text.clone(),
            start_line: self.text[..start]
                .bytes()
                .filter(|byte| *byte == b'\n')
                .count()
                + 1,
            digest: self.digest.clone(),
        })
    }
}

fn symbol_parts(symbol: &str) -> Result<Vec<String>, String> {
    let path = syn::parse_str::<syn::Path>(symbol).map_err(|error| {
        format!(
            "invalid demo source path: {error}; register a direct source function using crate::"
        )
    })?;
    if path.leading_colon.is_some()
        || path
            .segments
            .iter()
            .any(|segment| !matches!(segment.arguments, PathArguments::None))
    {
        return Err(
            "external or generic source paths need a direct source function using crate::"
                .to_string(),
        );
    }
    let mut parts: Vec<_> = path
        .segments
        .iter()
        .map(|segment| segment.ident.unraw().to_string())
        .collect();
    if matches!(parts.first().map(String::as_str), Some("crate" | "self")) {
        parts.remove(0);
    }
    if parts.is_empty()
        || parts
            .iter()
            .any(|part| matches!(part.as_str(), "crate" | "self" | "super"))
    {
        return Err("relative super paths need declaration context; register a direct source function using crate::".to_string());
    }
    Ok(parts)
}

fn module_path(attrs: &[Attribute]) -> Result<Option<PathBuf>, String> {
    let mut path = None;
    for attr in attrs {
        if attr.path().is_ident("cfg_attr") && conditional_path(&attr.meta)? {
            return Err("conditional module paths cannot uniquely locate a demo; register a direct source function in an unconditional module".to_string());
        }
        if !attr.path().is_ident("path") {
            continue;
        }
        let value = match &attr.meta {
            Meta::NameValue(value) => match &value.value {
                Expr::Lit(value) => match &value.lit {
                    Lit::Str(value) => PathBuf::from(value.value()),
                    _ => {
                        return Err(
                            "module path must be a string literal for a direct source function"
                                .to_string(),
                        )
                    }
                },
                _ => {
                    return Err(
                        "module path must be a string literal for a direct source function"
                            .to_string(),
                    )
                }
            },
            _ => {
                return Err(
                    "module path must be a string literal for a direct source function".to_string(),
                )
            }
        };
        if path.replace(value).is_some() {
            return Err(
                "ambiguous module has multiple path attributes; register a direct source function"
                    .to_string(),
            );
        }
    }
    Ok(path)
}

fn conditional_path(meta: &Meta) -> Result<bool, String> {
    if meta.path().is_ident("path") {
        return Ok(true);
    }
    if !meta.path().is_ident("cfg_attr") {
        return Ok(false);
    }
    let Meta::List(list) = meta else {
        return Ok(false);
    };
    let nested = list.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)
        .map_err(|error| format!("cannot parse conditional module attributes: {error}; register a direct source function"))?;
    for attr in nested.iter().skip(1) {
        if conditional_path(attr)? {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
pub(super) fn resolve(
    project_root: &Path,
    library_root: &Path,
    symbol: &str,
) -> Result<DemoSource, String> {
    Resolver::new(project_root, library_root)?.resolve(symbol)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct Project(PathBuf);

    impl Project {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(1);
            let root = std::env::temp_dir().join(format!(
                "rustify-source-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed),
            ));
            fs::create_dir_all(root.join("src")).unwrap();
            Self(root)
        }

        fn write(&self, path: &str, text: &str) {
            let path = self.0.join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, text).unwrap();
        }

        fn locate(&self, symbol: &str) -> Result<DemoSource, String> {
            resolve(&self.0, &self.0.join("src/lib.rs"), symbol)
        }
    }

    impl Drop for Project {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn exact_function_and_full_file_preserve_bom_unicode_crlf_and_attributes() {
        let project = Project::new();
        let function = concat!(
            "    /// 演示 résumé\r\n",
            "    #[allow(unused_variables)]\r\n",
            "    pub fn counter() -> &'static str {\r\n",
            "        // braces in comments: } {\r\n",
            "        let café = r###\"{ not a block }\"###;\r\n",
            "\r\n",
            "        café\r\n",
            "    }\r\n",
        );
        let file = format!("\u{feff}//! file résumé\r\n\r\nmod demos {{\r\n{function}}}\r\n");
        project.write("src/lib.rs", &file);

        let source = project.locate("crate::demos::counter").unwrap();

        assert_eq!(source.function, function);
        assert_eq!(source.file, file);
        assert_eq!(source.path, "src/lib.rs");
        assert_eq!(source.start_line, 4);
        assert_eq!(
            source.digest,
            format!("{:x}", Sha256::digest(file.as_bytes()))
        );
    }

    #[test]
    fn nested_rs_modules_select_the_requested_function_among_equal_names() {
        let project = Project::new();
        project.write("src/lib.rs", "mod left; mod right;\n");
        project.write("src/left.rs", "pub fn demo() { let wrong = true; }\n");
        project.write("src/right.rs", "mod detail;\n");
        project.write(
            "src/right/detail.rs",
            "pub fn demo() { let selected = true; }\n",
        );

        let source = project.locate("right::detail::demo").unwrap();

        assert_eq!(source.path, "src/right/detail.rs");
        assert_eq!(source.function, "pub fn demo() { let selected = true; }\n");
    }

    #[test]
    fn mod_rs_and_inline_modules_keep_their_child_directory() {
        let project = Project::new();
        project.write("src/lib.rs", "mod demos;\n");
        project.write("src/demos/mod.rs", "mod inline { mod controls; }\n");
        project.write("src/demos/inline/controls.rs", "pub fn demo() {}\n");

        let source = project
            .locate("self::demos::inline::controls::demo")
            .unwrap();

        assert_eq!(source.path, "src/demos/inline/controls.rs");
    }

    #[test]
    fn path_attribute_outside_inline_is_relative_to_the_containing_file() {
        let project = Project::new();
        project.write("src/lib.rs", "mod demos;\n");
        project.write("src/demos.rs", "#[path = \"chosen.rs\"] mod controls;\n");
        project.write("src/chosen.rs", "pub fn demo() {}\n");

        let source = project.locate("crate::demos::controls::demo").unwrap();

        assert_eq!(source.path, "src/chosen.rs");
    }

    #[test]
    fn an_explicit_file_path_gives_nested_modules_its_containing_directory() {
        let project = Project::new();
        project.write("src/lib.rs", "#[path = \"other.rs\"] mod renamed;\n");
        project.write("src/other.rs", "mod child;\n");
        project.write("src/child.rs", "pub fn demo() { let correct = true; }\n");
        project.write(
            "src/other/child.rs",
            "pub fn demo() { let wrong = true; }\n",
        );

        let source = project.locate("crate::renamed::child::demo").unwrap();

        assert_eq!(source.path, "src/child.rs");
        assert!(source.function.contains("correct"), "{}", source.function);
    }

    #[test]
    fn path_attribute_inside_non_mod_rs_inline_uses_the_owned_directory() {
        let project = Project::new();
        project.write("src/lib.rs", "mod demos;\n");
        project.write(
            "src/demos.rs",
            "mod inline { #[path = \"chosen.rs\"] mod controls; }\n",
        );
        project.write("src/demos/inline/chosen.rs", "pub fn demo() {}\n");

        let source = project
            .locate("crate::demos::inline::controls::demo")
            .unwrap();

        assert_eq!(source.path, "src/demos/inline/chosen.rs");
    }

    #[test]
    fn path_attribute_on_inline_module_changes_its_child_directory() {
        let project = Project::new();
        project.write(
            "src/lib.rs",
            "#[path = \"thread_files\"] mod demos { mod controls; }\n",
        );
        project.write("src/thread_files/controls.rs", "pub fn demo() {}\n");

        let source = project.locate("crate::demos::controls::demo").unwrap();

        assert_eq!(source.path, "src/thread_files/controls.rs");
    }

    #[test]
    fn custom_cargo_library_file_still_uses_its_parent_as_the_root_directory() {
        let project = Project::new();
        project.write("site/library.rs", "mod demos;\n");
        project.write("site/demos.rs", "pub fn demo() {}\n");

        let source = resolve(
            &project.0,
            &project.0.join("site/library.rs"),
            "crate::demos::demo",
        )
        .unwrap();

        assert_eq!(source.path, "site/demos.rs");
    }

    #[test]
    fn cfg_duplicates_are_rejected_instead_of_selecting_a_target_implementation() {
        let project = Project::new();
        project.write("src/lib.rs", "#[cfg(feature = \"ssr\")] pub fn demo() {}\n#[cfg(not(feature = \"ssr\"))] pub fn demo() {}\n");

        let error = project.locate("crate::demo").unwrap_err();

        assert!(error.contains("ambiguous"), "{error}");
        assert!(error.contains("direct"), "{error}");
        assert!(!error.contains(project.0.to_str().unwrap()), "{error}");
    }

    #[test]
    fn conflicting_external_module_files_are_rejected() {
        let project = Project::new();
        project.write("src/lib.rs", "mod demos;\n");
        project.write("src/demos.rs", "pub fn demo() {}\n");
        project.write("src/demos/mod.rs", "pub fn demo() {}\n");

        let error = project.locate("crate::demos::demo").unwrap_err();

        assert!(error.contains("ambiguous"), "{error}");
        assert!(error.contains("src/demos.rs"), "{error}");
        assert!(error.contains("src/demos/mod.rs"), "{error}");
    }

    #[test]
    fn reexports_generated_functions_external_crates_and_methods_require_a_direct_function() {
        let project = Project::new();
        project.write(
            "src/lib.rs",
            concat!(
                "mod demos { pub fn demo() {} }\n",
                "pub use demos::demo as exported;\n",
                "macro_rules! generate { () => { pub fn generated() {} } }\n",
                "generate!();\n",
                "extern crate external;\n",
                "struct Widget; impl Widget { pub fn method() {} }\n",
            ),
        );

        for symbol in [
            "crate::exported",
            "crate::generated",
            "external::demo",
            "crate::Widget::method",
            "super::demo",
        ] {
            let error = project.locate(symbol).unwrap_err();
            assert!(error.contains("direct"), "{symbol}: {error}");
        }
    }

    #[test]
    fn conditional_module_paths_are_rejected_when_the_source_is_not_unique() {
        let project = Project::new();
        project.write(
            "src/lib.rs",
            "#[cfg_attr(feature = \"ssr\", path = \"other.rs\")] mod demos;\n",
        );
        project.write("src/demos.rs", "pub fn demo() {}\n");
        project.write("src/other.rs", "pub fn demo() {}\n");

        let error = project.locate("crate::demos::demo").unwrap_err();

        assert!(error.contains("conditional"), "{error}");
        assert!(error.contains("direct"), "{error}");
    }

    #[test]
    fn one_resolver_keeps_the_same_source_snapshot_across_a_batch() {
        let project = Project::new();
        project.write(
            "src/lib.rs",
            "pub fn first() { let original = true; }\npub fn second() { let original = true; }\n",
        );
        let mut resolver = Resolver::new(&project.0, &project.0.join("src/lib.rs")).unwrap();
        let first = resolver.resolve("crate::first").unwrap();
        project.write(
            "src/lib.rs",
            "pub fn first() { let changed = true; }\npub fn second() { let changed = true; }\n",
        );

        let second = resolver.resolve("crate::second").unwrap();
        assert_eq!(second.file, first.file);
        assert_eq!(second.digest, first.digest);
        assert!(second.function.contains("original"), "{}", second.function);
        assert_eq!(resolver.resolve("self::first").unwrap(), first);
        assert_ne!(project.locate("crate::first").unwrap().digest, first.digest);
    }

    #[test]
    fn a_coordinator_snapshot_supplies_original_bytes_before_the_first_lookup() {
        let project = Project::new();
        let original = "pub fn demo() { let original = true; }\r\n";
        project.write("src/lib.rs", original);
        let library = fs::canonicalize(project.0.join("src/lib.rs")).unwrap();
        let files = BTreeMap::from([(library.clone(), Arc::<[u8]>::from(original.as_bytes()))]);
        project.write("src/lib.rs", "pub fn demo() { let changed = true; }\n");
        let mut resolver = Resolver::from_snapshot(&project.0, &library, &files).unwrap();

        let source = resolver.resolve("crate::demo").unwrap();

        assert_eq!(source.file, original);
        assert_eq!(source.function, original);
        assert_eq!(
            source.digest,
            format!("{:x}", Sha256::digest(original.as_bytes()))
        );
        assert_eq!(resolver.captured_files(), files);
    }

    #[test]
    fn a_function_at_eof_does_not_gain_a_newline_or_lose_its_indentation() {
        let project = Project::new();
        project.write("src/lib.rs", "\u{feff}    pub fn demo() {}");

        let source = project.locate("crate::demo").unwrap();

        assert_eq!(source.function, "    pub fn demo() {}");
        assert_eq!(source.start_line, 1);
        assert_eq!(source.file, "\u{feff}    pub fn demo() {}");
    }

    #[test]
    fn unicode_before_a_same_line_function_uses_byte_spans() {
        let project = Project::new();
        project.write(
            "src/lib.rs",
            "mod 演示 { const TEXT: &str = \"é\"; pub fn 函数() { let π = 'λ'; } }\n",
        );

        let source = project.locate("crate::演示::函数").unwrap();

        assert_eq!(source.function, "pub fn 函数() { let π = 'λ'; }");
        assert_eq!(source.start_line, 1);
    }

    #[test]
    fn a_shebang_and_bom_do_not_shift_the_function_slice_or_line_number() {
        let project = Project::new();
        let file = "\u{feff}#!/usr/bin/env rust-script\r\n\r\npub fn demo() {}\r\n";
        project.write("src/lib.rs", file);

        let source = project.locate("crate::demo").unwrap();

        assert_eq!(source.file, file);
        assert_eq!(source.function, "pub fn demo() {}\r\n");
        assert_eq!(source.start_line, 3);
    }

    #[cfg(unix)]
    #[test]
    fn a_module_symlink_outside_the_project_is_not_read_or_exposed() {
        let project = Project::new();
        let outside = Project::new();
        outside.write("src/secret.rs", "pub fn demo() {}\n");
        project.write("src/lib.rs", "mod demos;\n");
        std::os::unix::fs::symlink(
            outside.0.join("src/secret.rs"),
            project.0.join("src/demos.rs"),
        )
        .unwrap();

        let error = project.locate("crate::demos::demo").unwrap_err();

        assert!(error.contains("boundary"), "{error}");
        assert!(!error.contains(outside.0.to_str().unwrap()), "{error}");
    }
}
