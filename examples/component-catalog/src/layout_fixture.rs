//! DOM-only layout and typography evidence, mounted separately from the GPU catalogue.

use leptos::prelude::*;
use leptos::wasm_bindgen::{prelude::*, JsCast};
use rustify_components::layout::*;
use rustify_components::typography::*;
use rustify_components::{Button, GeometryStyles, Link, LinkUnderline, Responsive};
use rustify_ui::{
    mount, AccentColor, AppHandle, MountConfig, ThemeDocument, ThemeMode, ThemeScaling, ThemeScope,
};
use std::{cell::RefCell, sync::Arc};

thread_local! {
    static FIXTURE: RefCell<Option<AppHandle>> = const { RefCell::new(None) };
}

fn css(value: &str) -> Responsive<String> {
    value.to_owned().into()
}

fn space(value: i8) -> Responsive<Spacing> {
    Spacing::Scale(value).into()
}

const TEXT_SIZES: [TextSize; 9] = [
    TextSize::One,
    TextSize::Two,
    TextSize::Three,
    TextSize::Four,
    TextSize::Five,
    TextSize::Six,
    TextSize::Seven,
    TextSize::Eight,
    TextSize::Nine,
];

#[component]
fn TypographyMatrix() -> impl IntoView {
    view! {
        <section class="fixture-section" data-testid="type-heading-matrix">
            <Heading as_=HeadingLevel::H2>"Heading"</Heading>
            {TEXT_SIZES.into_iter().enumerate().rev().map(|(index, size)| view! {
                <div class="fixture-type-row">
                    <span class="fixture-prop">{format!("size={}", index + 1)}</span>
                    <Heading as_=HeadingLevel::H3 size=Responsive::from(size)>{if index == 8 { "The quick brown fox jumped" } else { "The quick brown fox jumped over the lazy dog" }}</Heading>
                </div>
            }).collect_view()}
            <div class="fixture-heading-wrap">
                {TEXT_SIZES.into_iter().enumerate().rev().map(|(index, size)| view! {
                    <div class="fixture-type-row">
                        <span class="fixture-prop">{format!("size={} · wrap", index + 1)}</span>
                        <Heading as_=HeadingLevel::H3 size=Responsive::from(size)>"Principles of the Typographic Craft"</Heading>
                    </div>
                }).collect_view()}
            </div>
        </section>
        <section class="fixture-section" data-testid="type-text-matrix">
            <Heading as_=HeadingLevel::H2>"Text"</Heading>
            {TEXT_SIZES.into_iter().enumerate().rev().map(|(index, size)| view! {
                <div class="fixture-type-row"><span class="fixture-prop">{format!("size={}", index + 1)}</span>
                    <Text as_=TextElement::P size=Responsive::from(size)>"The quick brown fox jumped over the lazy dog"</Text>
                </div>
            }).collect_view()}
            {[("light", TextWeight::Light), ("regular", TextWeight::Regular), ("medium", TextWeight::Medium), ("bold", TextWeight::Bold)].into_iter().map(|(name, weight)| view! {
                <div class="fixture-type-row"><span class="fixture-prop">{format!("weight={name}")}</span>
                    <Text weight=Responsive::from(weight)>"Type weight gives a sentence its emphasis."</Text>
                </div>
            }).collect_view()}
        </section>
        <section class="fixture-section" data-testid="type-code-matrix">
            <Heading as_=HeadingLevel::H2>"Code"</Heading>
            <div class="fixture-code-head fixture-prop"><span>"variant"</span><span>"color"</span><span>"high_contrast"</span><span>"gray"</span><span>"gray + high_contrast"</span></div>
            {[("solid", CodeVariant::Solid), ("soft", CodeVariant::Soft), ("outline", CodeVariant::Outline), ("ghost", CodeVariant::Ghost)].into_iter().map(|(name, variant)| view! {
                <div class="fixture-code-row"><span class="fixture-prop">{name}</span><div class="fixture-code-cells">
                    <div><span class="fixture-prop fixture-code-cell-label">"color=indigo"</span><Code variant=variant color=AccentColor::Indigo>"console.log()"</Code></div>
                    <div><span class="fixture-prop fixture-code-cell-label">"indigo · high_contrast"</span><Code variant=variant color=AccentColor::Indigo high_contrast=true>"console.log()"</Code></div>
                    <div><span class="fixture-prop fixture-code-cell-label">"color=gray"</span><Code variant=variant color=AccentColor::Gray>"console.log()"</Code></div>
                    <div><span class="fixture-prop fixture-code-cell-label">"gray · high_contrast"</span><Code variant=variant color=AccentColor::Gray high_contrast=true>"console.log()"</Code></div>
                </div></div>
            }).collect_view()}
            <p>"Inline "<Code>"code"</Code>" sits alongside "<Strong>"strong importance"</Strong>
                ", "<Em>"emphasis"</Em>" and "<Quote>"a native quotation"</Quote>". Press "<Kbd>"Enter"</Kbd>" to continue."</p>
            {TEXT_SIZES.into_iter().enumerate().rev().map(|(index, size)| view! {
                <div class="fixture-type-row"><span class="fixture-prop">{format!("ghost · size={}", index + 1)}</span>
                    <Code variant=CodeVariant::Ghost size=Responsive::from(size)>"The quick brown fox jumped"</Code>
                </div>
            }).collect_view()}
        </section>
        <section class="fixture-section" data-testid="type-kbd-matrix">
            <Heading as_=HeadingLevel::H2>"Kbd"</Heading>
            <p><Kbd>"Enter"</Kbd>" "<Kbd>"Tab"</Kbd>" "<Kbd>"Shift + Tab"</Kbd></p>
            <div class="fixture-kbd-row fixture-prop"><span>"size"</span><span>"classic (default)"</span><span>"soft"</span></div>
            {TEXT_SIZES.into_iter().enumerate().rev().map(|(index, size)| view! {
                <div class="fixture-kbd-row"><span class="fixture-prop">{format!("size={}", index + 1)}</span>
                    <span><Kbd size=Responsive::from(size)>"/"</Kbd>" "<Kbd size=Responsive::from(size)>"X"</Kbd></span>
                    <span><Kbd variant=KbdVariant::Soft size=Responsive::from(size)>"/"</Kbd>" "<Kbd variant=KbdVariant::Soft size=Responsive::from(size)>"X"</Kbd></span>
                </div>
            }).collect_view()}
        </section>
        <section class="fixture-section" data-testid="type-blockquote-matrix">
            <Heading as_=HeadingLevel::H2>"Blockquote"</Heading>
            {[("size=2", TextSize::Two), ("size=4", TextSize::Four), ("size=7", TextSize::Seven)].into_iter().map(|(name, size)| view! {
                <div class="fixture-type-row"><span class="fixture-prop">{name}</span>
                    <Blockquote size=Responsive::from(size)>"The native quotation remains a block quotation."</Blockquote>
                </div>
            }).collect_view()}
        </section>
        <section class="fixture-section" data-testid="type-wrap-matrix">
            <Heading as_=HeadingLevel::H2>"Wrap and trim"</Heading>
            <div class="fixture-wrap-grid">
                {[("default", None, false), ("wrap", Some(TextWrap::Wrap), false), ("nowrap", Some(TextWrap::Nowrap), false), ("balance", Some(TextWrap::Balance), false), ("pretty", Some(TextWrap::Pretty), false), ("truncate", None, true)].into_iter().map(|(name, wrap, truncate)| view! {
                    <div class="fixture-wrap-sample" tabindex=(name == "nowrap").then_some("0")
                        role=(name == "nowrap").then_some("region") aria-label=(name == "nowrap").then_some("No-wrap text sample; scroll horizontally")><p class="fixture-prop">{format!("wrap={name}")}</p>
                        <Text as_=TextElement::P wrap=Responsive { initial: wrap, ..Responsive::default() } truncate=truncate>"A thoughtful layout leaves room for words, meaning and a reader's changing viewport."</Text>
                    </div>
                }).collect_view()}
            </div>
            <div class="fixture-wrap-grid">
                {[("normal", TextTrim::Normal), ("start", TextTrim::Start), ("end", TextTrim::End), ("both", TextTrim::Both)].into_iter().map(|(name, trim)| view! {
                    <div class="fixture-trim-sample"><p class="fixture-prop">{format!("trim={name}")}</p>
                        <Text as_=TextElement::P size=Responsive::from(TextSize::Six) trim=Responsive::from(trim)>"Leading space\nsets the rhythm."</Text>
                    </div>
                }).collect_view()}
            </div>
        </section>
    }
}

