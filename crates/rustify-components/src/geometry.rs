//! External rules for a component's deterministic initial geometry.
//!
//! Provide one collector inside the view's Owner, render the view, then write
//! [`GeometryStyles::css`] into the page's external stylesheet. The same view
//! uses these classes during hydration; browser updates change CSSOM rules.

use crate::responsive::Responsive;
use leptos::prelude::*;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// A rejected component property, with its author-facing field name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropError {
    pub field: &'static str,
    pub message: String,
}

impl std::fmt::Display for PropError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid prop `{}`: {}", self.field, self.message)
    }
}
impl std::error::Error for PropError {}

/// Validated declarations grouped by minimum viewport width.
///
/// Component rules use the `components` layer and zero-specificity selectors,
/// so caller utility classes retain precedence even after reactive updates.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StyleSet(BTreeMap<u16, BTreeMap<&'static str, String>>);

impl StyleSet {
    /// Adds only explicitly set responsive values. Gaps inherit through CSS.
    pub fn set<T>(
        &mut self,
        property: &'static str,
        value: &Responsive<T>,
        map: impl Fn(&T) -> String,
    ) -> Result<(), PropError> {
        for (width, value) in value.iter() {
            self.insert(width, property, map(value))?;
        }
        Ok(())
    }

    /// Adds one declaration after shared host/browser validation.
    pub fn insert(
        &mut self,
        min_width: u16,
        property: &'static str,
        value: impl Into<String>,
    ) -> Result<(), PropError> {
        let value = value.into();
        validate_value(property, &value)?;
        self.0.entry(min_width).or_default().insert(property, value);
        Ok(())
    }

    pub(crate) fn value_at(&self, width: u16, property: &str) -> Option<&str> {
        self.0
            .range(..=width)
            .rev()
            .find_map(|(_, declarations)| declarations.get(property).map(String::as_str))
    }

