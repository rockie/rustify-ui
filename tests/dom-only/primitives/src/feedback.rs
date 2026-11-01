use leptos::prelude::*;
use rustify_components::primitives::number_field;
use rustify_components::primitives::{
    dialog, popover, progress, select, skeleton, spinner, toast as notice,
};
use rustify_ui::toast::{
    SwipeDirection, ToastConfig, ToastImportance, ToastQueuePolicy, ToastSwipe, ToastTone,
};
use rustify_ui::{Cancelable, Locale};

#[component]
pub fn Feedback() -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        let theme = rustify_ui::resolve(&rustify_ui::ThemeDocument::standard("feedback-fixture", "Feedback fixture"), rustify_ui::ThemeMode::Light, &rustify_ui::ResolveContext::default()).expect("standard fixture theme is valid");
        rustify_ui::provide_static_theme(std::sync::Arc::new(theme));
        let locale = rustify_ui::provide_locale(Locale::English);
        let dark = RwSignal::new(false);
        let mode = RwSignal::new(0_u8);
        let patch = Signal::derive(move || {
            let mut patch = rustify_ui::ThemeBoundaryPatch::default();
            patch.config.appearance = rustify_ui::ConfigChange::Set(if dark.get() { rustify_ui::Appearance::Dark } else { rustify_ui::Appearance::Light });
            patch
        });
        view! {
            <section class="feedback-samples feedback-raw" data-testid="feedback-samples" aria-label="Release feedback">
                <link rel="stylesheet" href="feedback.css" />
                <h2>"Release feedback"</h2>
                <p>"Review sync progress and keep saved-item notices available while you work."</p>
                <details data-testid="feedback-tools"><summary>"Feedback states and appearance"</summary>
                    <div class="feedback-actions">
                        <button type="button" data-testid="feedback-theme" on:click=move |_| dark.update(|value| *value = !*value)>"Change feedback appearance"</button>
                        <button type="button" data-testid="feedback-locale" on:click=move |_| locale.set(if locale.get_untracked() == Locale::English { Locale::Chinese } else { Locale::English })>"Change feedback language"</button>
                    </div>
                </details>
                <rustify_ui::ThemeBoundary patch=patch test_id="feedback-theme-boundary">
                    <LoadingExamples />

                    <section class="feedback-group" aria-label="Saved-item notices">
                        <h3>"Saved-item notices"</h3>
                        <div class="feedback-actions" role="group" aria-label="Notice examples">
                            <button type="button" data-testid="feedback-mode-queue" on:click=move |_| mode.set(0)>"Queued updates"</button>
                            <button type="button" data-testid="feedback-mode-replace" on:click=move |_| mode.set(1)>"Latest update"</button>
                            <button type="button" data-testid="feedback-mode-layers" on:click=move |_| mode.set(2)>"Review from a notice"</button>
                        </div>
                        <Show when=move || mode.get() == 0><QueueExample policy=ToastQueuePolicy::Fifo limit=2 /></Show>
                        <Show when=move || mode.get() == 1><QueueExample policy=ToastQueuePolicy::Replace limit=1 /></Show>
                        <Show when=move || mode.get() == 2><MigrationExample /></Show>
                    </section>
                </rustify_ui::ThemeBoundary>
                <button type="button" data-testid="feedback-outside">"Continue reviewing the release"</button>
            </section>
        }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

