//! Integration contract consumed by compiler_test_runner.py's temporary workspace.
#[cfg(test)]
mod tests {
    use rustify_style::compiler::{Compiler, CompilerInput, Limits};
    use rustify_style::program::GpuProgram;
    use rustify_ui::{
        theme::{font_context, resolve, ResolvedTheme, Theme, ThemeDocument, ThemeMode},
        utility::gpu::*,
    };
    fn theme() -> ResolvedTheme {
        resolve(
            &ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
            ThemeMode::Light,
            &font_context(16., false),
        )
        .unwrap()
    }
    fn programs(classes: &[&str]) -> (Vec<GpuProgram>, GpuResolveContext, ResolvedTheme) {
        let theme = theme();
        let mut context = GpuResolveContext::new(GpuComputedStyle::from_theme(&theme));
        let mut compiler=Compiler::new(CompilerInput{entry:"input.css".into(),css:format!("@theme {{ --spacing: 0.25rem; --color-brand: #123456; --font-sans: ui-sans-serif, system-ui, sans-serif; --text-base: 1rem; --text-base--line-height: 1.5; --font-weight-normal: 400; }}\n@tailwind utilities;").into()},vec![],Limits{input_bytes:256_000,asset_bytes:256_000,candidate_count:256,ast_depth:64,css_bytes:256_000,queued_bytes:256_000,cache_bytes:256_000}).unwrap();
        let prepared = compiler
            .prepare_add(
                &classes
                    .iter()
                    .map(|class| class.to_string())
                    .collect::<Vec<_>>(),
            )
            .unwrap();
        assert!(
            prepared.report().rejected.is_empty() && prepared.report().unsupported.is_empty(),
            "{:?}",
            (
                prepared.report().rejected.clone(),
                prepared.report().unsupported.clone()
            )
        );
        context.variables = prepared.report().gpu_variables.clone();
        context.viewport = [800., 600.];
        context.control_size = [Some(160.), Some(40.)];
        context.parent_size = [Some(320.), Some(200.)];
        (prepared.report().gpu_programs.clone(), context, theme)
    }
    fn applied(classes: &[&str]) -> GpuComputedStyle {
        let (programs, context, theme) = programs(classes);
        let result = resolve_gpu(&programs, &theme, &context);
        assert_eq!(
            result.status,
            GpuResolutionStatus::Applied,
            "classes {classes:?}: {:?}; programs {programs:?}",
            result.diagnostics
        );
        result.style
    }
    #[test]
    fn real_static_functional_and_arbitrary_layout_ir() {
        let style = applied(&[
            "flex",
            "flex-row",
            "items-center",
            "justify-between",
            "p-4",
            "px-2",
            "gap-2",
            "w-[137px]",
            "h-[40px]",
            "rounded-[4px]",
            "[border-width:2px]",
            "[border-style:solid]",
            "border-[#123456]",
            "[opacity:0.5]",
        ]);
        assert_eq!(style.flow, GpuFlow::Row);
        assert_eq!(style.items, GpuAlignment::Center);
        assert_eq!(style.justify, GpuJustification::SpaceBetween);
        assert_eq!(style.padding, [16., 8., 16., 8.]);
        assert_eq!(style.gap, [8.; 2]);
        assert_eq!(style.width, GpuLength::Pixels(137.));
        assert_eq!(style.radii, [4.; 4]);
        assert_eq!(style.border_widths, [2.; 4]);
        assert_eq!(style.opacity, 0.5);
    }
    #[test]
    fn real_theme_alpha_current_color_and_transparent() {
        let style = applied(&[
            "bg-brand/50",
            "text-[#654321]",
            "border-current",
            "[border-width:2px]",
            "[border-style:solid]",
        ]);
        assert_eq!(style.background.a, 0.5);
        assert_eq!(style.background.r, 18. / 255.);
        assert_eq!(style.border_colors, [style.color; 4]);
        assert_eq!(applied(&["bg-transparent"]).background.a, 0.);
    }
    #[test]
    fn real_ring_shadow_ir_uses_compiler_property_initializers() {
        let style = applied(&["ring-2", "ring-[#123456]", "shadow-[1px_2px_3px_#0008]"]);
        assert_eq!(style.shadows.len(), 2);
        assert_eq!(style.shadows[0].spread, 2.);
        assert_eq!(style.shadows[1].blur, 3.);
    }
    #[test]
    fn real_font_and_line_height_ir() {
        let style = applied(&[
            "[font-family:ui-sans-serif,_sans-serif]",
            "[font-size:1rem]",
            "[line-height:1.5]",
            "[letter-spacing:0.1em]",
            "[font-weight:400]",
        ]);
        assert_eq!(style.font_size_px, 16.);
        assert_eq!(style.font_weight, 400);
        assert_eq!(style.line_height, GpuLineHeight::Pixels(24.));
        assert_eq!(style.letter_spacing_em, 0.1);
    }
    #[test]
    fn real_state_and_arbitrary_media_programs_restore_baseline() {
        let (programs, mut context, theme) = programs(&[
            "p-2",
            "hover:p-4",
            "active:pl-6",
            "[@media(width>500px)]:py-3",
        ]);
        let result = resolve_gpu(&programs, &theme, &context);
        assert_eq!(
            result.status,
            GpuResolutionStatus::Applied,
            "{:?}",
            result.diagnostics
        );
        assert_eq!(result.style.padding, [12., 8., 12., 8.]);
        context.state.hover = true;
        context.state.active = true;
        assert_eq!(
            resolve_gpu(&programs, &theme, &context).style.padding,
            [16., 16., 16., 24.]
        );
        context.viewport[0] = 400.;
        context.state.hover = false;
        context.state.active = false;
        assert_eq!(
            resolve_gpu(&programs, &theme, &context).style.padding,
            [8.; 4]
        );
    }
    #[test]
    fn real_unsupported_ir_preserves_previous_or_reports_partial() {
        let (programs, mut context, theme) = programs(&[
            "p-4",
            "grid",
            "group-hover:bg-brand",
            "divide-brand",
            "placeholder-brand",
        ]);
        let mut previous = context.base_style.clone();
        previous.padding = [6.; 4];
        context.previous_style = Some(previous.clone());
        let strict = resolve_gpu(&programs, &theme, &context);
        assert_eq!(strict.status, GpuResolutionStatus::Rejected);
        assert_eq!(strict.style, previous);
        assert!(!strict.diagnostics.is_empty());
        context.policy = GpuApplyPolicy::Partial;
        let partial = resolve_gpu(&programs, &theme, &context);
        assert_eq!(partial.status, GpuResolutionStatus::Partial);
        assert_eq!(partial.style.padding, [16.; 4]);
    }
}
