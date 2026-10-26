use super::ResolvedTheme;
#[cfg(target_arch = "wasm32")]
use super::ThemeBoundaryPatch;
#[cfg(any(target_arch = "wasm32", test))]
use super::ThemeMode;
#[cfg(target_arch = "wasm32")]
use crate::mount::ScopeRoots;
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Clone, Copy)]
struct EffectiveTheme(Signal<Arc<ResolvedTheme>>);

/// The immutable snapshot in force at this position, including local patches.
pub fn use_resolved_theme() -> Option<Signal<Arc<ResolvedTheme>>> {
    use_context::<EffectiveTheme>().map(|theme| theme.0)
}

/// Provides a snapshot whose CSS was already generated with the page.
/// Unlike ThemeScope, this binding never rewrites the scope's DOM or styles.
pub fn provide_static_theme(resolved: impl Into<Signal<Arc<ResolvedTheme>>>) {
    provide_context(EffectiveTheme(resolved.into()));
}

#[cfg(any(target_arch = "wasm32", test))]
fn provider_snapshot(
    snapshot: Arc<ResolvedTheme>,
    preview: Option<ThemeMode>,
) -> Arc<ResolvedTheme> {
    let mode = preview.unwrap_or_else(|| snapshot.config.selected.appearance.mode(snapshot.mode));
    if snapshot.mode == mode {
        return snapshot;
    }
    match snapshot.in_mode(mode) {
        Ok(resolved) => Arc::new(resolved),
        Err(_) => {
            #[cfg(target_arch = "wasm32")]
            crate::record(crate::note(
                crate::ErrorKind::InvalidContainer,
                "invalid theme appearance; validate both source modes before publishing",
            ));
            snapshot
        }
    }
}

/// Claims the scope root and restores the values it replaced when disposed.
#[cfg(target_arch = "wasm32")]
pub(crate) fn claim_root(
    roots: Option<&ScopeRoots>,
    properties: impl IntoIterator<Item = String>,
    runtime: &str,
) -> bool {
    let Some(roots) = roots else { return false };
    let root = roots.container();
    if root.has_attribute("data-rustify-theme-owner") {
        crate::record(crate::note(
            crate::ErrorKind::OccupiedContainer,
            "the scope already has a theme provider",
        ));
        return false;
    }
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let claim = NEXT.fetch_add(1, Ordering::Relaxed).to_string();
    let _ = root.set_attribute("data-rustify-theme-owner", &claim);
    use leptos::wasm_bindgen::prelude::*;
    #[wasm_bindgen(module = "/web/theme.js")]
    extern "C" {
        #[wasm_bindgen(js_name = captureTheme)]
        fn capture_theme(
            root: &leptos::web_sys::HtmlElement,
            owner: &str,
            properties: &js_sys::Array,
        ) -> js_sys::Function;
    }
    let names = properties
        .into_iter()
        .map(JsValue::from)
        .collect::<js_sys::Array>();
    let release = capture_theme(&root, &claim, &names);
    let abort = crate::release_on_abort(release.clone());
    let _ = root.set_attribute("data-rustify-theme-runtime", runtime);
    let release = send_wrapper::SendWrapper::new((release, abort));
    on_cleanup(move || {
        let _ = release.0.call0(&JsValue::UNDEFINED);
        drop(release);
    });
    true
}

