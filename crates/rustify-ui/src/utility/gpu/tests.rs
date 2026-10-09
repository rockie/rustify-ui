use super::*;

#[test]
fn non_scalar_variable_values_are_visible_gpu_failures_without_replacing_the_previous_style() {
    let theme = theme();
    let mut context = context(&theme);
    context.variables.insert(
        "--probe".into(),
        rustify_style::text::Text::from_units(vec![0xd800]),
    );
    let before = context.base_style.clone();
    let resolved = resolve_gpu(
        &[css("width", 0, "width", "var(--probe)")],
        &theme,
        &context,
    );
    assert_eq!(resolved.style, before);
    assert_eq!(resolved.status, GpuResolutionStatus::Rejected);
    assert!(resolved
        .diagnostics
        .iter()
        .any(|item| item.reason.contains("non-scalar")));
}
use crate::theme::{font_context, resolve, Theme, ThemeDocument, ThemeMode};
use rustify_style::program::{GpuCondition, GpuDeclaration, GpuValue};
fn theme() -> ResolvedTheme {
    resolve(
        &ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
        ThemeMode::Light,
        &font_context(16., false),
    )
    .unwrap()
}
fn program(candidate: &str, order: usize, property: &str, value: GpuValue) -> GpuProgram {
    GpuProgram {
        candidate: candidate.into(),
        conditions: Vec::new(),
        declarations: vec![GpuDeclaration {
            property: property.into(),
            value,
            important: false,
        }],
        cascade_order: order,
    }
}
#[test]
fn compiler_order_and_importance_override_input_order() {
    let theme = theme();
    let context = GpuResolveContext::new(GpuComputedStyle::from_theme(&theme));
    let first = program(
        "later-in-input",
        1,
        "padding",
        GpuValue::Pixels { value: "4".into() },
    );
    let second = program(
        "earlier-in-input",
        2,
        "padding-left",
        GpuValue::Pixels { value: "12".into() },
    );
    let result = resolve_gpu(&[second, first], &theme, &context);
    assert_eq!(result.style.padding, [4., 4., 4., 12.]);
}

