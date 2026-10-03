use rustify_ui::theme::{
    format_color, hsla_to_rgba, parse_color, rgba_to_hsla, ColorFormat, FontFace, ResolveContext,
    ResolvedTheme, ThemeDocument, ThemeError, ThemeMode, COLOR_TOKENS, COMMON_TOKENS, VALUE_TOKENS,
};
use std::sync::Arc;

const HISTORY_LIMIT: usize = 100;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TokenScope {
    #[default]
    Both,
    Current,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HslShift {
    pub hue_degrees: f64,
    pub saturation_scale: f64,
    pub lightness_scale: f64,
}

impl Default for HslShift {
    fn default() -> Self {
        Self {
            hue_degrees: 0.,
            saturation_scale: 1.,
            lightness_scale: 1.,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Gesture {
    document: ThemeDocument,
    redo: Vec<ThemeDocument>,
}

#[derive(Clone, Debug, PartialEq)]
struct ContextKey {
    revision: u64,
    rem_bits: u64,
    reduce_motion: bool,
    fonts: Vec<FontFace>,
    fallback_fonts: [String; 3],
}

impl ContextKey {
    fn new(revision: u64, context: &ResolveContext<'_>) -> Self {
        Self {
            revision,
            rem_bits: context.rem_px.to_bits(),
            reduce_motion: context.reduce_motion,
            fonts: context.fonts.to_vec(),
            fallback_fonts: context.fallback_fonts.map(str::to_owned),
        }
    }

    fn matches(&self, revision: u64, context: &ResolveContext<'_>) -> bool {
        self.revision == revision
            && self.rem_bits == context.rem_px.to_bits()
            && self.reduce_motion == context.reduce_motion
            && self.fonts == context.fonts
            && self
                .fallback_fonts
                .iter()
                .zip(context.fallback_fonts)
                .all(|(owned, current)| owned == current)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ResolvedCache {
    key: ContextKey,
    pair: [Arc<ResolvedTheme>; 2],
}

#[derive(Clone, Debug, PartialEq)]
pub struct EditorState {
    pub document: ThemeDocument,
    pub mode: ThemeMode,
    pub revision: u64,
    undo: Vec<ThemeDocument>,
    redo: Vec<ThemeDocument>,
    checkpoint: ThemeDocument,
    gesture: Option<Gesture>,
    cache: Option<ResolvedCache>,
    resolution_count: u64,
}

impl EditorState {
    pub fn restore(
        document: ThemeDocument,
        mode: ThemeMode,
        checkpoint: ThemeDocument,
        context: &ResolveContext<'_>,
    ) -> Result<Self, ThemeError> {
        let mut state = Self::new(document, mode, context)?;
        if checkpoint != state.document {
            resolve_pair(&checkpoint, context, &mut state.resolution_count)?;
            state.checkpoint = checkpoint;
        }
        Ok(state)
    }
    pub fn new(
        document: ThemeDocument,
        mode: ThemeMode,
        context: &ResolveContext<'_>,
    ) -> Result<Self, ThemeError> {
        let mut resolution_count = 0;
        let pair = resolve_pair(&document, context, &mut resolution_count)?;
        Ok(Self {
            checkpoint: document.clone(),
            document,
            mode,
            revision: 0,
            undo: Vec::new(),
            redo: Vec::new(),
            gesture: None,
            cache: Some(ResolvedCache {
                key: ContextKey::new(0, context),
                pair,
            }),
            resolution_count,
        })
    }

    pub fn edit(
        &mut self,
        token: &str,
        value: &str,
        context: &ResolveContext<'_>,
    ) -> Result<bool, ThemeError> {
        self.edit_scoped(token, value, TokenScope::Current, context)
    }

    pub fn edit_scoped(
        &mut self,
        token: &str,
        value: &str,
        scope: TokenScope,
        context: &ResolveContext<'_>,
    ) -> Result<bool, ThemeError> {
        self.require_idle()?;
        let candidate = self.with_value(token, value, scope)?;
        self.apply(candidate, context, true)
    }

    /// Presets and imports replace the document while retaining the selected mode.
    pub fn apply_document(
        &mut self,
        document: ThemeDocument,
        context: &ResolveContext<'_>,
    ) -> Result<bool, ThemeError> {
        self.require_idle()?;
        self.apply(document, context, true)
    }

    pub fn begin_gesture(&mut self) -> bool {
        if self.gesture.is_some() {
            return false;
        }
        self.gesture = Some(Gesture {
            document: self.document.clone(),
            redo: self.redo.clone(),
        });
        true
    }

    /// The UI keeps incomplete input separately; only valid updates reach the document.
    pub fn update(
        &mut self,
        token: &str,
        value: &str,
        scope: TokenScope,
        context: &ResolveContext<'_>,
    ) -> Result<bool, ThemeError> {
        self.require_gesture()?;
        let candidate = self.with_value(token, value, scope)?;
        self.apply(candidate, context, false)
    }

    /// Every update starts from the gesture's author values, avoiding cumulative rounding.
    pub fn update_hsl(
        &mut self,
        shift: HslShift,
        scope: TokenScope,
        context: &ResolveContext<'_>,
    ) -> Result<bool, ThemeError> {
        let origin = &self.require_gesture()?.document;
        for (field, value, min, max) in [
            ("hue", shift.hue_degrees, -180., 180.),
            ("saturation", shift.saturation_scale, 0., 2.),
            ("lightness", shift.lightness_scale, 0.2, 2.),
        ] {
            if !value.is_finite() || value < min || value > max {
                return Err(error(
                    field,
                    &format!("use a finite value in {min}..={max}"),
                ));
            }
        }
        let mut candidate = origin.clone();
        if shift != HslShift::default() {
            for mode in [ThemeMode::Light, ThemeMode::Dark] {
                if scope == TokenScope::Current && mode != self.mode {
                    continue;
                }
                for &token in COLOR_TOKENS {
                    let color = parse_color(origin.values(mode).get(token)?)
                        .map_err(|message| error(token, &message))?
                        .rgba;
                    let mut hsl = rgba_to_hsla(color);
                    hsl.h = (hsl.h + shift.hue_degrees).rem_euclid(360.);
                    hsl.s = (hsl.s * shift.saturation_scale).clamp(0., 1.);
                    hsl.l = (hsl.l * shift.lightness_scale).clamp(0.1, 1.);
                    let adjusted = hsla_to_rgba(hsl);
                    // Hue changes do not rewrite achromatic author values.
                    if (color.r - adjusted.r).abs() < 1e-12
                        && (color.g - adjusted.g).abs() < 1e-12
                        && (color.b - adjusted.b).abs() < 1e-12
                    {
                        continue;
                    }
                    candidate
                        .values_mut(mode)
                        .0
                        .insert(token.into(), format_color(adjusted, ColorFormat::Rgb));
                }
            }
        }
        self.apply(candidate, context, false)
    }

    pub fn commit_gesture(&mut self) -> bool {
        let Some(gesture) = self.gesture.take() else {
            return false;
        };
        if self.document == gesture.document {
            self.redo = gesture.redo;
            return false;
        }
        push_history(&mut self.undo, gesture.document);
        self.redo.clear();
        true
    }

    pub fn cancel_gesture(&mut self) -> bool {
        let Some(gesture) = self.gesture.take() else {
            return false;
        };
        let changed = self.document != gesture.document;
        self.redo = gesture.redo;
        if changed {
            self.document = gesture.document;
            self.revision += 1;
            self.cache = None;
        }
        changed
    }

    pub fn undo(&mut self) -> bool {
        if self.gesture.is_some() {
            return false;
        }
        let Some(document) = self.undo.pop() else {
            return false;
        };
        push_history(
            &mut self.redo,
            std::mem::replace(&mut self.document, document),
        );
        self.revision += 1;
        self.cache = None;
        true
    }

    pub fn redo(&mut self) -> bool {
        if self.gesture.is_some() {
            return false;
        }
        let Some(document) = self.redo.pop() else {
            return false;
        };
        push_history(
            &mut self.undo,
            std::mem::replace(&mut self.document, document),
        );
        self.revision += 1;
        self.cache = None;
        true
    }

    pub fn reset(&mut self) -> bool {
        if self.gesture.is_some() || self.document == self.checkpoint {
            return false;
        }
        push_history(
            &mut self.undo,
            std::mem::replace(&mut self.document, self.checkpoint.clone()),
        );
        self.redo.clear();
        self.revision += 1;
        self.cache = None;
        true
    }
    pub fn checkpoint(&mut self) {
        self.checkpoint = self.document.clone();
    }
    pub fn set_checkpoint(
        &mut self,
        document: ThemeDocument,
        context: &ResolveContext<'_>,
    ) -> Result<(), ThemeError> {
        if document != self.document {
            resolve_pair(&document, context, &mut self.resolution_count)?;
        }
        self.checkpoint = document;
        Ok(())
    }
    pub fn checkpoint_document(&self) -> &ThemeDocument {
        &self.checkpoint
    }
    pub fn is_modified(&self) -> bool {
        self.document != self.checkpoint
    }
    pub fn undo_len(&self) -> usize {
        self.undo.len()
    }
    pub fn redo_len(&self) -> usize {
        self.redo.len()
    }
    pub fn gesture_active(&self) -> bool {
        self.gesture.is_some()
    }

    /// Includes validation calls, including a failed candidate or context.
    pub fn resolutions(&self) -> u64 {
        self.resolution_count
    }

    pub fn resolved_pair(
        &mut self,
        context: &ResolveContext<'_>,
    ) -> Result<[Arc<ResolvedTheme>; 2], ThemeError> {
        if let Some(cache) = self
            .cache
            .as_ref()
            .filter(|cache| cache.key.matches(self.revision, context))
        {
            return Ok(cache.pair.clone());
        }
        let pair = resolve_pair(&self.document, context, &mut self.resolution_count)?;
        self.cache = Some(ResolvedCache {
            key: ContextKey::new(self.revision, context),
            pair: pair.clone(),
        });
        Ok(pair)
    }

    fn require_idle(&self) -> Result<(), ThemeError> {
        if self.gesture.is_some() {
            Err(error(
                "gesture",
                "commit or cancel the current gesture first",
            ))
        } else {
            Ok(())
        }
    }

    fn require_gesture(&self) -> Result<&Gesture, ThemeError> {
        self.gesture
            .as_ref()
            .ok_or_else(|| error("gesture", "begin a gesture before updating"))
    }

    fn with_value(
        &self,
        token: &str,
        value: &str,
        scope: TokenScope,
    ) -> Result<ThemeDocument, ThemeError> {
        if !COLOR_TOKENS.contains(&token) && !VALUE_TOKENS.contains(&token) {
            return Err(error(token, "unknown theme token"));
        }
        let mut candidate = self.document.clone();
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            if scope == TokenScope::Both || COMMON_TOKENS.contains(&token) || mode == self.mode {
                candidate
                    .values_mut(mode)
                    .0
                    .insert(token.into(), value.into());
            }
        }
        Ok(candidate)
    }

    fn apply(
        &mut self,
        candidate: ThemeDocument,
        context: &ResolveContext<'_>,
        history: bool,
    ) -> Result<bool, ThemeError> {
        // This check deliberately precedes resolve, including for document replacements.
        if candidate == self.document {
            return Ok(false);
        }
        let pair = resolve_pair(&candidate, context, &mut self.resolution_count)?;
        let previous = std::mem::replace(&mut self.document, candidate);
        if history {
            push_history(&mut self.undo, previous);
        }
        self.redo.clear();
        self.revision += 1;
        self.cache = Some(ResolvedCache {
            key: ContextKey::new(self.revision, context),
            pair,
        });
        Ok(true)
    }
}

fn resolve_pair(
    document: &ThemeDocument,
    context: &ResolveContext<'_>,
    count: &mut u64,
) -> Result<[Arc<ResolvedTheme>; 2], ThemeError> {
    *count += 1;
    let light = Arc::new(rustify_ui::resolve(document, ThemeMode::Light, context)?);
    *count += 1;
    let dark = Arc::new(rustify_ui::resolve(document, ThemeMode::Dark, context)?);
    Ok([light, dark])
}

fn push_history(history: &mut Vec<ThemeDocument>, document: ThemeDocument) {
    if history.len() == HISTORY_LIMIT {
        history.remove(0);
    }
    history.push(document);
}

fn error(field: &str, message: &str) -> ThemeError {
    ThemeError {
        field: field.into(),
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustify_ui::{
        theme::{font_context, parse_color},
        Theme,
    };

    #[test]
    fn restored_modified_draft_resets_to_the_selected_checkpoint_without_startup_history() {
        let context = font_context(16., false);
        let checkpoint = state().document;
        let mut draft = checkpoint.clone();
        draft
            .styles
            .light
            .0
            .insert("primary".into(), "#882244".into());
        let mut restored = EditorState::restore(
            draft.clone(),
            ThemeMode::Light,
            checkpoint.clone(),
            &context,
        )
        .unwrap();
        assert!(restored.is_modified());
        assert_eq!(restored.undo_len(), 0);
        assert!(restored.reset());
        assert_eq!(restored.document, checkpoint);
        assert!(restored.undo());
        assert_eq!(restored.document, draft);
    }

    fn state() -> EditorState {
        EditorState::new(
            ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
            ThemeMode::Light,
            &font_context(16., false),
        )
        .unwrap()
    }

    fn value(state: &EditorState, mode: ThemeMode, token: &str) -> String {
        state.document.values(mode).get(token).unwrap().to_owned()
    }

    fn assert_valid_state_unchanged(state: &EditorState, before: &EditorState) {
        let mut actual = state.clone();
        // Rejected validations still count as actual resolver work.
        actual.resolution_count = before.resolution_count;
        assert_eq!(&actual, before);
    }

    #[test]
    fn common_edit_normalizes_both_modes_without_losing_initial_values() {
        let context = font_context(16., false);
        let mut document = state().document;
        document
            .styles
            .light
            .0
            .insert("radius".into(), "8px".into());
        document
            .styles
            .dark
            .0
            .insert("radius".into(), "12px".into());
        let original = document.clone();
        let mut state = EditorState::new(document, ThemeMode::Light, &context).unwrap();
        assert_eq!(state.document, original);
        assert!(state.edit("radius", "8px", &context).unwrap());
        assert_eq!(value(&state, ThemeMode::Dark, "radius"), "8px");
        assert!(state.undo());
        assert_eq!(state.document, original);
    }

    #[test]
    fn color_scope_is_explicit_and_common_fields_are_always_shared() {
        let context = font_context(16., false);
        let mut state = state();
        let dark = value(&state, ThemeMode::Dark, "primary");
        state.edit("primary", "#abc", &context).unwrap();
        assert_eq!(value(&state, ThemeMode::Dark, "primary"), dark);
        state
            .edit_scoped("primary", "#def", TokenScope::Both, &context)
            .unwrap();
        assert_eq!(value(&state, ThemeMode::Dark, "primary"), "#def");
        state
            .edit_scoped("font-size", "18px", TokenScope::Current, &context)
            .unwrap();
        assert_eq!(value(&state, ThemeMode::Dark, "font-size"), "18px");
    }

    #[test]
    fn invalid_half_value_and_other_mode_validation_are_atomic() {
        let context = font_context(16., false);
        let mut state = state();
        let before = state.clone();
        for (token, value) in [
            ("primary", "#"),
            ("letter-spacing", "0.51em"),
            ("unknown", "#abc"),
        ] {
            assert!(state.edit(token, value, &context).is_err());
            assert_valid_state_unchanged(&state, &before);
        }
        let mut candidate = state.document.clone();
        candidate
            .styles
            .dark
            .0
            .insert("primary".into(), "rgb(".into());
        assert!(state.apply_document(candidate, &context).is_err());
        assert_valid_state_unchanged(&state, &before);
    }

    #[test]
    fn no_op_skips_resolve_and_history() {
        let mut state = state();
        let context = font_context(f64::NAN, false);
        let before = state.clone();
        let primary = value(&state, ThemeMode::Light, "primary");
        assert!(!state.edit("primary", &primary, &context).unwrap());
        assert!(!state
            .apply_document(state.document.clone(), &context)
            .unwrap());
        assert_eq!(state, before);
    }

    #[test]
    fn resolved_pair_reuses_initial_and_accepted_edit_validation() {
        let context = font_context(16., false);
        let mut state = state();
        let initial = state.resolved_pair(&context).unwrap();
        let repeated = state.resolved_pair(&context).unwrap();
        assert!(Arc::ptr_eq(&initial[0], &repeated[0]));
        assert!(Arc::ptr_eq(&initial[1], &repeated[1]));
        assert_eq!(state.resolutions(), 2);
        state.edit("radius", "12px", &context).unwrap();
        assert_eq!(state.resolutions(), 4);
        let accepted = state.resolved_pair(&context).unwrap();
        assert_eq!(state.resolutions(), 4);
        assert_eq!(accepted[0].radius_px, 12.);
        assert_eq!(accepted[1].radius_px, 12.);
        assert!(!Arc::ptr_eq(&initial[0], &accepted[0]));
        state.edit("radius", "12px", &context).unwrap();
        state.mode = ThemeMode::Dark;
        let unchanged = state.resolved_pair(&context).unwrap();
        assert_eq!(state.resolutions(), 4);
        assert!(Arc::ptr_eq(&accepted[0], &unchanged[0]));
        assert!(Arc::ptr_eq(&accepted[1], &unchanged[1]));
    }

    #[test]
    fn undo_and_every_context_input_refresh_the_pair_once() {
        let context = font_context(16., false);
        let mut state = state();
        state.edit("radius", "12px", &context).unwrap();
        state.undo();
        assert_eq!(state.resolutions(), 4);
        state.resolved_pair(&context).unwrap();
        assert_eq!(state.resolutions(), 6);
        state.resolved_pair(&context).unwrap();
        assert_eq!(state.resolutions(), 6);
        let mut changed = font_context(20., false);
        for expected in [8, 10, 12, 14] {
            match expected {
                10 => changed.reduce_motion = true,
                12 => changed.fonts = &[],
                14 => changed.fallback_fonts = ["Noto Sans", "serif", "monospace"],
                _ => (),
            }
            let pair = state.resolved_pair(&changed).unwrap();
            assert_eq!(state.resolutions(), expected);
            let repeated = state.resolved_pair(&changed).unwrap();
            assert_eq!(state.resolutions(), expected);
            assert!(Arc::ptr_eq(&pair[0], &repeated[0]));
        }
        assert!(state.redo());
        state.resolved_pair(&changed).unwrap();
        assert_eq!(state.resolutions(), 16);
    }

    #[test]
    fn rejected_validation_counts_work_without_replacing_the_valid_cache() {
        let context = font_context(16., false);
        let mut state = state();
        let valid = state.resolved_pair(&context).unwrap();
        assert!(state.edit("primary", "#", &context).is_err());
        assert_eq!(state.resolutions(), 3);
        let mut candidate = state.document.clone();
        candidate.styles.dark.0.insert("primary".into(), "#".into());
        assert!(state.apply_document(candidate, &context).is_err());
        assert_eq!(state.resolutions(), 5);
        assert!(state.resolved_pair(&font_context(f64::NAN, false)).is_err());
        assert_eq!(state.resolutions(), 6);
        let retained = state.resolved_pair(&context).unwrap();
        assert_eq!(state.resolutions(), 6);
        assert!(Arc::ptr_eq(&valid[0], &retained[0]));
        assert!(Arc::ptr_eq(&valid[1], &retained[1]));
        assert_eq!(state.revision, 0);
        assert_eq!(state.undo_len(), 0);
    }

    #[test]
    fn commit_keeps_the_update_pair_and_cancel_refreshes_the_restored_document() {
        let context = font_context(16., false);
        let mut state = state();
        state.begin_gesture();
        state
            .update("radius", "12px", TokenScope::Both, &context)
            .unwrap();
        let updated = state.resolved_pair(&context).unwrap();
        state.commit_gesture();
        let committed = state.resolved_pair(&context).unwrap();
        assert!(Arc::ptr_eq(&updated[0], &committed[0]));
        assert_eq!(state.resolutions(), 4);
        state.begin_gesture();
        state
            .update("radius", "9px", TokenScope::Both, &context)
            .unwrap();
        state.cancel_gesture();
        assert_eq!(state.resolutions(), 6);
        let restored = state.resolved_pair(&context).unwrap();
        assert_eq!(restored[0].radius_px, 12.);
        assert_eq!(state.resolutions(), 8);
        state.resolved_pair(&context).unwrap();
        assert_eq!(state.resolutions(), 8);
        state.reset();
        assert_eq!(state.resolved_pair(&context).unwrap()[0].radius_px, 6.);
        assert_eq!(state.resolutions(), 10);
    }

    #[test]
    fn gesture_updates_make_one_history_entry_and_no_commit_revision() {
        let context = font_context(16., false);
        let mut state = state();
        let original = state.document.clone();
        assert!(state.begin_gesture());
        assert!(!state.begin_gesture());
        for radius in ["7px", "8px", "9px"] {
            assert!(state
                .update("radius", radius, TokenScope::Current, &context)
                .unwrap());
        }
        assert_eq!(state.revision, 3);
        assert_eq!(state.undo_len(), 0);
        assert!(state.commit_gesture());
        assert_eq!(state.revision, 3);
        assert_eq!(state.undo_len(), 1);
        assert!(state.undo());
        assert_eq!(state.document, original);
        assert!(state.redo());
        assert_eq!(value(&state, ThemeMode::Light, "radius"), "9px");
    }

    #[test]
    fn cancel_restores_document_and_redo_but_committed_branch_clears_redo() {
        let context = font_context(16., false);
        let mut state = state();
        state.edit("radius", "9px", &context).unwrap();
        state.undo();
        let original = state.document.clone();
        state.begin_gesture();
        state
            .update("radius", "12px", TokenScope::Current, &context)
            .unwrap();
        assert_eq!(state.redo_len(), 0);
        assert!(state.cancel_gesture());
        assert_eq!(state.document, original);
        assert_eq!(state.redo_len(), 1);
        state.begin_gesture();
        state
            .update("radius", "13px", TokenScope::Both, &context)
            .unwrap();
        state.commit_gesture();
        assert_eq!(state.redo_len(), 0);
        state.undo();
        state.edit("radius", "14px", &context).unwrap();
        assert_eq!(state.redo_len(), 0);
    }

    #[test]
    fn invalid_update_and_no_op_gesture_do_not_change_state() {
        let context = font_context(16., false);
        let mut state = state();
        state.begin_gesture();
        let before = state.clone();
        assert!(state
            .update("radius", "-", TokenScope::Both, &context)
            .is_err());
        assert_valid_state_unchanged(&state, &before);
        assert!(!state.commit_gesture());
        assert_eq!(state.revision, 0);
        assert_eq!(state.undo_len(), 0);
    }

    #[test]
    fn preset_mode_checkpoint_and_reset_are_independent_of_history() {
        let context = font_context(16., false);
        let mut state = state();
        state.mode = ThemeMode::Dark;
        let original = state.document.clone();
        let mut preset = original.clone();
        preset.id = "new-preset".into();
        state.apply_document(preset.clone(), &context).unwrap();
        assert_eq!(state.mode, ThemeMode::Dark);
        assert_eq!(state.undo_len(), 1);
        state.checkpoint();
        assert!(!state.is_modified());
        assert_eq!(state.checkpoint_document(), &preset);
        state.undo();
        assert_eq!(state.document, original);
        assert!(state.is_modified());
        assert_eq!(state.mode, ThemeMode::Dark);
        assert!(state.reset());
        assert_eq!(state.document, preset);
        assert!(!state.is_modified());
        state.undo();
        assert_eq!(state.document, original);
        assert!(!state.redo_len().eq(&0));
    }

    #[test]
    fn history_is_bounded_to_one_hundred_documents() {
        let context = font_context(16., false);
        let mut state = state();
        for radius in 1..=110 {
            state
                .edit("radius", &format!("{radius}px"), &context)
                .unwrap();
        }
        assert_eq!(state.undo_len(), HISTORY_LIMIT);
        for _ in 0..HISTORY_LIMIT {
            assert!(state.undo());
        }
        assert!(!state.undo());
        assert_eq!(value(&state, ThemeMode::Light, "radius"), "10px");
    }

    #[test]
    fn hsl_updates_use_gesture_origin_and_identity_preserves_author_values() {
        let context = font_context(16., false);
        let mut state = state();
        state
            .edit_scoped(
                "primary",
                "hsl(20 40% 30% / 0.4)",
                TokenScope::Both,
                &context,
            )
            .unwrap();
        let original = state.document.clone();
        state.begin_gesture();
        state
            .update_hsl(
                HslShift {
                    hue_degrees: 30.,
                    ..Default::default()
                },
                TokenScope::Both,
                &context,
            )
            .unwrap();
        state
            .update_hsl(
                HslShift {
                    hue_degrees: 60.,
                    saturation_scale: 1.5,
                    lightness_scale: 1.2,
                },
                TokenScope::Both,
                &context,
            )
            .unwrap();
        let color = parse_color(&value(&state, ThemeMode::Light, "primary"))
            .unwrap()
            .rgba;
        let hsl = rustify_ui::theme::rgba_to_hsla(color);
        assert!((hsl.h - 80.).abs() < 1e-5);
        assert!((hsl.s - 0.6).abs() < 1e-5);
        assert!((hsl.l - 0.36).abs() < 1e-5);
        assert_eq!(color.a, 0.4);
        state
            .update_hsl(HslShift::default(), TokenScope::Both, &context)
            .unwrap();
        assert_eq!(state.document, original);
        assert!(!state.commit_gesture());
        assert_eq!(state.undo_len(), 1);
    }

    #[test]
    fn next_hsl_gesture_starts_from_the_last_committed_result() {
        let context = font_context(16., false);
        let mut state = state();
        state.edit("primary", "hsl(20 40% 30%)", &context).unwrap();
        for expected_hue in [50., 80.] {
            state.begin_gesture();
            state
                .update_hsl(
                    HslShift {
                        hue_degrees: 30.,
                        ..Default::default()
                    },
                    TokenScope::Current,
                    &context,
                )
                .unwrap();
            assert!(state.commit_gesture());
            let color = parse_color(&value(&state, ThemeMode::Light, "primary"))
                .unwrap()
                .rgba;
            assert!((rgba_to_hsla(color).h - expected_hue).abs() < 1e-5);
        }
        assert!(state.undo());
        let color = parse_color(&value(&state, ThemeMode::Light, "primary"))
            .unwrap()
            .rgba;
        assert!((rgba_to_hsla(color).h - 50.).abs() < 1e-5);
    }

    #[test]
    fn gesture_returning_to_its_origin_keeps_the_redo_branch() {
        let context = font_context(16., false);
        let mut state = state();
        state.edit("radius", "9px", &context).unwrap();
        state.undo();
        state.begin_gesture();
        state
            .update("radius", "7px", TokenScope::Both, &context)
            .unwrap();
        state
            .update("radius", "6px", TokenScope::Both, &context)
            .unwrap();
        let revision = state.revision;
        assert!(!state.commit_gesture());
        assert_eq!(state.revision, revision);
        assert_eq!(state.undo_len(), 0);
        assert_eq!(state.redo_len(), 1);
        assert!(state.redo());
        assert_eq!(value(&state, ThemeMode::Light, "radius"), "9px");
    }

    #[test]
    fn hsl_default_scope_both_current_scope_and_grayscale_are_finite() {
        let context = font_context(16., false);
        let mut state = state();
        state
            .edit_scoped("primary", "#8888", TokenScope::Both, &context)
            .unwrap();
        let dark = state.document.styles.dark.clone();
        state.begin_gesture();
        state
            .update_hsl(
                HslShift {
                    hue_degrees: 180.,
                    saturation_scale: 2.,
                    lightness_scale: 1.2,
                },
                TokenScope::Current,
                &context,
            )
            .unwrap();
        assert_eq!(state.document.styles.dark, dark);
        let rgba = parse_color(&value(&state, ThemeMode::Light, "primary"))
            .unwrap()
            .rgba;
        assert!(rgba.r.is_finite() && rgba.g.is_finite() && rgba.b.is_finite());
        assert_eq!(rgba.r, rgba.g);
        assert_eq!(rgba.g, rgba.b);
        assert!((rgba.a - 8. / 15.).abs() < 1e-8);
        state.cancel_gesture();
        state.begin_gesture();
        state
            .update_hsl(
                HslShift {
                    hue_degrees: 30.,
                    ..Default::default()
                },
                TokenScope::default(),
                &context,
            )
            .unwrap();
        assert_ne!(state.document.styles.dark, dark);
    }

    #[test]
    fn invalid_hsl_and_gesture_commands_preserve_state() {
        let context = font_context(16., false);
        let mut state = state();
        assert!(state
            .update_hsl(HslShift::default(), TokenScope::Both, &context)
            .is_err());
        state.begin_gesture();
        let before = state.clone();
        for shift in [
            HslShift {
                hue_degrees: f64::NAN,
                ..Default::default()
            },
            HslShift {
                hue_degrees: 181.,
                ..Default::default()
            },
            HslShift {
                saturation_scale: 2.1,
                ..Default::default()
            },
            HslShift {
                lightness_scale: 0.1,
                ..Default::default()
            },
        ] {
            assert!(state.update_hsl(shift, TokenScope::Both, &context).is_err());
            assert_eq!(state, before);
        }
        assert!(state
            .apply_document(state.document.clone(), &context)
            .is_err());
        assert_eq!(state, before);
        assert!(!state.undo());
        assert!(!state.redo());
        assert!(!state.reset());
        assert_eq!(state, before);
    }
    #[test]
    fn adopting_an_external_draft_keeps_its_preset_reset_and_undo_history() {
        let context = ResolveContext::default();
        let mut editor = state();
        let original = editor.document.clone();
        let mut preset = original.clone();
        preset.id = "external-preset".into();
        let mut draft = preset.clone();
        draft
            .styles
            .light
            .0
            .insert("primary".into(), "#00a88a".into());
        editor.apply_document(draft.clone(), &context).unwrap();
        editor.set_checkpoint(preset.clone(), &context).unwrap();
        assert!(editor.is_modified());
        assert_eq!(editor.undo_len(), 1);
        assert!(editor.reset());
        assert_eq!(editor.document, preset);
        assert!(editor.undo());
        assert_eq!(editor.document, draft);
        assert!(editor.undo());
        assert_eq!(editor.document, original);
        let mut invalid = preset.clone();
        invalid.styles.dark.0.insert("primary".into(), "bad".into());
        assert!(editor.set_checkpoint(invalid, &context).is_err());
        assert_eq!(editor.checkpoint_document(), &preset);
    }
}
