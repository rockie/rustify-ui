use crate::preview_region::PreviewControls;
use crate::{
    persistence::{self, Envelope, LoadResult, Preferences},
    presets,
    state::{EditorState, HslShift, TokenScope},
};
use leptos::{prelude::*, wasm_bindgen::JsCast};
use rustify_ui::{
    theme::{font_context, ResolveContext},
    Locale, ResolvedTheme, ThemeDocument, ThemeError, ThemeMode, ThemeValuePatch,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct PublishedFrame {
    pub main: Arc<ResolvedTheme>,
    pub compare: Arc<ResolvedTheme>,
    pub revision: u64,
}

#[derive(Clone, Copy)]
pub struct StudioState {
    initial_editor: StoredValue<EditorState>,
    initial_envelope: StoredValue<Envelope>,
    autosave: StoredValue<persistence::Autosave>,
    pub reset_epoch: RwSignal<u64>,
    pub editor: RwSignal<EditorState>,
    pub rem: RwSignal<f64>,
    pub reduce_motion: RwSignal<bool>,
    pub compare_mode: RwSignal<ThemeMode>,
    pub frame: RwSignal<PublishedFrame>,
    pub request: RwSignal<u64>,
    pub patch: RwSignal<ThemeValuePatch>,
    pub error: RwSignal<String>,
    pub envelope: RwSignal<Envelope>,
    pub corrupt: RwSignal<Option<String>>,
    pub external: RwSignal<Option<String>>,
    pub storage_status: RwSignal<String>,
    pub selected_token: RwSignal<String>,
    pub editor_tab: RwSignal<String>,
    pub mobile_tab: RwSignal<String>,
    pub inspector: RwSignal<bool>,
    pub drawn: RwSignal<u64>,
    pub samples: RwSignal<Vec<serde_json::Value>>,
    pub controls: RwSignal<PreviewControls>,
    pub gpu_state: RwSignal<rustify_ui::RegionState>,
    pub publish_count: RwSignal<u64>,
    pub last_input_ms: RwSignal<f64>,
    pub timings: RwSignal<Vec<(u64, f64, f64, f64)>>,
}

pub fn now() -> f64 {
    leptos::web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or_default()
}

fn checkpoint_for(envelope: &Envelope) -> ThemeDocument {
    envelope
        .saved_themes
        .iter()
        .find(|theme| theme.id == envelope.draft.id)
        .cloned()
        .or_else(|| {
            presets::built_in().ok().and_then(|themes| {
                themes
                    .into_iter()
                    .find(|theme| theme.id == envelope.draft.id)
            })
        })
        .unwrap_or_else(|| envelope.draft.clone())
}

impl StudioState {
    pub fn new(rem: f64, reduce_motion: bool) -> Self {
        let context = font_context(rem, reduce_motion);
        let default = presets::built_in()
            .expect("validated built-in presets")
            .remove(0);
        let empty = Envelope {
            envelope_version: 1,
            draft: default,
            saved_themes: Vec::new(),
            preferences: Preferences::default(),
        };
        let (envelope, corrupt, status) = match persistence::load_storage(&context) {
            Ok(LoadResult::Valid(value)) => (value, None, "Restored local draft".to_owned()),
            Ok(LoadResult::Invalid { raw, error }) => (
                empty,
                Some(raw),
                format!("Local data could not be read: {error}"),
            ),
            Ok(LoadResult::Missing) => (empty, None, String::new()),
            Err(error) => (empty, None, format!("Local storage unavailable: {error}")),
        };
        let checkpoint = checkpoint_for(&envelope);
        let mut editor = EditorState::restore(
            envelope.draft.clone(),
            envelope.preferences.mode,
            checkpoint,
            &context,
        )
        .expect("validated initial draft");
        let pair = editor
            .resolved_pair(&context)
            .expect("initial resolved pair");
        let main = pair[usize::from(editor.mode == ThemeMode::Dark)].clone();
        Self {
            initial_editor: StoredValue::new(editor.clone()),
            initial_envelope: StoredValue::new(envelope.clone()),
            autosave: StoredValue::new(persistence::Autosave::default()),
            reset_epoch: RwSignal::new(0),
            editor: RwSignal::new(editor),
            rem: RwSignal::new(rem),
            reduce_motion: RwSignal::new(reduce_motion),
            compare_mode: RwSignal::new(ThemeMode::Dark),
            frame: RwSignal::new(PublishedFrame {
                main,
                compare: pair[1].clone(),
                revision: 0,
            }),
            request: RwSignal::new(0),
            patch: RwSignal::new(ThemeValuePatch::default()),
            error: RwSignal::new(String::new()),
            envelope: RwSignal::new(envelope),
            corrupt: RwSignal::new(corrupt),
            external: RwSignal::new(None),
            storage_status: RwSignal::new(status),
            selected_token: RwSignal::new("primary".into()),
            editor_tab: RwSignal::new("colors".into()),
            mobile_tab: RwSignal::new("edit".into()),
            inspector: RwSignal::new(false),
            drawn: RwSignal::new(0),
            samples: RwSignal::new(Vec::new()),
            controls: RwSignal::new(PreviewControls::default()),
            gpu_state: RwSignal::new(rustify_ui::RegionState::Starting),
            publish_count: RwSignal::new(0),
            last_input_ms: RwSignal::new(0.),
            timings: RwSignal::new(Vec::new()),
        }
    }

    pub fn context(self) -> ResolveContext<'static> {
        font_context(self.rem.get_untracked(), self.reduce_motion.get_untracked())
    }
    pub fn document(self) -> Signal<ThemeDocument> {
        Signal::derive(move || self.editor.with(|e| e.document.clone()))
    }
    pub fn locale(self) -> Signal<Locale> {
        Signal::derive(move || {
            self.envelope.with(|e| {
                if e.preferences.locale == "zh" {
                    Locale::Chinese
                } else {
                    Locale::English
                }
            })
        })
    }
    pub fn mode(self) -> Signal<ThemeMode> {
        Signal::derive(move || self.editor.with(|e| e.mode))
    }
    pub fn resolved(self) -> Signal<Arc<ResolvedTheme>> {
        Signal::derive(move || self.frame.with(|f| f.main.clone()))
    }
    pub fn values(self) -> Signal<rustify_ui::ThemeValues> {
        Signal::derive(move || self.editor.with(|e| e.document.values(e.mode).clone()))
    }
    pub fn baseline(self) -> Signal<rustify_ui::ThemeValues> {
        Signal::derive(move || {
            self.editor
                .with(|e| e.checkpoint_document().values(e.mode).clone())
        })
    }
    pub fn tr(self, en: &'static str, zh: &'static str) -> &'static str {
        if self.locale().get() == Locale::Chinese {
            zh
        } else {
            en
        }
    }

    /// One publication for all accepted changes before the next repaint.
    pub fn connect(self) {
        let pending = StoredValue::new_local(None::<rustify_ui::FrameHandle>);
        Effect::new(move || {
            self.request.get();
            if pending.with_value(Option::is_some) {
                return;
            }
            pending.set_value(Some(rustify_ui::next_frame(move || {
                pending.update_value(|value| {
                    value.take();
                });
                let context = self.context();
                let pair = self.editor.try_update(|e| e.resolved_pair(&context));
                match pair {
                    Some(Ok(pair)) => {
                        let mode = self.editor.with_untracked(|e| e.mode);
                        let main = pair[usize::from(mode == ThemeMode::Dark)].clone();
                        let compare = pair
                            [usize::from(self.compare_mode.get_untracked() == ThemeMode::Dark)]
                        .clone();
                        let unchanged = self.frame.with_untracked(|f| {
                            Arc::ptr_eq(&main, &f.main) && Arc::ptr_eq(&compare, &f.compare)
                        });
                        if unchanged {
                            return;
                        }
                        let revision = self.frame.with_untracked(|f| f.revision) + 1;
                        self.frame.set(PublishedFrame {
                            main,
                            compare,
                            revision,
                        });
                        self.publish_count.update(|n| *n += 1);
                        let input = self.last_input_ms.get_untracked();
                        self.timings.update(|values| {
                            if values.len() == 500 {
                                values.remove(0);
                            }
                            values.push((revision, input, now(), 0.));
                        });
                    }
                    Some(Err(error)) => self.error.set(error.to_string()),
                    None => {}
                }
            })));
        });
        on_cleanup(move || {
            pending.with_value(|value| {
                if let Some(frame) = value {
                    frame.cancel();
                }
            });
        });

        let target: leptos::web_sys::EventTarget = window().into();
        let storage = rustify_ui::listen(&target, "storage", Default::default(), move |event| {
            let Some(event) = event.dyn_ref::<leptos::web_sys::StorageEvent>() else {
                return;
            };
            if event.key().as_deref() != Some(persistence::STORAGE_KEY) {
                return;
            }
            self.external
                .set(Some(event.new_value().unwrap_or_default()));
            self.storage_status.set(
                self.tr(
                    "Another tab changed local data. Choose which draft to keep.",
                    "另一页更改了本地数据，请选择保留的草稿。",
                )
                .into(),
            );
        });
        let keyboard = rustify_ui::listen(&target, "keydown", Default::default(), move |event| {
            let Some(key) = event.dyn_ref::<leptos::web_sys::KeyboardEvent>() else {
                return;
            };
            if key.is_composing()
                || key.alt_key()
                || !(key.ctrl_key() || key.meta_key())
                || !key.key().eq_ignore_ascii_case("z")
                || key
                    .target()
                    .as_ref()
                    .is_some_and(rustify_ui::shortcut::is_text_entry)
            {
                return;
            }
            key.prevent_default();
            self.history(key.shift_key());
        });
        on_cleanup(move || {
            drop(storage);
            drop(keyboard);
        });
    }

    fn changed(self) {
        self.last_input_ms.set(now());
        self.request.update(|n| *n += 1);
    }
    fn command(
        self,
        run: impl FnOnce(&mut EditorState, &ResolveContext<'_>) -> Result<bool, ThemeError>,
        persist: bool,
    ) -> bool {
        let context = self.context();
        match self.editor.try_update(|e| run(e, &context)) {
            Some(Ok(changed)) => {
                self.error.set(String::new());
                if changed {
                    self.changed();
                    if persist {
                        self.persist();
                    }
                }
                true
            }
            Some(Err(error)) => {
                self.error.set(error.to_string());
                false
            }
            None => false,
        }
    }
    pub fn edit(self, token: &str, value: String) -> bool {
        self.command(|e, c| e.edit(token, &value, c), true)
    }
    pub fn begin(self) {
        self.editor.update(|e| {
            e.begin_gesture();
        });
    }
    pub fn update(self, token: String, value: String) {
        self.command(
            |e, c| e.update(&token, &value, TokenScope::Current, c),
            false,
        );
    }
    pub fn update_hsl(self, shift: HslShift, scope: TokenScope) {
        self.command(|e, c| e.update_hsl(shift, scope, c), false);
    }
    pub fn commit(self) {
        let changed = self
            .editor
            .try_update(EditorState::commit_gesture)
            .unwrap_or(false);
        if changed {
            self.persist();
        }
    }
    pub fn cancel(self) {
        if self
            .editor
            .try_update(EditorState::cancel_gesture)
            .unwrap_or(false)
        {
            self.changed();
        }
        self.error.set(String::new());
    }
    pub fn history(self, redo: bool) {
        if self
            .editor
            .try_update(|e| if redo { e.redo() } else { e.undo() })
            .unwrap_or(false)
        {
            self.changed();
            self.persist();
        }
    }
    pub fn reset(self) {
        if self.editor.try_update(EditorState::reset).unwrap_or(false) {
            self.changed();
            self.persist();
        }
        self.patch.set(ThemeValuePatch::default());
        self.controls.set(PreviewControls::default());
        self.error.set(String::new());
    }
    /// The browser fixture returns to its first-load state without deleting or
    /// overwriting any persisted user data, and keeps the same GPU region.
    pub fn reset_fixture(self) {
        self.editor.set(self.initial_editor.get_value());
        self.envelope.set(self.initial_envelope.get_value());
        self.compare_mode.set(ThemeMode::Dark);
        self.patch.set(ThemeValuePatch::default());
        self.controls.set(PreviewControls::default());
        self.error.set(String::new());
        self.external.set(None);
        self.selected_token.set("primary".into());
        self.editor_tab.set("colors".into());
        self.mobile_tab.set("edit".into());
        self.inspector.set(false);
        self.reset_epoch.update(|n| *n += 1);
        self.changed();
    }
    pub fn set_mode(self, mode: ThemeMode) {
        if self.editor.with_untracked(|e| e.mode) != mode {
            self.editor.update(|e| e.mode = mode);
            self.changed();
            self.persist();
        }
    }
    pub fn set_compare_mode(self, mode: ThemeMode) {
        if self.compare_mode.get_untracked() != mode {
            self.compare_mode.set(mode);
            self.changed();
        }
    }
    pub fn apply(self, document: ThemeDocument, checkpoint: bool) -> bool {
        let previous = self.editor.with_untracked(|e| e.revision);
        let accepted = self.command(|e, c| e.apply_document(document, c), false);
        if accepted {
            if checkpoint {
                self.editor.update(EditorState::checkpoint);
            }
            if self.editor.with_untracked(|e| e.revision) != previous {
                self.persist();
            }
        }
        accepted
    }
    pub fn preference(self, run: impl FnOnce(&mut Preferences)) {
        let previous = self.envelope.with_untracked(|e| e.preferences.clone());
        self.envelope.update(|e| run(&mut e.preferences));
        if self.envelope.with_untracked(|e| e.preferences != previous) {
            self.persist();
        }
    }
    pub fn favorite(self, id: String) {
        self.preference(|p| {
            if let Some(index) = p.favorites.iter().position(|value| *value == id) {
                p.favorites.remove(index);
            } else {
                p.favorites.push(id);
            }
        });
    }
    pub fn persist(self) -> bool {
        if self.corrupt.get_untracked().is_some() || self.external.get_untracked().is_some() {
            return false;
        }
        let mut editor = self.editor.get_untracked();
        if editor.gesture_active() {
            return false;
        }
        let mut envelope = self.envelope.get_untracked();
        envelope.draft = editor.document.clone();
        envelope.preferences.mode = editor.mode;
        let result = self
            .autosave
            .try_update_value(|autosave| autosave.save(&envelope, &mut editor, &self.context()));
        let Some(result) = result else { return false };
        match result {
            Ok(()) => {
                self.envelope.set(envelope);
                self.storage_status
                    .set(self.tr("Saved on this device", "已保存到此设备").into());
                true
            }
            Err(error) => {
                self.storage_status.set(format!(
                    "{}: {error}",
                    self.tr(
                        "Changes remain in memory; local save failed",
                        "改动仍保留在内存，本地保存失败"
                    )
                ));
                false
            }
        }
    }
    pub fn clear_cache(self) {
        match persistence::remove_storage() {
            Ok(()) => {
                self.corrupt.set(None);
                self.external.set(None);
                self.storage_status.set(
                    self.tr(
                        "Local cache cleared. Your open draft remains in memory.",
                        "本地缓存已清除，当前草稿保留在内存。",
                    )
                    .into(),
                );
            }
            Err(error) => self.storage_status.set(error.to_string()),
        }
    }
    pub fn load_external(self) {
        let Some(raw) = self.external.get_untracked() else {
            return;
        };
        match persistence::load((!raw.is_empty()).then_some(raw.as_str()), &self.context()) {
            LoadResult::Valid(envelope) => {
                let doc = envelope.draft.clone();
                let mode = envelope.preferences.mode;
                let checkpoint = checkpoint_for(&envelope);
                if self.command(|e, c| e.apply_document(doc, c), false) {
                    let context = self.context();
                    self.editor.update(|e| {
                        e.mode = mode;
                        e.set_checkpoint(checkpoint, &context)
                            .expect("validated external checkpoint");
                    });
                    self.envelope.set(envelope);
                    self.external.set(None);
                    self.corrupt.set(None);
                    self.changed();
                    self.storage_status.set(
                        self.tr("Loaded the other tab's draft", "已加载另一页的草稿")
                            .into(),
                    );
                }
            }
            LoadResult::Missing => {
                self.external.set(None);
                self.storage_status.set(
                    self.tr(
                        "The other tab cleared local data. Your draft remains in memory.",
                        "另一页已清除本地数据，当前草稿仍保留在内存。",
                    )
                    .into(),
                );
            }
            LoadResult::Invalid { error, .. } => self.storage_status.set(error.to_string()),
        }
    }
    pub fn keep_current(self) {
        let pending = self.external.get_untracked();
        self.external.set(None);
        if !self.persist() {
            self.external.set(pending);
        }
    }
    pub fn load_local(self, id: String) {
        if let Some(document) = self
            .envelope
            .with_untracked(|e| e.saved_themes.iter().find(|theme| theme.id == id).cloned())
        {
            self.apply(document, true);
        }
    }
    pub fn new_id(self) -> String {
        format!(
            "local-{}-{}",
            js_sys::Date::now() as u64,
            self.request.get_untracked()
        )
    }
    pub fn save_named(self, name: String, overwrite: Option<String>) {
        if self.corrupt.get_untracked().is_some() || self.external.get_untracked().is_some() {
            self.error.set(
                self.tr(
                    "Resolve the local data warning before saving a named theme.",
                    "请先处理本地数据提示，再保存命名主题。",
                )
                .into(),
            );
            return;
        }
        let id = overwrite.clone().unwrap_or_else(|| self.new_id());
        let document = self.editor.with_untracked(|e| e.document.clone());
        match self.envelope.with_untracked(|e| {
            e.save_named(&document, &id, &name, overwrite.as_deref(), &self.context())
        }) {
            Ok(mut envelope) => {
                let saved = envelope
                    .saved_themes
                    .iter()
                    .find(|t| t.id == id)
                    .cloned()
                    .expect("saved theme");
                let mut editor = self.editor.get_untracked();
                if let Err(error) = editor.apply_document(saved, &self.context()) {
                    self.error.set(error.to_string());
                    return;
                }
                editor.checkpoint();
                envelope.draft = editor.document.clone();
                envelope.preferences.mode = editor.mode;
                self.commit_named(envelope, Some(editor), false);
            }
            Err(error) => self.error.set(error.to_string()),
        }
    }
    pub fn copy_named(self, id: String, name: String) {
        let next = self.new_id();
        let result = self
            .envelope
            .with_untracked(|e| e.copy_theme(&id, &next, &name, &self.context()));
        self.library_result(result);
    }
    pub fn rename_named(self, id: String, name: String) {
        let result = self
            .envelope
            .with_untracked(|e| e.rename_theme(&id, &name, &self.context()));
        self.library_result(result);
    }
    pub fn delete_named(self, id: String) {
        let result = self
            .envelope
            .with_untracked(|e| e.delete_theme(&id, &self.context()));
        self.library_result(result);
    }
    fn library_result(self, result: Result<Envelope, ThemeError>) {
        match result {
            Ok(mut envelope) => {
                let editor = self.editor.get_untracked();
                envelope.draft = editor.document;
                envelope.preferences.mode = editor.mode;
                self.commit_named(envelope, None, false);
            }
            Err(error) => self.error.set(error.to_string()),
        }
    }
    fn commit_named(self, envelope: Envelope, editor: Option<EditorState>, clear_conflict: bool) {
        if self.corrupt.get_untracked().is_some()
            || (!clear_conflict && self.external.get_untracked().is_some())
        {
            self.error.set(
                self.tr(
                    "Resolve the local data warning first.",
                    "请先处理本地数据提示。",
                )
                .into(),
            );
            return;
        }
        match persistence::save_storage(&envelope, &self.context()) {
            Ok(()) => {
                self.envelope.set(envelope);
                if let Some(editor) = editor {
                    self.editor.set(editor);
                    self.changed();
                }
                if clear_conflict {
                    self.external.set(None);
                }
                self.error.set(String::new());
                self.storage_status
                    .set(self.tr("Saved on this device", "已保存到此设备").into());
            }
            Err(error) => {
                self.error.set(error.to_string());
                self.storage_status.set(format!(
                    "{}: {error}",
                    self.tr(
                        "Local save failed; your draft remains in memory",
                        "本地保存失败，草稿仍保留在内存"
                    )
                ));
            }
        }
    }
    pub fn conflict_copy(self, name: String) {
        let Some(raw) = self.external.get_untracked() else {
            return;
        };
        if let LoadResult::Valid(envelope) = persistence::load(Some(&raw), &self.context()) {
            let id = self.new_id();
            let mut editor = self.editor.get_untracked();
            match envelope.save_named(&editor.document, &id, &name, None, &self.context()) {
                Ok(mut candidate) => {
                    let saved = candidate
                        .saved_themes
                        .iter()
                        .find(|t| t.id == id)
                        .cloned()
                        .expect("saved conflict copy");
                    if let Err(error) = editor.apply_document(saved, &self.context()) {
                        self.error.set(error.to_string());
                        return;
                    }
                    editor.checkpoint();
                    candidate.draft = editor.document.clone();
                    candidate.preferences = self.envelope.get_untracked().preferences;
                    candidate.preferences.mode = editor.mode;
                    self.commit_named(candidate, Some(editor), true);
                }
                Err(error) => self.error.set(error.to_string()),
            }
        } else {
            self.error.set(
                self.tr(
                    "Load or clear the invalid external cache before saving a copy.",
                    "外部缓存无效，请先加载或清除，再保存副本。",
                )
                .into(),
            );
        }
    }
    pub fn inspect(self, token: String) {
        self.selected_token.set(token.clone());
        self.editor_tab.set(
            if rustify_ui::theme::COLOR_TOKENS.contains(&token.as_str()) {
                "colors"
            } else {
                "controls"
            }
            .into(),
        );
        self.mobile_tab.set("edit".into());
        rustify_ui::next_frame(move || {
            let id = if rustify_ui::theme::COLOR_TOKENS.contains(&token.as_str()) {
                format!("color-token-{token}")
            } else {
                format!("edit-{token}")
            };
            if let Ok(Some(element)) = document().query_selector(&format!("[data-testid='{id}']")) {
                element.scroll_into_view();
                if let Some(el) = element.dyn_ref::<leptos::web_sys::HtmlElement>() {
                    let _ = el.focus();
                }
            }
        });
    }
}