    fn rules(self) -> RuleSet {
        let mut previous = BTreeMap::new();
        let mut groups: BTreeMap<u8, BTreeMap<u16, String>> = BTreeMap::new();
        for (width, declarations) in self.0 {
            for (name, value) in declarations {
                if previous.get(name) != Some(&value) {
                    // Specific longhands retain precedence over responsive
                    // shorthands, even when set at a smaller breakpoint.
                    let priority = match name {
                        "grid-column-start" | "grid-column-end" | "grid-row-start"
                        | "grid-row-end" => 2,
                        "grid-column" | "grid-row" | "top" | "right" | "bottom" | "left"
                        | "overflow-x" | "overflow-y" | "margin-top" | "margin-right"
                        | "margin-bottom" | "margin-left" | "padding-top" | "padding-right"
                        | "padding-bottom" | "padding-left" | "column-gap" | "row-gap" => 1,
                        _ => 0,
                    };
                    groups
                        .entry(priority)
                        .or_default()
                        .entry(width)
                        .or_default()
                        .push_str(&format!("{name}:{value};"));
                    previous.insert(name, value);
                }
            }
        }
        RuleSet(
            groups
                .into_values()
                .flat_map(|rules| rules.into_iter())
                .collect(),
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct RuleSet(Vec<(u16, String)>);

impl RuleSet {
    fn plain(declarations: String) -> Self {
        Self(vec![(0, declarations)])
    }

    fn css(&self, selector: &str) -> String {
        if self.0.is_empty() {
            return String::new();
        }
        let mut css = String::from("@layer components{");
        for (width, declarations) in &self.0 {
            if *width != 0 {
                css.push_str(&format!("@media (min-width:{width}px){{"));
            }
            css.push_str(&format!(":where({selector}){{{declarations}}}"));
            if *width != 0 {
                css.push('}');
            }
        }
        css.push_str("}\n");
        css
    }
}

#[derive(Default)]
struct GeometryRecords {
    rules: BTreeMap<String, RuleSet>,
    diagnostics: BTreeMap<String, PropError>,
}

/// The geometry rules collected while a view is rendered.
///
/// A collector belongs to one rendered scope. Its class names come from the
/// same ID allocator as the component's ARIA wiring, so SSR, hydration and a
/// CSR rendering using the same seed give the same rules.
#[derive(Clone, Default)]
pub struct GeometryStyles(Arc<Mutex<GeometryRecords>>);

impl GeometryStyles {
    /// The collected rules, ready to append to an external stylesheet.
    ///
    /// Rules are ordered by class name. This remains readable after the
    /// rendering Owner drops.
    pub fn css(&self) -> String {
        let rules = self
            .0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut css = String::new();
        for (class, declarations) in rules.rules.iter() {
            css.push_str(&declarations.css(&format!(".{class}")));
        }
        css
    }

    /// Current invalid-property diagnostics, ordered by stable element identity.
    /// A valid replacement clears that element's diagnostic. Invalid rules are
    /// omitted so the component's static defaults remain usable.
    pub fn diagnostics(&self) -> Vec<PropError> {
        self.0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .diagnostics
            .values()
            .cloned()
            .collect()
    }

    fn record(&self, class: &str, candidate: Result<RuleSet, PropError>) -> RuleSet {
        let mut records = self
            .0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let declarations = match candidate {
            Ok(declarations) => {
                records.diagnostics.remove(class);
                declarations
            }
            Err(error) => {
                #[cfg(target_arch = "wasm32")]
                if records.diagnostics.get(class) != Some(&error) {
                    rustify_ui::record(
                        rustify_ui::note(
                            rustify_ui::ErrorKind::InvalidContainer,
                            "invalid component CSS property; using component defaults",
                        )
                        .about_field(error.field),
                    );
                }
                records.diagnostics.insert(class.into(), error);
                RuleSet::default()
            }
        };
        records
            .rules
            .insert(class.to_string(), declarations.clone());
        declarations
    }
}

#[derive(Clone)]
struct GeometryContext {
    styles: GeometryStyles,
    #[cfg(target_arch = "wasm32")]
    browser: StoredValue<browser::Rules, LocalStorage>,
}

/// Provides the geometry collector for components below the current Owner.
///
/// In the browser, the Owner also owns one constructed stylesheet. Dropping
/// it removes that stylesheet, and a runtime fatal clears it before static
/// content is restored.
pub fn provide_geometry_styles(styles: GeometryStyles) {
    provide_context(GeometryContext {
        styles,
        #[cfg(target_arch = "wasm32")]
        browser: StoredValue::new_local(browser::Rules::new()),
    });
}

fn context() -> GeometryContext {
    if let Some(context) = use_context::<GeometryContext>() {
        #[cfg(not(target_arch = "wasm32"))]
        return context;
        // A reactive Owner keeps its contexts when it cleans up the arena.
        // Reopening a view must replace the disposed stylesheet handle.
        #[cfg(target_arch = "wasm32")]
        if !context.browser.is_disposed() {
            return context;
        }
    }
    let context = GeometryContext {
        styles: GeometryStyles::default(),
        #[cfg(target_arch = "wasm32")]
        browser: StoredValue::new_local(browser::Rules::new()),
    };
    provide_context(context.clone());
    context
}

/// Establishes the fallback at the component boundary, before its keyed
/// children create their own Owners. Existing CSR consumers need no setup.
pub(crate) fn ensure_context() {
    let _ = context();
}

/// Names one element's rule, recording its initial declarations synchronously.
pub(crate) fn geometry_class(declarations: impl Fn() -> String + Send + Sync + 'static) -> String {
    geometry_class_named(crate::id::next("geometry"), declarations)
}

/// Uses a stable pool-slot identity when nested keyed views can be constructed
/// in a different order by host and browser rendering.
pub(crate) fn geometry_class_named(
    class: String,
    declarations: impl Fn() -> String + Send + Sync + 'static,
) -> String {
    rule_class_named(class, move || Ok(RuleSet::plain(declarations())))
}

/// Collects responsive rules synchronously for SSR and updates CSSOM in CSR.
/// Invalid props leave defaults in place and expose a field-named diagnostic.
pub(crate) fn style_class(
    styles: impl Fn() -> Result<StyleSet, PropError> + Send + Sync + 'static,
) -> String {
    rule_class_named(crate::id::next("geometry"), move || {
        styles().map(StyleSet::rules)
    })
}

fn rule_class_named(
    class: String,
    declarations: impl Fn() -> Result<RuleSet, PropError> + Send + Sync + 'static,
) -> String {
    let context = context();
    context.styles.record(&class, untrack(&declarations));
    #[cfg(target_arch = "wasm32")]
    {
        let browser = context.browser;
        let styles = context.styles.clone();
        let cleanup_class = class.clone();
        on_cleanup(move || {
            let mut records = styles
                .0
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            records.rules.remove(&cleanup_class);
            records.diagnostics.remove(&cleanup_class);
            drop(records);
            browser.try_update_value(|rules| rules.remove(&cleanup_class));
        });
        let class = class.clone();
        Effect::new(move || {
            let declarations = context.styles.record(&class, declarations());
            context
                .browser
                .update_value(|rules| rules.update(&class, &declarations));
        });
    }
    class
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use super::*;
    use leptos::wasm_bindgen::{JsCast, JsValue};
    use leptos::web_sys::{CssRule, CssRuleList, CssStyleRule, CssStyleSheet, HtmlElement};

    const SCOPE_ATTRIBUTE: &str = "data-rustify-geometry-scope";
    const SCOPE_SEQUENCE: &str = "__RUSTIFY_GEOMETRY_SCOPE_SEQUENCE";

    struct Scope {
        root: HtmlElement,
        token: String,
        previous: Option<String>,
    }

    type ScopeHandle = StoredValue<(Scope, rustify_ui::AbortRelease), LocalStorage>;

    impl Scope {
        fn shared_token() -> Option<String> {
            let root = use_context::<rustify_ui::ScopeRoots>()?.container();
            let mut owner = Owner::current()?;
            // Fallback collectors can belong to independent Show Owners. The
            // marker belongs to the highest Owner of this mount root instead.
            while let Some(parent) = owner.parent() {
                let same_root = parent.with(|| {
                    use_context::<rustify_ui::ScopeRoots>()
                        .is_some_and(|roots| roots.container() == root)
                });
                if !same_root {
                    break;
                }
                owner = parent;
            }
            Some(owner.with(|| {
                if let Some(handle) = use_context::<ScopeHandle>().filter(|handle| {
                    handle
                        .try_with_value(|(scope, _)| scope.root == root)
                        .unwrap_or(false)
                }) {
                    return handle.with_value(|(scope, _)| scope.token.clone());
                }
                let scope = Self::new(root);
                let token = scope.token.clone();
                let abort = rustify_ui::release_on_abort(scope.release());
                provide_context(ScopeHandle::new_local((scope, abort)));
                token
            }))
        }

        fn new(root: HtmlElement) -> Self {
            // The page owns this sequence, so different wasm instances also
            // get different selectors while their canonical class IDs stay
            // unchanged. It is a runtime marker, never part of SSR identity.
            let page = js_sys::global();
            let key = JsValue::from_str(SCOPE_SEQUENCE);
            let number = js_sys::Reflect::get(&page, &key)
                .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error))
                .as_f64()
                .unwrap_or(0.0)
                + 1.0;
            js_sys::Reflect::set(&page, &key, &JsValue::from_f64(number))
                .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
            let token = format!("rustify-geometry-{number}");
            let previous = root.get_attribute(SCOPE_ATTRIBUTE);
            root.set_attribute(SCOPE_ATTRIBUTE, &token)
                .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
            Self {
                root,
                token,
                previous,
            }
        }

        fn release(&self) -> js_sys::Function {
            let (method, argument) = match self.previous.as_deref() {
                Some(previous) => ("setAttribute", Some(previous)),
                None => ("removeAttribute", None),
            };
            let method = js_sys::Reflect::get(self.root.as_ref(), &JsValue::from_str(method))
                .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error))
                .unchecked_into::<js_sys::Function>();
            match argument {
                Some(previous) => method
                    .bind2(
                        self.root.as_ref(),
                        &JsValue::from_str(SCOPE_ATTRIBUTE),
                        &JsValue::from_str(previous),
                    )
                    .unchecked_into::<js_sys::Function>(),
                None => method
                    .bind1(self.root.as_ref(), &JsValue::from_str(SCOPE_ATTRIBUTE))
                    .unchecked_into::<js_sys::Function>(),
            }
        }
    }

    impl Drop for Scope {
        fn drop(&mut self) {
            // Restore the pre-mount marker only when this root Owner ends.
            if self.root.get_attribute(SCOPE_ATTRIBUTE).as_deref() == Some(&self.token) {
                match self.previous.as_deref() {
                    Some(previous) => {
                        let _ = self.root.set_attribute(SCOPE_ATTRIBUTE, previous);
                    }
                    None => {
                        let _ = self.root.remove_attribute(SCOPE_ATTRIBUTE);
                    }
                }
            }
        }
    }

    struct Rule {
        outer: CssRule,
        styles: Vec<CssStyleRule>,
        values: RuleSet,
    }

    fn collect_styles(rule: &CssRule, styles: &mut Vec<CssStyleRule>) {
        if let Some(rule) = rule.dyn_ref::<CssStyleRule>() {
            styles.push(rule.clone());
            return;
        }
        // cssRules is shared by layer and media grouping rules. Reading the
        // standard property avoids requiring a browser-specific layer class.
        let children = js_sys::Reflect::get(rule.as_ref(), &JsValue::from_str("cssRules"))
            .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error))
            .unchecked_into::<CssRuleList>();
        for index in 0..children.length() {
            if let Some(rule) = children.item(index) {
                collect_styles(&rule, styles);
            }
        }
    }

    pub(super) struct Rules {
        sheet: CssStyleSheet,
        rules: BTreeMap<String, Rule>,
        scope_token: Option<String>,
        _abort: rustify_ui::AbortRelease,
    }

    impl Rules {
        pub(super) fn new() -> Self {
            let sheet =
                CssStyleSheet::new().unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
            let sheets = document().adopted_style_sheets();
            sheets.push(sheet.as_ref());
            document().set_adopted_style_sheets(sheets.as_ref());
            // A fatal cannot call a closure back into the failed wasm. Bind
            // the native method instead, so it clears overrides on its own.
            let clear = js_sys::Reflect::get(sheet.as_ref(), &JsValue::from_str("replaceSync"))
                .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error))
                .unchecked_into::<js_sys::Function>()
                .bind1(sheet.as_ref(), &JsValue::from_str(""))
                .unchecked_into::<js_sys::Function>();
            Self {
                sheet,
                rules: BTreeMap::new(),
                scope_token: Scope::shared_token(),
                _abort: rustify_ui::release_on_abort(clear),
            }
        }

        pub(super) fn update(&mut self, class: &str, declarations: &RuleSet) {
            if let Some(rule) = self.rules.get_mut(class) {
                if rule.values == *declarations {
                    return;
                }
                if rule.styles.len() == declarations.0.len()
                    && rule
                        .values
                        .0
                        .iter()
                        .map(|(width, _)| width)
                        .eq(declarations.0.iter().map(|(width, _)| width))
                {
                    for ((style, (_, previous)), (_, next)) in
                        rule.styles.iter().zip(&rule.values.0).zip(&declarations.0)
                    {
                        if previous != next {
                            style.style().set_css_text(next);
                        }
                    }
                    rule.values = declarations.clone();
                    return;
                }
            }
            self.remove(class);
            if declarations.0.is_empty() {
                return;
            }
            let rules = self
                .sheet
                .css_rules()
                .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
            let selector = self.scope_token.as_ref().map_or_else(
                || format!(".{class}"),
                |token| format!("[{SCOPE_ATTRIBUTE}=\"{token}\"] .{class},[{SCOPE_ATTRIBUTE}=\"{token}\"].{class}")
            );
            let index = self
                .sheet
                .insert_rule_with_index(declarations.css(&selector).trim(), rules.length())
                .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
            if let Some(rule) = rules.item(index) {
                let mut styles = Vec::new();
                collect_styles(&rule, &mut styles);
                self.rules.insert(
                    class.to_string(),
                    Rule {
                        outer: rule,
                        styles,
                        values: declarations.clone(),
                    },
                );
            }
        }

        pub(super) fn remove(&mut self, class: &str) {
            let Some(removed) = self.rules.remove(class) else {
                return;
            };
            let rules = self
                .sheet
                .css_rules()
                .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
            for index in 0..rules.length() {
                if rules.item(index).is_some_and(|rule| rule == removed.outer) {
                    self.sheet
                        .delete_rule(index)
                        .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
                    break;
                }
            }
        }
    }

    impl Drop for Rules {
        fn drop(&mut self) {
            let current = document().adopted_style_sheets();
            let kept = current.slice(0, 0);
            let ours: &JsValue = self.sheet.as_ref();
            for sheet in current.iter() {
                if &sheet != ours {
                    kept.push(&sheet);
                }
            }
            document().set_adopted_style_sheets(kept.as_ref());
        }
    }
}

