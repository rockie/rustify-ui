use super::super::{ClassGroupDef, ConfigExtension};
use super::{BuildSourceTree, CnError};
use regex::Regex;
const NAMESPACES: &[&str] = &[
    "inset-shadow",
    "drop-shadow",
    "text-shadow",
    "font-weight",
    "perspective",
    "breakpoint",
    "container",
    "tracking",
    "spacing",
    "leading",
    "animate",
    "radius",
    "shadow",
    "aspect",
    "blur",
    "ease",
    "text",
];
fn regex(pattern: &str) -> Result<Regex, CnError> {
    Regex::new(&pattern.replace(r"\s", r"[ \t\n\r\x0B\x0C\u{00A0}\u{1680}\u{2000}-\u{200A}\u{2028}\u{2029}\u{202F}\u{205F}\u{3000}\u{FEFF}]").replace(r"\b", r"(?-u:\b)")).map_err(|err| CnError::new(err.to_string()))
}
/// Whether a stylesheet declares a theme or imports Tailwind, including comments.
pub fn is_tailwind_css(css: &str) -> bool {
    if css.match_indices("@theme").any(|(at, _)| {
        css[at + 6..]
            .chars()
            .next()
            .is_none_or(|c| !(c.is_ascii_alphanumeric() || c == '_'))
    }) {
        return true;
    }
    css.match_indices("@import").any(|(at, _)| {
        let rest = &css[at + 7..];
        let stripped = rest.trim_start_matches(super::super::validators::is_whitespace);
        if rest.len() == stripped.len() {
            return false;
        }
        let stripped = stripped.strip_prefix("url(").map_or(stripped, |rest| {
            rest.trim_start_matches(super::super::validators::is_whitespace)
        });
        stripped
            .strip_prefix(['\"', '\''])
            .is_some_and(|rest| rest.starts_with("tailwindcss"))
    })
}
/// Theme extension and the ordered, deduplicated files read (entries first).
pub struct CssTheme {
    pub extension: ConfigExtension,
    pub files: Vec<String>,
}
/// Read relative imports recursively, then apply @theme names and namespace resets.
pub fn theme_from_css(tree: &BuildSourceTree, entries: &[String]) -> Result<CssTheme, CnError> {
    let comments = regex(r"/\*[\s\S]*?\*/")?;
    let imports = regex(r#"@import\s+(?:url\(\s*)?["']([^"']+)["']"#)?;
    let themes = regex(r"@theme\b[^{;]*\{")?;
    let mut files = Vec::new();
    let mut names: Vec<(String, Vec<String>)> = Vec::new();
    let mut resets = Vec::new();
    struct Reader<'a> {
        tree: &'a BuildSourceTree,
        comments: Regex,
        imports: Regex,
        themes: Regex,
        files: &'a mut Vec<String>,
        names: &'a mut Vec<(String, Vec<String>)>,
        resets: &'a mut Vec<String>,
    }
    impl Reader<'_> {
        fn visit(&mut self, path: &str, entry: &str) -> Result<(), CnError> {
            if self.files.iter().any(|f| f == path) {
                return Ok(());
            }
            self.files.push(path.into());
            let text = self.tree.read_text(path).map_err(|err| {
                CnError::new(format!(
                    "cannot read css {}: {err}",
                    if path == entry {
                        path.to_owned()
                    } else {
                        format!("{path} (imported from {entry})")
                    }
                ))
            })?;
            let css = self.comments.replace_all(&text, "").into_owned();
            let imports: Vec<_> = self
                .imports
                .captures_iter(&css)
                .map(|cap| cap[1].to_owned())
                .collect();
            for spec in imports {
                if spec.starts_with("./") || spec.starts_with("../") {
                    self.visit(
                        &self
                            .tree
                            .resolve_path(&self.tree.parent_path(path)?, &spec)?,
                        entry,
                    )?;
                }
            }
            let mut cursor = 0;
            while let Some(found) = self.themes.find_at(&css, cursor) {
                let start = found.end();
                let mut end = start;
                let mut depth = 1;
                for (offset, c) in css[start..].char_indices() {
                    end = start + offset + c.len_utf8();
                    match c {
                        '{' => depth += 1,
                        '}' => depth -= 1,
                        _ => {}
                    }
                    if depth == 0 {
                        break;
                    }
                }
                let body = if depth == 0 {
                    std::borrow::Cow::Borrowed(&css[start..end - 1])
                } else {
                    // JS slice(i - 1) removes one UTF-16 unit. A resulting
                    // isolated surrogate can occur only in the unfinished
                    // declaration's value; names and the `initial` comparison
                    // are preserved by decoding that unit as a replacement.
                    let mut units: Vec<_> = css[start..end].encode_utf16().collect();
                    units.pop();
                    std::borrow::Cow::Owned(String::from_utf16_lossy(&units))
                };
                for decl in body.split(';') {
                    let Some((prop, value)) = decl.split_once(':') else {
                        continue;
                    };
                    let prop = prop.trim_matches(super::super::validators::is_whitespace);
                    let value = value.trim_matches(super::super::validators::is_whitespace);
                    if prop == "--*" && value == "initial" {
                        for ns in NAMESPACES {
                            if !self.resets.iter().any(|s| s == ns) {
                                self.resets.push((*ns).into());
                            }
                        }
                        continue;
                    }
                    let Some((ns, name)) = NAMESPACES.iter().find_map(|ns| {
                        prop.strip_prefix(&format!("--{ns}-"))
                            .map(|name| (*ns, name))
                    }) else {
                        continue;
                    };
                    if name == "*" {
                        if value == "initial" && !self.resets.iter().any(|s| s == ns) {
                            self.resets.push(ns.into());
                        }
                        continue;
                    }
                    if name.is_empty() || name.contains("--") {
                        continue;
                    }
                    if value == "initial" {
                        if let Some((_, set)) = self.names.iter_mut().find(|(key, _)| key == ns) {
                            set.retain(|s| s != name);
                        }
                        continue;
                    }
                    let index = match self.names.iter().position(|(key, _)| key == ns) {
                        Some(index) => index,
                        None => {
                            self.names.push((ns.into(), Vec::new()));
                            self.names.len() - 1
                        }
                    };
                    if !self.names[index].1.iter().any(|s| s == name) {
                        self.names[index].1.push(name.into());
                    }
                }
                cursor = end;
                if cursor >= css.len() {
                    break;
                }
            }
            Ok(())
        }
    }
    let mut reader = Reader {
        tree,
        comments,
        imports,
        themes,
        files: &mut files,
        names: &mut names,
        resets: &mut resets,
    };
    for file in entries {
        let entry = tree.cwd(file)?;
        reader.visit(&entry, &entry)?;
    }
    let mut extension = ConfigExtension::default();
    for (scale, set) in &names {
        if !resets.contains(scale) {
            extension.extend.theme.push((
                scale.clone(),
                set.iter().cloned().map(ClassGroupDef::Literal).collect(),
            ));
        }
    }
    for scale in resets {
        let set = names
            .iter()
            .find(|(key, _)| key == &scale)
            .map(|(_, set)| set.as_slice())
            .unwrap_or_default();
        extension.override_groups.theme.push((
            scale,
            set.iter().cloned().map(ClassGroupDef::Literal).collect(),
        ));
    }
    Ok(CssTheme { extension, files })
}
