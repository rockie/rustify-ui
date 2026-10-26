use crate::{controller::StudioState, write_queue::Permission};
use leptos::prelude::*;
use rustify_components::{Button, ButtonSize, ButtonVariant};

#[derive(Clone, Debug, PartialEq)]
enum ExternalVersion {
    Removed,
    Library {
        revision: u64,
        name: String,
        count: usize,
    },
    Invalid(String),
}

#[component]
pub fn StorageControls(state: StudioState) -> impl IntoView {
    let clear_open = RwSignal::new(false);
    let upgrade = move || {
        matches!(
            state.permission(),
            Permission::UpgradePending | Permission::UpgradeCancelled | Permission::Upgrading
        )
    };
    let pending = Signal::derive(move || state.pending_count());
    let external = Memo::new(move |_| {
        state.external.with(|value| {
            value.as_ref().map(|raw| match raw {
                None => ExternalVersion::Removed,
                Some(raw) => match crate::persistence::load(Some(raw), &state.context()) {
                    crate::persistence::LoadResult::Valid(envelope) => ExternalVersion::Library {
                        revision: envelope.revision,
                        name: envelope.draft.name.clone(),
                        count: envelope.saved_themes.len(),
                    },
                    crate::persistence::LoadResult::Invalid { error, .. } => {
                        ExternalVersion::Invalid(error.to_string())
                    }
                    crate::persistence::LoadResult::Missing => unreachable!(),
                },
            })
        })
    });
    view! {
        <section class="studio-storage tools-stack" aria-label=move || state.tr("Local data safety", "本地数据保护")>
            <Show when=upgrade>
                <div class="tools-notice" data-testid="upgrade-notice" data-upgrade-state=move || format!("{:?}", state.permission())>
                    <p>{move || state.tr("Your legacy draft and library were restored in memory. No old bytes, backup or v2 library are written until you review and confirm.", "旧草稿与主题库已恢复到内存。检查并确认前，不写入旧原文、备份或 v2 库。")}</p>
                    <p>{move || if state.permission() == Permission::UpgradeCancelled {state.tr("Upgrade cancelled: edits remain in memory only. Reopening restores the original legacy draft.", "已取消升级：改动仅保留在内存。重新打开会恢复原旧草稿。")} else {state.tr("Waiting for confirmation; editing, undo/redo and export remain available.", "等待确认；仍可编辑、撤销/重做与导出。")}}</p>
                    <div class="tools-actions">
                        <Button size=ButtonSize::One variant=ButtonVariant::Outline on_click=move || state.review_upgrade() test_id="upgrade-review-open">{move || state.tr("Review / recheck upgrade", "检查 / 重新检查升级")}</Button>
                        <Button size=ButtonSize::One variant=ButtonVariant::Ghost on_click=move || state.cancel_storage() test_id="upgrade-cancel">{move || state.tr("Cancel upgrade", "取消升级")}</Button>
                    </div>
                </div>
            </Show>
            <Show when=move || state.upgrade_review.with(Option::is_some)>
                <div class="tools-stack tools-notice" data-testid="upgrade-review">
                    <h2>{move || state.tr("Review the complete migration", "检查完整迁移")}</h2>
                    <p>{move || format!("{} {}", state.tr("Saved themes retained:", "保留的命名主题："), state.upgrade_review.with(|review| review.as_ref().map_or(0, |review| review.candidate.saved_themes.len())))}</p>
                    <p>{move || state.tr("This candidate contains every legacy saved theme plus the current draft and preferences. Confirmation does not reset history or preview mode.", "候选包含全部旧命名主题与当前草稿、偏好。确认不会重置历史或预览模式。")}</p>
                    <textarea class="tools-code" rows="12" readonly aria-label=move || state.tr("Complete upgrade candidate", "完整升级候选") data-testid="upgrade-candidate"
                        prop:value=move || state.upgrade_review.with(|review| review.as_ref().map(|review| serde_json::to_string_pretty(&review.candidate).unwrap_or_default()).unwrap_or_default())/>
                    <p role="status" data-testid="upgrade-review-status" data-stale=move || state.upgrade_review.with(|review| review.as_ref().is_some_and(|review| review.snapshot.epoch != state.editor_epoch.get())).to_string()>
                        {move || if state.upgrade_review.with(|review| review.as_ref().is_some_and(|review| review.snapshot.epoch != state.editor_epoch.get())) {state.tr("The draft changed after review. Confirm will refresh this candidate for another explicit review.", "检查后草稿已变化。确认会刷新候选，需再次明确检查。")} else {state.tr("Ready for explicit upgrade confirmation.", "可明确确认本次升级。")}}
                    </p>
                    <Button on_click=move || state.confirm_upgrade()
                        loading=Signal::derive(move || state.permission() == Permission::Upgrading)
                        test_id="upgrade-confirm">{move || state.tr("Upgrade and save current draft", "升级并保存当前草稿")}</Button>
                </div>
            </Show>
            <Show when=move || state.legacy_raw.with(Option::is_some) || state.envelope.with(|envelope| envelope.migrated_from_v1.is_some())>
                <Button size=ButtonSize::One variant=ButtonVariant::Outline on_click=move || state.download_legacy() test_id="legacy-download">{move || state.tr("Download original before upgrade", "下载升级前原始数据")}</Button>
            </Show>
            <Show when=move || state.legacy_changed.get()>
                <div class="tools-notice" data-testid="legacy-changed">
                    <p>{move || state.tr("An older window changed the v1 library. It was not merged into v2. The original backup remains separate.", "旧版窗口修改了 v1 库，未合并到 v2；升级前备份仍独立保留。")}</p>
                    <Button size=ButtonSize::One variant=ButtonVariant::Outline on_click=move || state.download_changed_legacy() test_id="legacy-changed-download">{move || state.tr("Download the changed v1 data", "下载变化后的 v1 原文")}</Button>
                </div>
            </Show>
            <Show when=move || !state.storage_issue.get().is_empty()>
                <p class="tools-error" role="alert" data-testid="storage-issue">{move || format!("{} {}", state.tr("Local save failed; draft and pending actions remain in memory:", "本地保存失败；草稿与待处理操作保留在内存："), state.storage_issue.get())}</p>
            </Show>
            <Show when=move || state.external.with(Option::is_some)>
                <div class="tools-notice tools-stack" data-testid="storage-external">
                    <p data-testid="storage-external-summary"
                        data-external-kind=move || match external.get() {Some(ExternalVersion::Library {..}) => "library", Some(ExternalVersion::Removed) => "removed", _ => "invalid"}
                        data-external-revision=move || match external.get() {Some(ExternalVersion::Library {revision, ..}) => Some(revision.to_string()), _ => None}>
                        {move || match external.get() {
                            Some(ExternalVersion::Library {revision, name, count}) => format!("{} {revision} · {name} · {} {count}", state.tr("External revision", "外部版本"), state.tr("Saved themes:", "命名主题：")),
                            Some(ExternalVersion::Removed) => state.tr("Another window removed the v2 library. Keeping your draft will create a new library.", "另一个窗口移除了 v2 库。保留当前草稿会创建新库。").into(),
                            Some(ExternalVersion::Invalid(error)) => format!("{}: {error}", state.tr("Invalid external v2 library", "外部 v2 库无效")),
                            None => String::new(),
                        }}
                    </p>
                    <p class="tools-status">{move || state.tr("Decisions use the version displayed here. A later external write requires another decision; your current draft stays unchanged until you choose.", "决定绑定此处展示的版本。之后外部再次写入须重新决定；选择前保留当前草稿。")}</p>
                    <Show when=move || state.external.with(|value| matches!(value, Some(Some(_))))>
                        <details data-testid="storage-external-raw">
                            <summary>{move || state.tr("View exact stored text", "查看存储原文")}</summary>
                            <textarea class="tools-code" rows="8" readonly data-testid="storage-external-source" aria-label=move || state.tr("Displayed external stored text", "当前展示的外部存储原文") prop:value=move || state.external.get().flatten().unwrap_or_default()/>
                        </details>
                        <Button size=ButtonSize::One variant=ButtonVariant::Outline test_id="storage-external-download" on_click=move || {
                            if let Some(Some(raw)) = state.external.get_untracked() {
                                if let Err(error) = rustify_ui::files::export("theme-studio-other-version.json", raw.as_bytes(), "application/json") {
                                    state.error.set(format!("{} {error:?}", state.tr("Download failed:", "下载失败：")));
                                }
                            }
                        }>{move || state.tr("Download displayed original", "下载当前展示原文")}</Button>
                    </Show>
                </div>
            </Show>
            <details data-testid="storage-operations">
                <summary>{move || format!("{} {}", state.tr("Pending / recent operations:", "待处理 / 最近操作："), pending.get())}</summary>
                <p class="tools-status">{move || state.tr("Retry explicitly approves retained actions in their original order. Cancel discards pending writes, preserving the open draft and history. Load other version discards all pending actions.", "重试会明确按原顺序执行已保留的操作。取消只丢弃待写操作，保留当前草稿与历史。载入另一版本会放弃全部待处理操作。")}</p>
                <ol><For each=move || {state.storage_epoch.get(); state.writer.with_value(|writer| writer.outcomes.clone())}
                    key=|outcome| (outcome.id, outcome.state) let:outcome>
                    <li data-op-id=outcome.id data-op-kind=outcome.kind data-op-state=outcome.state>
                        {move || {
                            let kind = match outcome.kind {"autosave" => state.tr("Draft", "草稿"), "save" => state.tr("Save", "保存"), "copy" | "conflict-copy" => state.tr("Copy", "副本"), "rename" => state.tr("Rename", "改名"), "delete" => state.tr("Delete", "删除"), "upgrade" => state.tr("Upgrade", "升级"), "clear" | "clear-backups" => state.tr("Clear", "清除"), "load" => state.tr("Load", "载入"), _ => state.tr("Keep", "保留")};
                            let status = match outcome.state {"saved" => state.tr("confirmed", "已确认"), "failed" => state.tr("failed; retained", "失败，已保留"), "superseded" => state.tr("merged into newer draft", "已合并到更新草稿"), "cancelled" => state.tr("cancelled", "已取消"), "rejected" => state.tr("not queued; storage protected", "未入队，存储受保护"), _ => state.tr("pending", "待处理")};
                            format!("{kind} {} · {status}", outcome.detail)
                        }}
                    </li>
                </For></ol>
            </details>
            <div class="tools-actions">
                <Button size=ButtonSize::One variant=ButtonVariant::Outline on_click=move || state.retry_storage() test_id="storage-retry">{move || if state.permission() == Permission::Cleared {state.tr("Save current draft again", "重新保存当前草稿")} else {state.tr("Retry retained saves", "重试已保留的保存")}}</Button>
                <Button size=ButtonSize::One variant=ButtonVariant::Ghost disabled=Signal::derive(move || pending.get() == 0) on_click=move || state.cancel_storage() test_id="storage-cancel">{move || state.tr("Cancel pending writes", "取消待写操作")}</Button>
            </div>
            <details class="tools-stack">
                <summary>{move || state.tr("Cache and original backups", "缓存与原始备份")}</summary>
                <Button size=ButtonSize::One variant=ButtonVariant::Ghost on_click=move || {state.review_clear_cache(); clear_open.set(true);} test_id="storage-clear-review">{move || state.tr("Review clearing v2 cache", "检查清除 v2 缓存")}</Button>
                <Show when=move || clear_open.get()>
                    <p>{move || state.tr("Clear only the reviewed v2 library and cancel all preceding pending writes. Keep the current draft, v1 source and backups. Later persistence requires an explicit save.", "只清除已检查的 v2 库，并取消此前全部待写操作。保留当前草稿、v1 原文与备份；之后须明确保存才恢复持久化。")}</p>
                    <div class="tools-actions">
                        <Button size=ButtonSize::One on_click=move || {state.clear_cache(); clear_open.set(false);} test_id="storage-clear-confirm">{move || state.tr("Clear reviewed v2 cache", "清除已检查的 v2 缓存")}</Button>
                        <Button size=ButtonSize::One variant=ButtonVariant::Ghost on_click=move || clear_open.set(false) test_id="storage-clear-cancel">{move || state.tr("Cancel", "取消")}</Button>
                    </div>
                </Show>
                <Button size=ButtonSize::One variant=ButtonVariant::Outline on_click=move || state.review_backups() test_id="backups-review">{move || state.tr("Review original backups", "检查原始备份")}</Button>
                <Show when=move || state.backup_review.with(Option::is_some)>
                    <p>{move || state.tr("These are independent original byte backups. Ordinary cache clearing keeps them. Removing them is a separate irreversible action.", "这些是独立保存的原始字节备份，普通清缓存会保留它们。删除备份是单独且不可撤销的操作。")}</p>
                    <ul>{move || state.backup_review.with(|review| review.as_ref().map(|values| values.keys().cloned().map(|key| {
                        let download = key.clone();
                        view! {<li><code>{key}</code><Button size=ButtonSize::One variant=ButtonVariant::Outline on_click=move || state.download_backup(&download) test_id="backup-download">{move || state.tr("Download original", "下载原文")}</Button></li>}
                    }).collect_view()))}</ul>
                    <Button size=ButtonSize::One variant=ButtonVariant::Outline disabled=Signal::derive(move || state.backup_review.with(|values| values.as_ref().is_none_or(BTreeMap::is_empty))) on_click=move || state.clear_backups() test_id="backups-clear-confirm">{move || state.tr("Delete only the reviewed backups", "只删除已检查的备份")}</Button>
                </Show>
            </details>
        </section>
    }
}

