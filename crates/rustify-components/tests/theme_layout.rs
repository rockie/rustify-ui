#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]

use std::sync::Arc;

use leptos::prelude::*;
use rustify_components::*;
use rustify_ui::{
    resolve, AccentColor, FontResolution, Locale, MountConfig, ResolveContext, ResolvedTheme,
    ThemeDocument, ThemeMode,
};

struct ScopeFixture {
    html: String,
    geometry: String,
    theme: String,
}

struct Fixture {
    html: String,
    css: String,
    geometry: String,
}

fn theme_css(seed: &str, snapshot: &ResolvedTheme) -> String {
    let mut declarations = String::new();
    for (property, value) in snapshot.properties() {
        declarations.push_str(&format!("{property}:{value};"));
    }
    declarations.push_str("font-family:var(--rustify-font-sans);font-size:var(--font-size);line-height:1.5;color:var(--foreground);background-color:var(--background);");
    format!("[data-rustify-scope=\"{seed}\"]{{{declarations}}}\n")
}

fn scope_fixture(
    seed: &'static str,
    snapshot: Arc<ResolvedTheme>,
    counts: ThemeOverrideCounts,
) -> ScopeFixture {
    let styles = GeometryStyles::default();
    let owner = Owner::new();
    let mode = snapshot.mode;
    let config = snapshot.config;
    let theme = theme_css(seed, &snapshot);
    let html = owner.with(|| {
        rustify_ui::ssr_scope(
            MountConfig {
                scope: seed.into(),
                ..Default::default()
            },
            || {
                rustify_ui::provide_static_theme(snapshot.clone());
                provide_geometry_styles(styles.clone());
                provide_current_path(Signal::stored("index.html".to_string()));
                view! {
                    // This ordinary wrapper declares the static mode. It is
                    // not the browser-only ThemeBoundary component.
                    <div data-rustify-theme-boundary="static-fixture" data-theme=mode.as_str() data-testid=seed>
                        <Container size=Responsive { initial: Some(ContainerSize::One), sm: Some(ContainerSize::Two), md: Some(ContainerSize::Four), ..Default::default() }
                            test_id=format!("{seed}-container")>
                            <Section size=Responsive::from(SectionSize::One) test_id=format!("{seed}-section")>
                                <Flex direction=Responsive::from(FlexDirection::Column)
                                    gap=Gap { gap: Responsive::from(Spacing::Scale(5)), ..Default::default() }
                                    layout=Layout { padding: Padding { px: Responsive::from(Spacing::Scale(4)), ..Default::default() }, ..Default::default() }
                                    test_id=format!("{seed}-flex")>
                                    <Heading as_=HeadingLevel::H2
                                        size=Responsive { initial: Some(TextSize::Six), md: Some(TextSize::Eight), ..Default::default() }
                                        wrap=Responsive::from(TextWrap::Balance)
                                        test_id=format!("{seed}-heading")>
                                        {format!("{} theme · static SSR", mode.as_str())}
                                    </Heading>
                                    <Text as_=TextElement::P size=Responsive::from(TextSize::Three)
                                        wrap=Responsive::from(TextWrap::Pretty) test_id=format!("{seed}-text")>
                                        "This text, "<Em test_id=format!("{seed}-em")>"emphasis"</Em>
                                        ", "<Strong test_id=format!("{seed}-strong")>"strong text"</Strong>
                                        " and "<Quote test_id=format!("{seed}-quote")>"inline quotation"</Quote>
                                        " are readable without JavaScript."
                                    </Text>
                                    <Grid columns=Responsive { initial: Some(GridTracks::Count(1)), sm: Some(GridTracks::Count(2)), lg: Some(GridTracks::Count(3)), ..Default::default() }
                                        gap=Gap { gap: Responsive::from(Spacing::Scale(4)), ..Default::default() }
                                        test_id=format!("{seed}-grid")>
                                        <Box layout=Layout { padding: Padding { p: Responsive::from(Spacing::Scale(3)), ..Default::default() }, min_width: Responsive::from("0px".to_string()), ..Default::default() }
                                            test_id=format!("{seed}-box")>
                                            <Inset side=Responsive::from(InsetSide::X)
                                                padding=InsetPadding { px: Responsive::from(InsetPaddingValue::Current), ..Default::default() }
                                                test_id=format!("{seed}-inset")>
                                                <Blockquote size=Responsive::from(TextSize::Two) color=Some(AccentColor::Indigo)
                                                    test_id=format!("{seed}-blockquote")>
                                                    "One theme snapshot supplies these static values."
                                                </Blockquote>
                                            </Inset>
                                        </Box>
                                        <Box layout=Layout { width: Responsive::from("180px".to_string()), ..Default::default() }
                                            test_id=format!("{seed}-constrained")>
                                            <Text as_=TextElement::Div truncate=true test_id=format!("{seed}-truncate")>
                                                "A deliberately long line with a visible ellipsis in this narrow box."
                                            </Text>
                                            <Code wrap=Responsive::from(TextWrap::Wrap) test_id=format!("{seed}-long-code")>
                                                "long_identifier_that_needs_to_wrap_without_any_spaces_or_browser_scripts"
                                            </Code>
                                        </Box>
                                        <Box test_id=format!("{seed}-trim-comparison")>
                                            <Text as_=TextElement::Div size=Responsive::from(TextSize::Five)
                                                trim=Responsive::from(TextTrim::Normal) test_id=format!("{seed}-trim-normal")>
                                                "First line"<br/>"Last line"
                                            </Text>
                                            <Text as_=TextElement::Div size=Responsive::from(TextSize::Five)
                                                trim=Responsive::from(TextTrim::Both) test_id=format!("{seed}-trim-both")>
                                                "First line"<br/>"Last line"
                                            </Text>
                                        </Box>
                                    </Grid>
                                    <Flex wrap=Responsive::from(FlexWrap::Wrap)
                                        gap=Gap { gap: Responsive::from(Spacing::Scale(3)), ..Default::default() }
                                        align=Responsive::from(Align::Center) test_id=format!("{seed}-actions")>
                                        <Button on_click=|| {} test_id=format!("{seed}-button")>"Primary action"</Button>
                                        <Button on_click=|| {} loading=true test_id=format!("{seed}-loading")>"Saving"</Button>
                                        <Button on_click=|| {} variant=ButtonVariant::Outline color=Some(AccentColor::Red)
                                            high_contrast=true test_id=format!("{seed}-explicit-color")>"Local red"</Button>
                                        <IconButton on_click=|| {} aria_label="Close example" test_id=format!("{seed}-icon-button")>
                                            <Icon glyph=Glyph::Close/>
                                        </IconButton>
                                        <Link href="index.html" underline=LinkUnderline::Always
                                            attr:rel="bookmark" test_id=format!("{seed}-link")>"Native current link"</Link>
                                        <Link href="index.html" disabled=true test_id=format!("{seed}-disabled-link")>"Disabled destination"</Link>
                                        {Reset(leptos::html::button().attr("type", "button").attr("data-testid", format!("{seed}-reset")).child("Reset native element"))}
                                    </Flex>
                                    <Flex wrap=Responsive::from(FlexWrap::Wrap)
                                        gap=Gap { gap: Responsive::from(Spacing::Scale(3)), ..Default::default() }
                                        align=Responsive::from(Align::Center) test_id=format!("{seed}-code-variants")>
                                        <Code variant=CodeVariant::Solid test_id=format!("{seed}-code-solid")>"solid"</Code>
                                        <Code test_id=format!("{seed}-code-soft")>"soft"</Code>
                                        <Code variant=CodeVariant::Outline test_id=format!("{seed}-code-outline")>"outline"</Code>
                                        <Code variant=CodeVariant::Ghost test_id=format!("{seed}-code-ghost")>"ghost"</Code>
                                        <Kbd test_id=format!("{seed}-kbd")>"Enter"</Kbd>
                                        <Kbd variant=KbdVariant::Soft size=Responsive::from(TextSize::Three)
                                            test_id=format!("{seed}-kbd-soft")>"Shift"</Kbd>
                                    </Flex>
                                    <Box layout=Layout { max_width: Responsive::from("320px".to_string()), ..Default::default() }
                                        test_id=format!("{seed}-panel-container")>
                                        <ThemePanel config=config overrides=counts on_change=|_| {} on_reset=|_| {}
                                            locale=if mode == ThemeMode::Dark { Locale::Chinese } else { Locale::English }
                                            test_id=format!("{seed}-panel")/>
                                    </Box>
                                </Flex>
                            </Section>
                        </Container>
                    </div>
                }
            },
        )
        .to_html()
    });
    drop(owner);
    assert!(
        styles.diagnostics().is_empty(),
        "{:?}",
        styles.diagnostics()
    );
    let palette = rustify_ui::ResolvedPalette::for_family(AccentColor::Red.family(), mode).unwrap();
    let local_high_contrast =
        rustify_ui::theme::format_color(palette.solid[11], rustify_ui::theme::ColorFormat::Rgb);
    assert!(
        styles.css().contains(&format!(
            "--rustify-component-color-12:{local_high_contrast};"
        )),
        "{seed} must project its own explicit color mode"
    );
    ScopeFixture {
        html,
        geometry: styles.css(),
        theme,
    }
}

