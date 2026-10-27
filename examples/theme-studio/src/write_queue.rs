//! One ordered writer. Operations capture editor facts, never an editor owner.

use crate::persistence::{self, Autosave, Envelope, Preferences};
use crate::state::ContextKey;
use rustify_ui::{ResolveContext, ResolvedTheme, ThemeDocument, ThemeError};
use serde::Serialize;
use std::{
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub epoch: u64,
    pub revision: u64,
    pub draft: ThemeDocument,
    pub preferences: Preferences,
    pub proof: [Arc<ResolvedTheme>; 2],
    pub context: ContextKey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Permission {
    Ready,
    UpgradePending,
    UpgradeCancelled,
    Upgrading,
    Protected,
    Clearing,
    Cleared,
}

#[derive(Clone, Debug)]
pub enum Intent {
    Autosave,
    Save {
        id: String,
        name: String,
        overwrite: Option<String>,
    },
    Copy {
        source: ThemeDocument,
        id: String,
        name: String,
    },
    Rename {
        id: String,
        name: String,
    },
    Delete {
        id: String,
    },
    Upgrade {
        legacy: String,
    },
    Keep {
        expected: Option<String>,
    },
    ConflictCopy {
        expected: Option<String>,
        id: String,
        name: String,
    },
    Load {
        expected: Option<String>,
    },
    Clear {
        expected: Option<String>,
    },
    ClearBackups {
        expected: BTreeMap<String, String>,
        pause_after: bool,
    },
}

impl Intent {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Autosave => "autosave",
            Self::Save { .. } => "save",
            Self::Copy { .. } => "copy",
            Self::Rename { .. } => "rename",
            Self::Delete { .. } => "delete",
            Self::Upgrade { .. } => "upgrade",
            Self::Keep { .. } => "keep",
            Self::ConflictCopy { .. } => "conflict-copy",
            Self::Load { .. } => "load",
            Self::Clear { .. } => "clear",
            Self::ClearBackups { .. } => "clear-backups",
        }
    }
    pub fn changes_identity(&self) -> bool {
        matches!(self, Self::Save { .. } | Self::ConflictCopy { .. })
    }
}

