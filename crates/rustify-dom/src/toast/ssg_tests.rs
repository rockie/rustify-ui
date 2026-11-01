use super::*;

#[test]
fn controlled_static_notification_keeps_text_parts_and_native_actions_without_a_timer() {
    let owner = Owner::new();
    let html = owner.with(|| crate::ssr_scope(crate::MountConfig { scope: "static-toast".into(), ..Default::default() }, || {
        provide_toasts();
        view! {
            <Root open=true duration_ms=0 on_open_change=|_| panic!("static rendering must not request close")>
                <Title>"Local draft"</Title><Description>"This content remains readable."</Description>
                <Action alt_text="Open the persistent drafts panel" on_action=|_|{}>"Open drafts"</Action>
                <Close/>
            </Root>
        }
    }).to_html());
    assert!(
        html.contains("role=\"group\"") && html.contains("data-state=\"open\""),
        "{html}"
    );
    assert!(
        !html.contains(" hidden") && !html.contains(" inert"),
        "{html}"
    );
    assert!(
        html.contains("Local draft") && html.contains("This content remains readable."),
        "{html}"
    );
    assert_eq!(html.matches("type=\"button\"").count(), 2, "{html}");
    assert!(
        html.contains("aria-description=\"Open the persistent drafts panel\""),
        "{html}"
    );
    let root = html
        .split('<')
        .find(|tag| tag.contains("data-rustify-toast-root="))
        .unwrap();
    for (attribute, part) in [
        ("aria-labelledby", "toast-title"),
        ("aria-describedby", "toast-description"),
    ] {
        let id = root
            .split(&format!(" {attribute}=\""))
            .nth(1)
            .unwrap()
            .split('"')
            .next()
            .unwrap();
        assert!(
            html.contains(&format!("id=\"{id}\"")) && id.contains(part),
            "{html}"
        );
    }
    assert!(
        !html.contains("aria-live=") && !html.contains(" style="),
        "{html}"
    );
}

#[test]
fn runtime_region_stays_empty_and_reports_accepted_messages_during_ssg() {
    Owner::new().with(|| {
        let issues = crate::StaticRenderIssues::default();
        crate::provide_static_render_issues(issues.clone());
        let handle = provide_toasts();
        handle.show("Runtime message", ToastOptions::default());
        let html = view! { <ToastRegion/> }.to_html();
        assert!(!html.contains("Runtime message"), "{html}");
        assert_eq!(issues.messages().len(), 1);
    });
}

#[test]
fn handle_snapshots_preserve_options_and_replace_atomically_without_another_scheduler() {
    Owner::new().with(|| {
        let handle = provide_toasts();
        let first = handle.show("First", ToastOptions::default());
        let second = handle.show(
            "Needs attention",
            ToastOptions {
                duration_ms: 0,
                importance: ToastImportance::Foreground,
                title: Some("Export".into()),
                action: Some(ToastAction {
                    label: "Open exports".into(),
                    alt_text: "Review saved exports".into(),
                    on_action: Callback::new(|_| {}),
                }),
                tone: ToastTone::Warning,
            },
        );
        let items = handle.items();
        assert_eq!(items.len(), 1);
        assert!(second > first);
        assert_eq!(items[0].seq, second);
        assert_eq!(items[0].title.as_deref(), Some("Export"));
        assert_eq!(items[0].importance, ToastImportance::Foreground);
        assert_eq!(items[0].duration_ms, 0);
        assert_eq!(
            items[0]
                .action
                .as_ref()
                .map(|action| action.alt_text.as_str()),
            Some("Review saved exports")
        );
        handle.dismiss(first);
        assert_eq!(handle.current().map(|toast| toast.seq), Some(second));
        handle.dismiss(second);
        assert!(handle.items().is_empty());
        assert_eq!(
            handle.runtime.model.with_untracked(Scheduler::next_due),
            None,
            "the handle does not start a second timer before a Root is visible"
        );
    });
}

#[test]
fn invalid_initial_fields_use_valid_defaults_and_keep_unrelated_configuration() {
    Owner::new().with(|| {
        let handle = provide_toasts_with(ToastConfig {
            visible_limit: 0,
            swipe_threshold: f64::INFINITY,
            policy: ToastQueuePolicy::Fifo,
            viewport_label: "Updates".into(),
            ..Default::default()
        });
        assert_eq!(
            handle
                .diagnostics()
                .iter()
                .map(|issue| issue.field)
                .collect::<Vec<_>>(),
            ["swipe_threshold", "visible_limit"]
        );
        handle.show(
            "Retained",
            ToastOptions {
                duration_ms: u32::MAX,
                ..Default::default()
            },
        );
        assert_eq!(
            handle.current().map(|toast| toast.duration_ms),
            Some(DEFAULT_DURATION_MS)
        );
        handle.show("Second", ToastOptions::default());
        assert_eq!(handle.items().len(), 2);
        assert_eq!(
            handle
                .runtime
                .model
                .with_untracked(|model| model.config.viewport_label.clone()),
            "Updates"
        );
    });
}