fn fixture() -> Fixture {
    let mut document = ThemeDocument::standard("ssr-components", "SSR components");
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        let values = document.base_values_mut(mode);
        for (token, value) in [
            ("font-sans", "system-ui, sans-serif"),
            ("font-serif", "serif"),
            ("font-mono", "monospace"),
        ] {
            values.0.insert(token.into(), value.into());
        }
    }
    // The default components must consume an effective semantic override,
    // rather than returning to the configured brand palette's ninth step.
    document
        .overrides
        .light
        .0
        .insert("primary".into(), "#7c3aed".into());
    document
        .overrides
        .dark
        .0
        .insert("primary".into(), "#a78bfa".into());
    let counts = ThemeOverrideCounts::from(&document.overrides);
    let context = ResolveContext {
        font_resolution: FontResolution::Css,
        ..Default::default()
    };
    let scopes = [
        scope_fixture(
            "ssr-light",
            Arc::new(resolve(&document, ThemeMode::Light, &context).unwrap()),
            counts,
        ),
        scope_fixture(
            "ssr-dark",
            Arc::new(resolve(&document, ThemeMode::Dark, &context).unwrap()),
            counts,
        ),
    ];
    let mut body = String::new();
    let mut geometry = String::new();
    let mut theme = String::new();
    for scope in scopes {
        body.push_str(&scope.html);
        geometry.push_str(&scope.geometry);
        theme.push_str(&scope.theme);
    }
    Fixture {
        html: format!("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>Rustify theme and layout SSR</title><link rel=\"icon\" href=\"data:,\"><link rel=\"stylesheet\" href=\"styles.css\"></head><body><button type=\"button\" data-testid=\"static-host\">Host sentinel</button><main id=\"fixtures\">{body}</main></body></html>"),
        css: format!("{}\n{theme}{geometry}", rustify_components::CSS),
        geometry,
    }
}