use std::collections::BTreeMap;

#[component]
pub fn TokenSource(state: StudioState) -> impl IntoView {
    let overridden = Signal::derive(move || {
        state.editor.with(|editor| {
            let token = state.selected_token.get();
            if rustify_ui::theme::COMMON_TOKENS.contains(&token.as_str()) {
                [rustify_ui::ThemeMode::Light, rustify_ui::ThemeMode::Dark]
                    .iter()
                    .any(|mode| {
                        editor
                            .document
                            .overrides
                            .values(*mode)
                            .0
                            .contains_key(&token)
                    })
            } else {
                editor
                    .document
                    .overrides
                    .values(editor.mode)
                    .0
                    .contains_key(&token)
            }
        })
    });
    view! {
        <section class="studio-token-source tools-stack" data-testid="token-source">
            <h3>{move || state.tr("Fine adjustment source", "精调来源")}</h3>
            <code data-testid="source-token">{move || state.selected_token.get()}</code>
            <p data-testid="source-kind">{move || {
                use rustify_ui::ThemeValueOrigin;
                let token = state.selected_token.get();
                let origin = state.resolved().get().origins.get(token.as_str()).copied();
                match origin {Some(ThemeValueOrigin::DocumentOverride) => state.tr("Explicit document override", "文档显式覆盖"), Some(ThemeValueOrigin::Configuration(_)) => state.tr("Generated by base configuration", "基础配置生成"), Some(ThemeValueOrigin::BoundaryConfiguration {..}) => state.tr("Boundary configuration", "边界配置生成"), Some(ThemeValueOrigin::BoundaryOverride {..}) => state.tr("Boundary override", "边界覆盖"), _ => state.tr("Authored base value", "作者基础值")}
            }}</p>
            <p class="tools-status">{move || state.tr("Removing this override reveals the nearest configured or authored base value. Other tokens stay unchanged.", "解除此覆盖会显示最近的配置值或作者基础值，其他 token 不变。")}</p>
            <Button size=ButtonSize::One variant=ButtonVariant::Outline disabled=Signal::derive(move || !overridden.get()) on_click=move || state.clear_override(&state.selected_token.get_untracked()) test_id="source-clear">
                {move || if rustify_ui::theme::COMMON_TOKENS.contains(&state.selected_token.get().as_str()) {state.tr("Clear this token · both modes", "解除此 token · 两模式")} else {state.tr("Clear this token · current mode", "解除此 token · 当前模式")}}
            </Button>
        </section>
    }
}