#[cfg(all(test, feature = "ssr", not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn invalid_component_css_keeps_the_view_and_exposes_a_prop_diagnostic() {
        let styles = GeometryStyles::default();
        let owner = Owner::new();
        let html = owner.with(|| {
            rustify_ui::id::provide("invalid-prop");
            provide_geometry_styles(styles.clone());
            view! { <crate::layout::Box layout=crate::layout::Layout { width: "12px;display:none".to_string().into(), ..Default::default() }>"Still here"</crate::layout::Box> }.to_html()
        });
        assert!(html.contains("Still here"), "{html}");
        assert!(styles.css().is_empty());
        assert_eq!(styles.diagnostics().len(), 1);
        assert_eq!(styles.diagnostics()[0].field, "width");
        drop(owner);
        assert_eq!(styles.diagnostics()[0].field, "width");
    }

    #[test]
    fn host_geometry_remains_available_after_the_render_owner_drops() {
        let styles = GeometryStyles::default();
        let owner = Owner::new();
        let html = owner.with(|| {
            rustify_ui::id::provide("geometry-test");
            provide_geometry_styles(styles.clone());
            view! { <crate::Progress value=38.125/> }.to_html()
        });
        drop(owner);
        assert!(html.contains("rui-geometry-test-geometry-1"), "{html}");
        assert_eq!(
            styles.css(),
            "@layer components{:where(.rui-geometry-test-geometry-1){width:38.125%;}}\n"
        );
    }
}

