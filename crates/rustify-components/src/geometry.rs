//! External rules for a component's deterministic initial geometry.
//!
//! Provide one collector inside the view's Owner, render the view, then write
//! [`GeometryStyles::css`] into the page's external stylesheet. The same view
//! uses these classes during hydration; browser updates change CSSOM rules.

use leptos::prelude::*;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// The geometry rules collected while a view is rendered.
///
/// A collector belongs to one rendered scope. Its class names come from the
/// same ID allocator as the component's ARIA wiring, so SSR, hydration and a
/// CSR rendering using the same seed give the same rules.
#[derive(Clone, Default)]
pub struct GeometryStyles(Arc<Mutex<BTreeMap<String, String>>>);

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
        for (class, declarations) in rules.iter() {
            css.push('.');
            css.push_str(class);
            css.push('{');
            css.push_str(declarations);
            css.push_str("}\n");
        }
        css
    }

    fn record(&self, class: &str, declarations: String) {
        self.0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(class.to_string(), declarations);
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
    let context = context();
    context.styles.record(&class, untrack(&declarations));
    #[cfg(target_arch = "wasm32")]
    {
        let browser = context.browser;
        let styles = context.styles.clone();
        let cleanup_class = class.clone();
        on_cleanup(move || {
            styles
                .0
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .remove(&cleanup_class);
            browser.try_update_value(|rules| rules.remove(&cleanup_class));
        });
        let class = class.clone();
        Effect::new(move || {
            let declarations = declarations();
            context.styles.record(&class, declarations.clone());
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
    use leptos::web_sys::{CssStyleRule, CssStyleSheet, HtmlElement};

    const SCOPE_ATTRIBUTE: &str = "data-rustify-geometry-scope";
    const SCOPE_SEQUENCE: &str = "__RUSTIFY_GEOMETRY_SCOPE_SEQUENCE";

    struct Scope {
        root: HtmlElement,
        token: String,
        previous: Option<String>,
    }

    impl Scope {
        fn new() -> Option<Self> {
            let root = use_context::<rustify_ui::ScopeRoots>()?.container();
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
            Some(Self {
                root,
                token,
                previous,
            })
        }

        fn selector(&self, class: &str) -> String {
            format!("[{SCOPE_ATTRIBUTE}=\"{}\"] .{class}", self.token)
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
            // A newer collector may have replaced the marker while this
            // Owner was being disposed; do not erase its ownership.
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

    pub(super) struct Rules {
        sheet: CssStyleSheet,
        rules: BTreeMap<String, CssStyleRule>,
        scope: Option<Scope>,
        _abort: Vec<rustify_ui::AbortRelease>,
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
            let scope = Scope::new();
            let mut abort = vec![rustify_ui::release_on_abort(clear)];
            if let Some(scope) = &scope {
                abort.push(rustify_ui::release_on_abort(scope.release()));
            }
            Self {
                sheet,
                rules: BTreeMap::new(),
                scope,
                _abort: abort,
            }
        }

        pub(super) fn update(&mut self, class: &str, declarations: &str) {
            if let Some(rule) = self.rules.get(class) {
                rule.style().set_css_text(declarations);
                return;
            }
            let rules = self
                .sheet
                .css_rules()
                .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
            let index = self
                .sheet
                .insert_rule_with_index(
                    &format!(
                        "{}{{{declarations}}}",
                        self.scope
                            .as_ref()
                            .map_or_else(|| format!(".{class}"), |scope| scope.selector(class))
                    ),
                    rules.length(),
                )
                .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
            if let Some(rule) = rules.item(index) {
                let rule = rule.unchecked_into::<CssStyleRule>();
                self.rules.insert(class.to_string(), rule);
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
                if rules
                    .item(index)
                    .and_then(|rule| rule.dyn_into::<CssStyleRule>().ok())
                    .is_some_and(|rule| rule == removed)
                {
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
            ".rui-geometry-test-geometry-1{width:38.125%;}\n"
        );
    }
}