fn css(candidate: &str, order: usize, property: &str, value: &str) -> GpuProgram {
    program(
        candidate,
        order,
        property,
        GpuValue::CssExpression {
            value: value.into(),
        },
    )
}
fn context(theme: &ResolvedTheme) -> GpuResolveContext {
    GpuResolveContext::new(GpuComputedStyle::from_theme(theme))
}
fn applied(
    programs: &[GpuProgram],
    theme: &ResolvedTheme,
    context: &GpuResolveContext,
) -> GpuComputedStyle {
    let result = resolve_gpu(programs, theme, context);
    assert_eq!(
        result.status,
        GpuResolutionStatus::Applied,
        "{:?}",
        result.diagnostics
    );
    result.style
}
fn rejected(property: &str, value: &str) {
    let theme = theme();
    let result = resolve_gpu(&[css("test", 0, property, value)], &theme, &context(&theme));
    assert_eq!(
        result.status,
        GpuResolutionStatus::Rejected,
        "{property}: {value}"
    );
    assert!(!result.diagnostics.is_empty());
}
#[test]
fn importance_precedes_specificity_and_order() {
    let theme = theme();
    let mut context = context(&theme);
    context.state.hover = true;
    let mut important = css("important", 0, "padding", "8px");
    important.declarations[0].important = true;
    let mut specific = css("hover", 20, "padding", "12px");
    specific.conditions.push(GpuCondition::Hover);
    assert_eq!(
        applied(&[specific, important], &theme, &context).padding,
        [8.; 4]
    );
}
#[test]
fn self_specificity_precedes_order_but_where_has_zero_weight() {
    let theme = theme();
    let mut context = context(&theme);
    context.state.hover = true;
    let mut specific = css("hover", 0, "padding", "12px");
    specific.conditions.push(GpuCondition::Selector {
        selector: "&:hover".into(),
    });
    let mut weak = css("where", 9, "padding", "2px");
    weak.conditions.push(GpuCondition::Selector {
        selector: "&:where(:hover)".into(),
    });
    assert_eq!(
        applied(&[weak, specific], &theme, &context).padding,
        [12.; 4]
    );
}
#[test]
fn colors_alpha_current_color_and_explicit_spacing() {
    let theme = theme();
    let mut context = context(&theme);
    context
        .variables
        .insert("--accent".into(), "rgb(18 52 86 / 50%)".into());
    context
        .variables
        .insert("--spacing".into(), "0.25rem".into());
    let style = applied(
        &[
            css("color", 0, "color", "var(--accent)"),
            css("background", 1, "background-color", "currentColor"),
            css("spacing", 2, "padding", "calc(var(--spacing) * 3)"),
            css("border", 3, "border-color", "var(--missing, #12345680)"),
        ],
        &theme,
        &context,
    );
    assert_eq!(style.background, style.color);
    assert_eq!(style.color.a, 0.5);
    assert_eq!(style.padding, [12.; 4]);
    assert!((style.border_colors[0].a - 128. / 255.).abs() < 1e-12);
    let missing = resolve_gpu(
        &[css("spacing", 0, "padding", "var(--spacing)")],
        &theme,
        &GpuResolveContext::new(GpuComputedStyle::from_theme(&theme)),
    );
    assert_eq!(missing.status, GpuResolutionStatus::Rejected);
}
#[test]
fn theme_raw_values_recompute_and_context_overrides() {
    let mut theme = theme();
    theme
        .values
        .0
        .insert("custom-color".into(), "#123456".into());
    let mut context = context(&theme);
    let programs = [css("custom", 0, "background-color", "var(--custom-color)")];
    let first = applied(&programs, &theme, &context);
    theme
        .values
        .0
        .insert("custom-color".into(), "#654321".into());
    assert_ne!(
        first.background,
        applied(&programs, &theme, &context).background
    );
    context
        .variables
        .insert("--custom-color".into(), "#fff".into());
    assert_eq!(applied(&programs, &theme, &context).background.r, 1.);
}
#[test]
fn custom_property_cascade_cycles_and_fallbacks() {
    let theme = theme();
    let context = context(&theme);
    let programs = [
        css("a", 0, "--a", "var(--b, 4px)"),
        css("b", 1, "--b", "var(--a)"),
        css("pad", 2, "padding", "var(--a, 7px)"),
    ];
    assert_eq!(applied(&programs, &theme, &context).padding, [7.; 4]);
    let mut important = css("important", 0, "--space", "8px");
    important.declarations[0].important = true;
    assert_eq!(
        applied(
            &[
                important,
                css("later", 10, "--space", "2px"),
                css("pad", 11, "padding", "var(--space)")
            ],
            &theme,
            &context
        )
        .padding,
        [8.; 4]
    );
    let mut context = context;
    for index in 0..70 {
        context.variables.insert(
            format!("--v{index}").into(),
            format!("var(--v{})", index + 1).into(),
        );
    }
    context.variables.insert("--v70".into(), "4px".into());
    assert_eq!(
        resolve_gpu(
            &[css("limit", 0, "padding", "var(--v0, 2px)")],
            &theme,
            &context
        )
        .status,
        GpuResolutionStatus::Rejected
    );
}
#[test]
fn logical_padding_shorthands_and_percentages_use_parent_width() {
    let theme = theme();
    let mut context = context(&theme);
    context.direction = Direction::RightToLeft;
    context.parent_size = [Some(200.), Some(100.)];
    let style = applied(
        &[
            css("base", 0, "padding", "1px 2px 3px 4px"),
            css("axis", 1, "padding-inline", "5% 2rem"),
            css("top", 2, "padding-block-start", "3px"),
        ],
        &theme,
        &context,
    );
    assert_eq!(style.padding, [3., 10., 3., 32.]);
}
#[test]
fn dimensions_auto_min_max_and_gap_reference_sizes() {
    let theme = theme();
    let mut context = context(&theme);
    context.parent_size = [Some(300.), Some(200.)];
    context.control_size = [Some(200.), Some(100.)];
    let style = applied(
        &[
            css("w", 0, "width", "50%"),
            css("min", 1, "min-width", "160px"),
            css("max", 2, "max-width", "140px"),
            css("h", 3, "height", "auto"),
            css("gap", 4, "gap", "10% 5%"),
        ],
        &theme,
        &context,
    );
    assert_eq!(style.width, GpuLength::Pixels(160.));
    assert_eq!(style.height, GpuLength::Auto);
    assert_eq!(style.gap, [10., 8.]);
    context.parent_size = [None, None];
    assert_eq!(
        resolve_gpu(&[css("percent", 0, "width", "50%")], &theme, &context).status,
        GpuResolutionStatus::Rejected
    );
}
#[test]
fn flex_display_does_not_override_direction() {
    let theme = theme();
    let style = applied(
        &[
            css("row", 0, "flex-direction", "row"),
            css("flex", 9, "display", "flex"),
            css("items", 10, "align-items", "center"),
            css("justify", 11, "justify-content", "space-between"),
        ],
        &theme,
        &context(&theme),
    );
    assert_eq!(style.flow, GpuFlow::Row);
    assert_eq!(style.items, GpuAlignment::Center);
    assert_eq!(style.justify, GpuJustification::SpaceBetween);
    for (property, value) in [
        ("flex-wrap", "wrap"),
        ("flex-shrink", "0"),
        ("flex-basis", "10px"),
        ("flex-direction", "row-reverse"),
        ("display", "grid"),
    ] {
        rejected(property, value);
    }
}
#[test]
fn uniform_border_radius_opacity_and_capability_rejection() {
    let theme = theme();
    let mut context = context(&theme);
    context.control_size = [Some(20.), Some(20.)];
    let style = applied(
        &[
            css("rounded", 0, "border-radius", "100px"),
            css("border", 1, "border", "2px solid #123456"),
            css("alpha", 2, "opacity", "50%"),
        ],
        &theme,
        &context,
    );
    assert_eq!(style.radii, [10.; 4]);
    assert_eq!(style.border_widths, [2.; 4]);
    assert_eq!(style.opacity, 0.5);
    for (property, value) in [
        ("border-radius", "2px 4px"),
        ("border-top-width", "3px"),
        ("border-radius", "2px / 4px"),
        ("border-style", "dashed"),
    ] {
        rejected(property, value);
    }
    context.capabilities.independent_leaf_opacity = false;
    assert_eq!(
        resolve_gpu(&[css("opacity", 0, "opacity", "0.5")], &theme, &context).status,
        GpuResolutionStatus::Rejected
    );
}
#[test]
fn text_reuses_packaged_faces_and_css_point_conversion() {
    let theme = theme();
    let mut context = context(&theme);
    context.parent_font_size_px = 20.;
    context.rem_px = 18.;
    let style = applied(
        &[
            css("family", 0, "font-family", "sans-serif"),
            css("size", 1, "font-size", "1.5rem"),
            css("leading", 2, "line-height", "1.5"),
            css("tracking", 3, "letter-spacing", "0.1em"),
            css("weight", 4, "font-weight", "400"),
        ],
        &theme,
        &context,
    );
    assert_eq!(style.font_size_px, 27.);
    assert_eq!(style.font_size_points, crate::theme::css_px_to_points(27.));
    assert_eq!(style.line_height, GpuLineHeight::Pixels(40.5));
    assert_eq!(style.letter_spacing_em, 0.1);
    assert!(style.font.resource_path.is_some());
    assert_eq!(
        resolve_gpu(&[css("weight", 0, "font-weight", "700")], &theme, &context).status,
        GpuResolutionStatus::Rejected
    );
    let family = theme.fonts[0].family.clone();
    context.fonts.push(GpuFontFace {
        family,
        weight: 700,
        resource_path: "packaged-bold.ttf".into(),
    });
    assert_eq!(
        applied(&[css("weight", 0, "font-weight", "700")], &theme, &context).font_weight,
        700
    );
}
#[test]
fn outer_shadows_and_ring_preserve_css_layer_order() {
    let theme = theme();
    let context = context(&theme);
    let style = applied(
        &[
            css("offset", 0, "--ring-offset", "0 0 0 2px #fff"),
            css("ring", 1, "--ring", "0 0 0 calc(2px + 3px) #123456"),
            css(
                "shadow",
                2,
                "box-shadow",
                "var(--ring-offset), var(--ring), 1px 2px 3px -1px rgb(0 0 0 / 50%)",
            ),
        ],
        &theme,
        &context,
    );
    assert_eq!(style.shadows.len(), 3);
    assert_eq!(style.shadows[0].spread, 2.);
    assert_eq!(style.shadows[1].spread, 5.);
    assert_eq!(style.shadows[2].color.a, 0.5);
    rejected("box-shadow", "inset 0 0 2px black");
}
#[test]
fn states_theme_and_focus_visible_are_explicit() {
    let mut theme = theme();
    let mut context = context(&theme);
    let condition = |condition: GpuCondition| {
        let mut program = css("state", 0, "padding", "8px");
        program.conditions.push(condition);
        program
    };
    for cond in [
        GpuCondition::Hover,
        GpuCondition::Active,
        GpuCondition::Focus,
        GpuCondition::FocusVisible,
        GpuCondition::Disabled,
        GpuCondition::Dark,
    ] {
        assert_eq!(
            applied(&[condition(cond)], &theme, &context).padding,
            [0.; 4]
        );
    }
    context.state = GpuControlState {
        hover: true,
        active: true,
        focused: true,
        focus_visible: true,
        disabled: true,
        modality: InputModality::Keyboard,
    };
    for cond in [
        GpuCondition::Hover,
        GpuCondition::Active,
        GpuCondition::Focus,
        GpuCondition::FocusVisible,
        GpuCondition::Disabled,
    ] {
        assert_eq!(
            applied(&[condition(cond)], &theme, &context).padding,
            [8.; 4]
        );
    }
    context.state.modality = InputModality::Pointer;
    assert_eq!(
        applied(&[condition(GpuCondition::FocusVisible)], &theme, &context).padding,
        [0.; 4]
    );
    theme.mode = ThemeMode::Dark;
    assert_eq!(
        applied(&[condition(GpuCondition::Dark)], &theme, &context).padding,
        [8.; 4]
    );
}
#[test]
fn responsive_viewport_uses_media_rem_not_control_or_layout_rem() {
    let theme = theme();
    let mut context = context(&theme);
    context.viewport = [639., 400.];
    context.control_size = [Some(1000.), Some(900.)];
    context.rem_px = 30.;
    context.media_rem_px = 16.;
    let mut program = css("responsive", 0, "padding", "1rem");
    program.conditions.push(GpuCondition::Media {
        query: "(width >= 40rem) and (orientation: landscape)".into(),
    });
    assert_eq!(
        applied(&[program.clone()], &theme, &context).padding,
        [0.; 4]
    );
    context.viewport[0] = 640.;
    assert_eq!(applied(&[program], &theme, &context).padding, [30.; 4]);
    for query in [
        "(width == 640px)",
        "(400px < width > 800px)",
        "(prefers-color-scheme: dark)",
        "(container-width > 0px)",
    ] {
        let mut program = css("unsupported", 0, "padding", "4px");
        program.conditions.push(GpuCondition::Media {
            query: query.into(),
        });
        assert_eq!(
            resolve_gpu(&[program], &theme, &context).status,
            GpuResolutionStatus::Rejected,
            "{query}"
        );
    }
}
#[test]
fn unsupported_variants_properties_empty_ir_and_values_are_visible() {
    let theme = theme();
    let context = context(&theme);
    for selector in ["& > *", ".group:hover &", "&::before", "&:has(.child)"] {
        let mut program = css("unknown", 0, "padding", "2px");
        program.conditions.push(GpuCondition::Selector {
            selector: selector.into(),
        });
        let result = resolve_gpu(&[program], &theme, &context);
        assert_eq!(result.status, GpuResolutionStatus::Rejected);
        assert_eq!(
            result.diagnostics[0].kind,
            GpuDiagnosticKind::UnsupportedVariant
        );
    }
    let mut empty = css("empty", 0, "padding", "2px");
    empty.declarations.clear();
    assert_eq!(
        resolve_gpu(&[empty], &theme, &context).status,
        GpuResolutionStatus::Rejected
    );
    for (property, value) in [
        ("padding", "NaNpx"),
        ("width", "infinity"),
        ("padding", "calc(2px / 0)"),
        ("padding", "2px+3px"),
        ("padding", "calc(2px+3px)"),
        ("padding", "(2px)"),
        ("color", "color-mix(in srgb, red, blue)"),
        ("transform", "translateX(2px)"),
        ("overflow", "hidden"),
        ("animation", "spin 1s"),
    ] {
        rejected(property, value);
    }
}
#[test]
fn strict_preserves_previous_partial_applies_supported_and_removal_restores() {
    let theme = theme();
    let mut context = context(&theme);
    let first = applied(&[css("padding", 0, "padding", "8px")], &theme, &context);
    context.previous_style = Some(first.clone());
    let programs = [
        css("padding", 0, "padding", "12px"),
        css("grid", 1, "display", "grid"),
    ];
    let strict = resolve_gpu(&programs, &theme, &context);
    assert_eq!(strict.status, GpuResolutionStatus::Rejected);
    assert_eq!(strict.style, first);
    assert!(!strict.changed);
    context.policy = GpuApplyPolicy::Partial;
    let partial = resolve_gpu(&programs, &theme, &context);
    assert_eq!(partial.status, GpuResolutionStatus::Partial);
    assert_eq!(partial.style.padding, [12.; 4]);
    assert!(partial.changed);
    context.previous_style = Some(partial.style);
    let removed = resolve_gpu(&[], &theme, &context);
    assert_eq!(removed.style, context.base_style);
    assert!(removed.changed);
    context.previous_style = Some(removed.style);
    assert!(!resolve_gpu(&[], &theme, &context).changed);
    let mut hover = css("hover", 0, "padding", "4px");
    hover.conditions.push(GpuCondition::Hover);
    context.state.hover = true;
    context.previous_style = Some(applied(&[hover.clone()], &theme, &context));
    context.state.hover = false;
    assert_eq!(
        resolve_gpu(&[hover], &theme, &context).style.padding,
        context.base_style.padding
    );
}
#[test]
fn invalid_measurement_context_rejects_even_partial() {
    let theme = theme();
    let mut context = context(&theme);
    context.policy = GpuApplyPolicy::Partial;
    for value in [f64::NAN, f64::INFINITY, -1., f64::MAX] {
        context.viewport[0] = value;
        let result = resolve_gpu(&[], &theme, &context);
        assert_eq!(result.status, GpuResolutionStatus::Rejected);
        assert_eq!(
            result.diagnostics[0].kind,
            GpuDiagnosticKind::InvalidContext
        );
    }
}