#[derive(Clone, Debug)]
enum ValueToken {
    Ident(String),
    Number(f32),
    Length(f32, String),
    Percent(f32),
    String(String),
    Function(String, Vec<ValueToken>),
    Bracket(Vec<ValueToken>),
    Symbol(char),
    Other,
}

fn value_tokens<'i>(
    input: &mut cssparser::Parser<'i, '_>,
) -> Result<Vec<ValueToken>, cssparser::ParseError<'i, ()>> {
    use cssparser::Token;
    let mut tokens = Vec::new();
    while !input.is_exhausted() {
        let token = input.next()?.clone();
        tokens.push(match token {
            Token::Ident(value) => ValueToken::Ident(value.to_ascii_lowercase()),
            Token::Number { value, .. } if value.is_finite() => ValueToken::Number(value),
            Token::Dimension { value, unit, .. } if value.is_finite() => {
                ValueToken::Length(value, unit.to_ascii_lowercase())
            }
            Token::Percentage { unit_value, .. } if unit_value.is_finite() => {
                ValueToken::Percent(unit_value)
            }
            Token::QuotedString(value) => ValueToken::String(value.to_string()),
            Token::Function(name) => {
                let name = name.to_ascii_lowercase();
                if matches!(
                    name.as_str(),
                    "url" | "src" | "expression" | "image" | "image-set" | "-webkit-image-set"
                ) {
                    return Err(input.new_custom_error(()));
                }
                let values = input.parse_nested_block(value_tokens)?;
                ValueToken::Function(name, values)
            }
            Token::SquareBracketBlock => {
                ValueToken::Bracket(input.parse_nested_block(value_tokens)?)
            }
            Token::ParenthesisBlock => {
                ValueToken::Function(String::new(), input.parse_nested_block(value_tokens)?)
            }
            Token::Comma => ValueToken::Symbol(','),
            Token::Delim(value @ ('+' | '-' | '*' | '/')) => ValueToken::Symbol(value),
            Token::Hash(_) | Token::IDHash(_) => ValueToken::Other,
            _ => return Err(input.new_custom_error(())),
        });
    }
    Ok(tokens)
}

