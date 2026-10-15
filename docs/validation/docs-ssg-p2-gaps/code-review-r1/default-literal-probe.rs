pub const BUILTIN_REFERENCE: &str =
    "https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md#static-builtins";

/// The repair for a payload whose YAML repeats a mapping key.
pub const DUPLICATE_KEY: &str =
    "Keep each YAML mapping key once; remove or rename the duplicate key.";

/// The repair for any other payload that is not a declaration.
pub const PAYLOAD: &str = "Write valid YAML for `rustify-component` (`name`, optional `props` mapping and `children` string) or `rustify-demo` (`name` only); correct the reported payload error.";

/// One declared prop as a `site!` manifest lists it, in normalized tokens.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub ty: String,
    pub default: Option<String>,
}

/// Where a component's props are declared, which decides the next step of
/// a repair that the declared type alone cannot describe.
#[derive(Clone, Copy, Debug)]
pub enum Owner<'a> {
    Builtin,
    Registered { registry: &'a str },
}

pub fn unregistered_component(names: &[&str]) -> String {
    format!(
        "Use a component from {}; for an author component, enable `[rust]` and register its name in `site!`.",
        names_list(names)
    )
}

pub fn unregistered_demo(names: &[&str]) -> String {
    if names.is_empty() {
        return "Enable `[rust]` and register a demo in `site!` before using a `rustify-demo` block."
            .into();
    }
    format!(
        "Use a demo from {}; register demos in `site!`.",
        names_list(names)
    )
}

pub fn unknown_property(component: &str, fields: &[Field], owner: Owner, field: &str) -> String {
    if fields.is_empty() {
        return format!("Remove `{field}`; `{component}` declares no props.");
    }
    format!(
        "Use one of {} for `{component}`; rename or remove `{field}`.",
        fields_list(fields, owner)
    )
}

pub fn reserved_component(name: &str) -> String {
    format!(
        "Rename `{name}` in `site!` and its Markdown references; builtin component names are reserved."
    )
}

/// The repair for props the declared types refused. `path` is the serde
/// path of the refused value; only its first segment names a declared field.
pub fn invalid_props(fields: &[Field], owner: Owner, path: Option<&str>) -> String {
    let path = path.filter(|path| !path.is_empty() && *path != ".");
    let top = path.and_then(|path| path.split(['.', '[']).next());
    let mut repair = match top.and_then(|top| fields.iter().find(|field| field.name == top)) {
        Some(field) => field_repair(field, owner),
        None => format!(
            "Add or correct `props` using the declared fields {}; {}.",
            fields_list(fields, owner),
            reference_step(owner)
        ),
    };
    if let Some(path) = path {
        repair.push_str(&format!(" Reported props path: `{path}`."));
    }
    repair
}

fn field_repair(field: &Field, owner: Owner) -> String {
    let ty = display(&field.ty);
    let boundary = boundary(field, owner);
    let name = &field.name;
    match (shape(&ty), owner) {
        (Some(shape), _) => format!("Write `{name}: {shape}` for declared type `{ty}` ({boundary})."),
        (None, Owner::Builtin) => format!(
            "Write `{name}` as `{ty}` ({boundary}); see {BUILTIN_REFERENCE} for its object fields."
        ),
        (None, Owner::Registered { registry }) => format!(
            "Check `{name}` of declared type `{ty}` ({boundary}) in `{registry}`; follow the reported serde requirement."
        ),
    }
}

fn reference_step(owner: Owner) -> String {
    match owner {
        Owner::Builtin => format!("see {BUILTIN_REFERENCE} for its object fields"),
        Owner::Registered { registry } => format!("check the registration in `{registry}`"),
    }
}

/// Whether the prop may be left out, from what the declaration states. A
/// type that is neither a direct `Option` nor a recognized form says nothing
/// about omission, because an alias or a custom type decides that itself.
fn boundary(field: &Field, owner: Owner) -> String {
    let ty = display(&field.ty);
    if let Some(default) = &field.default {
        format!("default expression `{}`", display(default))
    } else if generic(
        &ty,
        &["Option", "std::option::Option", "core::option::Option"],
    )
    .is_some()
    {
        "optional; omission is null".into()
    } else if shape(&ty).is_some() || matches!(owner, Owner::Builtin) {
        "required".into()
    } else {
        "no declared default; confirm omission semantics in the registration".into()
    }
}

/// A YAML value of the declared form, for the types whose form is fixed.
/// It shows the shape only; a value range is the refused message's to state.
fn shape(ty: &str) -> Option<String> {
    if let Some(inner) = generic(
        ty,
        &["Option", "std::option::Option", "core::option::Option"],
    ) {
        return scalar(inner).map(|_| "null".to_owned());
    }
    if let Some(inner) = generic(ty, &["Vec", "std::vec::Vec", "alloc::vec::Vec"]) {
        return scalar(inner).map(|shape| format!("[{shape}]"));
    }
    scalar(ty).map(str::to_owned)
}