#[component]
fn LoadingExamples() -> impl IntoView {
    let loading = RwSignal::new(false);
    let value = RwSignal::new(Some(37.5));
    let max = RwSignal::new(150.);
    let spinner_draft = RwSignal::new("Confirm the release owner".to_string());
    let skeleton_draft = RwSignal::new("Keep this local note".to_string());
    let clicks = RwSignal::new(0_u32);
    let label = Callback::new(|state: progress::ProgressValue| match state.value {
        Some(value) => format!("{value} of {} review items", state.max),
        None => "Waiting for the next review batch".into(),
    });
    let spinner_target = leptos::html::div().class("feedback-retained").attr("data-testid", "feedback-spinner-native").child(view! {
        <label for="feedback-spinner-draft">"Release owner note"</label>
        <input id="feedback-spinner-draft" data-testid="feedback-spinner-draft" prop:value=move || spinner_draft.get()
            on:input=move |event| spinner_draft.set(event_target_value(&event))
            on:keydown=move |event| { if event.key() == "F2" { event.prevent_default(); loading.set(true); } } />
        <button type="button" data-testid="feedback-retained-action" on:click=move |_| clicks.update(|value| *value += 1)>"Check owner note"</button>
    });
    let skeleton_target = leptos::html::article().class("feedback-retained").attr("data-testid", "feedback-skeleton-native").child(view! {
        <label for="feedback-skeleton-draft">"Delivery note"</label>
        <input id="feedback-skeleton-draft" data-testid="feedback-skeleton-draft" prop:value=move || skeleton_draft.get()
            on:input=move |event| skeleton_draft.set(event_target_value(&event)) />
    });
    view! {
        <section class="feedback-group" data-testid="feedback-loading" aria-label="Review sync">
            <h3>"Review sync"</h3>
            <p>"Review items synchronized: "<output data-testid="feedback-progress-units">{move || value.get().map_or_else(|| "Waiting for a total".into(), |value| format!("{value} / {}", max.get()))}</output></p>
            <div class="feedback-meter-frame">
                <progress::Root value=value max=max value_label=Some(label) aria_label="Review items synchronized" class="feedback-meter" test_id="feedback-progress">
                    <progress::Indicator test_id="feedback-progress-indicator" />
                </progress::Root>
            </div>
            <div class="feedback-actions">
                <button type="button" data-testid="feedback-loading-start" on:click=move |_| loading.set(true)>"Refresh review notes"</button>
                <button type="button" data-testid="feedback-loading-finish" on:click=move |_| loading.set(false)>"Finish refreshing"</button>
            </div>
            <div class="feedback-retained-grid">
                <div class="feedback-spinner-frame" data-testid="feedback-spinner-frame">{spinner::spinner_as(spinner_target, loading, Signal::stored(String::new()))}</div>
                <div>{skeleton::skeleton_as(skeleton_target, loading)}</div>
            </div>
            <p>"Owner-note checks: "<output data-testid="feedback-retained-clicks">{move || clicks.get()}</output></p>
            <details data-testid="feedback-progress-tools"><summary>"Sync states"</summary>
                <p>"F2 refreshes the notes while the release-owner draft is focused."</p>
                <div class="feedback-actions">
                    <button type="button" data-testid="feedback-progress-known" on:click=move |_| { max.set(150.); value.set(Some(37.5)); }>"Show the first batch"</button>
                    <button type="button" data-testid="feedback-progress-unknown" on:click=move |_| value.set(None)>"Wait for the next batch"</button>
                    <button type="button" data-testid="feedback-progress-complete" on:click=move |_| { max.set(150.); value.set(Some(150.)); }>"Finish all review items"</button>
                    <button type="button" data-testid="feedback-progress-bad-max" on:click=move |_| max.set(0.)>"Inspect an invalid total"</button>
                    <button type="button" data-testid="feedback-progress-bad-value" on:click=move |_| value.set(Some(f64::NAN))>"Inspect an invalid item count"</button>
                </div>
            </details>
        </section>
    }
}

#[derive(Clone, Copy)]
struct NoticeEntry {
    open: RwSignal<bool>,
    requests: RwSignal<u32>,
    pauses: RwSignal<u32>,
    resumes: RwSignal<u32>,
}
fn swipe_trace(
    trace: RwSignal<Vec<String>>,
    cancel: RwSignal<String>,
    phase: &'static str,
) -> Callback<Cancelable<ToastSwipe>> {
    Callback::new(move |event: Cancelable<ToastSwipe>| {
        trace.update(|events| {
            if events.len() == 24 {
                events.remove(0);
            }
            events.push(phase.into());
        });
        if cancel.get_untracked() == phase {
            event.prevent_default();
        }
    })
}