fn substitution(token: &ValueToken) -> bool {
    let ValueToken::Function(function, values) = token else {
        return false;
    };
    let Some(ValueToken::Ident(name)) = values.first() else {
        return false;
    };
    let valid_name = match function.as_str() {
        "var" => name.starts_with("--") && name.len() > 2,
        "env" => !name.is_empty(),
        _ => false,
    };
    valid_name && (values.len() == 1 || matches!(values.get(1), Some(ValueToken::Symbol(','))))
}

fn math_expression(tokens: &[ValueToken]) -> bool {
    let mut operand = true;
    for token in tokens {
        if operand {
            if matches!(token, ValueToken::Symbol('+' | '-')) {
                continue;
            }
            if !(length(token, true)
                || matches!(token, ValueToken::Number(_))
                || matches!(token, ValueToken::Ident(value) if matches!(value.as_str(), "e" | "pi" | "infinity" | "-infinity" | "nan"))
                || matches!(token, ValueToken::Function(name, values) if name.is_empty() && math_expression(values)))
            {
                return false;
            }
        } else if !matches!(token, ValueToken::Symbol('+' | '-' | '*' | '/')) {
            return false;
        }
        operand = !operand;
    }
    !operand
}

fn math_function(name: &str, values: &[ValueToken]) -> bool {
    let arguments = values
        .split(|value| matches!(value, ValueToken::Symbol(',')))
        .collect::<Vec<_>>();
    match name {
        "calc" | "abs" | "sign" | "sqrt" | "exp" => arguments.len() == 1 && math_expression(values),
        "min" | "max" | "hypot" => arguments.iter().all(|argument| math_expression(argument)),
        "clamp" => {
            arguments.len() == 3 && arguments.iter().all(|argument| math_expression(argument))
        }
        "mod" | "rem" | "pow" => {
            arguments.len() == 2 && arguments.iter().all(|argument| math_expression(argument))
        }
        "log" => {
            (1..=2).contains(&arguments.len())
                && arguments.iter().all(|argument| math_expression(argument))
        }
        "round" => {
            let arguments = if arguments.first().is_some_and(|argument| matches!(*argument, [ValueToken::Ident(value)] if matches!(value.as_str(), "nearest" | "up" | "down" | "to-zero"))) { &arguments[1..] } else { &arguments[..] };
            (1..=2).contains(&arguments.len())
                && arguments.iter().all(|argument| math_expression(argument))
        }
        _ => false,
    }
}