#[derive(Clone, Debug)]
pub struct Operation {
    pub id: u64,
    pub generation: u64,
    pub snapshot: Snapshot,
    pub intent: Intent,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Outcome {
    pub id: u64,
    pub kind: &'static str,
    pub epoch: u64,
    pub revision: u64,
    pub state: &'static str,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Backup {
    pub key: String,
    pub raw: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Wire {
    pub lock_name: &'static str,
    pub op_id: u64,
    pub kind: &'static str,
    pub expected: Option<String>,
    pub write: Option<String>,
    pub legacy: Option<String>,
    pub backup: Option<Backup>,
    pub backups: Option<BTreeMap<String, String>>,
}

#[derive(Clone, Debug)]
pub struct Prepared {
    pub operation: Operation,
    pub envelope: Option<Envelope>,
    pub identity: Option<ThemeDocument>,
    pub wire: Wire,
}

pub struct Writer {
    pub confirmed: Envelope,
    pub baseline: Option<String>,
    pub permission: Permission,
    pub paused: bool,
    pub generation: u64,
    pub latest_named: u64,
    pub acknowledged: Option<(u64, ThemeDocument, Preferences)>,
    pub outcomes: Vec<Outcome>,
    next_id: u64,
    queue: VecDeque<Operation>,
    active: Option<Operation>,
    permission_before_clear: Option<Permission>,
}

impl Writer {
    pub fn new(confirmed: Envelope, baseline: Option<String>, permission: Permission) -> Self {
        let acknowledged = (baseline.is_some() && permission == Permission::Ready)
            .then(|| (0, confirmed.draft.clone(), confirmed.preferences.clone()));
        Self {
            confirmed,
            baseline,
            permission,
            paused: false,
            generation: 0,
            latest_named: 0,
            acknowledged,
            outcomes: Vec::new(),
            next_id: 0,
            queue: VecDeque::new(),
            active: None,
            permission_before_clear: None,
        }
    }
    pub fn pending(&self) -> usize {
        self.queue.len() + usize::from(self.active.is_some())
    }
    pub fn active(&self) -> bool {
        self.active.is_some()
    }
    pub fn active_id(&self) -> Option<u64> {
        self.active.as_ref().map(|operation| operation.id)
    }
    pub fn first(&self) -> Option<&Operation> {
        self.queue.front()
    }
    pub fn reject(&mut self, snapshot: Snapshot, intent: Intent) {
        self.next_id += 1;
        if intent.changes_identity() {
            self.latest_named = self.next_id;
        }
        self.record(
            &Operation {
                id: self.next_id,
                generation: self.generation,
                snapshot,
                intent,
            },
            "rejected",
        );
    }
    pub fn enqueue(&mut self, snapshot: Snapshot, intent: Intent) -> Result<u64, ThemeError> {
        let exceptional = matches!(
            intent,
            Intent::Upgrade { .. }
                | Intent::Load { .. }
                | Intent::Keep { .. }
                | Intent::ConflictCopy { .. }
                | Intent::Clear { .. }
                | Intent::ClearBackups { .. }
        );
        if self.permission != Permission::Ready && !exceptional {
            self.reject(snapshot, intent);
            return Err(ThemeError { field: "storage".into(), message: if self.permission == Permission::Clearing {"cache removal is pending; this library action was not queued, and editing remains in memory"} else {"persistent writes are protected; review the migration or explicitly resume saving"}.into() });
        }
        self.next_id += 1;
        let id = self.next_id;
        if matches!(intent, Intent::Autosave)
            && self
                .queue
                .back()
                .is_some_and(|last| matches!(last.intent, Intent::Autosave))
        {
            if let Some(old) = self.queue.pop_back() {
                self.record(&old, "superseded");
            }
        }
        if intent.changes_identity() {
            self.latest_named = id;
        }
        if matches!(intent, Intent::Clear { .. }) && self.permission != Permission::Clearing {
            self.permission_before_clear = Some(if self.permission == Permission::Upgrading {
                Permission::UpgradePending
            } else {
                self.permission
            });
            self.permission = Permission::Clearing;
        }
        let operation = Operation {
            id,
            generation: self.generation,
            snapshot,
            intent,
        };
        self.record(&operation, "pending");
        self.queue.push_back(operation);
        Ok(id)
    }
    pub fn take(&mut self) -> Option<Operation> {
        if self.paused || self.active.is_some() {
            return None;
        }
        let operation = self.queue.pop_front()?;
        self.active = Some(operation.clone());
        Some(operation)
    }
    pub fn decide(&mut self, snapshot: Snapshot, intent: Intent) -> Result<u64, ThemeError> {
        let mut retained = VecDeque::new();
        while let Some(old) = self.queue.pop_front() {
            if matches!(
                old.intent,
                Intent::Keep { .. }
                    | Intent::ConflictCopy { .. }
                    | Intent::Load { .. }
                    | Intent::Clear { .. }
                    | Intent::ClearBackups { .. }
            ) {
                if matches!(old.intent, Intent::Clear { .. }) {
                    self.restore_clear_permission();
                }
                self.record(&old, "cancelled");
            } else {
                retained.push_back(old);
            }
        }
        self.queue = retained;
        let id = self.enqueue(snapshot, intent)?;
        if let Some(operation) = self.queue.pop_back() {
            self.queue.push_front(operation);
        }
        self.paused = false;
        Ok(id)
    }
    pub fn prepare(
        &self,
        operation: Operation,
        autosave: &mut Autosave,
        context: &ResolveContext<'_>,
    ) -> Result<Prepared, ThemeError> {
        let mut candidate = self.confirmed.clone();
        let mut expected = self.baseline.clone();
        let mut legacy = None;
        let mut backup = None;
        let mut backups = None;
        let mut identity = None;
        let mut kind = "write";
        match &operation.intent {
            Intent::Upgrade { legacy: raw } => {
                candidate = persistence::migrate_legacy(raw, context)?;
                expected = None;
                let source = persistence::MigrationSource::new(raw);
                backup = Some(Backup {
                    key: source.backup_key.clone(),
                    raw: raw.clone(),
                });
                candidate.migrated_from_v1 = Some(source);
                legacy = Some(raw.clone());
            }
            Intent::Keep { expected: raw }
            | Intent::ConflictCopy { expected: raw, .. }
            | Intent::Load { expected: raw } => {
                expected = raw.clone();
                if let Some(raw) = raw {
                    candidate = match persistence::load(Some(raw), context) {
                        persistence::LoadResult::Valid(value) => *value,
                        persistence::LoadResult::Invalid { error, .. } => return Err(error),
                        persistence::LoadResult::Missing => unreachable!(),
                    };
                } else {
                    candidate.saved_themes.clear();
                    candidate.revision = 0;
                    candidate.migrated_from_v1 = None;
                }
                if matches!(operation.intent, Intent::Load { .. }) {
                    kind = "load";
                }
            }
            Intent::Clear { expected: raw } => {
                kind = "clear";
                expected = raw.clone();
            }
            Intent::ClearBackups {
                expected: values, ..
            } => {
                kind = "clear-backups";
                backups = Some(values.clone());
            }
            _ => {}
        }
        if kind == "write" {
            let snapshot = &operation.snapshot;
            candidate.draft = snapshot.draft.clone();
            candidate.preferences = snapshot.preferences.clone();
            match &operation.intent {
                Intent::Save {
                    id,
                    name,
                    overwrite,
                } => {
                    candidate = candidate.save_named(
                        &snapshot.draft,
                        id,
                        name,
                        overwrite.as_deref(),
                        context,
                    )?;
                    identity = candidate
                        .saved_themes
                        .iter()
                        .find(|theme| &theme.id == id)
                        .cloned();
                }
                Intent::ConflictCopy { id, name, .. } => {
                    candidate = candidate.save_named(&snapshot.draft, id, name, None, context)?;
                    identity = candidate
                        .saved_themes
                        .iter()
                        .find(|theme| &theme.id == id)
                        .cloned();
                }
                Intent::Copy { source, id, name } => {
                    candidate = candidate.save_named(source, id, name, None, context)?;
                }
                Intent::Rename { id, name } => {
                    candidate = candidate.rename_theme(id, name, context)?;
                }
                Intent::Delete { id } => {
                    candidate = candidate.delete_theme(id, context)?;
                }
                _ => {}
            }
            if let Some(saved) = &identity {
                candidate.draft = saved.clone();
            }
            candidate.revision = candidate
                .revision
                .checked_add(1)
                .ok_or_else(|| ThemeError {
                    field: "revision".into(),
                    message: "local revision cannot advance further".into(),
                })?;
        }
        let write = if kind == "write" {
            Some(
                if matches!(operation.intent, Intent::Autosave | Intent::Keep { .. })
                    && operation
                        .snapshot
                        .context
                        .matches(operation.snapshot.revision, context)
                {
                    autosave.serialize_snapshot(&candidate, &operation.snapshot.proof, context)?
                } else {
                    persistence::serialize(&candidate, context)?
                },
            )
        } else {
            None
        };
        let envelope =
            if kind == "clear" || kind == "clear-backups" || kind == "load" && expected.is_none() {
                None
            } else {
                Some(candidate)
            };
        let wire = Wire {
            lock_name: persistence::LOCK_NAME,
            op_id: operation.id,
            kind,
            expected,
            write,
            legacy,
            backup,
            backups,
        };
        Ok(Prepared {
            operation,
            envelope,
            identity,
            wire,
        })
    }
    pub fn acknowledge(&mut self, prepared: &Prepared) -> bool {
        if !self
            .active
            .as_ref()
            .is_some_and(|op| op.id == prepared.operation.id && op.generation == self.generation)
        {
            return false;
        }
        let operation = self.active.take().unwrap_or_else(|| unreachable!());
        if prepared.wire.kind != "clear-backups" {
            self.baseline = if prepared.wire.kind == "load" {
                prepared.wire.expected.clone()
            } else {
                prepared.wire.write.clone()
            };
            if let Some(envelope) = &prepared.envelope {
                self.confirmed = envelope.clone();
            }
        }
        if prepared.wire.kind == "clear" {
            self.permission_before_clear = None;
            self.permission = Permission::Cleared;
            self.confirmed.saved_themes.clear();
            self.confirmed.migrated_from_v1 = None;
            self.confirmed.revision = 0;
            self.acknowledged = None;
        } else if prepared.wire.kind != "clear-backups" {
            self.permission = Permission::Ready;
            self.acknowledged = prepared.envelope.as_ref().map(|envelope| {
                (
                    operation.snapshot.epoch,
                    envelope.draft.clone(),
                    envelope.preferences.clone(),
                )
            });
        }
        self.record(&operation, "saved");
        true
    }
    pub fn fail(&mut self, id: u64) {
        if let Some(operation) = self.active.take() {
            if operation.id == id {
                self.record(&operation, "failed");
                self.queue.push_front(operation);
            } else {
                self.active = Some(operation);
            }
        }
        self.paused = true;
    }
    pub fn cancel_pending(&mut self) {
        self.generation += 1;
        self.restore_clear_permission();
        if let Some(operation) = self.active.take() {
            self.record(&operation, "cancelled");
        }
        while let Some(operation) = self.queue.pop_front() {
            self.record(&operation, "cancelled");
        }
        self.paused = false;
    }
    fn restore_clear_permission(&mut self) {
        if let Some(permission) = self.permission_before_clear.take() {
            self.permission = permission;
        }
    }
    pub fn is_current(&self, snapshot: &Snapshot) -> bool {
        self.permission == Permission::Ready
            && !self.paused
            && self.pending() == 0
            && self
                .acknowledged
                .as_ref()
                .is_some_and(|(epoch, draft, preferences)| {
                    *epoch == snapshot.epoch
                        && draft == &snapshot.draft
                        && preferences == &snapshot.preferences
                })
    }
    pub fn identity_is_current(&self, prepared: &Prepared, epoch: u64, gesture: bool) -> bool {
        !gesture
            && epoch == prepared.operation.snapshot.epoch
            && self.latest_named == prepared.operation.id
            && prepared.identity.is_some()
    }
    fn record(&mut self, operation: &Operation, state: &'static str) {
        if let Some(outcome) = self
            .outcomes
            .iter_mut()
            .find(|outcome| outcome.id == operation.id)
        {
            outcome.state = state;
        } else {
            self.outcomes.push(Outcome {
                id: operation.id,
                kind: operation.intent.name(),
                epoch: operation.snapshot.epoch,
                revision: operation.snapshot.revision,
                state,
                detail: match &operation.intent {
                    Intent::Save { name, .. }
                    | Intent::Copy { name, .. }
                    | Intent::Rename { name, .. }
                    | Intent::ConflictCopy { name, .. } => name.clone(),
                    Intent::Delete { id } => self
                        .confirmed
                        .saved_themes
                        .iter()
                        .find(|theme| &theme.id == id)
                        .map(|theme| theme.name.clone())
                        .unwrap_or_else(|| id.clone()),
                    _ => String::new(),
                },
            });
        }
        if self.outcomes.len() > 100 {
            if let Some(index) = self
                .outcomes
                .iter()
                .position(|outcome| !matches!(outcome.state, "pending" | "failed"))
            {
                self.outcomes.remove(index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustify_ui::{theme::font_context, ThemeMode};
    fn setup() -> (Writer, Snapshot) {
        let context = font_context(16., false);
        let document = ThemeDocument::standard("queue", "Queue");
        let mut editor =
            crate::state::EditorState::new(document.clone(), ThemeMode::Light, &context).unwrap();
        let snapshot = Snapshot {
            epoch: 1,
            revision: 0,
            draft: document.clone(),
            preferences: Preferences::default(),
            proof: editor.resolved_pair(&context).unwrap(),
            context: ContextKey::new(editor.revision, &context),
        };
        (
            Writer::new(
                Envelope {
                    envelope_version: 2,
                    revision: 0,
                    migrated_from_v1: None,
                    draft: document,
                    preferences: Preferences::default(),
                    saved_themes: Vec::new(),
                },
                None,
                Permission::Ready,
            ),
            snapshot,
        )
    }
    #[test]
    fn queued_autosave_revalidates_a_changed_context_and_retains_the_failed_request() {
        let (mut writer, mut snapshot) = setup();
        let context = font_context(16., false);
        let mut autosave = Autosave::default();
        writer.enqueue(snapshot.clone(), Intent::Autosave).unwrap();
        let first = writer.take().unwrap();
        let first = writer.prepare(first, &mut autosave, &context).unwrap();

        let mut editor =
            crate::state::EditorState::new(snapshot.draft.clone(), ThemeMode::Light, &context)
                .unwrap();
        editor.edit("radius", "200rem", &context).unwrap();
        snapshot.epoch += 1;
        snapshot.revision = editor.revision;
        snapshot.draft = editor.document.clone();
        snapshot.proof = editor.resolved_pair(&context).unwrap();
        snapshot.context = ContextKey::new(editor.revision, &context);
        let id = writer.enqueue(snapshot.clone(), Intent::Autosave).unwrap();
        writer.acknowledge(&first);
        let baseline = writer.baseline.clone();

        let operation = writer.take().unwrap();
        let error = writer
            .prepare(operation, &mut autosave, &font_context(32., false))
            .expect_err("a stale proof must not authorize an invalid draft");
        assert!(error.field.contains("radius"), "{error}");
        writer.fail(id);
        assert_eq!(writer.baseline, baseline);
        assert_eq!(writer.first().unwrap().snapshot.draft, snapshot.draft);
        assert!(writer.take().is_none());

        writer.paused = false;
        let operation = writer.take().unwrap();
        let prepared = writer.prepare(operation, &mut autosave, &context).unwrap();
        assert!(writer.acknowledge(&prepared));
        assert!(writer.is_current(&snapshot));
        assert_eq!(
            persistence::load(writer.baseline.as_deref(), &context),
            persistence::LoadResult::Valid(Box::new(writer.confirmed.clone()))
        );
    }

    #[test]
    fn queued_autosave_revalidates_every_resolve_context_input() {
        let fonts = [rustify_ui::theme::FontFace {
            family: "Additional font",
            resource_path: "fonts/additional.ttf",
        }];
        let original = font_context(16., false);
        for context in [
            ResolveContext {
                rem_px: 20.,
                ..original.clone()
            },
            ResolveContext {
                reduce_motion: true,
                ..original.clone()
            },
            ResolveContext {
                fonts: &fonts,
                ..original.clone()
            },
            ResolveContext {
                fallback_fonts: ["Other sans", "serif", "monospace"],
                ..original.clone()
            },
            ResolveContext {
                font_resolution: rustify_ui::FontResolution::Css,
                ..original.clone()
            },
        ] {
            let (mut writer, snapshot) = setup();
            writer.enqueue(snapshot, Intent::Autosave).unwrap();
            let operation = writer.take().unwrap();
            let before = rustify_ui::theme::resolution_count();
            let prepared = writer
                .prepare(operation, &mut Autosave::default(), &context)
                .unwrap();
            assert!(
                rustify_ui::theme::resolution_count() - before >= 2,
                "{context:?}"
            );
            assert!(matches!(
                persistence::load(prepared.wire.write.as_deref(), &context),
                persistence::LoadResult::Valid(_)
            ));
        }
    }

    #[test]
    fn queued_autosave_reuses_the_proof_and_warm_library_when_context_is_unchanged() {
        let (mut writer, snapshot) = setup();
        let context = font_context(16., false);
        for index in 0..persistence::SAVED_THEME_LIMIT {
            writer.confirmed.saved_themes.push(ThemeDocument::standard(
                format!("saved-{index}"),
                format!("Saved {index}"),
            ));
        }
        let mut autosave = Autosave::default();
        writer.enqueue(snapshot.clone(), Intent::Autosave).unwrap();
        let first = writer.take().unwrap();
        let first = writer.prepare(first, &mut autosave, &context).unwrap();
        writer.acknowledge(&first);
        writer.enqueue(snapshot, Intent::Autosave).unwrap();
        let second = writer.take().unwrap();
        let before = rustify_ui::theme::resolution_count();
        let second = writer.prepare(second, &mut autosave, &context).unwrap();
        assert_eq!(rustify_ui::theme::resolution_count() - before, 0);
        assert_eq!(
            second.envelope.unwrap().saved_themes.len(),
            persistence::SAVED_THEME_LIMIT
        );
    }

    #[test]
    fn restored_ready_v2_acknowledges_only_the_initial_draft_and_preferences() {
        let (writer, mut snapshot) = setup();
        snapshot.epoch = 0;
        let confirmed = writer.confirmed;
        let baseline = persistence::serialize(&confirmed, &font_context(16., false)).unwrap();
        let mut restored =
            Writer::new(confirmed.clone(), Some(baseline.clone()), Permission::Ready);
        assert!(restored.is_current(&snapshot));
        restored
            .enqueue(snapshot.clone(), Intent::Autosave)
            .unwrap();
        assert!(!restored.is_current(&snapshot));
        for permission in [
            Permission::Protected,
            Permission::UpgradePending,
            Permission::UpgradeCancelled,
        ] {
            assert!(
                Writer::new(confirmed.clone(), Some(baseline.clone()), permission)
                    .acknowledged
                    .is_none()
            );
        }
        assert!(Writer::new(confirmed, None, Permission::Ready)
            .acknowledged
            .is_none());
    }

    #[test]
    fn only_adjacent_unissued_autosaves_coalesce_and_library_intents_keep_order() {
        let (mut writer, mut snapshot) = setup();
        writer.enqueue(snapshot.clone(), Intent::Autosave).unwrap();
        let active = writer.take().unwrap();
        snapshot.epoch = 2;
        writer.enqueue(snapshot.clone(), Intent::Autosave).unwrap();
        snapshot.epoch = 3;
        writer.enqueue(snapshot.clone(), Intent::Autosave).unwrap();
        writer
            .enqueue(
                snapshot.clone(),
                Intent::Save {
                    id: "named".into(),
                    name: "Named".into(),
                    overwrite: None,
                },
            )
            .unwrap();
        snapshot.epoch = 4;
        writer.enqueue(snapshot.clone(), Intent::Autosave).unwrap();
        assert_eq!(writer.pending(), 4);
        let context = font_context(16., false);
        let mut autosave = Autosave::default();
        let first = writer.prepare(active, &mut autosave, &context).unwrap();
        assert!(writer.acknowledge(&first));
        let second = writer.take().unwrap();
        assert_eq!(second.snapshot.epoch, 3);
        let second = writer.prepare(second, &mut autosave, &context).unwrap();
        assert_eq!(second.wire.expected, first.wire.write);
        writer.acknowledge(&second);
        let named = writer.take().unwrap();
        assert_eq!(named.intent.name(), "save");
        let named = writer.prepare(named, &mut autosave, &context).unwrap();
        writer.acknowledge(&named);
        let last = writer.take().unwrap();
        let last = writer.prepare(last, &mut autosave, &context).unwrap();
        assert_eq!(last.envelope.as_ref().unwrap().saved_themes.len(), 1);
        writer.acknowledge(&last);
        assert_eq!(writer.confirmed.revision, 4);
    }
    #[test]
    fn protected_migrations_do_not_accumulate_autosaves_and_clear_invalidates_old_operations() {
        let (mut writer, snapshot) = setup();
        writer.permission = Permission::UpgradeCancelled;
        assert!(writer.enqueue(snapshot.clone(), Intent::Autosave).is_err());
        assert_eq!(writer.pending(), 0);
        writer.permission = Permission::Ready;
        writer.enqueue(snapshot.clone(), Intent::Autosave).unwrap();
        let operation = writer.take().unwrap();
        let prepared = writer
            .prepare(
                operation,
                &mut Autosave::default(),
                &font_context(16., false),
            )
            .unwrap();
        writer.cancel_pending();
        assert!(!writer.acknowledge(&prepared));
        assert!(writer.baseline.is_none());
    }
    #[test]
    fn failure_retains_the_exact_request_for_explicit_retry_and_old_ack_does_not_save_current_epoch(
    ) {
        let (mut writer, mut snapshot) = setup();
        let id = writer.enqueue(snapshot.clone(), Intent::Autosave).unwrap();
        let operation = writer.take().unwrap();
        writer.fail(id);
        assert!(writer.take().is_none());
        assert_eq!(writer.first().unwrap().id, id);
        writer.paused = false;
        let retried = writer.take().unwrap();
        assert_eq!(retried.snapshot.epoch, operation.snapshot.epoch);
        let prepared = writer
            .prepare(retried, &mut Autosave::default(), &font_context(16., false))
            .unwrap();
        writer.acknowledge(&prepared);
        assert!(writer.is_current(&snapshot));
        snapshot.epoch += 1;
        assert!(!writer.is_current(&snapshot));
    }

    #[test]
    fn named_identity_acknowledges_only_its_epoch_without_a_gesture_or_newer_name() {
        let (mut writer, snapshot) = setup();
        writer
            .enqueue(
                snapshot.clone(),
                Intent::Save {
                    id: "one".into(),
                    name: "One".into(),
                    overwrite: None,
                },
            )
            .unwrap();
        let operation = writer.take().unwrap();
        let prepared = writer
            .prepare(
                operation,
                &mut Autosave::default(),
                &font_context(16., false),
            )
            .unwrap();
        assert!(writer.identity_is_current(&prepared, snapshot.epoch, false));
        assert!(!writer.identity_is_current(&prepared, snapshot.epoch + 1, false));
        assert!(!writer.identity_is_current(&prepared, snapshot.epoch, true));
        writer
            .enqueue(
                snapshot.clone(),
                Intent::Save {
                    id: "two".into(),
                    name: "Two".into(),
                    overwrite: None,
                },
            )
            .unwrap();
        assert!(!writer.identity_is_current(&prepared, snapshot.epoch, false));
        assert!(writer.acknowledge(&prepared));
        assert_eq!(writer.confirmed.saved_themes[0].name, "One");
        assert_eq!(writer.pending(), 1);
    }

    #[test]
    fn rejected_named_requests_are_visible_without_writing_and_block_an_older_identity_ack() {
        let (mut writer, snapshot) = setup();
        writer
            .enqueue(
                snapshot.clone(),
                Intent::Save {
                    id: "old".into(),
                    name: "Old name".into(),
                    overwrite: None,
                },
            )
            .unwrap();
        let operation = writer.take().unwrap();
        let prepared = writer
            .prepare(
                operation,
                &mut Autosave::default(),
                &font_context(16., false),
            )
            .unwrap();
        writer.reject(
            snapshot.clone(),
            Intent::Save {
                id: "new".into(),
                name: "Rejected name".into(),
                overwrite: None,
            },
        );
        assert_eq!(writer.pending(), 1);
        assert_eq!(writer.confirmed.revision, 0);
        assert!(writer.baseline.is_none());
        assert_eq!(writer.outcomes.last().unwrap().state, "rejected");
        assert_eq!(writer.outcomes.last().unwrap().detail, "Rejected name");
        assert!(!writer.identity_is_current(&prepared, snapshot.epoch, false));
        assert!(writer.acknowledge(&prepared));
        assert_eq!(writer.confirmed.saved_themes.len(), 1);
    }

    #[test]
    fn conflict_decisions_bind_reviewed_bytes_and_retain_named_intents_until_reauthorised() {
        let (mut writer, snapshot) = setup();
        let named = writer
            .enqueue(
                snapshot.clone(),
                Intent::Save {
                    id: "pending".into(),
                    name: "Pending".into(),
                    overwrite: None,
                },
            )
            .unwrap();
        writer.paused = true;
        let mut external = writer.confirmed.clone();
        external.revision = 7;
        external = external
            .save_named(
                &snapshot.draft,
                "external",
                "External",
                None,
                &font_context(16., false),
            )
            .unwrap();
        let raw = persistence::serialize(&external, &font_context(16., false)).unwrap();
        writer
            .decide(
                snapshot.clone(),
                Intent::Keep {
                    expected: Some(raw.clone()),
                },
            )
            .unwrap();
        let decision = writer.take().unwrap();
        let prepared = writer
            .prepare(
                decision,
                &mut Autosave::default(),
                &font_context(16., false),
            )
            .unwrap();
        assert_eq!(prepared.wire.expected.as_deref(), Some(raw.as_str()));
        assert_eq!(prepared.envelope.as_ref().unwrap().revision, 8);
        assert_eq!(
            prepared.envelope.as_ref().unwrap().saved_themes[0].id,
            "external"
        );
        assert_eq!(writer.confirmed.revision, 0);
        writer.fail(prepared.operation.id);
        writer
            .decide(snapshot, Intent::Load { expected: None })
            .unwrap();
        let load = writer.take().unwrap();
        let load = writer
            .prepare(load, &mut Autosave::default(), &font_context(16., false))
            .unwrap();
        assert_eq!(load.wire.kind, "load");
        assert!(load.envelope.is_none());
        assert_eq!(writer.first().unwrap().id, named);
        assert!(writer
            .outcomes
            .iter()
            .any(|outcome| outcome.id == prepared.operation.id && outcome.state == "cancelled"));
    }

    #[test]
    fn upgrade_wire_preserves_original_bytes_library_and_current_snapshot_with_no_early_ack() {
        let (mut writer, snapshot) = setup();
        writer.permission = Permission::Upgrading;
        let mut old_saved = snapshot.draft.clone();
        old_saved.id = "old-saved".into();
        old_saved.name = "Old saved".into();
        let raw = serde_json::to_string_pretty(&serde_json::json!({
            "envelope_version": 1, "draft": writer.confirmed.draft, "saved_themes": [old_saved], "preferences": Preferences::default()
        })).unwrap();
        writer
            .enqueue(
                snapshot.clone(),
                Intent::Upgrade {
                    legacy: raw.clone(),
                },
            )
            .unwrap();
        let operation = writer.take().unwrap();
        let prepared = writer
            .prepare(
                operation,
                &mut Autosave::default(),
                &font_context(16., false),
            )
            .unwrap();
        assert_eq!(prepared.wire.expected, None);
        assert_eq!(prepared.wire.legacy.as_deref(), Some(raw.as_str()));
        let backup = prepared.wire.backup.as_ref().unwrap();
        assert_eq!(backup.raw, raw);
        assert_eq!(
            backup.key,
            persistence::MigrationSource::new(&raw).backup_key
        );
        let candidate = prepared.envelope.as_ref().unwrap();
        assert_eq!(candidate.draft, snapshot.draft);
        assert_eq!(candidate.saved_themes[0].id, "old-saved");
        assert_eq!(candidate.revision, 1);
        assert_eq!(writer.permission, Permission::Upgrading);
        assert!(writer.confirmed.saved_themes.is_empty());
        writer.acknowledge(&prepared);
        assert_eq!(writer.permission, Permission::Ready);
        assert_eq!(writer.baseline, prepared.wire.write);
    }

    #[test]
    fn clear_generation_cancels_old_acks_and_cleared_gate_does_not_resurrect_the_cache() {
        let (mut writer, snapshot) = setup();
        writer.enqueue(snapshot.clone(), Intent::Autosave).unwrap();
        let old = writer.take().unwrap();
        let old = writer
            .prepare(old, &mut Autosave::default(), &font_context(16., false))
            .unwrap();
        writer.cancel_pending();
        writer
            .decide(
                snapshot.clone(),
                Intent::Clear {
                    expected: Some("reviewed bytes".into()),
                },
            )
            .unwrap();
        assert!(!writer.acknowledge(&old));
        let clear = writer.take().unwrap();
        let clear = writer
            .prepare(clear, &mut Autosave::default(), &font_context(16., false))
            .unwrap();
        assert_eq!(clear.wire.expected.as_deref(), Some("reviewed bytes"));
        assert_eq!(writer.permission, Permission::Clearing);
        assert!(writer.enqueue(snapshot.clone(), Intent::Autosave).is_err());
        assert!(writer
            .enqueue(
                snapshot.clone(),
                Intent::Save {
                    id: "later".into(),
                    name: "Later named action".into(),
                    overwrite: None
                }
            )
            .is_err());
        writer.acknowledge(&clear);
        assert_eq!(writer.permission, Permission::Cleared);
        assert!(writer.enqueue(snapshot, Intent::Autosave).is_err());
        assert!(writer.baseline.is_none());
        assert_eq!(writer.pending(), 0);
        assert!(writer
            .outcomes
            .iter()
            .any(|outcome| outcome.detail == "Later named action" && outcome.state == "rejected"));
    }

    #[test]
    fn cancelling_or_replacing_a_clear_restores_the_preexisting_write_gate() {
        let (mut writer, snapshot) = setup();
        writer.permission = Permission::Protected;
        writer
            .decide(snapshot.clone(), Intent::Clear { expected: None })
            .unwrap();
        assert_eq!(writer.permission, Permission::Clearing);
        writer.cancel_pending();
        assert_eq!(writer.permission, Permission::Protected);
        writer.permission = Permission::Ready;
        writer
            .decide(snapshot.clone(), Intent::Clear { expected: None })
            .unwrap();
        writer
            .decide(snapshot, Intent::Keep { expected: None })
            .unwrap();
        assert_eq!(writer.permission, Permission::Ready);
    }

    #[test]
    fn invalid_named_candidate_and_revision_overflow_preserve_confirmed_library_and_baseline() {
        let (mut writer, snapshot) = setup();
        let before = writer.confirmed.clone();
        writer
            .enqueue(
                snapshot.clone(),
                Intent::Save {
                    id: "bad".into(),
                    name: "\n".into(),
                    overwrite: None,
                },
            )
            .unwrap();
        let operation = writer.take().unwrap();
        assert!(writer
            .prepare(
                operation,
                &mut Autosave::default(),
                &font_context(16., false)
            )
            .is_err());
        assert_eq!(writer.confirmed, before);
        assert!(writer.baseline.is_none());
        writer.cancel_pending();
        writer.confirmed.revision = u64::MAX;
        writer.enqueue(snapshot, Intent::Autosave).unwrap();
        let operation = writer.take().unwrap();
        assert!(writer
            .prepare(
                operation,
                &mut Autosave::default(),
                &font_context(16., false)
            )
            .is_err());
        assert_eq!(writer.confirmed.revision, u64::MAX);
    }

    #[test]
    fn empty_external_bytes_are_not_a_missing_library_and_only_explicit_clear_can_remove_them() {
        let (mut writer, snapshot) = setup();
        let context = font_context(16., false);
        writer
            .decide(
                snapshot.clone(),
                Intent::Keep {
                    expected: Some(String::new()),
                },
            )
            .unwrap();
        let invalid = writer.take().unwrap();
        assert!(writer
            .prepare(invalid, &mut Autosave::default(), &context)
            .is_err());
        assert_eq!(writer.confirmed.revision, 0);
        writer.cancel_pending();
        writer
            .decide(snapshot.clone(), Intent::Keep { expected: None })
            .unwrap();
        let missing = writer.take().unwrap();
        let missing = writer
            .prepare(missing, &mut Autosave::default(), &context)
            .unwrap();
        assert!(missing.wire.expected.is_none());
        assert!(missing.wire.write.is_some());
        writer.cancel_pending();
        writer
            .decide(
                snapshot,
                Intent::Clear {
                    expected: Some(String::new()),
                },
            )
            .unwrap();
        let clear = writer.take().unwrap();
        let clear = writer
            .prepare(clear, &mut Autosave::default(), &context)
            .unwrap();
        assert_eq!(clear.wire.expected.as_deref(), Some(""));
        assert_eq!(clear.wire.kind, "clear");
        assert!(clear.wire.write.is_none());
    }
}
