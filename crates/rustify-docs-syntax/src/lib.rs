//! The native syntax shared by the `site!` macro and the cold registry reader.
//!
//! The parser retains Rust types, default expressions and render expressions
//! for macro expansion. It never evaluates them. [`Manifest`] is a separate,
//! serializable description whose token strings are normalized in one place.

#[cfg(target_arch = "wasm32")]
compile_error!(
    "rustify-docs-syntax is a native build dependency, not a browser runtime dependency"
);

mod manifest;

pub use manifest::{
    ClientOnlyManifest, ComponentManifest, DemoManifest, FieldManifest, Manifest, PageManifest,
    Slot, SlotManifest,
};

use proc_macro2::{Delimiter, Spacing, Span, TokenStream, TokenTree};
use quote::ToTokens;
use std::collections::HashSet;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{braced, Attribute, Error, Expr, ExprPath, Ident, LitStr, Macro, Result, Token, Type};

/// Quotes a complete code value with a fence longer than any run it contains.
pub fn code_span(value: &str) -> String {
    if value.is_empty() {
        return "``  ``".into();
    }
    let longest = value.split(|ch| ch != '`').map(str::len).max().unwrap_or(0);
    if longest == 0 {
        return format!("`{value}`");
    }
    let fence = "`".repeat(longest + 1);
    format!("{fence} {value} {fence}")
}

/// Keeps ordinary author text as written, quoting the whole value if it has ticks.
pub fn literal(value: &str) -> String {
    if value.contains('`') {
        code_span(value)
    } else {
        value.to_owned()
    }
}

/// One declaration, with source spans and Rust AST for the procedural macro.
#[derive(Clone, Default)]
pub struct SiteDeclaration {
    pub components: Vec<ComponentDeclaration>,
    pub demos: Vec<DemoDeclaration>,
    pub pages: Vec<PageDeclaration>,
    pub slots: Vec<SlotDeclaration>,
}

/// A named component and its typed adapter expression.
#[derive(Clone)]
pub struct ComponentDeclaration {
    pub name: Ident,
    pub fields: Vec<FieldDeclaration>,
    pub adapter: Expr,
    pub client_only: Option<ClientOnlyDeclaration>,
}

/// A typed property, with an optional trusted Rust default expression.
#[derive(Clone)]
pub struct FieldDeclaration {
    pub name: Ident,
    pub ty: Type,
    pub default: Option<Expr>,
}

/// The fixed placeholder used before a client-only component can run.
#[derive(Clone)]
pub struct ClientOnlyDeclaration {
    pub class: LitStr,
    pub message: LitStr,
}

/// A demo whose function path also identifies the source shown to readers.
#[derive(Clone)]
pub struct DemoDeclaration {
    pub name: Ident,
    pub function: ExprPath,
}

/// A static page and its literal metadata.
#[derive(Clone)]
pub struct PageDeclaration {
    pub name: Ident,
    pub route: LitStr,
    pub title: LitStr,
    pub description: Option<LitStr>,
    pub lang: Option<LitStr>,
    pub function: ExprPath,
}

/// A renderer for one of the seven documented slots.
#[derive(Clone)]
pub struct SlotDeclaration {
    pub name: Ident,
    pub slot: Slot,
    pub function: ExprPath,
}

impl SiteDeclaration {
    /// Produces the same metadata for the macro and the cold registry reader.
    /// Source locations stay on the AST, outside this comparison contract.
    pub fn manifest(&self) -> Manifest {
        Manifest {
            components: self
                .components
                .iter()
                .map(|component| ComponentManifest {
                    name: name(&component.name),
                    fields: component
                        .fields
                        .iter()
                        .map(|field| FieldManifest {
                            name: name(&field.name),
                            ty: normalized(&field.ty),
                            default: field.default.as_ref().map(normalized),
                        })
                        .collect(),
                    adapter: normalized(&component.adapter),
                    client_only: component.client_only.as_ref().map(|placeholder| {
                        ClientOnlyManifest {
                            class: placeholder.class.value(),
                            message: placeholder.message.value(),
                        }
                    }),
                })
                .collect(),
            demos: self
                .demos
                .iter()
                .map(|demo| DemoManifest {
                    name: name(&demo.name),
                    function: normalized(&demo.function),
                })
                .collect(),
            pages: self
                .pages
                .iter()
                .map(|page| PageManifest {
                    name: name(&page.name),
                    route: page.route.value(),
                    title: page.title.value(),
                    description: page.description.as_ref().map(LitStr::value),
                    lang: page.lang.as_ref().map(LitStr::value),
                    function: normalized(&page.function),
                })
                .collect(),
            slots: self
                .slots
                .iter()
                .map(|slot| SlotManifest {
                    slot: slot.slot,
                    function: normalized(&slot.function),
                })
                .collect(),
        }
    }
}

