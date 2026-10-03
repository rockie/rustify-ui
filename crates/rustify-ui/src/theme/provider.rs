use super::{ResolvedTheme, ThemeValuePatch};
use crate::mount::ScopeRoots;
use leptos::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Clone, Copy)]
struct EffectiveTheme(Signal<Arc<ResolvedTheme>>);

/// The immutable snapshot in force at this position, including local patches.
pub fn use_resolved_theme() -> Option<Signal<Arc<ResolvedTheme>>> {
    use_context::<EffectiveTheme>().map(|theme| theme.0)
}

/// Claims the scope root and restores the values it replaced when disposed.
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
    let style = root.style();
    let previous: Vec<_> = properties
        .into_iter()
        .map(|name| {
            let value = style.get_property_value(&name).unwrap_or_default();
            let priority = style.get_property_priority(&name);
            (name, value, priority)
        })
        .collect();
    let attributes: Vec<_> = ["data-theme", "data-rustify-theme-runtime"]
        .into_iter()
        .map(|name| (name, root.get_attribute(name)))
        .collect();
    let _ = root.set_attribute("data-rustify-theme-runtime", runtime);
    let roots = roots.clone();
    on_cleanup(move || {
        let root = roots.container();
        if root.get_attribute("data-rustify-theme-owner").as_deref() != Some(&claim) {
            return;
        }
        let style = root.style();
        for (name, value, priority) in previous {
            if value.is_empty() {
                let _ = style.remove_property(&name);
            } else {
                let _ = style.set_property_with_priority(&name, &value, &priority);
            }
        }
        for (name, value) in attributes {
            if let Some(value) = value {
                let _ = root.set_attribute(name, &value);
            } else {
                let _ = root.remove_attribute(name);
            }
        }
        let _ = root.remove_attribute("data-rustify-theme-owner");
    });
    true
}

/// Writes the complete DOM projection of an application's resolved snapshot.
/// Load `theme-v4.css` in the application's Tailwind build to select the new
/// geometry, typography and shadow semantics.
#[component]
pub fn ThemeScope(#[prop(into)] resolved: Signal<Arc<ResolvedTheme>>) -> impl IntoView {
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

/// Applies a sparse, validated patch and recomputes dependent runtime values.
/// An empty patch removes the local projection and resumes DOM inheritance.
/// Invalid patches leave the outer theme in force and record a diagnostic.
#[component]
pub fn ThemeBoundary(
    #[prop(into)] patch: Signal<ThemeValuePatch>,
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
        if patch.0.is_empty() {
            Ok(outer)
        } else {
            outer.patched(&patch).map(Arc::new)
        }
    });
    let effective = Signal::derive(move || candidate.get().unwrap_or_else(|_| outer.get()));
    provide_context(EffectiveTheme(effective));
    let node = NodeRef::<leptos::html::Div>::new();
    Effect::new(move || {
        let patch = patch.get();
        let result = candidate.get();
        let Some(node) = node.get() else { return };
        let element: leptos::web_sys::HtmlElement = node.into();
        match result {
            Ok(snapshot) if !patch.0.is_empty() => write_snapshot(&element, &snapshot, false),
            result => {
                for (name, _) in outer.get().properties() {
                    let _ = element.style().remove_property(&name);
                }
                let _ = element.remove_attribute("data-rustify-theme-runtime");
                if result.is_err() {
                    crate::record(crate::note(
                        crate::ErrorKind::InvalidContainer,
                        "invalid theme patch; validate it with ResolvedTheme::patched before publishing",
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
