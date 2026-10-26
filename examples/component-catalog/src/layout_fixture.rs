//! DOM-only layout and typography evidence, mounted separately from the GPU catalogue.

use leptos::prelude::*;
use leptos::wasm_bindgen::{prelude::*, JsCast};
use rustify_components::layout::*;
use rustify_components::typography::*;
use rustify_components::{Button, GeometryStyles, Link, LinkUnderline, Responsive};
use rustify_ui::{
    mount, AppHandle, MountConfig, ThemeDocument, ThemeMode, ThemeScaling, ThemeScope,
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
            <p><Code variant=CodeVariant::Outline size=Responsive::from(TextSize::Three) test_id="type-code">"let layout = Responsive::from(value);"</Code></p>
            <p><Kbd variant=KbdVariant::Soft size=Responsive::from(TextSize::Two) test_id="type-kbd">"Ctrl + K"</Kbd></p>
            <p><Em test_id="type-em">"Emphasis"</Em>" · "<Quote test_id="type-quote">"Native quotation marks"</Quote>" · "<Strong test_id="type-strong">"Strong importance"</Strong></p>
            <Text as_=TextElement::Div size=Responsive::from(TextSize::Nine) class="text-sm" test_id="type-class-override">"A caller utility controls this size."</Text>
            <Text as_=TextElement::Div class="w-40" truncate=true test_id="type-truncate">"This long sentence is truncated deliberately without changing its accessible text."</Text>
            <Text as_=TextElement::Div margin=Margin { ml: space(2), ..Margin::default() } test_id="type-margin">"Shared margins follow theme space."</Text>
            <Link href="#layout-target" size=Responsive::from(TextSize::Four) underline=LinkUnderline::Always attr:target="_blank" attr:rel="noreferrer" test_id="type-link">"Open a native link"</Link>
            <Link href="#layout-target" class="block w-40" truncate=true test_id="type-long-link">"https://example.test/a/long/path/whose/native/link/name/is/preserved"</Link>
            <Link href="#layout-target" disabled=true test_id="type-disabled-link">"Unavailable destination"</Link>
            <div id="layout-target" data-testid="layout-link-target">"Native link destination"</div>
        </div>
    }
}