/// Parses the single concrete `site!` invocation in a registry source file.
///
/// Qualified invocations are accepted. Comments, string literals and opaque
/// macro bodies are not declarations. Returned `syn::Error` spans retain the
/// source's line and column coordinates.
///
/// ```
/// use rustify_docs_syntax::parse_registry;
/// let declaration = parse_registry(
///     "fn site() { site! { demos { CounterDemo => crate::demos::counter_demo } } }",
/// ).unwrap();
/// assert_eq!(declaration.manifest().demos[0].name, "CounterDemo");
/// ```
pub fn parse_registry(source: &str) -> Result<SiteDeclaration> {
    // syn strips a BOM. Rust whitespace of the same byte/character width
    // preserves its coordinates; a shebang line contains no declarations.
    let padded = source
        .strip_prefix('\u{feff}')
        .filter(|content| {
            !syn::parse_file(content.split('\n').next().unwrap_or(content))
                .is_ok_and(|file| file.shebang.is_some())
        })
        .map(|content| format!("\u{200e}{content}"));
    let file = syn::parse_file(padded.as_deref().unwrap_or(source))?;
    let mut finder = SiteFinder::default();
    finder.visit_file(&file);
    match finder.found.as_slice() {
        [] => Err(Error::new(
            Span::call_site(),
            "no site! declaration in registry source; declare one directly",
        )),
        [declaration] => {
            if declaration.tokens.is_empty() {
                return Err(Error::new_spanned(
                    &declaration.path,
                    "site! must register at least one component, demo, page or slot",
                ));
            }
            syn::parse2(declaration.tokens.clone())
        }
        [_, second, ..] => Err(Error::new_spanned(
            &second.path,
            "multiple site! declarations in registry source; keep exactly one",
        )),
    }
}

#[derive(Default)]
struct SiteFinder<'ast> {
    found: Vec<&'ast Macro>,
}

impl<'ast> Visit<'ast> for SiteFinder<'ast> {
    fn visit_macro(&mut self, invocation: &'ast Macro) {
        if invocation
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "site")
        {
            self.found.push(invocation);
        }
    }
}

