use crate::preview_region::PreviewControls;
use crate::{
    persistence::{self, Envelope, LoadResult, Preferences},
    presets,
    state::{EditorState, HslShift, TokenScope},
};
use crate::{
    storage_browser as storage,
    write_queue::{Intent, Permission, Prepared, Snapshot, Writer},
};
use leptos::{prelude::*, wasm_bindgen::JsCast};
use rustify_ui::{
    theme::{font_context, ResolveContext},
    Locale, ResolvedTheme, ThemeDocument, ThemeError, ThemeMode, ThemeValuePatch,
};
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct PublishedFrame {
    pub main: Arc<ResolvedTheme>,
    pub compare: Arc<ResolvedTheme>,
    pub revision: u64,
}

#[derive(Clone, Debug)]
pub struct UpgradeReview {
    pub raw: String,
    pub snapshot: Snapshot,
    pub candidate: Envelope,
}

#[derive(Clone, Copy)]
pub struct StudioState {
    pub writer: StoredValue<Writer>,
    lifetime: StoredValue<storage::Lifetime, LocalStorage>,
    pub editor_epoch: RwSignal<u64>,
    pub storage_epoch: RwSignal<u64>,
    pub legacy_raw: RwSignal<Option<String>>,
    pub legacy_changed: RwSignal<bool>,
    pub upgrade_review: RwSignal<Option<UpgradeReview>>,
    pub backup_review: RwSignal<Option<BTreeMap<String, String>>>,
    pub clear_review: RwSignal<Option<Option<String>>>,
    pub conflict_review: RwSignal<Option<Option<String>>>,

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
    pub external: RwSignal<Option<Option<String>>>,
    pub storage_status: RwSignal<String>,
    pub storage_issue: RwSignal<String>,
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
            envelope_version: 2,
            revision: 0,
            migrated_from_v1: None,
            draft: default,
            saved_themes: Vec::new(),
            preferences: Preferences::default(),
        };
        let raw_v2 = storage::read(persistence::STORAGE_KEY);
        let mut baseline = None;
        let mut legacy_raw = None;
        let mut permission = Permission::Ready;
        let (envelope, corrupt, mut status) = match raw_v2 {
            Ok(Some(raw)) => {
                baseline = Some(raw.clone());
                match persistence::load(Some(&raw), &context) {
                    LoadResult::Valid(value) => (*value, None, "Restored local draft".to_owned()),
                    LoadResult::Invalid { raw, error } => {
                        permission = Permission::Protected;
                        (
                            empty,
                            Some(raw),
                            format!("Local data could not be read: {error}"),
                        )
                    }
                    LoadResult::Missing => unreachable!(),
                }
            }
            Ok(None) => match storage::read(persistence::LEGACY_KEY) {
                Ok(Some(raw)) => {
                    legacy_raw = Some(raw.clone());
                    match persistence::migrate_legacy(&raw, &context) {
                        Ok(value) => {
                            permission = Permission::UpgradePending;
                            (value, None, "Legacy draft restored in memory; review and confirm the complete library before saving".into())
                        }
                        Err(error) => {
                            permission = Permission::Protected;
                            (
                                empty,
                                Some(raw),
                                format!("Legacy data could not be read: {error}"),
                            )
                        }
                    }
                }
                Ok(None) => (empty, None, String::new()),
                Err(error) => {
                    permission = Permission::Protected;
                    (empty, None, format!("Local storage unavailable: {error}"))
                }
            },
            Err(error) => {
                permission = Permission::Protected;
                (empty, None, format!("Local storage unavailable: {error}"))
            }
        };
        let legacy_changed = envelope.migrated_from_v1.as_ref().is_some_and(|source| {
            baseline.is_some()
                && match storage::read(persistence::LEGACY_KEY) {
                    Ok(Some(raw)) => persistence::digest(&raw) != source.sha256,
                    Ok(None) => true,
                    Err(_) => false,
                }
        });
        if !storage::writable() {
            status = "Local saving is unavailable in this browser. You can edit in memory and download or export your theme.".into();
        }
        let writer = Writer::new(envelope.clone(), baseline, permission);
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
            writer: StoredValue::new(writer),
            lifetime: StoredValue::new_local(storage::Lifetime::new()),
            editor_epoch: RwSignal::new(0),
            storage_epoch: RwSignal::new(0),
            legacy_raw: RwSignal::new(legacy_raw),
            legacy_changed: RwSignal::new(legacy_changed),
            upgrade_review: RwSignal::new(None),
            backup_review: RwSignal::new(None),
            clear_review: RwSignal::new(None),
            conflict_review: RwSignal::new(None),
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
            storage_issue: RwSignal::new(String::new()),
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
        Signal::derive(move || {
            self.editor
                .with(|e| e.values(e.mode))
                .unwrap_or_else(|_| self.frame.with(|frame| frame.main.values.clone()))
        })
    }
    pub fn baseline(self) -> Signal<rustify_ui::ThemeValues> {
        Signal::derive(move || {
            self.editor
                .with(|e| e.checkpoint_document().effective_values(e.mode))
                .unwrap_or_else(|_| self.frame.with(|frame| frame.main.values.clone()))
        })
    }
    pub fn tr(self, en: &'static str, zh: &'static str) -> &'static str {
        if self
            .envelope
            .with(|envelope| envelope.preferences.locale == "zh")
        {
            zh
        } else {
            en
        }
    }

    /// One publication for all accepted changes before the next repaint.
    pub fn connect(self) {
        self.refresh_storage();
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
        let storage_listener = rustify_ui::listen(
            &target,
            "storage",
            Default::default(),
            move |event| {
                let Some(event) = event.dyn_ref::<leptos::web_sys::StorageEvent>() else {
                    return;
                };
                let key = event.key();
                if key.as_deref() == Some(persistence::LEGACY_KEY) {
                    self.legacy_changed.set(true);
                    self.storage_status.set(self.tr("An older window changed the legacy library. Original data remains available; review before importing it.", "旧版窗口修改了旧库。原数据仍可取回；导入前请重新检查。").into());
                    return;
                }
                if key.is_some() && key.as_deref() != Some(persistence::STORAGE_KEY) {
                    return;
                }
                let raw = event.new_value();
                if self.writer.with_value(|writer| writer.baseline == raw) {
                    return;
                }
                self.pause_for_conflict(raw);
            },
        );
        let abort = self
            .lifetime
            .with_value(|lifetime| storage::abort_handle(&lifetime.guard));
        on_cleanup(move || {
            let _ = abort.call0(&leptos::wasm_bindgen::JsValue::NULL);
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
            drop(storage_listener);
            drop(keyboard);
        });
    }

    fn changed(self) {
        self.editor_epoch.update(|epoch| *epoch += 1);
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
        if self.editor.try_update(EditorState::begin_gesture) == Some(true) {
            self.editor_epoch.update(|epoch| *epoch += 1);
        }
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
        let active = self.editor.with_untracked(EditorState::gesture_active);
        self.editor.update(|editor| {
            editor.commit_gesture();
        });
        if active {
            self.editor_epoch.update(|epoch| *epoch += 1);
            self.persist();
        }
    }
    pub fn cancel(self) {
        let active = self.editor.with_untracked(EditorState::gesture_active);
        if self.editor.try_update(EditorState::cancel_gesture) == Some(true) {
            self.changed();
        }
        if active {
            self.editor_epoch.update(|epoch| *epoch += 1);
            self.persist();
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
        self.stop_pending();
        self.editor_epoch.update(|epoch| *epoch += 1);
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
                self.editor_epoch.update(|epoch| *epoch += 1);
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
            self.editor_epoch.update(|epoch| *epoch += 1);
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
    pub fn configure(self, patch: rustify_ui::ThemeConfigPatch) {
        self.command(|editor, context| editor.configure(patch, context), true);
    }
    pub fn clear_group(self, group: rustify_ui::ThemeConfigGroup) {
        self.command(|editor, context| editor.clear_group(group, context), true);
    }
    pub fn clear_override(self, token: &str) {
        self.command(
            |editor, context| editor.clear_override(token, context),
            true,
        );
    }
    fn snapshot(self) -> Option<Snapshot> {
        let epoch = self.editor_epoch.get_untracked();
        let mut preferences = self
            .envelope
            .with_untracked(|envelope| envelope.preferences.clone());
        let context = self.context();
        let snapshot = self
            .editor
            .try_update(|editor| -> Result<Snapshot, ThemeError> {
                preferences.mode = editor.mode;
                Ok(Snapshot {
                    epoch,
                    revision: editor.revision,
                    draft: editor.document.clone(),
                    preferences,
                    proof: editor.resolved_pair(&context)?,
                    context: crate::state::ContextKey::new(editor.revision, &context),
                })
            });
        match snapshot {
            Some(Ok(snapshot)) => Some(snapshot),
            Some(Err(error)) => {
                self.error.set(error.to_string());
                None
            }
            None => None,
        }
    }
    pub fn permission(self) -> Permission {
        self.storage_epoch.get();
        self.writer.with_value(|writer| writer.permission)
    }
    pub fn pending_count(self) -> usize {
        self.storage_epoch.get();
        self.writer.with_value(Writer::pending)
    }
    pub fn refresh_storage(self) {
        self.storage_epoch.update(|epoch| *epoch += 1);
        let status = if !storage::writable() {
            self.tr(
                "Local saving is unavailable in this browser. You can edit in memory and download or export your theme.",
                "此浏览器无法保存到本机。可以在内存中编辑，并下载或导出主题。",
            )
            .to_owned()
        } else {
            self.writer.with_value(|writer| {
                match writer.permission {
                    Permission::UpgradePending => return self.tr("Legacy draft restored in memory. Review the complete library before upgrading.", "旧草稿已恢复到内存，请先检查完整库再升级。").into(),
                    Permission::UpgradeCancelled => return self.tr("Upgrade cancelled. Memory-only editing; reopening restores the original legacy draft.", "升级已取消。当前仅内存编辑；重开会恢复原旧版草稿。").into(),
                    Permission::Upgrading => return self.tr("Upgrade is waiting for its storage commit. Editing and export remain available.", "升级正在等待存储提交，仍可编辑与导出。").into(),
                    Permission::Protected => return self.tr("Stored data is protected. Download the original data before an explicit reset.", "存储数据已受保护；显式重置前可先下载原文。").into(),
                    Permission::Clearing => return self.tr("Cache removal is pending. Editing stays in memory; library actions are rejected until removal finishes or is cancelled.", "缓存清除待提交。编辑仅保留在内存；完成或取消清除前不接受库写入。").into(),
                    Permission::Cleared => return self.tr("Local v2 cache cleared. Current draft is in memory; explicitly save to resume.", "v2 缓存已清除。当前草稿仅在内存；显式保存后恢复持久化。").into(),
                    Permission::Ready => {}
                }
                if writer.paused {
                    return if writer.pending()==0 {self.tr("Saving paused while another tab's version awaits your decision. No pending writes.","保存已暂停，请决定如何处理另一标签页的版本。没有待写操作。").into()}
                    else {format!("{} {}",writer.pending(),self.tr("pending actions retained. Resolve the conflict or retry/cancel these writes.","项待处理操作已保留，请解决冲突或重试/取消这些写入。"))};
                }
                if writer.pending() > 0 { return format!("{} {}", self.tr("Ordered saves waiting for storage:", "按顺序等待存储的操作："), writer.pending()); }
                let mut preferences = self.envelope.with_untracked(|envelope| envelope.preferences.clone());
                preferences.mode = self.editor.with_untracked(|editor| editor.mode);
                let current = writer.acknowledged.as_ref().is_some_and(|(epoch, draft, saved_preferences)| *epoch == self.editor_epoch.get_untracked() && self.editor.with_untracked(|editor| &editor.document == draft) && &preferences == saved_preferences);
                if current { self.tr("Saved on this device", "已保存到此设备").into() }
                else if writer.acknowledged.is_some() { self.tr("A previous version was saved; current changes are pending.", "先前版本已保存，当前改动待保存。").into() }
                else { self.tr("Draft remains in memory; no current save has been confirmed.", "草稿保留在内存，尚未确认当前保存。").into() }
            })
        };
        self.storage_status.set(status);
    }
    pub fn persist(self) -> bool {
        if self.editor.with_untracked(EditorState::gesture_active) {
            return false;
        }
        if !storage::writable()
            || self
                .writer
                .with_value(|writer| writer.permission != Permission::Ready)
        {
            self.refresh_storage();
            return false;
        }
        self.enqueue(Intent::Autosave)
    }
    fn enqueue(self, intent: Intent) -> bool {
        if !storage::writable() {
            if let Some(snapshot) = self.snapshot() {
                self.writer
                    .update_value(|writer| writer.reject(snapshot, intent));
            }
            self.error.set(self.tr(
                "Local writing is unavailable. This action was not queued; memory editing and downloads remain available.",
                "本地写入不可用，本次操作未入队；仍可在内存编辑与下载。",
            ).into());
            self.refresh_storage();
            return false;
        }
        if self.editor.with_untracked(EditorState::gesture_active) {
            self.error.set(
                self.tr(
                    "Commit or cancel the current gesture before saving.",
                    "保存前请先提交或取消当前手势。",
                )
                .into(),
            );
            return false;
        }
        let Some(snapshot) = self.snapshot() else {
            return false;
        };
        match self
            .writer
            .try_update_value(|writer| writer.enqueue(snapshot, intent))
        {
            Some(Ok(_)) => {
                self.refresh_storage();
                self.pump();
                true
            }
            Some(Err(error)) => {
                if error.field != "storage" {
                    self.error.set(error.to_string());
                } else {
                    self.error.set(String::new());
                }
                self.refresh_storage();
                false
            }
            None => false,
        }
    }
    fn pump(self) {
        if rustify_ui::instance_failed() {
            return;
        }
        let Some(operation) = self.writer.try_update_value(Writer::take).flatten() else {
            return;
        };
        let Some(prepared) = self.writer.with_value(|writer| {
            self.autosave.try_update_value(|autosave| {
                writer.prepare(operation.clone(), autosave, &self.context())
            })
        }) else {
            return;
        };
        let prepared = match prepared {
            Ok(prepared) => prepared,
            Err(error) => {
                self.writer.update_value(|writer| writer.fail(operation.id));
                self.error.set(error.to_string());
                self.refresh_storage();
                return;
            }
        };
        let wire = match serde_json::to_string(&prepared.wire) {
            Ok(wire) => wire,
            Err(error) => {
                self.writer.update_value(|writer| writer.fail(operation.id));
                self.error.set(error.to_string());
                self.refresh_storage();
                return;
            }
        };
        let guard = self.lifetime.with_value(|lifetime| lifetime.guard.clone());
        leptos::task::spawn_local(async move {
            if rustify_ui::instance_failed() || !storage::alive(&guard) {
                return;
            }
            let result = leptos::__reexports::wasm_bindgen_futures::JsFuture::from(
                storage::commit(&wire, &guard),
            )
            .await;
            // These checks precede every access to arena handles. The native
            // abort hook also guards a granted lock without calling the wasm.
            if rustify_ui::instance_failed() || !storage::alive(&guard) {
                return;
            }
            self.finish_write(prepared, result);
        });
    }
    fn finish_write(
        self,
        prepared: Prepared,
        result: Result<leptos::wasm_bindgen::JsValue, leptos::wasm_bindgen::JsValue>,
    ) {
        let id = prepared.operation.id;
        if !self.writer.with_value(|writer| {
            writer.generation == prepared.operation.generation && writer.active_id() == Some(id)
        }) {
            return;
        }
        let answer = result.map_err(storage::message).and_then(|value| {
            serde_json::from_str::<storage::Answer>(&value.as_string().unwrap_or_default())
                .map_err(|error| error.to_string())
        });
        let answer = match answer {
            Ok(answer) => answer,
            Err(error) => {
                self.writer.update_value(|writer| writer.fail(id));
                self.storage_issue.set(error);
                self.error.set(String::new());
                self.refresh_storage();
                return;
            }
        };
        if answer.kind == "conflict" {
            self.writer.update_value(|writer| writer.fail(id));
            self.external.set(Some(answer.raw));
            self.refresh_storage();
            return;
        }
        if matches!(answer.kind.as_str(), "legacy-changed" | "backup-conflict") {
            self.writer.update_value(|writer| {
                writer.fail(id);
                if answer.kind == "legacy-changed" {
                    writer.permission = Permission::UpgradePending;
                }
            });
            if answer.kind == "legacy-changed" {
                self.legacy_changed.set(true);
            }
            self.upgrade_review.set(None);
            self.backup_review.set(None);
            self.error.set(
                self.tr(
                    "The reviewed source changed. Review it again before retrying.",
                    "已检查的原文发生变化，请重新检查后重试。",
                )
                .into(),
            );
            self.refresh_storage();
            return;
        }
        if !matches!(
            answer.kind.as_str(),
            "written" | "loaded" | "cleared" | "cleared-backups"
        ) {
            self.writer.update_value(|writer| writer.fail(id));
            self.error.set("Unexpected storage result".into());
            self.refresh_storage();
            return;
        }
        if self
            .writer
            .try_update_value(|writer| writer.acknowledge(&prepared))
            != Some(true)
        {
            return;
        }
        let same_epoch = self.editor_epoch.get_untracked() == prepared.operation.snapshot.epoch
            && !self.editor.with_untracked(EditorState::gesture_active);
        if prepared.wire.kind != "clear-backups" {
            if let Some(envelope) = &prepared.envelope {
                self.envelope.update(|current| {
                    current.saved_themes = envelope.saved_themes.clone();
                    current.revision = envelope.revision;
                    current.migrated_from_v1 = envelope.migrated_from_v1.clone();
                    current.draft = envelope.draft.clone();
                    if same_epoch {
                        current.preferences = envelope.preferences.clone();
                    }
                });
            }
        }
        if prepared.wire.kind == "load" {
            if let Some(envelope) = &prepared.envelope {
                if same_epoch {
                    let checkpoint = checkpoint_for(envelope);
                    let context = self.context();
                    self.command(
                        |editor, context| editor.apply_document(envelope.draft.clone(), context),
                        false,
                    );
                    self.editor.update(|editor| {
                        editor.mode = envelope.preferences.mode;
                        let _ = editor.set_checkpoint(checkpoint, &context);
                    });
                    self.changed();
                }
            } else {
                self.envelope
                    .update(|envelope| envelope.saved_themes.clear());
                self.writer.update_value(|writer| {
                    writer.confirmed.saved_themes.clear();
                    writer.permission = Permission::Cleared;
                });
            }
        } else if self.writer.with_value(|writer| {
            writer.identity_is_current(
                &prepared,
                self.editor_epoch.get_untracked(),
                self.editor.with_untracked(EditorState::gesture_active),
            )
        }) {
            if let Some(identity) = &prepared.identity {
                if self.command(
                    |editor, context| editor.apply_document(identity.clone(), context),
                    false,
                ) {
                    self.editor.update(EditorState::checkpoint);
                    self.editor_epoch.update(|epoch| *epoch += 1);
                }
            }
        }
        if prepared.wire.kind == "clear" {
            self.corrupt.set(None);
            self.external.set(None);
            self.upgrade_review.set(None);
            self.envelope.update(|envelope| {
                envelope.saved_themes.clear();
                envelope.migrated_from_v1 = None;
                envelope.revision = 0;
            });
        }
        if matches!(prepared.operation.intent, Intent::Upgrade { .. }) {
            self.upgrade_review.set(None);
        }
        let decision = matches!(
            prepared.operation.intent,
            Intent::Load { .. } | Intent::Keep { .. } | Intent::ConflictCopy { .. }
        );
        if decision {
            if self.external.get_untracked() == Some(prepared.wire.expected.clone()) {
                self.external.set(None);
            }
            if self.writer.with_value(Writer::pending) > 0 {
                self.writer.update_value(|writer| writer.paused = true);
            }
        }
        if prepared.wire.kind == "clear-backups" {
            self.backup_review.set(None);
            if matches!(
                prepared.operation.intent,
                Intent::ClearBackups {
                    pause_after: true,
                    ..
                }
            ) && self.writer.with_value(Writer::pending) > 0
            {
                self.writer.update_value(|writer| writer.paused = true);
            }
        }
        self.error.set(String::new());
        self.storage_issue.set(String::new());
        self.refresh_storage();
        if self.external.get_untracked().is_some() || self.writer.with_value(|writer| writer.paused)
        {
            self.writer.update_value(|writer| writer.paused = true);
            return;
        }
        if self.writer.with_value(Writer::pending) > 0 {
            self.pump();
        } else if prepared.wire.kind != "clear"
            && self
                .writer
                .with_value(|writer| writer.permission == Permission::Ready)
            && !self.editor.with_untracked(EditorState::gesture_active)
        {
            if let Some(snapshot) = self.snapshot() {
                if !self
                    .writer
                    .with_value(|writer| writer.is_current(&snapshot))
                {
                    self.enqueue(Intent::Autosave);
                }
            }
        }
    }
    fn stop_pending(self) {
        self.lifetime.set_value(storage::Lifetime::new());
        self.writer.update_value(Writer::cancel_pending);
        self.storage_epoch.update(|epoch| *epoch += 1);
    }
    fn pause_active(self) {
        self.lifetime.set_value(storage::Lifetime::new());
        self.writer.update_value(|writer| {
            if let Some(id) = writer.active_id() {
                writer.fail(id);
            }
            writer.paused = true;
        });
    }
    fn pause_for_conflict(self, raw: Option<String>) {
        self.pause_active();
        self.external.set(Some(raw));
        self.refresh_storage();
    }
    fn resolve_conflict(self, intent: Intent, discard_pending: bool) {
        if !storage::writable() {
            self.refresh_storage();
            return;
        }
        self.pause_active();
        if discard_pending {
            self.stop_pending();
        }
        let Some(snapshot) = self.snapshot() else {
            return;
        };
        match self
            .writer
            .try_update_value(|writer| writer.decide(snapshot, intent))
        {
            Some(Ok(_)) => {
                self.refresh_storage();
                self.pump();
            }
            Some(Err(error)) => self.error.set(error.to_string()),
            None => {}
        }
    }
    pub fn clear_cache(self) {
        let Some(expected) = self.clear_review.get_untracked() else {
            self.error.set(
                self.tr(
                    "Review the cache removal again.",
                    "请重新检查要清除的缓存。",
                )
                .into(),
            );
            return;
        };
        self.resolve_conflict(Intent::Clear { expected }, true);
    }
    pub fn review_clear_cache(self) {
        match storage::read(persistence::STORAGE_KEY) {
            Ok(raw) => self.clear_review.set(Some(raw)),
            Err(error) => self.error.set(error),
        }
    }
    pub fn load_external(self) {
        if let Some(raw) = self.conflict_review.get_untracked() {
            self.resolve_conflict(Intent::Load { expected: raw }, true);
        }
    }
    pub fn keep_current(self) {
        if let Some(raw) = self.conflict_review.get_untracked() {
            self.resolve_conflict(Intent::Keep { expected: raw }, false);
        }
    }
    pub fn conflict_copy(self, name: String) {
        if let Some(raw) = self.conflict_review.get_untracked() {
            self.resolve_conflict(
                Intent::ConflictCopy {
                    expected: raw,
                    id: self.new_id(),
                    name,
                },
                false,
            );
        }
    }
    pub fn review_conflict(self) {
        self.conflict_review.set(self.external.get_untracked());
    }
    pub fn review_external(self) {
        match storage::read(persistence::STORAGE_KEY) {
            Ok(raw) => {
                self.external.set(Some(raw.clone()));
                self.conflict_review.set(Some(raw));
            }
            Err(error) => self.error.set(error),
        }
    }
    pub fn retry_storage(self) {
        if self.external.get_untracked().is_some() {
            self.refresh_storage();
            return;
        }
        if matches!(
            self.permission(),
            Permission::UpgradePending | Permission::UpgradeCancelled | Permission::Upgrading
        ) {
            self.review_upgrade();
            return;
        }
        if self.writer.with_value(|writer| {
            writer
                .first()
                .is_some_and(|operation| matches!(operation.intent, Intent::ClearBackups { .. }))
        }) {
            self.review_backups();
            return;
        }
        self.error.set(String::new());
        self.storage_issue.set(String::new());
        self.writer.update_value(|writer| {
            writer.paused = false;
            if writer.permission == Permission::Cleared {
                writer.permission = Permission::Ready;
            }
        });
        if self.writer.with_value(Writer::pending) > 0 {
            self.pump();
        } else {
            self.persist();
        }
        self.refresh_storage();
    }
    pub fn cancel_storage(self) {
        let upgrade = matches!(
            self.permission(),
            Permission::UpgradePending | Permission::UpgradeCancelled | Permission::Upgrading
        );
        self.stop_pending();
        if upgrade {
            self.writer
                .update_value(|writer| writer.permission = Permission::UpgradeCancelled);
        }
        self.upgrade_review.set(None);
        self.error.set(String::new());
        self.storage_issue.set(String::new());
        self.refresh_storage();
    }
    pub fn review_upgrade(self) {
        if self.editor.with_untracked(EditorState::gesture_active) {
            self.error.set(
                self.tr(
                    "Commit or cancel the current gesture before review.",
                    "检查前请先提交或取消当前手势。",
                )
                .into(),
            );
            return;
        }
        if let Ok(Some(raw)) = storage::read(persistence::STORAGE_KEY) {
            self.pause_for_conflict(Some(raw));
            return;
        }
        let Some(snapshot) = self.snapshot() else {
            return;
        };
        let review = storage::read(persistence::LEGACY_KEY)
            .map_err(|message| ThemeError {
                field: "storage".into(),
                message,
            })
            .and_then(|raw| {
                raw.ok_or_else(|| ThemeError {
                    field: "legacy".into(),
                    message: "Legacy source was removed; review storage again".into(),
                })
            })
            .and_then(|raw| {
                let mut candidate = persistence::migrate_legacy(&raw, &self.context())?;
                candidate.draft = snapshot.draft.clone();
                candidate.preferences = snapshot.preferences.clone();
                persistence::validate(&candidate, &self.context())?;
                Ok(UpgradeReview {
                    raw,
                    snapshot,
                    candidate,
                })
            });
        match review {
            Ok(review) => {
                self.stop_pending();
                self.writer
                    .update_value(|writer| writer.permission = Permission::UpgradePending);
                self.upgrade_review.set(Some(review));
                self.error.set(String::new());
            }
            Err(error) => self.error.set(error.to_string()),
        }
        self.refresh_storage();
    }
    pub fn confirm_upgrade(self) {
        let Some(review) = self.upgrade_review.get_untracked() else {
            return;
        };
        if self.editor_epoch.get_untracked() != review.snapshot.epoch {
            self.review_upgrade();
            self.error.set(self.tr("The draft changed. The review was refreshed; confirm the new snapshot explicitly.", "草稿已变化，检查内容已刷新；请再次明确确认新快照。").into());
            return;
        }
        self.stop_pending();
        self.writer.update_value(|writer| {
            writer.permission = Permission::Upgrading;
            let _ = writer.enqueue(review.snapshot, Intent::Upgrade { legacy: review.raw });
        });
        self.refresh_storage();
        self.pump();
    }
    pub fn review_backups(self) {
        match storage::backups()
            .map_err(storage::message)
            .and_then(|raw| serde_json::from_str(&raw).map_err(|error| error.to_string()))
        {
            Ok(values) => self.backup_review.set(Some(values)),
            Err(error) => self.error.set(error),
        }
    }
    pub fn clear_backups(self) {
        let Some(expected) = self.backup_review.get_untracked() else {
            return;
        };
        let pause_after = self.writer.with_value(|writer| writer.paused);
        let intent = Intent::ClearBackups {
            expected,
            pause_after,
        };
        if pause_after {
            self.resolve_conflict(intent, false);
        } else {
            self.enqueue(intent);
        }
    }
    pub fn download_backup(self, key: &str) {
        let expected = key.strip_prefix(persistence::BACKUP_PREFIX);
        let result = storage::read(key)
            .and_then(|raw| raw.ok_or_else(|| "Backup is missing".into()))
            .and_then(|raw| {
                if expected == Some(persistence::digest(&raw).as_str()) {
                    Ok(raw)
                } else {
                    Err("Backup digest does not match".into())
                }
            });
        match result {
            Ok(raw) => {
                if let Err(error) = rustify_ui::files::export(
                    "theme-studio-original.json",
                    raw.as_bytes(),
                    "application/json",
                ) {
                    self.error.set(format!("{error:?}"));
                }
            }
            Err(error) => self.error.set(error),
        }
    }
    pub fn download_changed_legacy(self) {
        match storage::read(persistence::LEGACY_KEY) {
            Ok(Some(raw)) => {
                if let Err(error) = rustify_ui::files::export(
                    "theme-studio-current-v1.json",
                    raw.as_bytes(),
                    "application/json",
                ) {
                    self.error.set(format!("{error:?}"));
                }
            }
            Ok(None) => self.error.set(
                self.tr("Legacy source is missing.", "旧版原文已不存在。")
                    .into(),
            ),
            Err(error) => self.error.set(error),
        }
    }
    pub fn download_legacy(self) {
        let source = self
            .envelope
            .with_untracked(|envelope| envelope.migrated_from_v1.clone());
        let raw = if let Some(source) =
            source.filter(|_| self.writer.with_value(|writer| writer.baseline.is_some()))
        {
            storage::read(&source.backup_key)
                .and_then(|raw| raw.ok_or_else(|| "Original backup is missing".into()))
                .and_then(|raw| {
                    if persistence::digest(&raw) == source.sha256 {
                        Ok(raw)
                    } else {
                        Err(
                            "Backup digest does not match; original data was not substituted"
                                .into(),
                        )
                    }
                })
        } else {
            self.legacy_raw
                .get_untracked()
                .ok_or_else(|| "Original legacy data is unavailable".into())
        };
        match raw {
            Ok(raw) => {
                if let Err(error) = rustify_ui::files::export(
                    "theme-studio-before-upgrade.json",
                    raw.as_bytes(),
                    "application/json",
                ) {
                    self.error.set(format!("{error:?}"));
                }
            }
            Err(error) => self.error.set(format!(
                "{} {error}",
                self.tr("Original download failed:", "原文下载失败：")
            )),
        }
    }
    pub fn load_local(self, id: String) {
        if let Some(document) = self.envelope.with_untracked(|envelope| {
            envelope
                .saved_themes
                .iter()
                .find(|theme| theme.id == id)
                .cloned()
        }) {
            self.apply(document, true);
        }
    }
    pub fn new_id(self) -> String {
        format!(
            "local-{}-{:x}",
            js_sys::Date::now() as u64,
            (js_sys::Math::random() * u64::MAX as f64) as u64
        )
    }
    pub fn save_named(self, name: String, overwrite: Option<String>) {
        let id = overwrite.clone().unwrap_or_else(|| self.new_id());
        self.enqueue(Intent::Save {
            id,
            name,
            overwrite,
        });
    }
    pub fn copy_named(self, id: String, name: String) {
        let source = self.envelope.with_untracked(|envelope| {
            envelope
                .saved_themes
                .iter()
                .find(|theme| theme.id == id)
                .cloned()
        });
        if let Some(source) = source {
            self.enqueue(Intent::Copy {
                source,
                id: self.new_id(),
                name,
            });
        }
    }
    pub fn rename_named(self, id: String, name: String) {
        self.enqueue(Intent::Rename { id, name });
    }
    pub fn delete_named(self, id: String) {
        self.enqueue(Intent::Delete { id });
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
            let color = rustify_ui::theme::COLOR_TOKENS.contains(&token.as_str());
            let id = if color {
                "token-source".into()
            } else {
                format!("edit-{token}")
            };
            if let Ok(Some(element)) = document().query_selector(&format!("[data-testid='{id}']")) {
                let source = color.then(|| element.clone());
                element.scroll_into_view_with_bool(false);
                let focus = if color {
                    document()
                        .query_selector("[data-testid='color-value']")
                        .ok()
                        .flatten()
                } else {
                    Some(element)
                };
                if let Some(el) = focus
                    .as_ref()
                    .and_then(|element| element.dyn_ref::<leptos::web_sys::HtmlElement>())
                {
                    let _ = el.focus();
                }
                if let Some(source) = source {
                    source.scroll_into_view();
                }
            }
        });
    }
}