#[component]
fn QueueExample(policy: ToastQueuePolicy, limit: usize) -> impl IntoView {
    let direction = RwSignal::new(SwipeDirection::Right);
    view! {
        <details data-testid="feedback-swipe-config"><summary>"Notice swipe direction"</summary>
            <label>"Start a notice session with swipe direction"
                <select data-testid="feedback-swipe-direction" on:change=move |event| direction.set(match event_target_value(&event).as_str() {
                    "up" => SwipeDirection::Up, "down" => SwipeDirection::Down, "left" => SwipeDirection::Left, _ => SwipeDirection::Right,
                })>
                    <option value="right">"Right"</option><option value="left">"Left"</option><option value="up">"Up"</option><option value="down">"Down"</option>
                </select>
            </label>
        </details>
        <For each=move || vec![direction.get()] key=|direction| direction.as_str() let:direction>
            <notice::Provider config=ToastConfig { policy, visible_limit: limit, swipe_direction: direction, ..Default::default() }><NoticeBatch policy=policy /></notice::Provider>
        </For>
    }
}

#[component]
fn NoticeBatch(policy: ToastQueuePolicy) -> impl IntoView {
    let entries: [NoticeEntry; 4] = std::array::from_fn(|_| NoticeEntry {
        open: RwSignal::new(false),
        requests: RwSignal::new(0),
        pauses: RwSignal::new(0),
        resumes: RwSignal::new(0),
    });
    let duration = RwSignal::new(1800_u32);
    let refuse = RwSignal::new(false);
    let cancel_action = RwSignal::new(false);
    let cancel_escape = RwSignal::new(false);
    let cancel_swipe = RwSignal::new(String::new());
    let actions = RwSignal::new(0_u32);
    let applied = RwSignal::new(0_u32);
    let trace = RwSignal::new(Vec::<String>::new());
    let titles = [
        "Summary saved",
        "Attachment ready",
        "Review needs attention",
        "Pinned delivery reminder",
    ];
    let descriptions = [
        "The local release summary is ready to review.",
        "The checklist attachment is ready to open.",
        "Review the remaining delivery note before continuing.",
        "The owner reminder stays until you dismiss it.",
    ];
    view! {
        <section class="feedback-notice-workflow" data-testid="feedback-notice-workflow" data-policy=if policy == ToastQueuePolicy::Fifo { "fifo" } else { "replace" }>
            <p>{if policy == ToastQueuePolicy::Fifo { "Up to two queued updates appear together. Waiting updates keep their full reading time." } else { "The latest update takes the visible slot after the previous notice accepts dismissal." }}</p>
            <div class="feedback-actions">
                {(0..4).map(move |index| view! {
                    <button type="button" data-testid=format!("feedback-notice-show-{index}") on:click=move |_| entries[index].open.set(true)>{titles[index]}</button>
                }).collect_view()}
                <button type="button" data-testid="feedback-notices-clear" on:click=move |_| { for entry in entries { entry.open.set(false); } }>"Clear notices"</button>
                <button type="button" data-testid="feedback-review-persistent" on:click=move |_| applied.update(|count| *count += 1)>"Mark current summary reviewed"</button>
            </div>
            <p>"Summaries marked reviewed locally: "<output data-testid="feedback-action-result">{move || applied.get()}</output></p>
            <details data-testid="feedback-notice-tools"><summary>"Notice policy and cancellation states"</summary>
                <label>"Reading time in milliseconds"<input type="number" min="800" max="5000" step="1" data-testid="feedback-notice-duration" prop:value=move || duration.get().to_string()
                    on:input=move |event| { if let Ok(value) = event_target_value(&event).parse::<u32>() { if (800..=5000).contains(&value) { duration.set(value); } } } /></label>
                <label><input type="checkbox" data-testid="feedback-notice-refuse" prop:checked=move || refuse.get() on:change=move |event| refuse.set(event_target_checked(&event)) />"Keep notices when dismissal is requested"</label>
                <label><input type="checkbox" data-testid="feedback-action-cancel" prop:checked=move || cancel_action.get() on:change=move |event| cancel_action.set(event_target_checked(&event)) />"Cancel the notice action"</label>
                <label><input type="checkbox" data-testid="feedback-escape-cancel" prop:checked=move || cancel_escape.get() on:change=move |event| cancel_escape.set(event_target_checked(&event)) />"Cancel notice Escape"</label>
                <label>"Cancel swipe at"<select data-testid="feedback-swipe-cancel" on:change=move |event| cancel_swipe.set(event_target_value(&event))>
                    <option value="">"No cancellation"</option><option value="start">"Start"</option><option value="move">"Move"</option><option value="end">"End"</option>
                </select></label>
                <button type="button" data-testid="feedback-accept-first-close" on:click=move |_| { refuse.set(false); entries[0].open.set(false); }>"Accept the first pending dismissal"</button>
                <p>"Action requests: "<output data-testid="feedback-action-requests">{move || actions.get()}</output></p>
                <p>"Dismiss requests: "<output data-testid="feedback-close-requests">{move || entries.map(|entry| entry.requests.get().to_string()).join(",")}</output></p>
                <p>"Pause transitions: "<output data-testid="feedback-pause-counts">{move || entries.map(|entry| entry.pauses.get().to_string()).join(",")}</output></p>
                <p>"Resume transitions: "<output data-testid="feedback-resume-counts">{move || entries.map(|entry| entry.resumes.get().to_string()).join(",")}</output></p>
                <p>"Swipe events: "<output data-testid="feedback-swipe-events">{move || trace.get().join(" ")}</output></p>
            </details>
            <notice::Viewport class="feedback-toast-viewport" test_id="feedback-toast-viewport">
                {(0..4).map(move |index| {
                    let entry = entries[index];
                    let tone = [ToastTone::Success, ToastTone::Warning, ToastTone::Error, ToastTone::Neutral][index];
                    view! {
                        <notice::Root open=entry.open on_open_change=move |next: bool| { if !next { entry.requests.update(|count| *count += 1); } if !refuse.get_untracked() { entry.open.set(next); } }
                            duration_ms=Signal::derive(move || if index == 3 { 0 } else { duration.get() })
                            importance=if index == 2 { ToastImportance::Foreground } else { ToastImportance::Background }
                             class="feedback-notice feedback-raw" test_id=format!("feedback-toast-{index}") attr:data-sample-tone=tone.key()
                            on_pause=Some(Callback::new(move |_| entry.pauses.update(|count| *count += 1)))
                            on_resume=Some(Callback::new(move |_| entry.resumes.update(|count| *count += 1)))
                            on_escape_key_down=Some(Callback::new(move |event: Cancelable<rustify_ui::LayerDismiss>| { if cancel_escape.get_untracked() { event.prevent_default(); } }))
                            on_swipe_start=Some(swipe_trace(trace, cancel_swipe, "start")) on_swipe_move=Some(swipe_trace(trace, cancel_swipe, "move"))
                            on_swipe_cancel=Some(swipe_trace(trace, cancel_swipe, "cancel")) on_swipe_end=Some(swipe_trace(trace, cancel_swipe, "end"))>
                            <notice::Title attr:data-testid=format!("feedback-toast-title-{index}")>{titles[index]}</notice::Title>
                            <notice::Description>{descriptions[index]}</notice::Description>
                            <notice::Action alt_text="Mark the saved release summary reviewed; the same action remains in the release panel" attr:data-testid=format!("feedback-toast-action-{index}")
                                on_action={move |event: Cancelable<()>| { actions.update(|count| *count += 1); if cancel_action.get_untracked() { event.prevent_default(); } else { applied.update(|count| *count += 1); } }}>"Mark reviewed"</notice::Action>
                            <notice::Close test_id=format!("feedback-toast-close-{index}") />
                        </notice::Root>
                    }
                }).collect_view()}
            </notice::Viewport>
        </section>
    }
}