fn length(token: &ValueToken, signed: bool) -> bool {
    match token {
        ValueToken::Number(value) => *value == 0.0,
        ValueToken::Percent(value) => signed || *value >= 0.0,
        ValueToken::Length(value, unit) => {
            (signed || *value >= 0.0)
                && matches!(
                    unit.as_str(),
                    "px" | "cm"
                        | "mm"
                        | "q"
                        | "in"
                        | "pt"
                        | "pc"
                        | "em"
                        | "rem"
                        | "ex"
                        | "rex"
                        | "cap"
                        | "rcap"
                        | "ch"
                        | "rch"
                        | "ic"
                        | "ric"
                        | "lh"
                        | "rlh"
                        | "vw"
                        | "vh"
                        | "vi"
                        | "vb"
                        | "vmin"
                        | "vmax"
                        | "svw"
                        | "svh"
                        | "svi"
                        | "svb"
                        | "svmin"
                        | "svmax"
                        | "lvw"
                        | "lvh"
                        | "lvi"
                        | "lvb"
                        | "lvmin"
                        | "lvmax"
                        | "dvw"
                        | "dvh"
                        | "dvi"
                        | "dvb"
                        | "dvmin"
                        | "dvmax"
                        | "cqw"
                        | "cqh"
                        | "cqi"
                        | "cqb"
                        | "cqmin"
                        | "cqmax"
                )
        }
        ValueToken::Function(name, values) => substitution(token) || math_function(name, values),
        _ => false,
    }
}

fn identifier(token: &ValueToken, values: &[&str]) -> bool {
    matches!(token, ValueToken::Ident(value) if values.contains(&value.as_str()))
}

fn grid_line(tokens: &[ValueToken]) -> bool {
    if tokens.len() == 1 && (substitution(&tokens[0]) || identifier(&tokens[0], &["auto"])) {
        return true;
    }
    if tokens.is_empty() || tokens.len() > 3 {
        return false;
    }
    let mut span = false;
    let mut integer = false;
    let mut name = false;
    for token in tokens {
        match token {
            ValueToken::Ident(value) if value == "span" && !span => span = true,
            ValueToken::Ident(value)
                if !name
                    && !matches!(
                        value.as_str(),
                        "auto"
                            | "span"
                            | "none"
                            | "inherit"
                            | "initial"
                            | "unset"
                            | "revert"
                            | "revert-layer"
                    ) =>
            {
                name = true
            }
            ValueToken::Number(value) if !integer && *value != 0.0 && value.fract() == 0.0 => {
                integer = true
            }
            _ => return false,
        }
    }
    (!span || integer || name)
        && (!span
            || tokens
                .iter()
                .all(|value| !matches!(value, ValueToken::Number(value) if *value < 0.0)))
}

fn tracks(tokens: &[ValueToken]) -> bool {
    !tokens.is_empty() && tokens.iter().all(|token| match token {
        ValueToken::Ident(value) => matches!(value.as_str(), "none" | "subgrid" | "masonry" | "auto" | "min-content" | "max-content"),
        ValueToken::Length(value, unit) if unit == "fr" => *value >= 0.0,
        ValueToken::Bracket(values) => !values.is_empty() && values.iter().all(|value| matches!(value, ValueToken::Ident(_))),
        ValueToken::Function(name, values) if name == "repeat" => {
            let Some(comma) = values.iter().position(|value| matches!(value, ValueToken::Symbol(','))) else { return false; };
            comma == 1 && matches!(&values[0], ValueToken::Number(number) if *number >= 1.0 && number.fract() == 0.0)
                && tracks(&values[2..]) || comma == 1 && identifier(&values[0], &["auto-fill", "auto-fit"]) && tracks(&values[2..])
        }
        ValueToken::Function(name, values) if name == "minmax" => {
            let Some(comma) = values.iter().position(|value| matches!(value, ValueToken::Symbol(','))) else { return false; };
            comma == 1 && values.len() == 3 && tracks(&values[..1]) && tracks(&values[2..])
        }
        ValueToken::Function(name, values) if name == "fit-content" => values.len() == 1 && length(&values[0], false),
        value => length(value, false),
    })
}

