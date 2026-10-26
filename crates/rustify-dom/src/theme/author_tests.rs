use super::*;
use std::sync::Arc;

fn context() -> ResolveContext<'static> {
    ResolveContext::default()
}
fn legacy() -> ThemeDocument {
    ThemeDocument::from_legacy(Theme::light(), Theme::dark())
}
fn standard() -> ThemeDocument {
    ThemeDocument::standard("brand", "Brand")
}
fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-9, "{actual} != {expected}");
}

fn v1_json() -> serde_json::Value {
    let document = legacy();
    serde_json::json!({"schema_version":1,"id":document.id,"name":document.name,"styles":document.base})
}

#[test]
fn v1_upgrade_keeps_both_tables_unknown_strings_and_only_configuration_responsibility_overrides() {
    let mut wire = v1_json();
    wire["styles"]["light"]["future-token"] = "var(--uninterpreted)".into();
    wire["styles"]["dark"]["font-sans"] = "Different Dark Face, serif".into();
    let imported = import_json(&wire.to_string(), &context()).unwrap().document;
    assert_eq!(imported.schema_version, 2);
    assert_eq!(imported.config, ThemeConfig::custom());
    assert_eq!(
        serde_json::to_value(&imported.base).unwrap(),
        wire["styles"]
    );
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        let overrides = imported.overrides.values(mode);
        assert_eq!(overrides.0.len(), 24);
        for group in ThemeConfigGroup::ALL {
            for token in group.tokens() {
                assert_eq!(
                    overrides.0[*token],
                    imported.base_values(mode).get(token).unwrap()
                );
            }
        }
        for token in [
            "accent",
            "shadow-color",
            "font-sans",
            "future-token",
            "chart-1",
        ] {
            assert!(!overrides.0.contains_key(token));
        }
    }
    let canonical = export_json(&imported, &context()).unwrap();
    assert!(canonical.contains("\"schema_version\":2"));
    assert!(!canonical.contains("\"config\""));
    assert_eq!(
        import_json(&canonical, &context()).unwrap().document,
        imported
    );
    let rendered = resolve(&imported, ThemeMode::Light, &context()).unwrap();
    assert!(!rendered
        .properties()
        .iter()
        .any(|(key, _)| key.contains("future-token")));
}

#[test]
fn full_intent_round_trips_configuration_base_and_each_modes_sparse_overrides() {
    let mut document = standard();
    document.config.appearance = Appearance::Dark;
    document.config.accent_color = Some(AccentColor::Amber);
    document.config.scaling = ThemeScaling::P105;
    document
        .base
        .light
        .0
        .insert("future".into(), "original author string".into());
    document
        .overrides
        .light
        .0
        .insert("primary".into(), "oklch(.7 .2 20 / .5)".into());
    document
        .overrides
        .dark
        .0
        .insert("spacing".into(), ".5rem".into());
    let wire = export_json(&document, &context()).unwrap();
    let value: serde_json::Value = serde_json::from_str(&wire).unwrap();
    assert!(value.get("styles").is_none());
    assert_eq!(value["config"]["scaling"], 105);
    assert_eq!(import_json(&wire, &context()).unwrap().document, document);
    assert_eq!(
        resolve(&document, ThemeMode::Light, &context())
            .unwrap()
            .mode,
        ThemeMode::Light
    );
}

#[test]
fn all_structural_objects_and_override_names_are_strict_and_null_cannot_hide_mixed_forms() {
    let valid = serde_json::to_value(standard()).unwrap();
    for field in ["base", "config", "overrides"] {
        let mut null = valid.clone();
        null[field] = serde_json::Value::Null;
        assert!(
            import_json(&null.to_string(), &context()).is_err(),
            "{field}"
        );
        let mut unknown = valid.clone();
        unknown[field]["surprise"] = "value".into();
        assert!(
            import_json(&unknown.to_string(), &context()).is_err(),
            "{field}"
        );
    }
    let mut mixed = valid.clone();
    mixed["styles"] = mixed["base"].clone();
    assert!(import_json(&mixed.to_string(), &context()).is_err());
    let mut compact = serde_json::to_value(legacy()).unwrap();
    compact["config"] = serde_json::Value::Null;
    assert!(import_json(&compact.to_string(), &context()).is_err());
    for version in [0, 3, 99] {
        let mut unsupported = valid.clone();
        unsupported["schema_version"] = version.into();
        assert!(import_json(&unsupported.to_string(), &context()).is_err());
    }
    let mut bad_override = valid.clone();
    bad_override["overrides"]["light"]["future"] = "#ffffff".into();
    assert_eq!(
        import_json(&bad_override.to_string(), &context())
            .unwrap_err()
            .field,
        "overrides"
    );
    let mut bad_config = valid;
    bad_config["config"]["accent_color"] = "sage".into();
    assert_eq!(
        import_json(&bad_config.to_string(), &context())
            .unwrap_err()
            .field,
        "config"
    );
}