#[component]
fn MigrationExample() -> impl IntoView {
    view! { <notice::Provider config=ToastConfig { policy: ToastQueuePolicy::Fifo, visible_limit: 2, ..Default::default() }><MigrationBody /></notice::Provider> }
}

#[component]
fn MigrationBody() -> impl IntoView {
    let open = RwSignal::new(false);
    let persistent = RwSignal::new(true);
    let modal = RwSignal::new(false);
    let toast_modal = RwSignal::new(false);
    let nested_modal = RwSignal::new(false);
    let popup = RwSignal::new(false);
    let selected = RwSignal::new(Some("planned".to_string()));
    let selected_open = RwSignal::new(false);
    let note = RwSignal::new("Ask the release owner to review the checklist".to_string());
    let popup_focus = RwSignal::new(0_u32);
    let popup_close = RwSignal::new(0_u32);
    let select_focus = RwSignal::new(0_u32);
    let select_close = RwSignal::new(0_u32);
    let close_requests = RwSignal::new(0_u32);
    let estimate = RwSignal::new(3.0);
    let draft_cancels = RwSignal::new(0_u32);
    let modal_focus = RwSignal::new(0_u32);
    let modal_close = RwSignal::new(0_u32);
    let toggle_host = move |event: leptos::ev::KeyboardEvent| {
        if event.key() == "F2" {
            event.prevent_default();
            modal.update(|value| *value = !*value);
        }
    };
    view! {
        <section class="feedback-migration" data-testid="feedback-migration">
            <h4>"Keep reviewing while delivery details open"</h4>
            <p>"A saved-item notice can keep its note and status selection while a delivery review opens or closes."</p>
            <div class="feedback-actions">
                <button type="button" data-testid="feedback-layer-show" on:click=move |_| open.set(true)>"Show saved-item notice"</button>
                <dialog::Root open=modal on_open_change=move |next| modal.set(next)>
                    <dialog::Trigger test_id="feedback-host-open">"Review delivery details"</dialog::Trigger>
                    <dialog::Portal><dialog::Overlay class="feedback-overlay" />
                        <dialog::Content class="feedback-dialog feedback-raw-panel" test_id="feedback-host-dialog"
                            on_open_auto_focus=Some(Callback::new(move |_| modal_focus.update(|count| *count += 1)))
                            on_close_auto_focus=Some(Callback::new(move |_| modal_close.update(|count| *count += 1)))>
                            <dialog::Title>"Delivery details"</dialog::Title>
                            <dialog::Description>"Review the delivery checklist. Saved-item notices remain available here."</dialog::Description>
                            <input aria-label="Delivery reference" data-testid="feedback-host-reference" on:keydown=toggle_host />
                            <dialog::Close test_id="feedback-host-close" on:keydown=toggle_host>"Done reviewing delivery"</dialog::Close>
                        </dialog::Content>
                    </dialog::Portal>
                </dialog::Root>
            </div>
            <p>"Saved status: "<output data-testid="feedback-layer-status">{move || selected.get().unwrap_or_else(|| "No status".into())}</output></p>
            <details data-testid="feedback-migration-tools"><summary>"Review migration states"</summary>
                <label><input type="checkbox" data-testid="feedback-layer-persistent" prop:checked=move || persistent.get() on:change=move |event| persistent.set(event_target_checked(&event)) />"Keep this notice until dismissed"</label>
                <p>"F2 opens or closes delivery details while editing a saved-item note or status."</p>
                <output data-testid="feedback-layer-focus-counts">{move || format!("{},{},{},{},{},{}", popup_focus.get(), popup_close.get(), select_focus.get(), select_close.get(), modal_focus.get(), modal_close.get())}</output>
                <output data-testid="feedback-layer-close-requests">{move || close_requests.get()}</output>
                <output data-testid="feedback-draft-cancels">{move || draft_cancels.get()}</output>
            </details>
            <notice::Viewport class="feedback-toast-viewport" test_id="feedback-layer-viewport">
                <notice::Root open=open on_open_change=move |next: bool| { if !next { close_requests.update(|count| *count += 1); } open.set(next); }
                    duration_ms=Signal::derive(move || if persistent.get() { 0 } else { 6000 }) class="feedback-notice feedback-raw" test_id="feedback-layer-toast">
                    <notice::Title>"Release summary saved"</notice::Title>
                    <notice::Description>"Review the saved status or add a local delivery note."</notice::Description>
                    <popover::Root open=popup on_open_change=move |next| popup.set(next)>
                        <popover::Trigger test_id="feedback-review-open">"Review saved item"</popover::Trigger>
                        <popover::Portal>
                            <popover::Content aria_label="Saved item review" class="feedback-popover feedback-raw-panel" test_id="feedback-review-popup"
                                on_open_auto_focus=Some(Callback::new(move |_| popup_focus.update(|count| *count += 1)))
                                on_close_auto_focus=Some(Callback::new(move |_| popup_close.update(|count| *count += 1)))>
                                <label for="feedback-layer-note">"Saved-item note"</label>
                                <input id="feedback-layer-note" data-testid="feedback-layer-note" prop:value=move || note.get()
                                    on:input=move |event| note.set(event_target_value(&event)) on:keydown=toggle_host />
                                <number_field::Root value=estimate on_change=move |next| estimate.set(next) aria_label="Remaining review hours"
                                    on_cancel=Some(Callback::new(move |_| draft_cancels.update(|count| *count += 1)))>
                                    <number_field::Input test_id="feedback-layer-estimate" />
                                </number_field::Root>
                                <select::Root value=selected on_change=move |next| selected.set(next) open=selected_open on_open_change=move |next| selected_open.set(next)
                                    entries=vec![select::SelectEntry::new("planned", "Planned"), select::SelectEntry::new("in-review", "In review"), select::SelectEntry::new("ready", "Ready")]
                                    aria_label="Saved-item status"
                                    on_open_auto_focus=Some(Callback::new(move |_| select_focus.update(|count| *count += 1)))
                                    on_close_auto_focus=Some(Callback::new(move |_| select_close.update(|count| *count += 1)))>
                                    <select::Trigger test_id="feedback-layer-select" on:keydown=toggle_host><select::Value /></select::Trigger>
                                    <select::Content test_id="feedback-layer-options" class="feedback-select feedback-raw-panel"><select::Viewport>
                                        <select::Item value="planned" test_id="feedback-layer-planned" />
                                        <select::Item value="in-review" test_id="feedback-layer-review" />
                                        <select::Item value="ready" test_id="feedback-layer-ready" />
                                    </select::Viewport></select::Content>
                                </select::Root>
                                <dialog::Root open=nested_modal on_open_change=move |next| nested_modal.set(next)>
                                    <dialog::Trigger test_id="feedback-nested-dialog-open">"Approve from this review"</dialog::Trigger>
                                    <dialog::Portal><dialog::Overlay class="feedback-overlay" />
                                        <dialog::Content test_id="feedback-nested-dialog" class="feedback-dialog feedback-raw-panel">
                                            <dialog::Title>"Approve saved-item review"</dialog::Title>
                                            <dialog::Description>"This dialog was opened from the notification review."</dialog::Description>
                                            <dialog::Close test_id="feedback-nested-dialog-close">"Keep reviewing"</dialog::Close>
                                        </dialog::Content>
                                    </dialog::Portal>
                                </dialog::Root>
                                <popover::Close test_id="feedback-review-close">"Done with saved item"</popover::Close>
                            </popover::Content>
                        </popover::Portal>
                    </popover::Root>
                    <dialog::Root open=toast_modal on_open_change=move |next| toast_modal.set(next)>
                        <dialog::Trigger test_id="feedback-toast-dialog-open">"Approve this saved summary"</dialog::Trigger>
                        <dialog::Portal><dialog::Overlay class="feedback-overlay" />
                            <dialog::Content test_id="feedback-toast-dialog" class="feedback-dialog feedback-raw-panel">
                                <dialog::Title>"Approve saved summary"</dialog::Title>
                                <dialog::Description>"Confirm the local review, then return to its saved-item notice."</dialog::Description>
                                <dialog::Close test_id="feedback-toast-dialog-close">"Return to summary"</dialog::Close>
                            </dialog::Content>
                        </dialog::Portal>
                    </dialog::Root>
                    <notice::Close test_id="feedback-layer-toast-close" />
                </notice::Root>
            </notice::Viewport>
        </section>
    }
}