fn scalar(ty: &str) -> Option<&'static str> {
    Some(match ty {
        "String" | "std::string::String" | "alloc::string::String" => "\"text\"",
        "bool" => "false",
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128"
        | "usize" => "0",
        "f32" | "f64" => "0.0",
        "char" => "\"a\"",
        _ => return None,
    })
}

fn generic<'a>(ty: &'a str, names: &[&str]) -> Option<&'a str> {
    names
        .iter()
        .find_map(|name| ty.strip_prefix(name)?.strip_prefix('<')?.strip_suffix('>'))
}

fn names_list(names: &[&str]) -> String {
    let mut names = names.to_vec();
    names.sort_unstable();
    names.dedup();
    if names.is_empty() {
        return "none".into();
    }
    names
        .iter()
        .map(|name| format!("`{name}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn fields_list(fields: &[Field], owner: Owner) -> String {
    let mut sorted: Vec<&Field> = fields.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));
    sorted.dedup_by(|a, b| a.name == b.name);
    if sorted.is_empty() {
        return "none".into();
    }
    sorted
        .iter()
        .map(|field| {
            format!(
                "`{}` (`{}`; {})",
                field.name,
                display(&field.ty),
                boundary(field, owner)
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// A manifest's normalized tokens as an author writes them: `Vec<StepSpec>`
/// and `"Counter".to_string()` rather than `Vec < StepSpec >` and
/// `"Counter" . to_string ()`. String and character literals are kept whole.
pub fn display(tokens: &str) -> String {
    const SPACED: [&str; 10] = ["->", "=>", "=", "==", "!=", "<=", ">=", "+", "&&", "||"];
    let word = |ch: char| ch.is_alphanumeric() || matches!(ch, '_' | '"' | '\'');
    let mut out = String::new();
    let mut previous: Option<String> = None;
    for piece in pieces(tokens) {
        if let Some(previous) = &previous {
            let spaced = previous.ends_with([',', ';'])
                || SPACED.contains(&previous.as_str())
                || SPACED.contains(&piece.as_str())
                || (previous.ends_with(word) && piece.starts_with(word));
            if spaced {
                out.push(' ');
            }
        }
        out.push_str(&piece);
        previous = Some(piece);
    }
    out
}

fn pieces(tokens: &str) -> Vec<String> {
    let chars: Vec<char> = tokens.chars().collect();
    let mut pieces = Vec::new();
    let mut current = String::new();
    let mut index = 0;
    while index < chars.len() {
        let ch = chars[index];
        if ch == ' ' {
            if !current.is_empty() {
                pieces.push(std::mem::take(&mut current));
            }
            index += 1;
            continue;
        }
        let closing = match ch {
            '"' => Some('"'),
            // A character literal, not a lifetime such as `'static`.
            '\'' if chars.get(index + 1) == Some(&'\\') || chars.get(index + 2) == Some(&'\'') => {
                Some('\'')
            }
            _ => None,
        };
        current.push(ch);
        index += 1;
        if let Some(closing) = closing {
            while index < chars.len() {
                let ch = chars[index];
                current.push(ch);
                index += 1;
                if ch == '\\' {
                    if let Some(&escaped) = chars.get(index) {
                        current.push(escaped);
                        index += 1;
                    }
                } else if ch == closing {
                    break;
                }
            }
        }
    }
    if !current.is_empty() {
        pieces.push(current);
    }
    pieces
}


fn main() {
    let source = r##"fn site() { rustify_docs::site! { components {
      Counter { label: String = r#"a"   b   "c"#.to_string() } => |props, children| props.label,
    } } }"##;
    let manifest = rustify_docs_syntax::parse_registry(source).unwrap().manifest();
    let fields: Vec<Field> = manifest.components[0].fields.iter().map(|field| Field {
        name: field.name.clone(), ty: field.ty.clone(), default: field.default.clone(),
    }).collect();
    println!("source={source}");
    println!("manifest_default={:?}", fields[0].default);
    println!("repair={}", unknown_property("Counter", &fields, Owner::Registered { registry: "src/lib.rs" }, "unexpected"));
    let exact = r##"r#"a"   b   "c"#"##;
    let repair = unknown_property("Counter", &fields, Owner::Registered { registry: "src/lib.rs" }, "unexpected");
    println!("literal_preserved={}", repair.contains(exact));
    println!("runtime_literal={:?}", r#"a"   b   "c"#);
}