#[test]
fn duplicates_are_rejected_in_v1_base_overrides_and_configuration() {
    for value in [v1_json(), serde_json::to_value(standard()).unwrap()] {
        let wire =
            value
                .to_string()
                .replacen("\"primary\":", "\"primary\":\"#ffffff\",\"primary\":", 1);
        assert!(import_json(&wire, &context())
            .unwrap_err()
            .message
            .contains("duplicate token"));
    }
    let mut document = standard();
    document
        .overrides
        .light
        .0
        .insert("primary".into(), "#123456".into());
    let wire = export_json(&document, &context()).unwrap();
    let duplicate = wire.replace(
        "\"primary\":\"#123456\"",
        "\"primary\":\"#123456\",\"primary\":\"#abcdef\"",
    );
    assert!(import_json(&duplicate, &context())
        .unwrap_err()
        .message
        .contains("duplicate token"));
    let duplicate = wire.replace("\"scaling\":100", "\"scaling\":100,\"scaling\":95");
    assert!(import_json(&duplicate, &context()).is_err());
}

#[test]
fn exact_limit_v1_can_upgrade_without_growth_but_general_intent_must_fit_the_same_limit() {
    let mut wire = v1_json();
    wire["styles"]["light"]["future-blob"] = "".into();
    let padding = THEME_INPUT_LIMIT - wire.to_string().len();
    wire["styles"]["light"]["future-blob"] = "x".repeat(padding).into();
    let input = wire.to_string();
    assert_eq!(input.len(), THEME_INPUT_LIMIT);
    let document = import_json(&input, &context()).unwrap().document;
    let output = export_json(&document, &context()).unwrap();
    assert_eq!(output.len(), input.len());
    assert_eq!(import_json(&output, &context()).unwrap().document, document);
    assert!(import_json(&(input + " "), &context()).is_err());
    let mut configured = document.clone();
    configured.config.accent_color = Some(AccentColor::Blue);
    assert!(check_document_size(&configured).is_err());
    let mut released = document;
    released.overrides.light.0.remove("primary");
    assert!(check_document_size(&released).is_err());
}

#[test]
fn configuration_does_not_erase_legacy_overrides_and_releasing_one_group_has_exact_scope() {
    let mut document = legacy();
    let before = resolve(&document, ThemeMode::Light, &context()).unwrap();
    document.config = ThemeConfig::standard();
    document.config.accent_color = Some(AccentColor::Tomato);
    let overridden = resolve(&document, ThemeMode::Light, &context()).unwrap();
    for group in ThemeConfigGroup::ALL {
        for token in group.tokens() {
            assert_eq!(
                overridden.values.get(token).unwrap(),
                before.values.get(token).unwrap(),
                "{token}"
            );
            assert_eq!(
                overridden.origins[token],
                ThemeValueOrigin::DocumentOverride
            );
        }
    }
    for token in ThemeConfigGroup::Accent.tokens() {
        document.overrides.light.0.remove(*token);
    }
    let effective = resolve(&document, ThemeMode::Light, &context()).unwrap();
    assert_eq!(
        effective.values.get("primary").unwrap(),
        rustify_colors::ColorFamily::Tomato
            .scale(
                rustify_colors::ColorMode::Light,
                rustify_colors::ColorGamut::Srgb,
                false
            )
            .steps()[8]
    );
    for token in [
        "accent",
        "accent-foreground",
        "destructive",
        "success",
        "warning",
        "chart-1",
        "border",
    ] {
        assert_eq!(
            effective.values.get(token).unwrap(),
            before.values.get(token).unwrap(),
            "{token}"
        );
    }
    assert_eq!(
        effective.origins["primary"],
        ThemeValueOrigin::Configuration(ThemeConfigGroup::Accent)
    );
    assert_eq!(
        resolve(&document, ThemeMode::Dark, &context())
            .unwrap()
            .colors["primary"],
        before
            .source
            .resolve(ThemeMode::Dark, &context())
            .unwrap()
            .colors["primary"]
    );
}