#[component]
fn TypographySamples() -> impl IntoView {
    view! {
        <div class="space-y-4" data-testid="layout-typography">
            <Heading as_=HeadingLevel::H3 size=Responsive::from(TextSize::Nine) wrap=Responsive::from(TextWrap::Balance) test_id="type-heading">"Native structure, flexible composition"</Heading>
            <Heading test_id="type-default-heading">"A heading keeps its own level"</Heading>
            <Text as_=TextElement::P size=Responsive::from(TextSize::Three) weight=Responsive::from(TextWeight::Regular) wrap=Responsive::from(TextWrap::Pretty) trim=Responsive::from(TextTrim::Both) test_id="type-paragraph">
                "A layout should leave room for real words. This longer paragraph combines readable English with 中文内容，检查窄屏换行与实际行高。 Its meaning stays intact as the viewport changes."
            </Text>
            <Text as_=TextElement::Div test_id="type-inherited">"An unset text size inherits the effective theme."</Text>
            <Text as_=TextElement::Label test_id="type-label" attr:r#for="layout-name">"Your name"</Text>
            <input id="layout-name" aria-label="Your name" class="border border-border rounded-md p-2"/>
            <Text test_id="type-responsive" as_=TextElement::Div
                size=Responsive { initial: Some(TextSize::Two), xs: Some(TextSize::Three), sm: Some(TextSize::Four), md: Some(TextSize::Five), lg: Some(TextSize::Six), xl: Some(TextSize::Seven) }
                weight=Responsive { initial: Some(TextWeight::Regular), sm: Some(TextWeight::Medium), lg: Some(TextWeight::Bold), ..Responsive::default() }
                align=Responsive { initial: Some(TextAlign::Left), md: Some(TextAlign::Center), ..Responsive::default() }
                trim=Responsive { md: Some(TextTrim::Both), ..Responsive::default() }
                wrap=Responsive { initial: Some(TextWrap::Wrap), xl: Some(TextWrap::Nowrap), ..Responsive::default() }
            >"Text follows the same six breakpoints."</Text>
            <Blockquote size=Responsive::from(TextSize::Four) weight=Responsive::from(TextWeight::Medium) wrap=Responsive::from(TextWrap::Balance) test_id="type-blockquote">"The native quotation remains a block quotation."</Blockquote>
            <p><span class="fixture-prop">"Code · variant=soft (default) · size=3 "</span><Code size=Responsive::from(TextSize::Three) test_id="type-code">"let layout = Responsive::from(value);"</Code></p>
            <p><span class="fixture-prop">"Kbd · variant=classic (default) · size=2 "</span><Kbd size=Responsive::from(TextSize::Two) test_id="type-kbd">"Ctrl + K"</Kbd></p>
            <p><Em test_id="type-em">"Emphasis"</Em>" · "<Quote test_id="type-quote">"Native quotation marks"</Quote>" · "<Strong test_id="type-strong">"Strong importance"</Strong></p>
            <Text as_=TextElement::Div size=Responsive::from(TextSize::Nine) class="text-sm" test_id="type-class-override">"A caller utility controls this size."</Text>
            <Text as_=TextElement::Div class="w-40" truncate=true test_id="type-truncate">"This long sentence is truncated deliberately without changing its accessible text."</Text>
            <Text as_=TextElement::Div margin=Margin { ml: space(2), ..Margin::default() } test_id="type-margin">"Shared margins follow theme space."</Text>
            <Link href="#layout-target" size=Responsive::from(TextSize::Four) underline=LinkUnderline::Always attr:target="_blank" attr:rel="noreferrer" test_id="type-link">"Open a native link"</Link>
            <Link href="#layout-target" class="block w-40" truncate=true test_id="type-long-link">"https://example.test/a/long/path/whose/native/link/name/is/preserved"</Link>
            <Link href="#layout-target" disabled=true test_id="type-disabled-link">"Unavailable destination"</Link>
            <div id="layout-target" data-testid="layout-link-target">"Native link destination"</div>
            <TypographyMatrix/>
        </div>
    }
}