impl Parse for SiteDeclaration {
    fn parse(input: ParseStream) -> Result<Self> {
        let start = input.span();
        let mut declaration = Self::default();
        let mut sections = HashSet::new();
        let mut names = HashSet::new();
        let mut routes = HashSet::new();
        let mut slots = HashSet::new();
        while !input.is_empty() {
            let section: Ident = input.parse()?;
            let section_name = name(&section);
            if !sections.insert(section_name.clone()) {
                return Err(Error::new(
                    section.span(),
                    format!("duplicate site section {}", code_span(&section_name)),
                ));
            }
            let content;
            braced!(content in input);
            match section_name.as_str() {
                "components" => {
                    while !content.is_empty() {
                        let component = parse_component(&content)?;
                        register(&component.name, &mut names)?;
                        declaration.components.push(component);
                        separator(&content)?;
                    }
                }
                "demos" => {
                    while !content.is_empty() {
                        let name = content.parse()?;
                        register(&name, &mut names)?;
                        content.parse::<Token![=>]>()?;
                        let function = function_path(&content, "demo")?;
                        declaration.demos.push(DemoDeclaration { name, function });
                        separator(&content)?;
                    }
                }
                "pages" => {
                    while !content.is_empty() {
                        let page = parse_page(&content)?;
                        register(&page.name, &mut names)?;
                        let route = page.route.value();
                        let lang = page
                            .lang
                            .as_ref()
                            .map(|lang| lang.value().to_ascii_lowercase());
                        if !routes.insert((lang, route.clone())) {
                            return Err(Error::new(
                                page.route.span(),
                                format!("duplicate page route {}", code_span(&route)),
                            ));
                        }
                        declaration.pages.push(page);
                        separator(&content)?;
                    }
                }
                "slots" => {
                    while !content.is_empty() {
                        let name: Ident = content.parse()?;
                        let slot = slot_name(&name)?;
                        if !slots.insert(slot) {
                            return Err(Error::new(
                                name.span(),
                                format!("duplicate slot {}", code_span(slot.key())),
                            ));
                        }
                        content.parse::<Token![=>]>()?;
                        let function = function_path(&content, "slot")?;
                        declaration.slots.push(SlotDeclaration {
                            name,
                            slot,
                            function,
                        });
                        separator(&content)?;
                    }
                }
                _ => {
                    return Err(Error::new(
                        section.span(),
                        format!(
                            "unknown site section {}; expected components, demos, pages or slots",
                            code_span(&section_name)
                        ),
                    ));
                }
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        if declaration.components.is_empty()
            && declaration.demos.is_empty()
            && declaration.pages.is_empty()
            && declaration.slots.is_empty()
        {
            return Err(Error::new(
                start,
                "site! must register at least one component, demo, page or slot",
            ));
        }
        Ok(declaration)
    }
}

fn parse_component(input: ParseStream) -> Result<ComponentDeclaration> {
    let attributes = input.call(Attribute::parse_outer)?;
    let mut client_only = None;
    for attribute in attributes {
        if !attribute.path().is_ident("client_only") {
            return Err(Error::new_spanned(
                attribute,
                "unsupported component attribute; only #[client_only(class = \"...\", message = \"...\")] is supported",
            ));
        }
        if client_only.is_some() {
            return Err(Error::new_spanned(
                attribute,
                "duplicate client_only attribute",
            ));
        }
        client_only = Some(parse_client_only(&attribute)?);
    }
    let name = input.parse()?;
    let content;
    braced!(content in input);
    let mut fields = Vec::new();
    let mut seen = HashSet::new();
    while !content.is_empty() {
        let field_name: Ident = content.parse()?;
        let field_key = self::name(&field_name);
        if !seen.insert(field_key.clone()) {
            return Err(Error::new(
                field_name.span(),
                format!("duplicate property {}", code_span(&field_key)),
            ));
        }
        content.parse::<Token![:]>()?;
        let ty = content.parse()?;
        let default = if content.peek(Token![=]) {
            content.parse::<Token![=]>()?;
            Some(content.parse()?)
        } else {
            None
        };
        fields.push(FieldDeclaration {
            name: field_name,
            ty,
            default,
        });
        separator(&content)?;
    }
    input.parse::<Token![=>]>()?;
    let adapter = input.parse()?;
    Ok(ComponentDeclaration {
        name,
        fields,
        adapter,
        client_only,
    })
}

fn parse_client_only(attribute: &Attribute) -> Result<ClientOnlyDeclaration> {
    let mut class = None;
    let mut message = None;
    attribute.parse_nested_meta(|meta| {
        let key = normalized(&meta.path);
        let destination = if meta.path.is_ident("class") {
            &mut class
        } else if meta.path.is_ident("message") {
            &mut message
        } else {
            return Err(meta.error(format!("unknown client_only metadata {}", code_span(&key))));
        };
        if destination.is_some() {
            return Err(meta.error(format!(
                "duplicate client_only metadata {}",
                code_span(&key)
            )));
        }
        let literal: LitStr = meta.value()?.parse()?;
        if literal.value().trim().is_empty() {
            return Err(Error::new(
                literal.span(),
                format!("client_only {} must not be empty", code_span(&key)),
            ));
        }
        *destination = Some(literal);
        Ok(())
    })?;
    Ok(ClientOnlyDeclaration {
        class: class
            .ok_or_else(|| Error::new(attribute.span(), "missing client_only metadata `class`"))?,
        message: message.ok_or_else(|| {
            Error::new(attribute.span(), "missing client_only metadata `message`")
        })?,
    })
}

fn parse_page(input: ParseStream) -> Result<PageDeclaration> {
    let name: Ident = input.parse()?;
    let content;
    braced!(content in input);
    let mut route = None;
    let mut title = None;
    let mut description = None;
    let mut lang = None;
    while !content.is_empty() {
        let key: Ident = content.parse()?;
        let key_name = self::name(&key);
        let destination = match key_name.as_str() {
            "route" => &mut route,
            "title" => &mut title,
            "description" => &mut description,
            "lang" => &mut lang,
            _ => {
                return Err(Error::new(
                    key.span(),
                    format!("unknown page metadata {}", code_span(&key_name)),
                ));
            }
        };
        if destination.is_some() {
            return Err(Error::new(
                key.span(),
                format!("duplicate page metadata {}", code_span(&key_name)),
            ));
        }
        content.parse::<Token![:]>()?;
        *destination = Some(content.parse::<LitStr>()?);
        separator(&content)?;
    }
    let route = route.ok_or_else(|| Error::new(name.span(), "missing page metadata `route`"))?;
    let title = title.ok_or_else(|| Error::new(name.span(), "missing page metadata `title`"))?;
    input.parse::<Token![=>]>()?;
    let function = function_path(input, "page")?;
    Ok(PageDeclaration {
        name,
        route,
        title,
        description,
        lang,
        function,
    })
}

fn slot_name(ident: &Ident) -> Result<Slot> {
    match name(ident).as_str() {
        "header" => Ok(Slot::Header),
        "footer" => Ok(Slot::Footer),
        "sidebar_top" => Ok(Slot::SidebarTop),
        "sidebar_bottom" => Ok(Slot::SidebarBottom),
        "before_content" => Ok(Slot::BeforeContent),
        "after_content" => Ok(Slot::AfterContent),
        "not_found" => Ok(Slot::NotFound),
        other => Err(Error::new(
            ident.span(),
            format!("unknown slot {}; expected header, footer, sidebar_top, sidebar_bottom, before_content, after_content or not_found", code_span(other)),
        )),
    }
}

fn function_path(input: ParseStream, kind: &str) -> Result<ExprPath> {
    let expression: Expr = input.parse()?;
    match expression {
        Expr::Path(path) => Ok(path),
        other => Err(Error::new_spanned(
            other,
            format!("{kind} render function must be a direct Rust path"),
        )),
    }
}

fn register(ident: &Ident, names: &mut HashSet<String>) -> Result<()> {
    let key = name(ident);
    if names.insert(key.clone()) {
        Ok(())
    } else {
        Err(Error::new(
            ident.span(),
            format!("duplicate registration name {}", code_span(&key)),
        ))
    }
}

fn separator(input: ParseStream) -> Result<()> {
    if !input.is_empty() {
        input.parse::<Token![,]>()?;
    }
    Ok(())
}

fn name(ident: &Ident) -> String {
    ident.unraw().to_string()
}

fn normalized(tokens: &impl ToTokens) -> String {
    fn write(tokens: TokenStream) -> String {
        let mut out = String::new();
        let mut joint = false;
        for token in tokens {
            if !out.is_empty() && !joint {
                out.push(' ');
            }
            joint = false;
            match token {
                TokenTree::Ident(ident) => out.push_str(&ident.to_string()),
                TokenTree::Literal(literal) => out.push_str(&literal.to_string()),
                TokenTree::Punct(punct) => {
                    out.push(punct.as_char());
                    joint = punct.spacing() == Spacing::Joint;
                }
                TokenTree::Group(group) => {
                    let (open, close) = match group.delimiter() {
                        Delimiter::Parenthesis => ("(", ")"),
                        Delimiter::Brace => ("{", "}"),
                        Delimiter::Bracket => ("[", "]"),
                        Delimiter::None => ("", ""),
                    };
                    out.push_str(open);
                    out.push_str(&write(group.stream()));
                    out.push_str(close);
                }
            }
        }
        out
    }
    // Native procedural-macro Display omits spaces which the CLI's fallback
    // Display retains. Canonicalize the token tree, keeping joint operators,
    // rather than using either backend's formatting as manifest identity.
    write(tokens.to_token_stream())
}