#[component]
fn LayoutSamples(invalid: RwSignal<bool>, clicks: RwSignal<u32>) -> impl IntoView {
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
            .class("p-2 text-primary")
            .on(leptos::ev::click, move |_| {
                clicks.update(|value| *value += 1)
            })
            .child("Native reset action"),
    );
    view! {
        <div class="space-y-4" data-testid="layout-primitives">
            <Heading as_=HeadingLevel::H2>"Layout primitives"</Heading>
            <Box as_=LayoutElement::Span test_id="layout-box-default">"A semantic span starts as a block."</Box>
            <Box test_id="layout-responsive-width" layout=Layout {
                width: Responsive { initial: Some("10px".into()), xs: Some("20px".into()), sm: Some("30px".into()), md: Some("40px".into()), lg: Some("50px".into()), xl: Some("60px".into()) },
                height: css("8px"), ..Layout::default()
            } class="bg-primary">{ "" }</Box>
            <Box as_=LayoutElement::Span test_id="layout-sparse-display" display=Responsive { md: Some(BoxDisplay::None), lg: Some(BoxDisplay::Block), ..Responsive::default() }>"Sparse display keeps the component default before md."</Box>
            // Keep the margin sample below the parent's space-y utility target.
            <div class="flow-root">
                <Box test_id="layout-shared" layout=shared margin=Margin { m: space(2), mx: space(3), my: space(4), mt: space(5), mr: space(6), mb: space(7), ml: space(8) } class="bg-muted">"Shared fields"</Box>
            </div>
            <Box test_id="layout-class-override" layout=Layout { width: css("37px"), padding: Padding { p: space(4), ..Padding::default() }, ..Layout::default() } class="w-20 p-0">"Utilities"</Box>
            <Flex test_id="layout-flex" as_=LayoutElement::Span
                direction=Responsive { initial: Some(FlexDirection::Row), sm: Some(FlexDirection::Column), lg: Some(FlexDirection::RowReverse), ..Responsive::default() }
                align=Responsive::from(Align::Center) justify=Responsive::from(Justify::Between) wrap=Responsive::from(FlexWrap::Wrap)
                gap=Gap { gap: space(3), gap_x: space(2), gap_y: space(4) }>
                <span>"First"</span><span>"Second"</span><span>"Third"</span>
            </Flex>
            <Grid test_id="layout-grid" columns=Responsive::from(GridTracks::Count(3)) rows=Responsive::from(GridTracks::Css("auto 1fr".into()))
                areas=css("\"a a b\" \"c d b\"") flow=Responsive::from(GridFlow::RowDense)
                align=Responsive::from(Align::Stretch) justify=Responsive::from(Justify::Between)
                align_content=Responsive::from(AlignContent::Around) justify_items=Responsive::from(Align::Center)
                gap=Gap { gap: space(2), gap_x: space(3), gap_y: space(4) }>
                <span>"One"</span><span>"Two"</span><span>"Three"</span><span>"Four"</span><span>"Five"</span>
            </Grid>
            <Container test_id="layout-container" size=Responsive { initial: Some(ContainerSize::One), sm: Some(ContainerSize::Two), md: Some(ContainerSize::Three), lg: Some(ContainerSize::Four), ..Responsive::default() }
                align=Responsive { initial: Some(ContainerAlign::Left), md: Some(ContainerAlign::Right), ..Responsive::default() }>
                <div class="bg-muted p-2">"The inner content has a responsive maximum width."</div>
            </Container>
            <Container test_id="layout-container-projection" layout=Layout { width: css("240px"), min_width: css("120px"), max_width: css("280px"), height: css("72px"), min_height: css("40px"), max_height: css("90px"), ..Layout::default() }>
                "Dimensions describe this content layer."
            </Container>
            <Section test_id="layout-section" size=Responsive { initial: Some(SectionSize::One), xs: Some(SectionSize::Two), md: Some(SectionSize::Three), xl: Some(SectionSize::Four), ..Responsive::default() } class="bg-muted">"Semantic section spacing"</Section>
            <Box test_id="layout-inset-outer" layout=Layout { width: css("300px"), padding: Padding { p: space(6), ..Padding::default() }, ..Layout::default() } class="bg-muted">
                <Box test_id="layout-inset-nearest" layout=Layout { padding: Padding { p: space(2), ..Padding::default() }, ..Layout::default() } class="bg-card">
                    <Inset test_id="layout-inset" padding=InsetPadding { p: InsetPaddingValue::Current.into(), px: InsetPaddingValue::Zero.into(), ..InsetPadding::default() }>
                        <div><Inset test_id="layout-inset-nested">"Nearest padding is consumed once."</Inset></div>
                    </Inset>
                </Box>
            </Box>
            <Box test_id="layout-scaled-space" layout=Layout { padding: Padding { p: space(4), ..Padding::default() }, ..Layout::default() }>"Theme space scales once."</Box>
            <Box test_id="layout-absolute" layout=Layout { width: css("37px"), height: css("19px"), padding: Padding { p: Spacing::Css("3px".into()).into(), ..Padding::default() }, ..Layout::default() }>"px"</Box>
            <Box test_id="layout-invalid-prop" layout=Signal::derive(move || Layout { width: css(if invalid.get() { "12px;display:none" } else { "37px" }), ..Layout::default() })>"A bad prop keeps this view alive."</Box>
            <div>{reset_button}<output data-testid="layout-reset-count">{move || clicks.get()}</output></div>
        </div>
    }
}

#[component]
fn Fixture() -> impl IntoView {
    let styles = GeometryStyles::default();
    rustify_components::provide_geometry_styles(styles);
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