#[test]
fn nested_brand_inherits_resolved_gray_until_auto_is_explicitly_selected() {
    let root = resolve(&standard(), ThemeMode::Light, &context()).unwrap();
    let patch = ThemeBoundaryPatch {
        config: ThemeConfigPatch {
            accent_color: ConfigChange::Set(Some(AccentColor::Tomato)),
            ..Default::default()
        },
        ..Default::default()
    };
    let inherited = root.with_boundary(&patch).unwrap();
    assert_eq!(inherited.config.gray, ColorFamily::Slate);
    let mut auto = patch.clone();
    auto.config.gray_color = ConfigChange::Set(Some(GrayColor::Auto));
    let explicit = root.with_boundary(&auto).unwrap();
    assert_eq!(explicit.config.gray, ColorFamily::Mauve);
    assert_eq!(root.config.gray, ColorFamily::Slate);
    let mut custom = standard();
    custom.config.accent_color = None;
    let fallback = resolve(&custom, ThemeMode::Light, &context()).unwrap();
    assert_eq!(
        fallback.config.gray_fallback_accent,
        Some(AccentColor::Indigo)
    );
    assert_eq!(fallback.config.gray, ColorFamily::Slate);
}

#[test]
fn boundary_mode_uses_its_own_author_table_and_scaling_never_compounds() {
    let mut document = standard();
    document.config.appearance = Appearance::Dark;
    document.config.scaling = ThemeScaling::P110;
    document
        .base
        .dark
        .0
        .insert("font-size".into(), "28px".into());
    let root = resolve(&document, ThemeMode::Light, &context()).unwrap();
    close(root.font_size_px, 16.5);
    let inherited = root
        .with_boundary(&ThemeBoundaryPatch {
            config: ThemeConfigPatch {
                appearance: ConfigChange::Set(Appearance::Dark),
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap();
    close(inherited.font_size_px, 30.8);
    close(inherited.geometry.control_heights_px[1], 35.2);
    let child = root
        .with_boundary(&ThemeBoundaryPatch {
            config: ThemeConfigPatch {
                appearance: ConfigChange::Set(Appearance::Dark),
                scaling: ConfigChange::Set(ThemeScaling::P95),
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap();
    close(child.font_size_px, 26.6);
    close(child.geometry.control_heights_px[1], 30.4);
    let light = child
        .with_boundary(&ThemeBoundaryPatch {
            config: ThemeConfigPatch {
                appearance: ConfigChange::Set(Appearance::Light),
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap();
    close(light.font_size_px, 14.25);
    close(light.geometry.control_heights_px[1], 30.4);
    close(root.geometry.control_heights_px[1], 35.2);
    assert_eq!(
        root.with_boundary(&ThemeBoundaryPatch::default()).unwrap(),
        root
    );
    assert_eq!(child.values.get("font-size").unwrap(), "28px");
}

#[test]
fn all_shadow_layers_and_finite_geometry_scale_once_while_sentinels_and_color_stay_fixed() {
    let mut document = standard();
    document.config.scaling = ThemeScaling::P110;
    document.config.radius = Some(ThemeRadius::Full);
    for (token, value) in [
        ("shadow-opacity", ".5"),
        ("shadow-offset-x", "2px"),
        ("shadow-offset-y", "3px"),
        ("shadow-blur", "8px"),
        ("shadow-spread", "2px"),
        ("letter-spacing", ".1em"),
    ] {
        document.base.light.0.insert(token.into(), value.into());
    }
    let root = resolve(&document, ThemeMode::Light, &context()).unwrap();
    close(root.shadows["shadow-lg"][1].offset_y, 4.4);
    close(root.shadows["shadow-lg"][1].blur, 6.6);
    close(root.shadows["shadow-lg"][1].spread, 1.1);
    close(root.geometry.radii_px[2], 9.9);
    close(root.geometry.spaces_px[5], 26.4);
    close(root.letter_spacing_em, 0.1);
    assert_eq!(root.geometry.radius_full_px, 9999.);
    assert_eq!(root.geometry.radius_thumb_px, 9999.);
    let child = root
        .with_boundary(&ThemeBoundaryPatch {
            config: ThemeConfigPatch {
                scaling: ConfigChange::Set(ThemeScaling::P95),
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap();
    close(child.shadows["shadow-lg"][1].offset_y, 3.8);
    close(child.shadows["shadow-lg"][1].blur, 5.7);
    assert_eq!(root.colors, child.colors);
}

#[test]
fn document_overrides_beat_nested_configuration_then_inner_explicit_overrides_win() {
    let mut document = standard();
    document
        .overrides
        .light
        .0
        .insert("primary".into(), "#123456".into());
    let root = resolve(&document, ThemeMode::Light, &context()).unwrap();
    let configured = root
        .with_boundary(&ThemeBoundaryPatch {
            config: ThemeConfigPatch {
                accent_color: ConfigChange::Set(Some(AccentColor::Amber)),
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap();
    assert_eq!(configured.values.get("primary").unwrap(), "#123456");
    let local = configured
        .patched(&ThemeValuePatch(std::collections::BTreeMap::from([(
            "primary".into(),
            "#abcdef".into(),
        )])))
        .unwrap();
    assert_eq!(local.values.get("primary").unwrap(), "#abcdef");
    assert_eq!(
        local.origins["primary"],
        ThemeValueOrigin::BoundaryOverride { depth: 1 }
    );
    assert_eq!(root.source.document(), &document);
}

#[test]
fn masked_invalid_authored_values_and_unknown_runtime_override_keys_are_rejected() {
    let mut document = standard();
    document
        .base
        .light
        .0
        .insert("primary".into(), "url(evil)".into());
    assert_eq!(
        resolve(&document, ThemeMode::Light, &context())
            .unwrap_err()
            .field,
        "base.light.primary"
    );
    let mut document = standard();
    document
        .overrides
        .light
        .0
        .insert("future".into(), "#ffffff".into());
    assert_eq!(
        resolve(&document, ThemeMode::Light, &context())
            .unwrap_err()
            .field,
        "overrides.light.future"
    );
}

#[test]
fn supplementary_roles_use_reference_contrast_and_track_rules_and_cached_palette_identity() {
    let mut document = standard();
    document.config.accent_color = Some(AccentColor::Yellow);
    let light = resolve(&document, ThemeMode::Light, &context()).unwrap();
    let dark = resolve(&document, ThemeMode::Dark, &context()).unwrap();
    assert_eq!(light.brand.contrast, parse_color("#21201c").unwrap().rgba);
    assert_eq!(light.brand.indicator, light.brand.solid[9]);
    assert_eq!(dark.brand.indicator, dark.brand.solid[8]);
    assert_ne!(dark.brand.track, dark.brand.solid[8]);
    let again = resolve(&document, ThemeMode::Light, &context()).unwrap();
    assert!(Arc::ptr_eq(&light.brand, &again.brand));
    let mixed = super::color::mix_oklab(
        parse_color("#000000").unwrap().rgba,
        parse_color("#ffffff").unwrap().rgba,
        0.5,
    );
    assert!((mixed.r - 0.38857286).abs() < 1e-6);
}

#[test]
fn full_v2_uses_utf8_bytes_for_the_exact_limit_and_rejects_one_more_character() {
    let mut value = serde_json::to_value(standard()).unwrap();
    value["base"]["light"]["future-blob"] = "".into();
    let remaining = THEME_INPUT_LIMIT - value.to_string().len();
    value["base"]["light"]["future-blob"] =
        ("中".repeat(remaining / 3) + &"x".repeat(remaining % 3)).into();
    let input = value.to_string();
    assert_eq!(input.len(), THEME_INPUT_LIMIT);
    let document = import_json(&input, &context()).unwrap().document;
    assert_eq!(
        export_json(&document, &context()).unwrap().len(),
        THEME_INPUT_LIMIT
    );
    let larger = value["base"]["light"]["future-blob"]
        .as_str()
        .unwrap()
        .to_owned()
        + "中";
    value["base"]["light"]["future-blob"] = larger.into();
    assert!(import_json(&value.to_string(), &context()).is_err());
}

#[test]
fn every_brand_panel_radius_and_scaling_option_is_resolvable_in_both_modes() {
    for &brand in AccentColor::ALL {
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            let mut document = standard();
            document.config.accent_color = Some(brand);
            let theme = resolve(&document, mode, &context()).unwrap();
            assert_eq!(theme.config.gray, brand.matching_gray().family(brand));
            assert_eq!(theme.colors["primary"], theme.brand.solid[8]);
            assert_eq!(theme.colors["primary-foreground"], theme.brand.contrast);
        }
    }
    for &radius in ThemeRadius::ALL {
        for &scale in ThemeScaling::ALL {
            for panel in [PanelBackground::Solid, PanelBackground::Translucent] {
                for mode in [ThemeMode::Light, ThemeMode::Dark] {
                    let mut document = standard();
                    document.config.radius = Some(radius);
                    document.config.scaling = scale;
                    document.config.panel_background = Some(panel);
                    let theme = resolve(&document, mode, &context()).unwrap();
                    close(
                        theme.geometry.radii_px[2],
                        6. * radius.factor() * scale.factor(),
                    );
                    close(theme.geometry.control_heights_px[1], 32. * scale.factor());
                    close(
                        theme.colors["card"].a,
                        if panel == PanelBackground::Translucent {
                            if mode == ThemeMode::Light {
                                0.7
                            } else {
                                theme.gray.alpha[1].a
                            }
                        } else {
                            1.
                        },
                    );
                    assert_eq!(theme.panel, theme.colors["card"]);
                    close(
                        theme.surface.a,
                        if mode == ThemeMode::Light { 0.85 } else { 0.25 },
                    );
                }
            }
        }
    }
}