fn grid_areas(tokens: &[ValueToken]) -> bool {
    if tokens.len() == 1 && (identifier(&tokens[0], &["none"]) || substitution(&tokens[0])) {
        return true;
    }
    let mut columns = None;
    let mut bounds: BTreeMap<String, (usize, usize, usize, usize, usize)> = BTreeMap::new();
    for (row_index, token) in tokens.iter().enumerate() {
        let ValueToken::String(row) = token else {
            return false;
        };
        let mut cells = Vec::new();
        let mut characters = row.chars().peekable();
        while let Some(character) = characters.next() {
            if character.is_whitespace() {
                continue;
            }
            if character == '.' {
                while characters.peek() == Some(&'.') {
                    characters.next();
                }
                cells.push(None);
                continue;
            }
            if !(character.is_alphanumeric() || matches!(character, '-' | '_')) {
                return false;
            }
            let mut name = String::from(character);
            while let Some(&character) = characters.peek() {
                if character.is_whitespace() || character == '.' {
                    break;
                }
                if !(character.is_alphanumeric() || matches!(character, '-' | '_')) {
                    return false;
                }
                name.push(character);
                characters.next();
            }
            cells.push(Some(name));
        }
        if cells.is_empty() || columns.is_some_and(|columns| columns != cells.len()) {
            return false;
        }
        columns = Some(cells.len());
        for (column, name) in cells.into_iter().enumerate() {
            if let Some(name) = name {
                let bounds = bounds
                    .entry(name)
                    .or_insert((row_index, row_index, column, column, 0));
                bounds.1 = row_index;
                bounds.2 = bounds.2.min(column);
                bounds.3 = bounds.3.max(column);
                bounds.4 += 1;
            }
        }
    }
    !tokens.is_empty()
        && bounds.values().all(|&(top, bottom, left, right, count)| {
            count == (bottom - top + 1) * (right - left + 1)
        })
}

