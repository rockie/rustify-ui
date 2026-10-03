//! Versioned local data. Candidates never mutate the active editor or saved cache.

use rustify_ui::theme::{ResolveContext, ThemeDocument, ThemeError, ThemeMode};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const STORAGE_KEY: &str = "rustify-ui.theme-studio.v1";
pub const SAVED_THEME_LIMIT: usize = 100;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Envelope {
    pub envelope_version: u32,
    pub draft: ThemeDocument,
    pub saved_themes: Vec<ThemeDocument>,
    pub preferences: Preferences,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Preferences {
    pub locale: String,
    pub favorites: Vec<String>,
    pub scene: String,
    pub renderer: String,
    pub preview_width: String,
    pub mode: ThemeMode,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            locale: "en".into(),
            favorites: Vec::new(),
            scene: "Cards".into(),
            renderer: "compare".into(),
            preview_width: "responsive".into(),
            mode: ThemeMode::Light,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoadResult {
    Missing,
    Valid(Envelope),
    Invalid { raw: String, error: ThemeError },
}

fn error(field: &str, message: impl Into<String>) -> ThemeError {
    ThemeError {
        field: field.into(),
        message: message.into(),
    }
}

fn validate_metadata(envelope: &Envelope) -> Result<(), ThemeError> {
    if envelope.envelope_version != 1 {
        return Err(error("envelope_version", "unsupported local data version"));
    }
    if envelope.saved_themes.len() > SAVED_THEME_LIMIT {
        return Err(error("saved_themes", "save at most 100 named themes"));
    }
    let preferences = &envelope.preferences;
    for (field, value, allowed) in [
        ("locale", preferences.locale.as_str(), &["en", "zh"][..]),
        (
            "scene",
            preferences.scene.as_str(),
            &[
                "Cards",
                "Dashboard",
                "Application",
                "Marketing",
                "Mail",
                "Typography",
                "Color Palette",
            ][..],
        ),
        (
            "renderer",
            preferences.renderer.as_str(),
            &["dom", "gpu", "compare"][..],
        ),
        (
            "preview_width",
            preferences.preview_width.as_str(),
            &["responsive", "desktop", "tablet", "mobile"][..],
        ),
    ] {
        if !allowed.contains(&value) {
            return Err(error(
                &format!("preferences.{field}"),
                "unsupported preference value",
            ));
        }
    }
    let mut favorites = BTreeSet::new();
    for id in &preferences.favorites {
        check_identity("preferences.favorites", id)?;
        if !favorites.insert(id) {
            return Err(error("preferences.favorites", "duplicate favorite id"));
        }
    }
    let mut ids = BTreeSet::new();
    let mut names = BTreeSet::new();
    for theme in &envelope.saved_themes {
        if !ids.insert(&theme.id) {
            return Err(error("saved_themes.id", "duplicate saved theme id"));
        }
        if !names.insert(theme.name.trim()) {
            return Err(error("saved_themes.name", "duplicate saved theme name"));
        }
    }
    Ok(())
}

pub fn validate(envelope: &Envelope, context: &ResolveContext<'_>) -> Result<(), ThemeError> {
    validate_metadata(envelope)?;
    for (prefix, document) in std::iter::once(("draft".to_owned(), &envelope.draft)).chain(
        envelope
            .saved_themes
            .iter()
            .enumerate()
            .map(|(index, document)| (format!("saved_themes[{index}]"), document)),
    ) {
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            rustify_ui::resolve(document, mode, context).map_err(|value| {
                error(
                    &format!("{prefix}.{}.{}", mode.as_str(), value.field),
                    value.message,
                )
            })?;
        }
    }
    Ok(())
}

fn check_identity(field: &str, value: &str) -> Result<(), ThemeError> {
    if value.trim().is_empty() || value.chars().count() > 80 || value.chars().any(char::is_control)
    {
        return Err(error(
            field,
            "use 1–80 characters without control characters",
        ));
    }
    Ok(())
}

fn name(value: &str) -> Result<String, ThemeError> {
    check_identity("saved_themes.name", value)?;
    Ok(value.trim().into())
}

pub fn load(raw: Option<&str>, context: &ResolveContext<'_>) -> LoadResult {
    let Some(raw) = raw else {
        return LoadResult::Missing;
    };
    match serde_json::from_str::<Envelope>(raw)
        .map_err(|value| error("envelope", value.to_string()))
        .and_then(|envelope| {
            validate(&envelope, context)?;
            Ok(envelope)
        }) {
        Ok(envelope) => LoadResult::Valid(envelope),
        Err(error) => LoadResult::Invalid {
            raw: raw.into(),
            error,
        },
    }
}

pub fn serialize(envelope: &Envelope, context: &ResolveContext<'_>) -> Result<String, ThemeError> {
    validate(envelope, context)?;
    serde_json::to_string(envelope).map_err(|value| error("envelope", value.to_string()))
}

#[derive(Default)]
/// Autosave reuses the editor's validated snapshot and caches the unchanged library.
/// External input still goes through `load` or the ordinary complete validator.
pub struct Autosave {
    saved: Option<SavedValidation>,
}

struct SavedValidation {
    documents: Vec<ThemeDocument>,
    rem_px: f64,
    reduce_motion: bool,
    fonts: Vec<rustify_ui::theme::FontFace>,
    fallback_fonts: [String; 3],
}

impl Autosave {
    pub fn serialize(
        &mut self,
        envelope: &Envelope,
        editor: &mut crate::state::EditorState,
        context: &ResolveContext<'_>,
    ) -> Result<String, ThemeError> {
        validate_metadata(envelope)?;
        envelope.draft.validate_identity()?;
        if envelope.draft != editor.document {
            return Err(error(
                "draft",
                "autosave must use the current validated editor draft",
            ));
        }
        let pair = editor.resolved_pair(context)?;
        for (mode, theme) in [ThemeMode::Light, ThemeMode::Dark].into_iter().zip(pair) {
            if theme.mode != mode
                || theme.values != *envelope.draft.values(mode)
                || theme.document_id != envelope.draft.id
                || theme.name != envelope.draft.name
            {
                return Err(error("draft", "editor snapshot does not match the draft"));
            }
        }
        let cached = self.saved.as_ref().is_some_and(|saved| {
            saved.documents == envelope.saved_themes
                && saved.rem_px == context.rem_px
                && saved.reduce_motion == context.reduce_motion
                && saved.fonts == context.fonts
                && saved
                    .fallback_fonts
                    .iter()
                    .zip(context.fallback_fonts)
                    .all(|(a, b)| a == b)
        });
        if !cached {
            for (index, document) in envelope.saved_themes.iter().enumerate() {
                for mode in [ThemeMode::Light, ThemeMode::Dark] {
                    rustify_ui::resolve(document, mode, context).map_err(|value| {
                        error(
                            &format!("saved_themes[{index}].{}.{}", mode.as_str(), value.field),
                            value.message,
                        )
                    })?;
                }
            }
            self.saved = Some(SavedValidation {
                documents: envelope.saved_themes.clone(),
                rem_px: context.rem_px,
                reduce_motion: context.reduce_motion,
                fonts: context.fonts.to_vec(),
                fallback_fonts: context.fallback_fonts.map(str::to_owned),
            });
        }
        serde_json::to_string(envelope).map_err(|value| error("envelope", value.to_string()))
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save(
        &mut self,
        envelope: &Envelope,
        editor: &mut crate::state::EditorState,
        context: &ResolveContext<'_>,
    ) -> Result<(), ThemeError> {
        let raw = self.serialize(envelope, editor, context)?;
        storage()?
            .set_item(STORAGE_KEY, &raw)
            .map_err(|value| storage_error("write", value))
    }
}

impl Envelope {
    /// New saves use new_id; an explicit overwrite must pass the target id as new_id.
    pub fn save_named(
        &self,
        source: &ThemeDocument,
        new_id: &str,
        new_name: &str,
        overwrite: Option<&str>,
        context: &ResolveContext<'_>,
    ) -> Result<Self, ThemeError> {
        let mut document = source.clone();
        document.id = new_id.into();
        document.name = name(new_name)?;
        let mut candidate = self.clone();
        if let Some(target_id) = overwrite {
            if new_id != target_id {
                return Err(error(
                    "saved_themes.id",
                    "overwrite must preserve the target id",
                ));
            }
            *candidate
                .saved_themes
                .iter_mut()
                .find(|theme| theme.id == target_id)
                .ok_or_else(|| error("saved_themes.id", "saved theme does not exist"))? = document;
        } else {
            candidate.saved_themes.push(document);
        }
        validate(&candidate, context)?;
        Ok(candidate)
    }
    pub fn copy_theme(
        &self,
        source_id: &str,
        new_id: &str,
        new_name: &str,
        context: &ResolveContext<'_>,
    ) -> Result<Self, ThemeError> {
        let source = self
            .saved_themes
            .iter()
            .find(|theme| theme.id == source_id)
            .ok_or_else(|| error("saved_themes.id", "saved theme does not exist"))?;
        self.save_named(source, new_id, new_name, None, context)
    }
    pub fn rename_theme(
        &self,
        id: &str,
        new_name: &str,
        context: &ResolveContext<'_>,
    ) -> Result<Self, ThemeError> {
        let mut candidate = self.clone();
        candidate
            .saved_themes
            .iter_mut()
            .find(|theme| theme.id == id)
            .ok_or_else(|| error("saved_themes.id", "saved theme does not exist"))?
            .name = name(new_name)?;
        validate(&candidate, context)?;
        Ok(candidate)
    }
    pub fn delete_theme(&self, id: &str, context: &ResolveContext<'_>) -> Result<Self, ThemeError> {
        let mut candidate = self.clone();
        let index = candidate
            .saved_themes
            .iter()
            .position(|theme| theme.id == id)
            .ok_or_else(|| error("saved_themes.id", "saved theme does not exist"))?;
        candidate.saved_themes.remove(index);
        candidate
            .preferences
            .favorites
            .retain(|favorite| favorite != id);
        validate(&candidate, context)?;
        Ok(candidate)
    }
}

#[cfg(target_arch = "wasm32")]
fn storage() -> Result<web_sys::Storage, ThemeError> {
    web_sys::window()
        .ok_or_else(|| error("storage", "browser window is unavailable"))?
        .local_storage()
        .map_err(|value| storage_error("access", value))?
        .ok_or_else(|| error("storage", "localStorage is unavailable"))
}

#[cfg(target_arch = "wasm32")]
fn storage_error(operation: &str, value: wasm_bindgen::JsValue) -> ThemeError {
    error(
        "storage",
        format!("localStorage {operation} failed: {value:?}"),
    )
}

#[cfg(target_arch = "wasm32")]
pub fn load_storage(context: &ResolveContext<'_>) -> Result<LoadResult, ThemeError> {
    let raw = storage()?
        .get_item(STORAGE_KEY)
        .map_err(|value| storage_error("read", value))?;
    Ok(load(raw.as_deref(), context))
}

/// Writes once after complete validation. The caller commits its candidate only on success.
#[cfg(target_arch = "wasm32")]
pub fn save_storage(envelope: &Envelope, context: &ResolveContext<'_>) -> Result<(), ThemeError> {
    let raw = serialize(envelope, context)?;
    storage()?
        .set_item(STORAGE_KEY, &raw)
        .map_err(|value| storage_error("write", value))
}

/// Only an explicit user reset calls this; invalid cache loading never removes data.
#[cfg(target_arch = "wasm32")]
pub fn remove_storage() -> Result<(), ThemeError> {
    storage()?
        .remove_item(STORAGE_KEY)
        .map_err(|value| storage_error("remove", value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustify_ui::{theme::font_context, Theme};

    fn base() -> Envelope {
        Envelope {
            envelope_version: 1,
            draft: ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
            saved_themes: Vec::new(),
            preferences: Preferences::default(),
        }
    }

    #[test]
    fn autosave_reuses_editor_resolution_and_unchanged_saved_library() {
        let context = font_context(16., false);
        let mut envelope = base();
        for index in 0..SAVED_THEME_LIMIT {
            let mut theme = envelope.draft.clone();
            theme.id = format!("local-{index}");
            theme.name = format!("Saved {index}");
            envelope.saved_themes.push(theme);
        }
        let mut editor =
            crate::state::EditorState::new(envelope.draft.clone(), ThemeMode::Light, &context)
                .unwrap();
        let mut autosave = Autosave::default();
        autosave
            .serialize(&envelope, &mut editor, &context)
            .unwrap();
        editor.edit("primary", "#123456", &context).unwrap();
        envelope.draft = editor.document.clone();
        let before = rustify_ui::theme::resolution_count();
        let raw = autosave
            .serialize(&envelope, &mut editor, &context)
            .unwrap();
        assert_eq!(rustify_ui::theme::resolution_count() - before, 0);
        assert_eq!(load(Some(&raw), &context), LoadResult::Valid(envelope));
    }

    #[test]
    fn autosave_rejects_changed_library_metadata_and_unresolved_drafts() {
        let context = font_context(16., false);
        let mut envelope = base()
            .save_named(&base().draft, "saved", "Saved", None, &context)
            .unwrap();
        let mut editor =
            crate::state::EditorState::new(envelope.draft.clone(), ThemeMode::Light, &context)
                .unwrap();
        let mut autosave = Autosave::default();
        autosave
            .serialize(&envelope, &mut editor, &context)
            .unwrap();
        let original = envelope.clone();
        envelope.saved_themes[0]
            .styles
            .dark
            .0
            .insert("radius".into(), "-1px".into());
        assert!(autosave
            .serialize(&envelope, &mut editor, &context)
            .is_err());
        envelope = original.clone();
        envelope.preferences.locale = "invalid".into();
        assert!(autosave
            .serialize(&envelope, &mut editor, &context)
            .is_err());
        envelope = original.clone();
        envelope.saved_themes.push(envelope.saved_themes[0].clone());
        assert!(autosave
            .serialize(&envelope, &mut editor, &context)
            .is_err());
        envelope = original.clone();
        envelope
            .draft
            .styles
            .light
            .0
            .insert("primary".into(), "#123456".into());
        assert!(autosave
            .serialize(&envelope, &mut editor, &context)
            .is_err());
        // Even a direct mutation of the editor cannot masquerade as its cached snapshot.
        editor.document = envelope.draft.clone();
        assert!(autosave
            .serialize(&envelope, &mut editor, &context)
            .is_err());
        editor = crate::state::EditorState::new(original.draft.clone(), ThemeMode::Light, &context)
            .unwrap();
        assert!(autosave.serialize(&original, &mut editor, &context).is_ok());
    }

    #[test]
    fn autosave_revalidates_saved_dimensions_when_root_size_changes() {
        let context = font_context(16., false);
        let mut envelope = base();
        let mut saved = envelope.draft.clone();
        saved.styles.dark.0.insert("radius".into(), "200rem".into());
        envelope.saved_themes.push(saved);
        let mut editor =
            crate::state::EditorState::new(envelope.draft.clone(), ThemeMode::Light, &context)
                .unwrap();
        let mut autosave = Autosave::default();
        autosave
            .serialize(&envelope, &mut editor, &context)
            .unwrap();
        let error = autosave
            .serialize(&envelope, &mut editor, &font_context(32., false))
            .unwrap_err();
        assert!(error.field.contains("saved_themes[0].dark"));
        assert!(autosave.serialize(&envelope, &mut editor, &context).is_ok());
    }

    #[test]
    fn valid_cache_round_trip_preserves_author_values_preferences_and_unknown_tokens() {
        let context = font_context(16., false);
        let mut envelope = base();
        envelope
            .draft
            .styles
            .light
            .0
            .insert("primary".into(), "oklch(.7 .3 20)".into());
        envelope
            .draft
            .styles
            .dark
            .0
            .insert("future-value".into(), "url(https://example.test)".into());
        envelope.preferences.locale = "zh".into();
        envelope.preferences.favorites = vec!["builtin-preset".into()];
        envelope.preferences.mode = ThemeMode::Dark;
        let raw = serialize(&envelope, &context).unwrap();
        assert_eq!(load(Some(&raw), &context), LoadResult::Valid(envelope));
        assert_eq!(load(None, &context), LoadResult::Missing);
    }

    #[test]
    fn bad_cache_keeps_exact_raw_and_rejects_shape_version_and_invalid_dark_mode() {
        let context = font_context(16., false);
        let mut invalid_dark = base();
        invalid_dark
            .draft
            .styles
            .dark
            .0
            .insert("primary".into(), "var(--x)".into());
        let mut wrong_version = base();
        wrong_version.envelope_version = 2;
        for raw in [
            "{ broken json ".into(),
            "{}".into(),
            serde_json::to_string(&invalid_dark).unwrap(),
            serde_json::to_string(&wrong_version).unwrap(),
        ] {
            match load(Some(&raw), &context) {
                LoadResult::Invalid { raw: retained, .. } => assert_eq!(retained, raw),
                result => panic!("invalid cache accepted: {result:?}"),
            }
        }
    }

    #[test]
    fn named_crud_preserves_identity_intent_and_never_changes_original() {
        let context = font_context(16., false);
        let original = base();
        let mut saved = original
            .save_named(&original.draft, "local-a", "  Alpha  ", None, &context)
            .unwrap();
        assert!(original.saved_themes.is_empty());
        assert_eq!(saved.draft, original.draft);
        assert_eq!(saved.saved_themes[0].id, "local-a");
        assert_eq!(saved.saved_themes[0].name, "Alpha");
        let copied = saved
            .copy_theme("local-a", "local-b", "Beta", &context)
            .unwrap();
        assert_eq!(copied.saved_themes[0], saved.saved_themes[0]);
        assert_eq!(copied.saved_themes[1].styles, saved.saved_themes[0].styles);
        let renamed = copied.rename_theme("local-b", "Gamma", &context).unwrap();
        assert_eq!(renamed.saved_themes[1].id, "local-b");
        assert_eq!(renamed.saved_themes[1].name, "Gamma");
        saved.preferences.favorites = vec!["local-a".into(), "builtin-preset".into()];
        let deleted = saved.delete_theme("local-a", &context).unwrap();
        assert!(deleted.saved_themes.is_empty());
        assert_eq!(deleted.preferences.favorites, ["builtin-preset"]);
        assert_eq!(saved.saved_themes.len(), 1);
        assert_eq!(deleted.draft, original.draft);
    }

    #[test]
    fn invalid_candidate_duplicate_names_ids_and_unknown_target_leave_envelope_unchanged() {
        let context = font_context(16., false);
        let envelope = base()
            .save_named(&base().draft, "local-a", "Alpha", None, &context)
            .unwrap();
        let before = envelope.clone();
        let mut invalid = envelope.draft.clone();
        invalid
            .styles
            .dark
            .0
            .insert("radius".into(), "calc(1rem + 2px)".into());
        assert!(envelope
            .save_named(&invalid, "local-b", "Beta", None, &context)
            .is_err());
        for name in [
            "",
            "   ",
            "line\nbreak",
            "\nBeta",
            &"中".repeat(81),
            " Alpha ",
        ] {
            assert!(
                envelope
                    .save_named(&envelope.draft, "local-b", name, None, &context)
                    .is_err(),
                "{name:?}"
            );
        }
        assert!(envelope
            .copy_theme("local-a", "local-a", "Beta", &context)
            .is_err());
        assert!(envelope.rename_theme("missing", "Beta", &context).is_err());
        assert!(envelope.delete_theme("missing", &context).is_err());
        assert!(envelope
            .save_named(
                &envelope.draft,
                "different-id",
                "Beta",
                Some("local-a"),
                &context
            )
            .is_err());
        assert_eq!(envelope, before);
    }

    #[test]
    fn hundred_theme_limit_allows_explicit_overwrite_and_rejects_extra_or_duplicate_saved_ids() {
        let context = font_context(16., false);
        let mut full = base();
        for index in 0..SAVED_THEME_LIMIT {
            let mut theme = full.draft.clone();
            theme.id = format!("local-{index}");
            theme.name = format!("Theme {index}");
            full.saved_themes.push(theme);
        }
        assert!(validate(&full, &context).is_ok());
        assert!(full
            .save_named(&full.draft, "extra", "Extra", None, &context)
            .is_err());
        let mut changed = full.draft.clone();
        changed
            .styles
            .light
            .0
            .insert("primary".into(), "#abc".into());
        let overwritten = full
            .save_named(&changed, "local-0", "Theme 0", Some("local-0"), &context)
            .unwrap();
        assert_eq!(overwritten.saved_themes.len(), SAVED_THEME_LIMIT);
        assert_eq!(overwritten.saved_themes[0].styles, changed.styles);
        assert_eq!(full.saved_themes[0].styles, full.draft.styles);
        let mut duplicate = full.clone();
        duplicate.saved_themes[1].id = duplicate.saved_themes[0].id.clone();
        assert!(validate(&duplicate, &context).is_err());
        let mut duplicate = full.clone();
        duplicate.saved_themes[1].name = format!(" {} ", duplicate.saved_themes[0].name);
        assert!(validate(&duplicate, &context).is_err());
        let mut too_many = full.clone();
        too_many.saved_themes.push(changed);
        assert!(validate(&too_many, &context).is_err());
    }

    #[test]
    fn preference_enums_duplicates_and_unknown_fields_are_rejected_without_losing_raw() {
        let context = font_context(16., false);
        let original = base();
        for (field, invalid) in [
            ("locale", "fr"),
            ("scene", "unknown"),
            ("renderer", "server"),
            ("preview_width", "999px"),
        ] {
            let mut value = serde_json::to_value(&original).unwrap();
            value["preferences"][field] = serde_json::json!(invalid);
            let raw = serde_json::to_string(&value).unwrap();
            assert!(
                matches!(load(Some(&raw), &context), LoadResult::Invalid {raw: retained, ..} if retained == raw),
                "{field}"
            );
        }
        for invalid_favorites in [vec![""], vec!["id", "id"], vec!["bad\nname"]] {
            let mut value = original.clone();
            value.preferences.favorites = invalid_favorites.into_iter().map(String::from).collect();
            assert!(serialize(&value, &context).is_err());
        }
        let mut value = serde_json::to_value(&original).unwrap();
        value["obsolete"] = serde_json::json!(true);
        let raw = serde_json::to_string(&value).unwrap();
        assert!(
            matches!(load(Some(&raw), &context), LoadResult::Invalid {raw: retained, ..} if retained == raw)
        );
        let mut value = serde_json::to_value(&original).unwrap();
        value["draft"]["styles"]
            .as_object_mut()
            .unwrap()
            .remove("dark");
        let raw = serde_json::to_string(&value).unwrap();
        assert!(matches!(
            load(Some(&raw), &context),
            LoadResult::Invalid { .. }
        ));
        assert_eq!(original, base());
    }

    #[test]
    fn every_supported_preference_value_and_eighty_unicode_character_name_round_trip() {
        let context = font_context(16., false);
        let mut envelope = base();
        envelope.draft.name = "中".repeat(80);
        for locale in ["en", "zh"] {
            envelope.preferences.locale = locale.into();
            for scene in [
                "Cards",
                "Dashboard",
                "Application",
                "Marketing",
                "Mail",
                "Typography",
                "Color Palette",
            ] {
                envelope.preferences.scene = scene.into();
                for renderer in ["dom", "gpu", "compare"] {
                    envelope.preferences.renderer = renderer.into();
                    for width in ["responsive", "desktop", "tablet", "mobile"] {
                        envelope.preferences.preview_width = width.into();
                        assert!(validate(&envelope, &context).is_ok());
                    }
                }
            }
        }
        let raw = serialize(&envelope, &context).unwrap();
        assert_eq!(load(Some(&raw), &context), LoadResult::Valid(envelope));
    }
}
