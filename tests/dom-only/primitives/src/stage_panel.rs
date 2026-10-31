use leptos::prelude::*;
use rustify_components::primitives::tabs::PanelEntry;
use rustify_components::primitives::{accordion, alert_dialog, collapsible, dialog, popover, select, tabs};

#[component]
pub fn StagePanel() -> impl IntoView {
    let present = RwSignal::new(true);
    let name = RwSignal::new("Launch checklist".to_string());
    let status = RwSignal::new(Some("planned".to_string()));
    let notes = RwSignal::new("Confirm the release owner and review the final checklist.".to_string());
    let detail_notes = RwSignal::new("Review with the delivery team before marking Ready.".to_string());
    let tab = RwSignal::new(Some("summary".to_string()));
    let expanded = RwSignal::new(accordion::AccordionValue::Single(None));
    let references = RwSignal::new(false);
    let editor = RwSignal::new(false);
    let options = RwSignal::new(false);
    let status_open = RwSignal::new(false);
    let confirmation = RwSignal::new(false);
    let draft_name = RwSignal::new(String::new());
    let draft_status = RwSignal::new(None::<String>);
    let feedback = RwSignal::new(String::new());
    let position = Signal::stored(rustify_ui::FloatingPosition {
        align: rustify_ui::Align::Start,
        side_offset: 8.,
        ..Default::default()
    });
    view! {<section class="stage-panel" data-testid="stage-panel" aria-label="Release task">
        <header class="stage-heading">
            <h2>"Release task"</h2>
            <p>"Review the task, update its status, and keep supporting notes together."</p>
        </header>
        <Show when=move ||present.get() fallback=||view! {<p data-testid="stage-empty">"This task has been removed."</p>}>
            <div class="stage-entry">
                <h3 data-testid="stage-name">{move ||name.get()}</h3>
                <p>"Status: "<span data-testid="stage-status">{move ||match status.get().as_deref(){Some("ready")=>"Ready",Some("in-review")=>"In review",_=>"Planned"}}</span></p>
            </div>
        </Show>
        <div class="stage-actions">
            <dialog::Root open=editor on_open_change=move |next| {
                if next {
                    draft_name.set(name.get_untracked());
                    draft_status.set(status.get_untracked());
                }
                options.set(false);
                status_open.set(false);
                editor.set(next);
            }>
                {dialog::trigger_as(leptos::html::button().attr("data-testid","stage-edit").disabled(move ||!present.get()).child("Edit task"))}
                <dialog::Portal>
                    <dialog::Overlay class="stage-dialog-overlay" />
                    <dialog::Content class="stage-dialog-content" test_id="stage-editor">
                        <dialog::Title>"Edit release task"</dialog::Title>
                        <dialog::Description>"Save to update the task. Cancel keeps the current name and status."</dialog::Description>
                        <form class="stage-editor-form" on:submit=move |event| {
                            event.prevent_default();
                            name.set(draft_name.get_untracked());
                            status.set(draft_status.get_untracked());
                            feedback.set("Task changes saved.".into());
                            status_open.set(false);
                            options.set(false);
                            editor.set(false);
                        }>
                            <label for="stage-draft-name">"Task name"</label>
                            <input id="stage-draft-name" data-testid="stage-draft-name" required=true
                                prop:value=move ||draft_name.get() on:input=move |event|draft_name.set(event_target_value(&event)) />
                            <popover::Root open=options on_open_change=move |next:bool| {if !next {status_open.set(false);}options.set(next);}>
                                <popover::Trigger class="stage-options-trigger" test_id="stage-options">"Task options"</popover::Trigger>
                                <popover::Portal>
                                    <popover::Content position=position aria_label="Task options" class="stage-options-content" test_id="stage-options-content">
                                        <h3>"Task options"</h3>
                                        <p>"Choose the status to apply when you save."</p>
                                        <select::Root value=draft_status on_change=move |next|draft_status.set(next)
                                            open=status_open on_open_change=move |next|status_open.set(next)
                                            entries=vec![select::SelectEntry::new("planned","Planned"),select::SelectEntry::new("in-review","In review"),select::SelectEntry::new("ready","Ready")]
                                            aria_label="Draft task status">
                                            <select::Trigger test_id="stage-status-trigger"><select::Value /></select::Trigger>
                                            <select::Content class="stage-select-content" test_id="stage-status-list">
                                                <select::Viewport>
                                                    <select::Item value="planned" test_id="stage-status-planned" />
                                                    <select::Item value="in-review" test_id="stage-status-in-review" />
                                                    <select::Item value="ready" test_id="stage-status-ready" />
                                                </select::Viewport>
                                            </select::Content>
                                        </select::Root>
                                        <popover::Close class="stage-options-close" test_id="stage-options-close">"Done with options"</popover::Close>
                                        <popover::Arrow />
                                    </popover::Content>
                                </popover::Portal>
                            </popover::Root>
                            <div class="stage-actions">
                                <button type="submit" data-testid="stage-save">"Save task"</button>
                                <dialog::Close test_id="stage-cancel">"Cancel"</dialog::Close>
                            </div>
                        </form>
                    </dialog::Content>
                </dialog::Portal>
            </dialog::Root>
            <alert_dialog::Root open=confirmation on_open_change=move |next|confirmation.set(next)>
                {alert_dialog::trigger_as(leptos::html::button().attr("data-testid","stage-delete").disabled(move ||!present.get()).child("Remove task"))}
                <alert_dialog::Portal>
                    <alert_dialog::Overlay class="stage-dialog-overlay" />
                    <alert_dialog::Content class="stage-dialog-content" test_id="stage-confirmation">
                        <alert_dialog::Title>"Remove this task?"</alert_dialog::Title>
                        <alert_dialog::Description>"The task will be removed from this local panel. You can restore the sample task afterward."</alert_dialog::Description>
                        <div class="stage-actions">
                            <alert_dialog::Cancel test_id="stage-keep">"Keep task"</alert_dialog::Cancel>
                            <alert_dialog::Action test_id="stage-confirm-delete" on_action=Callback::new(move |_| {
                                present.set(false);
                                feedback.set("Task removed.".into());
                            })>"Remove task"</alert_dialog::Action>
                        </div>
                    </alert_dialog::Content>
                </alert_dialog::Portal>
            </alert_dialog::Root>
            <button type="button" data-testid="stage-restore" disabled=move ||present.get() on:click=move |_| {
                name.set("Launch checklist".into());
                status.set(Some("planned".into()));
                notes.set("Confirm the release owner and review the final checklist.".into());
                detail_notes.set("Review with the delivery team before marking Ready.".into());
                tab.set(Some("summary".into()));
                expanded.set(accordion::AccordionValue::Single(None));
                references.set(false);
                present.set(true);
                feedback.set("Sample task restored.".into());
            }>"Restore sample task"</button>
        </div>
        <p class="stage-feedback" data-testid="stage-feedback">{move ||feedback.get()}</p>
        <Show when=move ||present.get()>
        <tabs::Root value=tab on_change=move |next|tab.set(next)
            entries=vec![PanelEntry::new("summary"),PanelEntry::new("details")] test_id="stage-tabs">
            <tabs::List aria_label="Task sections">
                <tabs::Trigger value="summary" test_id="stage-summary-tab">"Summary"</tabs::Trigger>
                <tabs::Trigger value="details" test_id="stage-details-tab">"Details"</tabs::Trigger>
            </tabs::List>
            <tabs::Content value="summary" class="stage-tab-content" test_id="stage-summary">
                <label for="stage-notes">"Summary notes"</label>
                <textarea id="stage-notes" data-testid="stage-notes" prop:value=move ||notes.get()
                    on:input=move |event|notes.set(event_target_value(&event)) />
            </tabs::Content>
            <tabs::Content value="details" class="stage-tab-content" test_id="stage-details">
                <label for="stage-detail-notes">"Review notes"</label>
                <textarea id="stage-detail-notes" data-testid="stage-detail-notes" prop:value=move ||detail_notes.get()
                    on:input=move |event|detail_notes.set(event_target_value(&event)) />
                <accordion::Root value=expanded on_change=move |next|expanded.set(next) collapsible=true>
                    <accordion::Item value="checklist">
                        <accordion::Header><accordion::Trigger test_id="stage-checklist">"Delivery checklist"</accordion::Trigger></accordion::Header>
                        <accordion::Content test_id="stage-checklist-content">
                            <ul><li>"Confirm the release owner."</li><li>"Review outstanding work before moving to Ready."</li></ul>
                        </accordion::Content>
                    </accordion::Item>
                    <accordion::Item value="history">
                        <accordion::Header><accordion::Trigger test_id="stage-history">"Change history"</accordion::Trigger></accordion::Header>
                        <accordion::Content test_id="stage-history-content"><p>"The sample starts in Planned. Saved edits update the task above."</p></accordion::Content>
                    </accordion::Item>
                </accordion::Root>
                <collapsible::Root open=references on_open_change=move |next|references.set(next)>
                    <collapsible::Trigger test_id="stage-references">"Reference notes"</collapsible::Trigger>
                    <collapsible::Content test_id="stage-reference-content"><p>"Keep the checklist concise and discuss blocking work with the delivery team before marking Ready."</p></collapsible::Content>
                </collapsible::Root>
            </tabs::Content>
        </tabs::Root>
        </Show>
    </section>}
}
