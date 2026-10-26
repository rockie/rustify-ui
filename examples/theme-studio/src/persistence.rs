//! Versioned local data. Candidates never mutate the active editor or saved cache.

use rustify_ui::theme::{ResolveContext, ThemeDocument, ThemeError, ThemeMode};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const STORAGE_KEY: &str = "rustify-ui.theme-studio.v2";
pub const LEGACY_KEY: &str = "rustify-ui.theme-studio.v1";
pub const BACKUP_PREFIX: &str = "rustify-ui.theme-studio.v1.backup.";
pub const LOCK_NAME: &str = "rustify-ui.theme-studio.storage";
pub const SAVED_THEME_LIMIT: usize = 100;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Envelope {
    pub envelope_version: u32,
    pub revision: u64,
    pub migrated_from_v1: Option<MigrationSource>,
    pub draft: ThemeDocument,
    pub saved_themes: Vec<ThemeDocument>,
    pub preferences: Preferences,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationSource {
    pub sha256: String,
    pub backup_key: String,
}

pub fn digest(raw: &str) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(raw.as_bytes()))
}

impl MigrationSource {
    pub fn new(raw: &str) -> Self {
        let sha256 = digest(raw);
        Self {
            backup_key: format!("{BACKUP_PREFIX}{sha256}"),
            sha256,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacyEnvelope {
    envelope_version: u32,
    draft: ThemeDocument,
    saved_themes: Vec<ThemeDocument>,
    preferences: Preferences,
}

pub fn migrate_legacy(raw: &str, context: &ResolveContext<'_>) -> Result<Envelope, ThemeError> {
    let old: LegacyEnvelope =
        serde_json::from_str(raw).map_err(|error| self::error("legacy", error.to_string()))?;
    if old.envelope_version != 1 {
        return Err(error(
            "envelope_version",
            "unsupported legacy local data version",
        ));
    }
    let candidate = Envelope {
        envelope_version: 2,
        revision: 0,
        migrated_from_v1: Some(MigrationSource::new(raw)),
        draft: old.draft,
        saved_themes: old.saved_themes,
        preferences: old.preferences,
    };
    validate(&candidate, context)?;
    Ok(candidate)
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
    Valid(Box<Envelope>),
    Invalid { raw: String, error: ThemeError },
}

fn error(field: &str, message: impl Into<String>) -> ThemeError {
    ThemeError {
        field: field.into(),
        message: message.into(),
    }
}

fn validate_metadata(envelope: &Envelope) -> Result<(), ThemeError> {
    if envelope.envelope_version != 2 {
        return Err(error("envelope_version", "unsupported local data version"));
    }
    if let Some(source) = &envelope.migrated_from_v1 {
        if source.sha256.len() != 64
            || !source
                .sha256
                .bytes()
                .all(|value| value.is_ascii_hexdigit() && !value.is_ascii_uppercase())
            || source.backup_key != format!("{BACKUP_PREFIX}{}", source.sha256)
        {
            return Err(error("migrated_from_v1", "invalid backup identity"));
        }
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
        rustify_ui::theme::check_document_size(document)
            .map_err(|value| error(&format!("{prefix}.{}", value.field), value.message))?;
        let source = rustify_ui::ThemeSource::new(std::sync::Arc::new(document.clone()));
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            source.resolve(mode, context).map_err(|value| {
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
        Ok(envelope) => LoadResult::Valid(Box::new(envelope)),
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
    font_resolution: rustify_ui::FontResolution,
}

impl Autosave {
    #[cfg(test)]
    pub fn serialize(
        &mut self,
        envelope: &Envelope,
        editor: &mut crate::state::EditorState,
        context: &ResolveContext<'_>,
    ) -> Result<String, ThemeError> {
        if envelope.draft != editor.document {
            return Err(error(
                "draft",
                "autosave must use the current validated editor draft",
            ));
        }
        let pair = editor.resolved_pair(context)?;
        self.serialize_snapshot(envelope, &pair, context)
    }

    pub fn serialize_snapshot(
        &mut self,
        envelope: &Envelope,
        pair: &[std::sync::Arc<rustify_ui::ResolvedTheme>; 2],
        context: &ResolveContext<'_>,
    ) -> Result<String, ThemeError> {
        validate_metadata(envelope)?;
        envelope.draft.validate_identity()?;
        for (mode, theme) in [ThemeMode::Light, ThemeMode::Dark].into_iter().zip(pair) {
            if theme.mode != mode
                || theme.source.document() != &envelope.draft
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
                && saved.font_resolution == context.font_resolution
                && saved
                    .fallback_fonts
                    .iter()
                    .zip(context.fallback_fonts)
                    .all(|(a, b)| a == b)
        });
        if !cached {
            for (index, document) in envelope.saved_themes.iter().enumerate() {
                rustify_ui::theme::check_document_size(document).map_err(|value| {
                    error(
                        &format!("saved_themes[{index}].{}", value.field),
                        value.message,
                    )
                })?;
                let source = rustify_ui::ThemeSource::new(std::sync::Arc::new(document.clone()));
                for mode in [ThemeMode::Light, ThemeMode::Dark] {
                    source.resolve(mode, context).map_err(|value| {
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
                font_resolution: context.font_resolution,
            });
        }
        serde_json::to_string(envelope).map_err(|value| error("envelope", value.to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use rustify_ui::{theme::font_context, Theme};

    fn base() -> Envelope {
        Envelope {
            envelope_version: 2,
            revision: 0,
            migrated_from_v1: None,
            draft: ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
            saved_themes: Vec::new(),
            preferences: Preferences::default(),
        }
    }

    fn legacy_raw() -> String {
        let document = base().draft;
        let theme = serde_json::json!({
            "schema_version": 1, "id": "old-draft", "name": "旧草稿",
            "styles": {"light": document.effective_values(ThemeMode::Light).unwrap(), "dark": document.effective_values(ThemeMode::Dark).unwrap()}
        });
        let mut saved = theme.clone();
        saved["id"] = serde_json::json!("old-saved");
        saved["name"] = serde_json::json!("旧命名主题");
        saved["styles"]["dark"]["future-token"] = serde_json::json!("untouched author text");
        serde_json::to_string_pretty(&serde_json::json!({
            "envelope_version": 1, "draft": theme, "saved_themes": [saved], "preferences": Preferences::default()
        })).unwrap()
    }

    #[test]
    fn legacy_conversion_restores_the_entire_library_and_binds_exact_original_bytes() {
        let context = font_context(16., false);
        let raw = legacy_raw();
        let converted = migrate_legacy(&raw, &context).unwrap();
        assert_eq!(converted.envelope_version, 2);
        assert_eq!(converted.revision, 0);
        assert_eq!(converted.saved_themes.len(), 1);
        assert_eq!(converted.draft.id, "old-draft");
        assert_eq!(converted.saved_themes[0].id, "old-saved");
        assert_eq!(
            converted.saved_themes[0]
                .effective_values(ThemeMode::Dark)
                .unwrap()
                .get("future-token")
                .unwrap(),
            "untouched author text"
        );
        let source = converted.migrated_from_v1.unwrap();
        assert_eq!(source.sha256.len(), 64);
        assert_eq!(
            source.backup_key,
            format!("{BACKUP_PREFIX}{}", source.sha256)
        );
        assert_ne!(MigrationSource::new(&format!("{raw}\n")), source);
        assert_eq!(
            digest("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert!(matches!(
            load(Some(&raw), &context),
            LoadResult::Invalid { .. }
        ));
    }

    #[test]
    fn legacy_migration_rejects_one_bad_saved_theme_or_unknown_version_as_one_candidate() {
        let context = font_context(16., false);
        let valid: serde_json::Value = serde_json::from_str(&legacy_raw()).unwrap();
        for (path, bad) in [
            (vec!["envelope_version"], serde_json::json!(99)),
            (
                vec!["saved_themes", "0", "schema_version"],
                serde_json::json!(99),
            ),
            (
                vec!["saved_themes", "0", "styles", "dark", "primary"],
                serde_json::json!("var(--unsafe)"),
            ),
        ] {
            let mut value = valid.clone();
            let mut target = &mut value;
            for part in path {
                target = if part == "0" {
                    &mut target[0]
                } else {
                    &mut target[part]
                };
            }
            *target = bad;
            assert!(migrate_legacy(&serde_json::to_string(&value).unwrap(), &context).is_err());
        }
        assert!(migrate_legacy(&legacy_raw(), &context).is_ok());
    }

    #[test]
    fn migration_metadata_rejects_unbound_or_noncanonical_backup_identity() {
        let context = font_context(16., false);
        let mut candidate = base();
        let source = MigrationSource::new("original");
        candidate.migrated_from_v1 = Some(source.clone());
        assert!(serialize(&candidate, &context).is_ok());
        for changed in [
            MigrationSource {
                sha256: source.sha256.to_ascii_uppercase(),
                ..source.clone()
            },
            MigrationSource {
                backup_key: "another-app".into(),
                ..source.clone()
            },
            MigrationSource {
                sha256: "a".repeat(63),
                ..source
            },
        ] {
            candidate.migrated_from_v1 = Some(changed);
            assert!(serialize(&candidate, &context).is_err());
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
        assert_eq!(
            load(Some(&raw), &context),
            LoadResult::Valid(Box::new(envelope))
        );
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
            .overrides
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
            .overrides
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
        saved
            .overrides
            .dark
            .0
            .insert("radius".into(), "200rem".into());
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
            .overrides
            .light
            .0
            .insert("primary".into(), "oklch(.7 .3 20)".into());
        envelope
            .draft
            .base
            .dark
            .0
            .insert("future-value".into(), "url(https://example.test)".into());
        envelope.preferences.locale = "zh".into();
        envelope.preferences.favorites = vec!["builtin-preset".into()];
        envelope.preferences.mode = ThemeMode::Dark;
        let raw = serialize(&envelope, &context).unwrap();
        assert_eq!(
            load(Some(&raw), &context),
            LoadResult::Valid(Box::new(envelope))
        );
        assert_eq!(load(None, &context), LoadResult::Missing);
    }

    #[test]
    fn bad_cache_keeps_exact_raw_and_rejects_shape_version_and_invalid_dark_mode() {
        let context = font_context(16., false);
        let mut invalid_dark = base();
        invalid_dark
            .draft
            .overrides
            .dark
            .0
            .insert("primary".into(), "var(--x)".into());
        let mut wrong_version = base();
        wrong_version.envelope_version = 99;
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
            .save_named(&saved.saved_themes[0], "local-b", "Beta", None, &context)
            .unwrap();
        assert_eq!(copied.saved_themes[0], saved.saved_themes[0]);
        assert_eq!(copied.saved_themes[1].base, saved.saved_themes[0].base);
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
            .overrides
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
            .save_named(&envelope.saved_themes[0], "local-a", "Beta", None, &context)
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
            .overrides
            .light
            .0
            .insert("primary".into(), "#abc".into());
        let overwritten = full
            .save_named(&changed, "local-0", "Theme 0", Some("local-0"), &context)
            .unwrap();
        assert_eq!(overwritten.saved_themes.len(), SAVED_THEME_LIMIT);
        assert_eq!(overwritten.saved_themes[0].base, changed.base);
        assert_eq!(full.saved_themes[0].base, full.draft.base);
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
        let layer = if value["draft"].get("base").is_some() {
            "base"
        } else {
            "styles"
        };
        value["draft"][layer]
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
        assert_eq!(
            load(Some(&raw), &context),
            LoadResult::Valid(Box::new(envelope))
        );
    }
}
