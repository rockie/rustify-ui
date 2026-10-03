use rustify_ui::{resolve, ResolveContext, Theme, ThemeDocument, ThemeMode, ThemeStyles};
use serde::Deserialize;

#[derive(Deserialize)]
struct Preset {
    id: String,
    name: String,
    styles: ThemeStyles,
}

pub fn built_in() -> Result<Vec<ThemeDocument>, String> {
    let raw: Vec<Preset> = serde_json::from_str(include_str!("../vendor/tweakcn/presets.json"))
        .map_err(|error| format!("built-in presets: {error}"))?;
    let default = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
    let mut light = default.clone();
    light.id = "rustify-light".into();
    light.name = "Rustify Light".into();
    let mut dark = default.clone();
    dark.id = "rustify-dark".into();
    dark.name = "Rustify Dark".into();
    let mut themes = vec![light, dark];
    for preset in raw {
        let mut theme = default.clone();
        theme.id = preset.id;
        theme.name = preset.name;
        theme.styles.light.0.extend(preset.styles.light.0);
        theme.styles.dark.0.extend(preset.styles.dark.0);
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            resolve(&theme, mode, &ResolveContext::default())
                .map_err(|error| format!("{}: {error}", theme.id))?;
        }
        themes.push(theme);
    }
    Ok(themes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_reference_preset_and_both_rustify_presets_resolve_in_both_modes() {
        let themes = built_in().unwrap();
        assert_eq!(themes.len(), 44);
        let provenance: serde_json::Value =
            serde_json::from_str(include_str!("../vendor/tweakcn/provenance.json")).unwrap();
        let keys: Vec<_> = provenance["preset_keys"]
            .as_array()
            .unwrap()
            .iter()
            .map(|key| key.as_str().unwrap())
            .collect();
        assert_eq!(
            themes[2..]
                .iter()
                .map(|theme| theme.id.as_str())
                .collect::<Vec<_>>(),
            keys
        );
        for theme in themes {
            for mode in [ThemeMode::Light, ThemeMode::Dark] {
                assert!(
                    resolve(&theme, mode, &ResolveContext::default()).is_ok(),
                    "{}/{mode:?}",
                    theme.id
                );
            }
        }
    }

    #[test]
    fn normalization_preserves_every_reference_author_value() {
        let themes = built_in().unwrap();
        let raw: Vec<Preset> =
            serde_json::from_str(include_str!("../vendor/tweakcn/presets.json")).unwrap();
        for (actual, expected) in themes[2..].iter().zip(raw) {
            for mode in [ThemeMode::Light, ThemeMode::Dark] {
                let expected = match mode {
                    ThemeMode::Light => &expected.styles.light,
                    ThemeMode::Dark => &expected.styles.dark,
                };
                for (token, value) in &expected.0 {
                    assert_eq!(
                        actual.values(mode).get(token).unwrap(),
                        value,
                        "{}/{mode:?}/{token}",
                        actual.id
                    );
                }
            }
        }
    }

    #[test]
    fn reference_mode_differences_and_normal_letter_spacing_remain_intact() {
        let themes = built_in().unwrap();
        let graphite = themes.iter().find(|theme| theme.id == "graphite").unwrap();
        assert_ne!(
            graphite.styles.light.get("font-sans").unwrap(),
            graphite.styles.dark.get("font-sans").unwrap()
        );
        let soft_pop = themes.iter().find(|theme| theme.id == "soft-pop").unwrap();
        assert_eq!(
            soft_pop.styles.dark.get("letter-spacing").unwrap(),
            "normal"
        );
    }

    #[test]
    fn rustify_presets_keep_the_legacy_starting_mode_palettes() {
        let themes = built_in().unwrap();
        assert_eq!(themes[0].styles.light.get("primary").unwrap(), "#1570ef");
        assert_eq!(themes[1].styles.dark.get("primary").unwrap(), "#53b1fd");
    }
}