#[test]
fn unchanged_font_render_values_do_not_request_redraw() {
    let theme = theme();
    let mut context = context(&theme);
    let previous = applied(
        &[css("font", 0, "font-family", "sans-serif")],
        &theme,
        &context,
    );
    context.previous_style = Some(previous);
    let result = resolve_gpu(
        &[css(
            "font",
            0,
            "font-family",
            &format!("\"{}\"", theme.fonts[0].family),
        )],
        &theme,
        &context,
    );
    assert_eq!(result.status, GpuResolutionStatus::Applied);
    assert!(!result.changed);
}
#[test]
fn invalid_base_snapshot_rejects_nonfinite_color_or_size() {
    let theme = theme();
    let mut context = context(&theme);
    context.base_style.color.r = f64::NAN;
    assert_eq!(
        resolve_gpu(&[], &theme, &context).status,
        GpuResolutionStatus::Rejected
    );
    context.base_style = GpuComputedStyle::from_theme(&theme);
    context.base_style.padding[0] = f64::INFINITY;
    assert_eq!(
        resolve_gpu(&[], &theme, &context).status,
        GpuResolutionStatus::Rejected
    );
}
#[test]
fn font_weight_percentage_is_not_a_weight() {
    rejected("font-weight", "40000%");
}
#[test]
fn expanded_variables_have_bounded_output() {
    let theme = theme();
    let mut context = context(&theme);
    for index in 0..30 {
        context.variables.insert(
            format!("--v{index}").into(),
            format!("var(--v{})var(--v{})", index + 1, index + 1).into(),
        );
    }
    context.variables.insert("--v30".into(), "0".into());
    assert_eq!(
        resolve_gpu(&[css("bound", 0, "width", "var(--v0)")], &theme, &context).status,
        GpuResolutionStatus::Rejected
    );
}

#[test]
fn compiler_transparent_oklab_mix_multiplies_alpha() {
    let theme = theme();
    let mut context = context(&theme);
    context
        .variables
        .insert("--color-brand".into(), "rgb(18 52 86 / 50%)".into());
    let style = applied(
        &[css(
            "alpha",
            0,
            "background-color",
            "color-mix(in oklab, var(--color-brand) 50%, transparent)",
        )],
        &theme,
        &context,
    );
    assert_eq!(style.background.a, 0.25);
    assert_eq!(style.background.r, 18. / 255.);
    rejected("color", "color-mix(in oklab, #123456 50%, #fff)");
}

#[test]
fn empty_program_set_restores_exact_widget_radius_baseline() {
    let theme = theme();
    let mut context = context(&theme);
    context.base_style.radii = [40.; 4];
    context.control_size = [Some(20.), Some(20.)];
    assert_eq!(resolve_gpu(&[], &theme, &context).style, context.base_style);
}

#[path = "../../gpu/utility/model.rs"]
mod widget_model;

#[path = "../../gpu/button/utility.rs"]
mod button_utility;