#[component]
fn LayoutSamples(invalid: RwSignal<bool>, clicks: RwSignal<u32>) -> impl IntoView {
    let checked_layout = Memo::new(move |_| Layout {
        width: css(if invalid.get() {
            "12px;display:none"
        } else {
            "240px"
        }),
        ..Layout::default()
    });
    let shared = Layout {
        padding: Padding {
            p: space(4),
            px: space(3),
            pt: space(2),
            ..Padding::default()
        },
        width: css("160px"),
        min_width: css("80px"),
        max_width: css("240px"),
        height: css("90px"),
        min_height: css("40px"),
        max_height: css("180px"),
        position: Position::Relative.into(),
        inset: space(0),
        top: space(1),
        right: space(2),
        bottom: space(3),
        left: space(4),
        overflow: Overflow::Hidden.into(),
        overflow_x: Overflow::Auto.into(),
        overflow_y: Overflow::Scroll.into(),
        flex_basis: css("100px"),
        flex_grow: 2.0.into(),
        flex_shrink: 0.5.into(),
        grid_area: css("main"),
        grid_column: css("2 / 4"),
        grid_column_start: css("3"),
        grid_column_end: css("5"),
        grid_row: css("1 / 3"),
        grid_row_start: css("2"),
        grid_row_end: css("4"),
        align_self: Align::Start.into(),
        justify_self: Align::End.into(),
    };
    let reset_button = Reset(
        leptos::html::button()
            .attr("type", "button")
            .attr("aria-label", "Native reset action")
            .attr("data-testid", "layout-reset")
            .class("fixture-reset-outline")
            .on(leptos::ev::click, move |_| {
                clicks.update(|value| *value += 1)
            })
            .child("Native reset action"),
    );
    view! {
        <div class="space-y-4" data-testid="layout-primitives">
            <Heading as_=HeadingLevel::H2>"Layout primitives"</Heading>
            <p class="fixture-prop">"Box · as=span · display=block (default)"</p>
            <Box as_=LayoutElement::Span test_id="layout-box-default">"A semantic span starts as a block."</Box>
            <p class="fixture-prop">"Box · width=10 / xs=20 / sm=30 / md=40 / lg=50 / xl=60px · height=8px"</p>
            <Box test_id="layout-responsive-width" layout=Layout {
                width: Responsive { initial: Some("10px".into()), xs: Some("20px".into()), sm: Some("30px".into()), md: Some("40px".into()), lg: Some("50px".into()), xl: Some("60px".into()) },
                height: css("8px"), ..Layout::default()
            } class="bg-primary">{ "" }</Box>
            <p class="fixture-prop">"Box · display=block / md=none / lg=block (sparse)"</p>
            <Box as_=LayoutElement::Span test_id="layout-sparse-display" display=Responsive { md: Some(BoxDisplay::None), lg: Some(BoxDisplay::Block), ..Responsive::default() }>"Sparse display keeps the component default before md."</Box>
            // Keep the margin sample below the parent's space-y utility target.
            <div class="flow-root">
                <p class="fixture-prop">"Box · 160 × 90px · padding=4 / px=3 / pt=2 · relative, scroll, flex and grid fields"</p>
            <Box test_id="layout-shared" layout=shared margin=Margin { m: space(2), mx: space(3), my: space(4), mt: space(5), mr: space(6), mb: space(7), ml: space(8) } class="bg-muted">"Shared fields"</Box>
            </div>
            <p class="fixture-prop">"Box · width=37px / padding=4; caller class=w-20 p-0"</p>
            <Box test_id="layout-class-override" layout=Layout { width: css("37px"), padding: Padding { p: space(4), ..Padding::default() }, ..Layout::default() } class="w-20 p-0">"Utilities"</Box>
            <p class="fixture-prop">"Flex · direction=row / sm=column / lg=row-reverse · align=center · justify=between · wrap · gap=3 / x=2 / y=4"</p>
            <Flex test_id="layout-flex" as_=LayoutElement::Span
                direction=Responsive { initial: Some(FlexDirection::Row), sm: Some(FlexDirection::Column), lg: Some(FlexDirection::RowReverse), ..Responsive::default() }
                align=Responsive::from(Align::Center) justify=Responsive::from(Justify::Between) wrap=Responsive::from(FlexWrap::Wrap)
                gap=Gap { gap: space(3), gap_x: space(2), gap_y: space(4) }>
                <span>"First"</span><span>"Second"</span><span>"Third"</span>
            </Flex>
            <p class="fixture-prop">"Grid · columns=3 · rows=auto 1fr · row-dense · align=stretch · justify=between · gap=2 / x=3 / y=4"</p>
            <Grid test_id="layout-grid" columns=Responsive::from(GridTracks::Count(3)) rows=Responsive::from(GridTracks::Css("auto 1fr".into()))
                areas=css("\"a a b\" \"c d b\"") flow=Responsive::from(GridFlow::RowDense)
                align=Responsive::from(Align::Stretch) justify=Responsive::from(Justify::Between)
                align_content=Responsive::from(AlignContent::Around) justify_items=Responsive::from(Align::Center)
                gap=Gap { gap: space(2), gap_x: space(3), gap_y: space(4) }>
                <span>"One"</span><span>"Two"</span><span>"Three"</span><span>"Four"</span><span>"Five"</span>
            </Grid>
            <Container test_id="layout-container" size=Responsive { initial: Some(ContainerSize::One), sm: Some(ContainerSize::Two), md: Some(ContainerSize::Three), lg: Some(ContainerSize::Four), ..Responsive::default() }
                align=Responsive { initial: Some(ContainerAlign::Left), md: Some(ContainerAlign::Right), ..Responsive::default() }>
                <div class="bg-muted p-2">"Responsive size=1/2/3/4 · align=left/right. Maximum width stays in CSS pixels."</div>
            </Container>
            <Container test_id="layout-container-projection" layout=Layout { width: css("240px"), min_width: css("120px"), max_width: css("280px"), height: css("72px"), min_height: css("40px"), max_height: css("90px"), ..Layout::default() }>
                <div class="fixture-projection">"width=240px · height=72px · content layer"</div>
            </Container>
            <Section test_id="layout-section" size=Responsive { initial: Some(SectionSize::One), xs: Some(SectionSize::Two), md: Some(SectionSize::Three), xl: Some(SectionSize::Four), ..Responsive::default() } class="bg-muted">"Section · size=1 / xs=2 / md=3 / xl=4 (24 / 40 / 64 / 80px)"</Section>
            <p class="fixture-prop">"Outer padding=32px; dashed nearest container padding=8px; nested Inset consumes it once."</p>
            <Box test_id="layout-inset-outer" layout=Layout { width: css("300px"), padding: Padding { p: space(6), ..Padding::default() }, ..Layout::default() } class="bg-muted">
                <Box test_id="layout-inset-nearest" layout=Layout { padding: Padding { p: space(2), ..Padding::default() }, ..Layout::default() } class="bg-card fixture-inset-nearest">
                    <Inset test_id="layout-inset" padding=InsetPadding { p: InsetPaddingValue::Current.into(), px: InsetPaddingValue::Zero.into(), ..InsetPadding::default() }>
                        <div><Inset test_id="layout-inset-nested">"Nearest padding is consumed once."</Inset></div>
                    </Inset>
                </Box>
            </Box>
            <div class="fixture-inset-sides" data-testid="layout-inset-sides">
                {[
                    ("all", Responsive::from(InsetSide::All)),
                    ("x", Responsive::from(InsetSide::X)),
                    ("y", Responsive::from(InsetSide::Y)),
                    ("top", Responsive::from(InsetSide::Top)),
                    ("bottom", Responsive::from(InsetSide::Bottom)),
                    ("left", Responsive::from(InsetSide::Left)),
                    ("right", Responsive::from(InsetSide::Right)),
                    ("responsive", Responsive {
                        initial: Some(InsetSide::Top), xs: Some(InsetSide::Right),
                        sm: Some(InsetSide::Bottom), md: Some(InsetSide::Left),
                        ..Responsive::default()
                    }),
                ].into_iter().map(|(label, side)| view! {
                    <div><p class="fixture-prop">{format!("side={label} · padding=16px · margin=3px")}</p>
                    <Box class="fixture-inset-side" layout=Layout { padding: Padding { p: space(4), ..Padding::default() }, ..Layout::default() }>
                        <Inset test_id=format!("layout-inset-{label}") side=side
                            margin=Margin { m: Spacing::Css("3px".into()).into(), ..Margin::default() }>
                            <div class="fixture-inset-fill">{label}</div>
                        </Inset>
                    </Box></div>
                }).collect_view()}
            </div>
            <p class="fixture-prop">"Box · padding=4 (theme spacing, scales once)"</p>
            <Box test_id="layout-scaled-space" layout=Layout { padding: Padding { p: space(4), ..Padding::default() }, ..Layout::default() }>"Theme space scales once."</Box>
            <p class="fixture-prop">"Box · width=37px · height=19px · padding=3px (explicit CSS)"</p>
            <Box test_id="layout-absolute" class="fixture-explicit-box" layout=Layout { width: css("37px"), height: css("19px"), padding: Padding { p: Spacing::Css("3px".into()).into(), ..Padding::default() }, ..Layout::default() }><span class="sr-only">"37px × 19px; padding=3px"</span></Box>
            <p class="fixture-prop">"Box · width=240px; Invalid prop submits width=12px;display:none"</p>
            <Box test_id="layout-invalid-prop" class="fixture-explicit-box" layout=Signal::derive(move ||checked_layout.get())>"A bad prop keeps this view alive."</Box>
            <p role="status" data-testid="layout-prop-diagnostics" class="fixture-prop-diagnostic">{move || {
                let mut styles=rustify_components::geometry::StyleSet::default();
                checked_layout.get().write(&mut styles).err().map(|_|view! {<span>"Box · "<Code size=Responsive::from(TextSize::One)>"width"</Code>{": rejected \"12px;display:none\" (expected one CSS value) · fallback width=auto"}</span>})
            }}</p>
            <section class="fixture-section" data-testid="layout-reset-comparison">
                <Heading as_=HeadingLevel::H2>"Reset"</Heading>
                <p class="fixture-prop">"Only the dashed native element receives Reset; siblings keep their browser defaults. No wrapper is added."</p>
                <div class="fixture-reset-row"><span class="fixture-prop">"button · native"</span><button type="button">"Native action"</button></div>
                <div class="fixture-reset-row"><span class="fixture-prop">"button · Reset"</span>{reset_button}<span class="fixture-prop fixture-reset-clicks">"clicks: "<output data-testid="layout-reset-count">{move || clicks.get()}</output></span></div>
                <div class="fixture-reset-row"><span class="fixture-prop">"h2 · native"</span><h2>"Native heading"</h2></div>
                <div class="fixture-reset-row"><span class="fixture-prop">"h2 · Reset"</span>{Reset(leptos::html::h2().class("fixture-reset-outline").child("Reset heading"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"ul · native"</span><ul><li>"Native list item"</li></ul></div>
                <div class="fixture-reset-row"><span class="fixture-prop">"ul · Reset"</span>{Reset(leptos::html::ul().class("fixture-reset-outline").child(leptos::html::li().child("Reset list item")))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"a · native"</span>{leptos::html::a().href("#layout-reset-comparison").child("Native link")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"a · Reset"</span>{Reset(leptos::html::a().href("#layout-reset-comparison").child("Native link").class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"strong · native"</span>{leptos::html::strong().child("Strong importance")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"strong · Reset"</span>{Reset(leptos::html::strong().child("Strong importance").class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"em · native"</span>{leptos::html::em().child("Emphasis")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"em · Reset"</span>{Reset(leptos::html::em().child("Emphasis").class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"code · native"</span>{leptos::html::code().child("let answer = 42;")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"code · Reset"</span>{Reset(leptos::html::code().child("let answer = 42;").class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"kbd · native"</span>{leptos::html::kbd().child("Ctrl + K")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"kbd · Reset"</span>{Reset(leptos::html::kbd().child("Ctrl + K").class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"mark · native"</span>{leptos::html::mark().child("Marked text")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"mark · Reset"</span>{Reset(leptos::html::mark().child("Marked text").class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"sub · native"</span>{leptos::html::sub().child("Subscript")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"sub · Reset"</span>{Reset(leptos::html::sub().child("Subscript").class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"sup · native"</span>{leptos::html::sup().child("Superscript")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"sup · Reset"</span>{Reset(leptos::html::sup().child("Superscript").class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"input · native"</span>{leptos::html::input().attr("aria-label","Sample input").attr("placeholder","Native input")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"input · Reset"</span>{Reset(leptos::html::input().attr("aria-label","Sample input").attr("placeholder","Native input").class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"select · native"</span>{leptos::html::select().attr("aria-label","Sample select").child(leptos::html::option().child("One option"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"select · Reset"</span>{Reset(leptos::html::select().attr("aria-label","Sample select").child(leptos::html::option().child("One option")).class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"textarea · native"</span>{leptos::html::textarea().attr("aria-label","Sample textarea").attr("rows","2").child("A native textarea")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"textarea · Reset"</span>{Reset(leptos::html::textarea().attr("aria-label","Sample textarea").attr("rows","2").child("A native textarea").class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"fieldset · native"</span>{leptos::html::fieldset().child(leptos::html::legend().child("Fieldset legend"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"fieldset · Reset"</span>{Reset(leptos::html::fieldset().child(leptos::html::legend().child("Fieldset legend")).class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"ol · native"</span>{leptos::html::ol().child(leptos::html::li().child("Ordered item"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"ol · Reset"</span>{Reset(leptos::html::ol().child(leptos::html::li().child("Ordered item")).class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"dl · native"</span>{leptos::html::dl().child((leptos::html::dt().child("Term"),leptos::html::dd().child("Definition")))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"dl · Reset"</span>{Reset(leptos::html::dl().child((leptos::html::dt().child("Term"),leptos::html::dd().child("Definition"))).class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"table · native"</span>{leptos::html::table().child(leptos::html::tbody().child(leptos::html::tr().child(leptos::html::td().child("Table cell"))))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"table · Reset"</span>{Reset(leptos::html::table().child(leptos::html::tbody().child(leptos::html::tr().child(leptos::html::td().child("Table cell")))).class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"pre · native"</span>{leptos::html::pre().child("Preformatted text")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"pre · Reset"</span>{Reset(leptos::html::pre().child("Preformatted text").class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"blockquote · native"</span>{leptos::html::blockquote().child("Native block quotation")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"blockquote · Reset"</span>{Reset(leptos::html::blockquote().child("Native block quotation").class("fixture-reset-outline"))}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"img · native"</span>{leptos::html::img().attr("src","./public/reset-sample.svg").attr("alt","Three color swatches").attr("width","96").attr("height","32")}</div>
                <div class="fixture-reset-row"><span class="fixture-prop">"img · Reset"</span>{Reset(leptos::html::img().attr("src","./public/reset-sample.svg").attr("alt","Three color swatches").attr("width","96").attr("height","32").class("fixture-reset-outline"))}</div>
            </section>
            <LayoutDemonstrations/>
        </div>
    }
}

#[component]
fn LayoutDemonstrations() -> impl IntoView {
    view! {
        <section class="fixture-section" data-testid="layout-container-matrix">
            <Heading as_=HeadingLevel::H2>"Container"</Heading>
            {[(1, ContainerSize::One, 448), (2, ContainerSize::Two, 688), (3, ContainerSize::Three, 880), (4, ContainerSize::Four, 1136)].into_iter().map(|(index, size, width)| view! {
                <div class="fixture-container-parent">
                    <p class="fixture-prop">{format!("size={index} · align=center · max-width={width}px")}</p>
                    <Container size=Responsive::from(size) class="fixture-container-demo"><div>"A centered content layer inside a full-width alignment layer."</div></Container>
                </div>
            }).collect_view()}
            {[("left", ContainerAlign::Left), ("center", ContainerAlign::Center), ("right", ContainerAlign::Right)].into_iter().map(|(name, align)| view! {
                <div class="fixture-container-parent"><p class="fixture-prop">{format!("size=1 · align={name}")}</p>
                    <Container size=Responsive::from(ContainerSize::One) align=Responsive::from(align) class="fixture-container-demo"><div>{name}</div></Container>
                </div>
            }).collect_view()}
        </section>
        <section class="fixture-section" data-testid="layout-inset-demonstration">
            <Heading as_=HeadingLevel::H2>"Inset"</Heading>
            <div class="fixture-inset-demo-grid">
                {[("top", InsetSide::Top), ("bottom", InsetSide::Bottom), ("left",InsetSide::Left), ("right",InsetSide::Right)].into_iter().map(|(name, side)| view! {
                    <Box class="fixture-inset-card" attr:data-side=name layout=Layout { padding: Padding { p: space(5), ..Padding::default() }, ..Layout::default() }>
                        <Show when=move || matches!(side,InsetSide::Bottom|InsetSide::Right)><p>"Content keeps the card's declared padding."</p></Show>
                        <Inset side=Responsive::from(side)><div class="fixture-inset-hero">{format!("side={name} · flush to card edges")}</div></Inset>
                        <Show when=move || matches!(side,InsetSide::Top|InsetSide::Left)><p>"Content keeps the card's declared padding."</p></Show>
                    </Box>
                }).collect_view()}
            </div>
        </section>
    }
}

#[component]
fn Fixture() -> impl IntoView {
    rustify_components::provide_geometry_styles(GeometryStyles::default());
    let document = RwSignal::new(ThemeDocument::standard(
        "layout-fixture",
        "DOM layout and type",
    ));
    let resolved = Memo::new(move |_| {
        Arc::new(
            rustify_ui::resolve(
                &document.get(),
                ThemeMode::Light,
                &rustify_ui::theme::font_context(16., false),
            )
            .expect("layout fixture theme"),
        )
    });
    let invalid = RwSignal::new(false);
    let clicks = RwSignal::new(0);
    view! {
        <ThemeScope resolved=resolved/>
        <div class="p-4 space-y-6 bg-background text-foreground" data-testid="layout-body">
            <div class="flex flex-wrap gap-2">
                <Button test_id="layout-scale-100" on_click=move || document.update(|document| document.config.scaling = ThemeScaling::P100)>"Scale 100%"</Button>
                <Button test_id="layout-scale-110" on_click=move || document.update(|document| document.config.scaling = ThemeScaling::P110)>"Scale 110%"</Button>
                <Button test_id="layout-scale-95" on_click=move || document.update(|document| document.config.scaling = ThemeScaling::P95)>"Scale 95%"</Button>
                <Button test_id="layout-invalid" on_click=move || invalid.set(true)>"Invalid prop"</Button>
                <Button test_id="layout-valid" on_click=move || invalid.set(false)>"Restore prop"</Button>
            </div>
            <LayoutSamples invalid=invalid clicks=clicks/>
            <TypographySamples/>
        </div>
    }
}

#[wasm_bindgen]
pub fn catalog_layout_fixtures(root: &str) -> Result<(), JsValue> {
    catalog_layout_fixtures_dispose();
    let element = document()
        .get_element_by_id(root)
        .ok_or_else(|| JsValue::from_str("missing layout fixture root"))?
        .dyn_into::<leptos::web_sys::HtmlElement>()?;
    let handle = mount(
        element,
        MountConfig {
            scope: "catalog-layout".into(),
            ..Default::default()
        },
        || view! { <Fixture/> },
    )
    .map_err(|error| JsValue::from_str(&error.to_string()))?;
    FIXTURE.with(|fixture| *fixture.borrow_mut() = Some(handle));
    Ok(())
}

#[wasm_bindgen]
pub fn catalog_layout_fixtures_dispose() {
    FIXTURE.with(|fixture| {
        fixture.borrow_mut().take();
    });
}
