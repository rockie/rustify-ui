use leptos::prelude::*;
use rustify_components::{
    aspect_ratio, avatar, badge, callout, card, checkbox_cards as cc, data_list as dl,
    radio_cards as rc, scroll_area as s, separator, table as t,
};
use rustify_components::{Align, Orientation, Responsive, TextTrim};
use rustify_ui::{AccentColor, Direction, ThemeRadius};

#[derive(Clone, Copy)]
struct StyleChoices {
    sizes: RwSignal<usize>,
    variants: RwSignal<usize>,
    local: Signal<Option<AccentColor>>,
    high: Signal<bool>,
    radius: Signal<Option<ThemeRadius>>,
    utility: Signal<bool>,
}
#[component]
pub fn Content() -> impl IntoView {
    let owner = Owner::new();
    let view=owner.with(move || {
        let theme=rustify_ui::resolve(&rustify_ui::ThemeDocument::standard("content-fixture","Content fixture"),rustify_ui::ThemeMode::Light,&rustify_ui::ResolveContext::default()).expect("standard fixture theme is valid");
        rustify_ui::provide_static_theme(std::sync::Arc::new(theme));
        let dark=RwSignal::new(false);let local=RwSignal::new(false);let high=RwSignal::new(false);let radius=RwSignal::new(0_usize);let utility=RwSignal::new(false);let rtl=RwSignal::new(false);
        let sizes=RwSignal::new(0_usize);let variants=RwSignal::new(0_usize);
        let style=StyleChoices {sizes,variants,local:Signal::derive(move ||local.get().then_some(AccentColor::Amber)),high:high.into(),radius:Signal::derive(move ||{let index=radius.get()%(ThemeRadius::ALL.len()+1);if index==0{None}else{Some(ThemeRadius::ALL[index-1])}}),utility:utility.into()};
        let patch=Signal::derive(move ||{let mut patch=rustify_ui::ThemeBoundaryPatch::default();patch.config.appearance=rustify_ui::ConfigChange::Set(if dark.get(){rustify_ui::Appearance::Dark}else{rustify_ui::Appearance::Light});patch});
        view! {<section class="content-samples" data-testid="content-samples">
            <link rel="stylesheet" href="content.css" />
            <h2>"Structured content examples"</h2>
            <details data-testid="content-tools"><summary>"Content states and parameters"</summary>
                <div class="content-actions">
                    <button type="button" data-testid="content-theme" on:click=move |_|dark.update(|value|*value=!*value)>"Change content appearance"</button>
                    <button type="button" data-testid="content-size" on:click=move |_|sizes.update(|value|*value+=1)>"Next component sizes"</button>
                    <button type="button" data-testid="content-variant" on:click=move |_|variants.update(|value|*value+=1)>"Next component variants"</button>
                    <button type="button" data-testid="content-local-color" on:click=move |_|local.update(|value|*value=!*value)>"Toggle local amber color"</button>
                    <button type="button" data-testid="content-high-contrast" on:click=move |_|high.update(|value|*value=!*value)>"Toggle high contrast"</button>
                    <button type="button" data-testid="content-radius" on:click=move |_|radius.update(|value|*value+=1)>"Next radius"</button>
                    <button type="button" data-testid="content-utility" on:click=move |_|utility.update(|value|*value=!*value)>"Toggle caller padding utility"</button>
                    <button type="button" data-testid="content-reading-direction" on:click=move |_|rtl.update(|value|*value=!*value)>"Change content reading direction"</button>
                </div>
                <rustify_ui::ThemeBoundary patch=patch test_id="content-theme-boundary">
                    <rustify_components::primitives::DirectionProvider direction=Signal::derive(move ||if rtl.get(){Direction::RightToLeft}else{Direction::LeftToRight})>
                        <div dir=move ||if rtl.get(){"rtl"}else{"ltr"}>
                            <MediaExamples style=style /><StructuredExamples style=style /><ScrollExamples style=style />
                        </div>
                    </rustify_components::primitives::DirectionProvider>
                </rustify_ui::ThemeBoundary>
            </details>
        </section>}
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

#[component]
fn MediaExamples(style: StyleChoices) -> impl IntoView {
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
            <avatar::Root size=Signal::derive(move ||Responsive::from(avatar::AvatarSize::ALL[(style.sizes.get()+2)%avatar::AvatarSize::ALL.len()])) variant=Signal::derive(move ||avatar::AvatarVariant::ALL[(style.variants.get()+1)%avatar::AvatarVariant::ALL.len()]) color=style.local high_contrast=style.high radius=style.radius class="content-avatar" test_id="content-avatar" attr:role="img" attr:aria-label="Jamie Morgan">
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
        <div class="content-separator-box"><separator::Root size=Signal::derive(move ||Responsive::from(separator::SeparatorSize::ALL[style.sizes.get()%separator::SeparatorSize::ALL.len()])) orientation=Signal::derive(move ||Responsive::from(if vertical.get(){Orientation::Vertical}else{Orientation::Horizontal})) decorative=decorative test_id="content-separator" attr:aria-label="Profile and release details" /></div>
    </section>}
}

#[component]
fn ScrollExamples(style: StyleChoices) -> impl IntoView {
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
            <s::Root size=Signal::derive(move ||Responsive::from(s::ScrollAreaSize::ALL[style.sizes.get()%s::ScrollAreaSize::ALL.len()])) radius=style.radius visibility=Signal::derive(move ||s::Visibility::ALL[visibility.get()%s::Visibility::ALL.len()]) scrollbars=Signal::derive(move ||s::Scrollbars::ALL[axes.get()%s::Scrollbars::ALL.len()]) boundary=boundary hide_delay_ms=80 direction=Some(direction.into())
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

#[component]
fn StructuredExamples(style: StyleChoices) -> impl IntoView {
    let empty = RwSignal::new(false);
    let action = RwSignal::new(String::from("No link opened yet."));
    let role = RwSignal::new(0_usize);
    let channels = RwSignal::new(vec![String::from("email"), String::from("offline")]);
    let delivery = RwSignal::new(Some(String::from("review")));
    let saved = RwSignal::new(false);
    let trims = [
        TextTrim::Normal,
        TextTrim::Start,
        TextTrim::End,
        TextTrim::Both,
    ];
    let alignments = [
        Align::Start,
        Align::Center,
        Align::End,
        Align::Baseline,
        Align::Stretch,
    ];
    view! {<section class="content-group" aria-label="Structured details and native targets">
        <h3>"Release details"</h3>
        <div class="content-actions">
            <button type="button" data-testid="content-empty" on:click=move |_|empty.update(|value|*value=!*value)>"Toggle empty details"</button>
            <button type="button" data-testid="content-callout-role" on:click=move |_|role.update(|value|*value=(*value+1)%3)>"Next announcement policy"</button>
        </div>
        <div class="content-native-targets">
            {badge::badge_as(leptos::html::a().href("#content-native-result").attr("data-testid","content-badge").class(("p-8",move ||style.utility.get()))
                .on(leptos::ev::click,move |_|action.set("Status link opened.".into())).child("Ready for review"),badge::BadgeOptions {
                    size:Signal::derive(move ||Responsive::from(badge::BadgeSize::ALL[style.sizes.get()%badge::BadgeSize::ALL.len()])),
                    variant:Signal::derive(move ||badge::BadgeVariant::ALL[(style.variants.get()+1)%badge::BadgeVariant::ALL.len()]),
                    color:style.local,high_contrast:style.high,radius:style.radius,..Default::default()
                })}
            {card::card_as(leptos::html::a().href("#content-native-result").attr("data-testid","content-card").class("content-native-card")
                .on(leptos::ev::click,move |_|action.set("Release details link opened.".into())).child(view! {<strong>"Review release details"</strong><span>"Open the owner notes and delivery checklist."</span>}),card::CardOptions {
                    size:Signal::derive(move ||Responsive::from(card::CardSize::ALL[style.sizes.get()%card::CardSize::ALL.len()])),
                    variant:Signal::derive(move ||card::CardVariant::ALL[style.variants.get()%card::CardVariant::ALL.len()]),..Default::default()
                })}
        </div>
        <p id="content-native-result" data-testid="content-native-result">{move ||action.get()}</p>
        <callout::Root size=Signal::derive(move ||Responsive::from(callout::CalloutSize::ALL[(style.sizes.get()+1)%callout::CalloutSize::ALL.len()]))
            variant=Signal::derive(move ||callout::CalloutVariant::ALL[style.variants.get()%callout::CalloutVariant::ALL.len()]) color=style.local high_contrast=style.high
            role=Signal::derive(move ||if role.get()==0{None}else{Some(callout::CalloutRole::ALL[role.get()-1])}) test_id="content-callout">
            <callout::Icon test_id="content-callout-icon"><rustify_components::Icon glyph=rustify_components::Glyph::Dot /></callout::Icon>
            <callout::Text>"Review the release owner and status before sharing these details."</callout::Text>
        </callout::Root>
        <card::Card class="content-owner-card" test_id="content-owner-card">
        <dl::Root size=Signal::derive(move ||Responsive::from(dl::DataListSize::ALL[(style.sizes.get()+1)%dl::DataListSize::ALL.len()]))
            orientation=Signal::derive(move ||Responsive::from(dl::DataListOrientation::ALL[style.variants.get()%dl::DataListOrientation::ALL.len()]))
            trim=Signal::derive(move ||Responsive::from(trims[style.variants.get()%trims.len()])) test_id="content-data-list">
            <Show when=move ||!empty.get() fallback=||view! {<dl::Item><dl::Label>"Details"</dl::Label><dl::Value>"No release details yet."</dl::Value></dl::Item>}>
                <dl::Item align=Signal::derive(move ||Responsive::from(alignments[style.sizes.get()%alignments.len()])) test_id="content-owner-item">
                    <dl::Label width=Responsive {initial:Some("6rem".into()),md:Some("8rem".into()),..Default::default()} min_width=Responsive::from("4rem".to_string()) max_width=Responsive::from("10rem".to_string()) color=style.local high_contrast=style.high test_id="content-owner-label">"Owner"</dl::Label>
                    <dl::Value test_id="content-owner-first">"Jamie Morgan"</dl::Value><dl::Value test_id="content-owner-second"><badge::Badge class="content-owner-badge" test_id="content-owner-badge">"Release operations"</badge::Badge></dl::Value>
                </dl::Item>
                <dl::Item><dl::Label>"Reference"</dl::Label><dl::Value test_id="content-long-value">"https://example.test/releases/2026/november/review-checklist-with-a-deliberately-long-reference-without-shortening-the-original-value"</dl::Value></dl::Item>
            </Show>
        </dl::Root>
        </card::Card>
        <div class="content-table-scroll" tabindex="0" aria-label="Release details table scrollport" data-testid="content-table-scroll">
            <t::Root size=Signal::derive(move ||Responsive::from(t::TableSize::ALL[(style.sizes.get()+1)%t::TableSize::ALL.len()]))
                variant=Signal::derive(move ||t::TableVariant::ALL[(style.variants.get()+1)%t::TableVariant::ALL.len()])
                layout=Signal::derive(move ||Responsive::from(t::TableLayout::ALL[style.variants.get()%t::TableLayout::ALL.len()])) test_id="content-table">
                <t::Caption>"Release review checklist"</t::Caption>
                <t::Header><t::Row>
                    <t::ColumnHeaderCell id="content-task-column" width=Responsive::from("10rem".to_string())>"Task"</t::ColumnHeaderCell>
                    <t::ColumnHeaderCell id="content-status-column">"Status"</t::ColumnHeaderCell>
                    <t::ColumnHeaderCell id="content-reference-column">"Reference"</t::ColumnHeaderCell>
                </t::Row></t::Header>
                <t::Body>
                    <Show when=move ||!empty.get() fallback=||view! {<t::Row><t::Cell colspan=Some(3) test_id="content-table-empty">"No review entries yet."</t::Cell></t::Row>}>
                        <t::Row align=Signal::derive(move ||Responsive::from(t::TableRowAlign::ALL[style.sizes.get()%t::TableRowAlign::ALL.len()])) test_id="content-table-row">
                            <t::RowHeaderCell id="content-review-row" headers="content-task-column">"Launch checklist"</t::RowHeaderCell>
                            <t::Cell headers="content-status-column content-review-row" test_id="content-table-status"
                                justify=Signal::derive(move ||Responsive::from(t::TableCellJustify::ALL[style.variants.get()%t::TableCellJustify::ALL.len()]))>"In review"</t::Cell>
                            <t::Cell headers="content-reference-column content-review-row" min_width=Responsive::from("12rem".to_string()) max_width=Responsive::from("20rem".to_string())
                                padding=rustify_components::Padding {px:Responsive {initial:Some(rustify_components::Spacing::Scale(2)),md:Some(rustify_components::Spacing::Scale(3)),..Default::default()},..Default::default()}>"release-checklist-reference-with-a-long-continuous-identifier"</t::Cell>
                        </t::Row>
                    </Show>
                </t::Body>
            </t::Root>
        </div>
        <form data-testid="content-choice-form" class="content-choice-form" on:submit=move |event|{event.prevent_default();saved.set(true);}>
            <h4>"Share review updates"</h4>
            <cc::Root value=channels on_change=move |next|channels.set(next) name="channels" aria_label="Update channels" columns=Responsive {initial:Some(rustify_components::GridTracks::Count(1)),sm:Some(rustify_components::GridTracks::Count(2)),..Default::default()}>
                <cc::Item value="email" test_id="content-channel-email" description=Box::new(||"Send a weekly review digest.".into_any())>"Email updates"</cc::Item>
                <cc::Item value="feed" test_id="content-channel-feed" description=Box::new(||"Keep release notes in the team feed.".into_any())>"Team feed"</cc::Item>
                <cc::Item value="offline" disabled=true test_id="content-channel-disabled" description=Box::new(||"Offline export is unavailable for this sample.".into_any())>"Offline export"</cc::Item>
            </cc::Root>
            <rc::Root value=delivery on_change=move |next|delivery.set(next) name="delivery" aria_label="Review readiness" columns=Responsive {initial:Some(rustify_components::GridTracks::Count(1)),sm:Some(rustify_components::GridTracks::Count(2)),..Default::default()}>
                <rc::Item value="review" test_id="content-delivery-review" description=Box::new(||"Ask the release owner to review.".into_any())>"Keep in review"</rc::Item>
                <rc::Item value="ready" test_id="content-delivery-ready" description=Box::new(||"Prepare the release notes for sharing.".into_any())>"Ready to share"</rc::Item>
            </rc::Root>
            <button type="submit" data-testid="content-choice-save">"Save sharing preferences"</button>
            <p data-testid="content-choice-saved">{move ||if saved.get(){"Sharing preferences saved locally."}else{"Preferences have not been saved."}}</p>
        </form>
    </section>}
}
