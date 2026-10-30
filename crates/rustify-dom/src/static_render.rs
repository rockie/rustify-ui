//! Observes browser-only initial state during a synchronous static render.

use leptos::prelude::*;
use std::sync::{Arc, Mutex};

/// Initial states the browser can draw but a static scope cannot preserve.
/// Provide one collector within each rendering Owner and inspect it after
/// rendering. Closed floating controls and an empty toast region are valid.
#[derive(Clone, Default)]
pub struct StaticRenderIssues(Arc<Mutex<Vec<&'static str>>>);

impl StaticRenderIssues {
    pub fn messages(&self) -> Vec<&'static str> {
        self.0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    /// Records unsupported initial browser state from a component renderer.
    pub fn record(message: &'static str) {
        if let Some(issues) = use_context::<Self>() {
            issues
                .0
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .push(message);
        }
    }
}

/// The collector is inherited by the view's nested static scopes.
pub fn provide_static_render_issues(issues: StaticRenderIssues) {
    provide_context(issues);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executing_a_host_layer_reports_unsupported_initial_floating_content() {
        let owner = Owner::new();
        owner.with(|| {
            let issues = StaticRenderIssues::default();
            provide_static_render_issues(issues.clone());
            let _ = view! {
                <crate::Layer anchor=Signal::derive(|| crate::Anchor::Centred) on_close=|| {}>
                    <p>"Initially open"</p>
                </crate::Layer>
            }
            .to_html();
            assert_eq!(issues.messages().len(), 1);
        });
    }

    #[test]
    fn initial_toasts_are_observed_but_an_empty_region_is_valid() {
        let owner = Owner::new();
        owner.with(|| {
            let issues = StaticRenderIssues::default();
            provide_static_render_issues(issues.clone());
            let toasts = crate::provide_toasts();
            let _ = view! { <crate::ToastRegion /> }.to_html();
            assert!(issues.messages().is_empty());
            toasts.show("Initial toast", Default::default());
            let _ = view! { <crate::ToastRegion /> }.to_html();
            assert_eq!(issues.messages().len(), 1);
        });
    }
}
