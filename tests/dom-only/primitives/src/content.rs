use leptos::prelude::*;
use rustify_components::primitives::{aspect_ratio, avatar, scroll_area as s, separator};
use rustify_components::Orientation;
use rustify_ui::Direction;

#[component]
pub fn Content() -> impl IntoView {
    let dark = RwSignal::new(false);
    view! {<section class="content-samples content-unstyled" data-testid="content-samples" data-mode=move ||if dark.get(){"dark"}else{"light"}>
        <link rel="stylesheet" href="content.css" />
        <h2>"Unstyled content parts"</h2>
        <details data-testid="content-tools"><summary>"Content states and parameters"</summary>
            <button type="button" data-testid="content-theme" on:click=move |_|dark.update(|value|*value=!*value)>"Change content appearance"</button>
            <MediaExamples /><ScrollExamples />
        </details>
    </section>}
}

#[component]
fn MediaExamples() -> impl IntoView {
    let src = RwSignal::new(String::from("content-avatar.svg"));
    let status = RwSignal::new(String::from("idle"));
    let events = RwSignal::new(Vec::<String>::new());
    let ratio = RwSignal::new(1.5);
    let vertical = RwSignal::new(false);
    let decorative = RwSignal::new(false);
    view! {<section class="content-group" aria-label="Images and geometry">
        <h3>"Image fallback and geometry"</h3>
        <div class="content-actions">
            <button type="button" data-testid="content-photo-good" on:click=move |_|src.set("content-avatar.svg".into())>"Use profile photo"</button>
            <button type="button" data-testid="content-photo-broken" on:click=move |_|src.set("content-avatar-broken.svg".into())>"Use unavailable photo"</button>
            <button type="button" data-testid="content-photo-slow" on:click=move |_|src.set("content-avatar-slow.svg".into())>"Use delayed photo"</button>
            <button type="button" data-testid="content-photo-empty" on:click=move |_|src.set(String::new())>"Clear photo URL"</button>
        </div>
        <div class="content-media-row">
            <avatar::Root  class="content-avatar" test_id="content-avatar" attr:role="img" attr:aria-label="Jamie Morgan">
                <avatar::Image src=src alt="Jamie Morgan" test_id="content-avatar-image" attr:loading="lazy" attr:data-caller="retained"
                    on_loading_status_change=Callback::new(move |next:avatar::ImageLoadingStatus|{status.set(next.as_str().into());events.update(|all|all.push(next.as_str().into()));}) />
                <avatar::Fallback delay_ms=80 test_id="content-avatar-fallback" attr:data-letters="2">"JM"</avatar::Fallback>
            </avatar::Root>
            <p>"Jamie Morgan · "<output data-testid="content-image-status">{move ||status.get()}</output></p>
        </div>
        <output class="content-history" data-testid="content-image-events">{move ||events.get().join(" → ")}</output>
        <div class="content-actions">
            <button type="button" data-testid="content-ratio-wide" on:click=move |_|ratio.set(1.5)>"Landscape ratio"</button>
            <button type="button" data-testid="content-ratio-square" on:click=move |_|ratio.set(1.)>"Square ratio"</button>
            <button type="button" data-testid="content-ratio-invalid" on:click=move |_|ratio.set(f64::NAN)>"Reject invalid ratio"</button>
            <button type="button" data-testid="content-separator-direction" on:click=move |_|vertical.update(|value|*value=!*value)>"Change separator direction"</button>
            <button type="button" data-testid="content-separator-decorative" on:click=move |_|decorative.update(|value|*value=!*value)>"Toggle decorative separator"</button>
        </div>
        <aspect_ratio::Root ratio=ratio class="content-ratio" test_id="content-ratio">
            <img src="content-landscape.svg" alt="Sun over a coastal ridge" />
        </aspect_ratio::Root>
        <div class="content-separator-box"><separator::Root  orientation=Signal::derive(move ||if vertical.get(){Orientation::Vertical}else{Orientation::Horizontal}) decorative=decorative test_id="content-separator" attr:aria-label="Profile and release details" /></div>
    </section>}
}

#[component]
fn ScrollExamples() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let long = RwSignal::new(true);
    let wide = RwSignal::new(false);
    let direction = RwSignal::new(Direction::LeftToRight);
    let visibility = RwSignal::new(1_usize);
    let axes = RwSignal::new(2_usize);
    let boundary = RwSignal::new(s::Boundary::Stop);
    view! {<section class="content-group" aria-label="Native scrolling">
        <h3>"Release history"</h3>
        <div class="content-actions">
            <button type="button" data-testid="content-scroll-direction" on:click=move |_|direction.update(|value|*value=if *value==Direction::LeftToRight{Direction::RightToLeft}else{Direction::LeftToRight})>"Change scroll direction"</button>
            <button type="button" data-testid="content-scroll-visibility" on:click=move |_|visibility.update(|value|*value+=1)>"Next scrollbar visibility"</button>
            <button type="button" data-testid="content-scroll-axes" on:click=move |_|axes.update(|value|*value+=1)>"Next scroll axes"</button>
            <button type="button" data-testid="content-scroll-boundary" on:click=move |_|boundary.update(|value|*value=if *value==s::Boundary::Stop{s::Boundary::Propagate}else{s::Boundary::Stop})>"Change scroll boundary"</button>
            <button type="button" data-testid="content-scroll-length" on:click=move |_|long.update(|value|*value=!*value)>"Toggle long history"</button>
            <button type="button" data-testid="content-scroll-resize" on:click=move |_|wide.update(|value|*value=!*value)>"Resize history viewport"</button>
            <button type="button" data-testid="content-scroll-mount" on:click=move |_|mounted.update(|value|*value=!*value)>{move ||if mounted.get(){"Remove scroll area"}else{"Restore scroll area"}}</button>
            <button type="button" data-testid="content-outside">"Continue outside history"</button>
        </div>
        <Show when=move ||mounted.get()>
            <s::Root  visibility=Signal::derive(move ||s::Visibility::ALL[visibility.get()%s::Visibility::ALL.len()]) scrollbars=Signal::derive(move ||s::Scrollbars::ALL[axes.get()%s::Scrollbars::ALL.len()]) boundary=boundary hide_delay_ms=80 direction=Some(direction.into())
                aria_label="Release history" class="content-scroll" test_id="content-scroll" attr:id="content-scroll-root" attr:data-wide=move ||wide.get().then_some("")>
                <s::Viewport test_id="content-scroll-viewport">
                    <div class="content-scroll-grid" data-long=move ||long.get().then_some("") data-testid="content-scroll-grid">
                        <For each=move ||if long.get(){(0..32).collect::<Vec<_>>()}else{vec![0]} key=|index|*index let:index>
                            <button type="button" data-testid=format!("content-history-{index}")>{format!("Review checkpoint {}",index+1)}</button>
                        </For>
                    </div>
                </s::Viewport>
                <s::Scrollbar orientation=Orientation::Horizontal test_id="content-scroll-x"><s::Thumb test_id="content-thumb-x" /></s::Scrollbar>
                <s::Scrollbar orientation=Orientation::Vertical test_id="content-scroll-y"><s::Thumb test_id="content-thumb-y" /></s::Scrollbar>
                <s::Corner test_id="content-scroll-corner" />
            </s::Root>
        </Show>
    </section>}
}