#[test]
fn external_theme_projection_preserves_scope_cascade_priority() {
    let rendered = fixture();
    let external = rendered.css[rustify_components::CSS.len()..].trim_start();
    // SDK fallback tokens are unlayered [data-rustify-scope] rules. A static
    // binding follows them with the same attribute specificity; component
    // skins and geometry keep their separate, lower-priority layer.
    assert!(
        external.starts_with("[data-rustify-scope=\"ssr-light\"]{"),
        "external theme bindings must be unlayered explicit scope rules after SDK CSS"
    );
    for seed in ["ssr-light", "ssr-dark"] {
        assert!(external.contains(&format!("[data-rustify-scope=\"{seed}\"]{{")));
    }
    assert!(external.contains("--primary:rgb(124 58 237);"));
    assert!(external.contains("--primary:rgb(167 139 250);"));
    assert!(rendered.geometry.starts_with("@layer components{"));
}

#[test]
fn static_theme_layout_is_deterministic_and_uses_only_external_styles() {
    let first = fixture();
    let second = fixture();
    assert_eq!(first.html, second.html);
    assert_eq!(first.geometry, second.geometry);
    assert_eq!(first.css, second.css);
    assert!(
        !first.html.contains(" style="),
        "inline style in SSR fixture"
    );
    assert!(!first.html.contains("<script"), "script in SSR fixture");
    assert!(
        !first.html.contains("<style"),
        "embedded CSS in SSR fixture"
    );
    assert!(!first.html.contains(".wasm"), "WASM asset in SSR fixture");
    assert!(
        !first.css.contains("data:font"),
        "embedded font in SSR fixture"
    );
    assert!(first.geometry.contains("@media (min-width:768px)"));
    assert!(first.geometry.contains("--rustify-component-color-12:"));
    for seed in ["ssr-light", "ssr-dark"] {
        assert!(first
            .html
            .contains(&format!("data-rustify-scope=\"{seed}\"")));
        assert!(first
            .css
            .contains(&format!("[data-rustify-scope=\"{seed}\"]")));
        for component in [
            "container",
            "section",
            "flex",
            "grid",
            "box",
            "inset",
            "reset",
            "heading",
            "text",
            "blockquote",
            "code-soft",
            "kbd",
            "em",
            "quote",
            "strong",
            "button",
            "icon-button",
            "link",
            "panel",
        ] {
            assert!(
                first
                    .html
                    .contains(&format!("data-testid=\"{seed}-{component}\"")),
                "{seed}-{component}"
            );
        }
    }
    assert!(first.html.contains("data-theme=\"light\""));
    assert!(first.html.contains("data-theme=\"dark\""));
    assert!(first.html.contains("aria-current=\"page\""));
    if let Some(directory) = std::env::var_os("RUSTIFY_COMPONENTS_SSR_OUT") {
        let directory = std::path::PathBuf::from(directory);
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(directory.join("index.html"), first.html).unwrap();
        std::fs::write(directory.join("styles.css"), first.css).unwrap();
    }
}