/// Writes the complete DOM projection of an application's resolved snapshot.
/// Document appearance selects the root mode unless `preview_mode` overrides it.
/// Load `theme-v4.css` in the application's Tailwind build to select the new
/// geometry, typography and shadow semantics.
#[cfg(target_arch = "wasm32")]
#[component]
pub fn ThemeScope(
    #[prop(into)] resolved: Signal<Arc<ResolvedTheme>>,
    /// An explicit preview choice takes precedence over the root document's appearance.
    #[prop(optional, into)]
    preview_mode: Option<Signal<ThemeMode>>,
) -> impl IntoView {
    let effective = Memo::new(move |_| {
        let snapshot = resolved.get();
        provider_snapshot(snapshot, preview_mode.map(|mode| mode.get()))
    });
    let resolved = Signal::derive(move || effective.get());
    let roots = use_context::<ScopeRoots>();
    let names = resolved
        .get_untracked()
        .properties()
        .into_iter()
        .map(|(name, _)| name);
    if claim_root(roots.as_ref(), names, "v4") {
        provide_context(EffectiveTheme(resolved));
        Effect::new(move || {
            let snapshot = resolved.get();
            if let Some(roots) = &roots {
                write_snapshot(&roots.container(), &snapshot, true);
            }
        });
    }
    ()
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn write_snapshot(
    element: &leptos::web_sys::HtmlElement,
    snapshot: &ResolvedTheme,
    mode: bool,
) {
    for (name, value) in snapshot.properties() {
        let _ = element.style().set_property(&name, &value);
    }
    let _ = element.set_attribute("data-rustify-theme-runtime", "v4");
    if mode {
        let _ = element.set_attribute("data-theme", snapshot.mode.as_str());
    }
}

/// Applies sparse two-mode tokens and configuration using the original author source.
/// An empty patch removes the local projection and resumes DOM inheritance.
/// Invalid patches leave the outer theme in force and record a diagnostic.
#[cfg(target_arch = "wasm32")]
#[component]
pub fn ThemeBoundary(
    #[prop(into)] patch: Signal<ThemeBoundaryPatch>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    // Context belongs to the boundary subtree; sibling previews keep their outer theme.
    let owner = Owner::new();
    let view = owner.with(|| {
    let outer = use_resolved_theme().expect("ThemeBoundary requires a ThemeScope");
    let candidate = Memo::new(move |_| {
        let outer = outer.get();
        let patch = patch.get();
        if patch.is_empty() {
            Ok(outer)
        } else {
            outer.with_boundary(&patch).map(Arc::new)
        }
    });
    let effective = Signal::derive(move || candidate.get().unwrap_or_else(|_| outer.get()));
    provide_context(EffectiveTheme(effective));
    let node = NodeRef::<leptos::html::Div>::new();
    let projected = StoredValue::new(Vec::<String>::new());
    Effect::new(move || {
        let patch = patch.get();
        let result = candidate.get();
        let Some(node) = node.get() else { return };
        let element: leptos::web_sys::HtmlElement = node.into();
        match result {
            Ok(snapshot) if !patch.is_empty() => {
                let names: Vec<_> = snapshot.properties().into_iter().map(|(name, _)| name).collect();
                projected.with_value(|previous| {
                    for name in previous.iter().filter(|name| !names.contains(name)) {
                        let _ = element.style().remove_property(name);
                    }
                });
                write_snapshot(&element, &snapshot, true);
                projected.set_value(names);
            }
            result => {
                projected.update_value(|previous| {
                    for name in previous.drain(..) {
                        let _ = element.style().remove_property(&name);
                    }
                });
                let _ = element.remove_attribute("data-theme");
                let _ = element.remove_attribute("data-rustify-theme-runtime");
                if result.is_err() {
                    crate::record(crate::note(
                        crate::ErrorKind::InvalidContainer,
                        "invalid theme patch; validate it with ResolvedTheme::with_boundary before publishing",
                    ));
                }
            }
        }
    });
    view! {
        <div node_ref=node class=class data-testid=test_id data-rustify-theme-boundary="">
            {children()}
        </div>
    }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{
        Appearance, ConfigChange, ResolveContext, ThemeBoundaryPatch, ThemeConfigPatch,
        ThemeDocument,
    };

    #[test]
    fn root_appearance_preview_and_nested_override_keep_the_same_two_mode_source() {
        let mut document = ThemeDocument::standard("scope", "Scope");
        document.config.appearance = Appearance::Dark;
        document
            .base
            .dark
            .0
            .insert("font-size".into(), "20px".into());
        let source = Arc::new(
            crate::theme::resolve(&document, ThemeMode::Light, &ResolveContext::default()).unwrap(),
        );
        let root = provider_snapshot(source.clone(), None);
        assert_eq!(root.mode, ThemeMode::Dark);
        assert_eq!(root.font_size_px, 20.);
        assert!(Arc::ptr_eq(&root.source, &source.source));
        let preview = provider_snapshot(source.clone(), Some(ThemeMode::Light));
        assert!(Arc::ptr_eq(&preview, &source));
        let nested = preview
            .with_boundary(&ThemeBoundaryPatch {
                config: ThemeConfigPatch {
                    appearance: ConfigChange::Set(Appearance::Dark),
                    ..Default::default()
                },
                ..Default::default()
            })
            .unwrap();
        assert_eq!(nested.mode, ThemeMode::Dark);
        assert_eq!(nested.font_size_px, 20.);
        assert_eq!(nested.source.document(), &document);
    }

    #[test]
    fn static_binding_keeps_the_source_for_an_opposite_mode_boundary() {
        let owner = Owner::new();
        owner.with(|| {
            let document = ThemeDocument::standard("static", "Static");
            let root = Arc::new(
                crate::theme::resolve(&document, ThemeMode::Light, &ResolveContext::default())
                    .unwrap(),
            );
            provide_static_theme(root.clone());
            let signal = use_resolved_theme().unwrap();
            assert!(Arc::ptr_eq(&signal.get_untracked(), &root));
            let nested = signal
                .get_untracked()
                .with_boundary(&ThemeBoundaryPatch {
                    config: ThemeConfigPatch {
                        appearance: ConfigChange::Set(Appearance::Dark),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .unwrap();
            assert_eq!(nested.source.document(), &document);
            assert_eq!(nested.mode, ThemeMode::Dark);
            assert_ne!(nested.colors["background"], root.colors["background"]);
        });
    }
}