/// Shared CSS-value validation; this does not ask the browser to parse values.
fn validate_value(field: &'static str, value: &str) -> Result<(), PropError> {
    let invalid = || PropError {
        field,
        message: format!("expected one valid CSS value, received {value:?}"),
    };
    if field.is_empty()
        || !field
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        || value.trim().is_empty()
        || value.contains([';', '{', '}', '@', '!', '\0'])
        || value.contains("/*")
        || value.contains("*/")
    {
        return Err(invalid());
    }
    // CSS tokenization recovers from missing closing delimiters. Author props
    // must be complete so a serialized value cannot consume the next rule.
    let mut delimiters = Vec::new();
    let mut quote = None;
    let mut escaped = false;
    for character in value.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if let Some(current) = quote {
            if character == current {
                quote = None;
            }
            continue;
        }
        match character {
            '\'' | '"' => quote = Some(character),
            '(' => delimiters.push(')'),
            '[' => delimiters.push(']'),
            ')' | ']' if delimiters.pop() != Some(character) => return Err(invalid()),
            _ => {}
        }
    }
    if quote.is_some() || !delimiters.is_empty() || escaped {
        return Err(invalid());
    }
    let mut input = cssparser::ParserInput::new(value);
    let mut parser = cssparser::Parser::new(&mut input);
    let tokens = value_tokens(&mut parser).map_err(|_| invalid())?;
    if tokens.is_empty() {
        return Err(invalid());
    }
    if tokens.len() == 1
        && identifier(
            &tokens[0],
            &["inherit", "initial", "unset", "revert", "revert-layer"],
        )
    {
        return Ok(());
    }
    let valid = match field {
        "flex-grow" | "flex-shrink" => {
            matches!(tokens.as_slice(), [ValueToken::Number(value)] if *value >= 0.0)
        }
        "width" | "min-width" | "height" | "min-height" | "max-width" | "max-height"
        | "flex-basis" => {
            tokens.len() == 1
                && (length(&tokens[0], false)
                    || identifier(
                        &tokens[0],
                        &[
                            "auto",
                            "min-content",
                            "max-content",
                            "fit-content",
                            "stretch",
                            "contain",
                            "content",
                        ],
                    )
                    || field.starts_with("max-") && identifier(&tokens[0], &["none"])
                    || matches!(&tokens[0], ValueToken::Function(name, values) if name == "fit-content" && values.len() == 1 && length(&values[0], false)))
        }
        "padding" | "padding-top" | "padding-right" | "padding-bottom" | "padding-left" | "gap"
        | "row-gap" | "column-gap" => {
            let max = if field == "padding" {
                4
            } else if field == "gap" {
                2
            } else {
                1
            };
            tokens.len() <= max
                && tokens.iter().all(|value| {
                    length(value, false) || field.contains("gap") && identifier(value, &["normal"])
                })
        }
        "margin" | "margin-top" | "margin-right" | "margin-bottom" | "margin-left" | "inset"
        | "top" | "right" | "bottom" | "left" => {
            let max = if matches!(field, "margin" | "inset") {
                4
            } else {
                1
            };
            tokens.len() <= max
                && tokens
                    .iter()
                    .all(|value| length(value, true) || identifier(value, &["auto"]))
        }
        "grid-template-columns" | "grid-template-rows" => tracks(&tokens),
        "grid-area" | "grid-column" | "grid-row" | "grid-column-start" | "grid-column-end"
        | "grid-row-start" | "grid-row-end" => {
            let parts = tokens
                .split(|value| matches!(value, ValueToken::Symbol('/')))
                .collect::<Vec<_>>();
            let max = if field == "grid-area" {
                4
            } else if matches!(field, "grid-column" | "grid-row") {
                2
            } else {
                1
            };
            parts.len() <= max && parts.iter().all(|part| grid_line(part))
        }
        "grid-template-areas" => grid_areas(&tokens),
        _ => true,
    };
    if valid {
        Ok(())
    } else {
        Err(invalid())
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;

    #[test]
    fn declarations_and_escaped_external_resources_are_rejected_at_the_prop() {
        for value in [
            "12px; color:red",
            "12px}body{display:none",
            "url(https://example.test/a)",
            "URL('https://example.test/a')",
            "\\75rl(https://example.test/a)",
            "var(--x, url(a))",
            "@import 'a'",
            "12px!important",
            "calc(100% - 2px",
            "10px/*",
            "foo",
            "12flurbs",
            "12px 24px",
            "calc(1px +)",
            "calc(foo)",
        ] {
            let error = StyleSet::default().insert(0, "width", value).unwrap_err();
            assert_eq!(error.field, "width", "{value}");
        }
    }

    #[test]
    fn legal_geometry_values_are_shared_by_host_and_browser() {
        for (property, value) in [
            ("width", "37px"),
            ("min-width", "min(20rem, 100%)"),
            ("max-width", "clamp(12rem, calc(100% - 2px), 80rem)"),
            ("height", "var(--height, 12dvh)"),
            ("margin", "-2rem auto"),
            ("inset", "0 10% auto -1px"),
            ("padding", "0 1em 2px 3vw"),
            (
                "grid-template-columns",
                "[start] repeat(3, minmax(0, 1fr)) [end]",
            ),
            ("grid-template-rows", "auto minmax(10px, 1fr)"),
            ("grid-area", "header"),
            ("grid-column", "2 / span 3"),
            ("grid-template-areas", "\"head head\" \"side main\""),
            ("flex-grow", "2.5"),
            ("--rustify-component-color-9", "rgb(25 68 180 / .7)"),
            ("--rustify-button-height", "var(--rustify-space-8)"),
        ] {
            assert!(
                StyleSet::default().insert(0, property, value).is_ok(),
                "{property}: {value}"
            );
        }
    }

    #[test]
    fn numerical_and_property_grammars_reject_invalid_values() {
        for (property, value) in [
            ("padding", "-1px"),
            ("gap", "-2rem"),
            ("width", "-3px"),
            ("flex-grow", "-1"),
            ("flex-shrink", "NaN"),
            ("flex-grow", "1px"),
            ("grid-column-start", "0"),
            ("grid-column", "span -2"),
            ("grid-column", "1 / 2 / 3"),
            ("grid-template-columns", "repeat(0, 1fr)"),
            ("grid-template-rows", "abc"),
            ("width", "var(color)"),
        ] {
            assert!(
                StyleSet::default().insert(0, property, value).is_err(),
                "{property}: {value}"
            );
        }
    }

    #[test]
    fn a_valid_replacement_clears_only_its_own_diagnostic() {
        let styles = GeometryStyles::default();
        let error = || PropError {
            field: "width",
            message: "invalid width".into(),
        };
        styles.record("first", Err(error()));
        styles.record("second", Err(error()));
        assert_eq!(styles.diagnostics().len(), 2);
        styles.record("first", Ok(RuleSet::plain("width:37px;".into())));
        assert_eq!(styles.diagnostics().len(), 1);
        assert!(styles.css().contains("width:37px"));
    }

    #[test]
    fn malformed_grid_areas_are_rejected_before_reaching_cssom() {
        for value in ["\"a a\" \"b\"", "\"a a\" \"a b\""] {
            assert!(
                StyleSet::default()
                    .insert(0, "grid-template-areas", value)
                    .is_err(),
                "{value}"
            );
        }
    }

    #[test]
    fn sparse_media_rules_are_deterministic_and_below_utilities() {
        let mut styles = StyleSet::default();
        styles
            .set(
                "width",
                &Responsive {
                    initial: Some("37px"),
                    md: Some("50%"),
                    xl: Some("50%"),
                    ..Responsive::default()
                },
                |value| (*value).into(),
            )
            .unwrap();
        styles.insert(520, "height", "48px").unwrap();
        assert_eq!(styles.rules().css(".example"), "@layer components{:where(.example){width:37px;}@media (min-width:520px){:where(.example){height:48px;}}@media (min-width:1024px){:where(.example){width:50%;}}}\n");
    }
}
