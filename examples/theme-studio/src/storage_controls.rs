use crate::{controller::StudioState, write_queue::Permission};
use leptos::prelude::*;
use rustify_components::{Button, ButtonSize, ButtonVariant, Dialog};

pub(super) fn reference_status(
    document: &rustify_ui::ThemeDocument,
    reference: &rustify_ui::ThemeDocument,
    locale: rustify_ui::Locale,
) -> String {
    match (document != reference, locale) {
        (true, rustify_ui::Locale::Chinese) => "已偏离参考文档".into(),
        (false, rustify_ui::Locale::Chinese) => "与参考文档一致".into(),
        (true, _) => "Modified from reference document".into(),
        (false, _) => "Matches reference document".into(),
    }
}

pub(super) fn draft_saved(state: StudioState) -> bool {
    state.storage_epoch.get();
    let epoch = state.editor_epoch.get();
    state.writer.with_value(|writer| {
        writer
            .acknowledged
            .as_ref()
            .is_some_and(|(saved, _, _)| *saved == epoch)
    })
}

pub(super) fn storage_label(state: StudioState) -> &'static str {
    if !crate::storage_browser::writable() || state.permission() == Permission::UpgradeCancelled {
        return state.tr("Memory only", "仅内存");
    }
    if state.permission() != Permission::Ready || state.external.with(Option::is_some) {
        return state.tr("Saving paused", "保存已暂停");
    }
    if !state.storage_issue.get().is_empty() {
        return if draft_saved(state) {
            state.tr("Draft saved · action failed", "草稿已保存 · 操作失败")
        } else {
            state.tr("Save failed", "保存失败")
        };
    }
    if state.pending_count() > 0 {
        return state.tr("Saving…", "正在保存…");
    }
    if draft_saved(state) {
        state.tr("Saved on this device", "已保存到本机")
    } else {
        state.tr("Not saved yet", "尚未保存")
    }
}

#[component]
pub fn StorageControls(state: StudioState) -> impl IntoView {
    let clear_open = RwSignal::new(false);
    let dismissed_outcome = RwSignal::new(None::<u64>);
    let download_status = RwSignal::new(String::new());
    let upgrade = move || {
        matches!(
            state.permission(),
            Permission::UpgradePending | Permission::UpgradeCancelled | Permission::Upgrading
        )
    };
    let normal_state = move || {
        !upgrade() && state.external.with(Option::is_none) && state.corrupt.with(Option::is_none)
    };
    let pending = Signal::derive(move || state.pending_count());
    let unsaved_draft = Signal::derive(move || {
        state.storage_epoch.get();
        let epoch = state.editor_epoch.get();
        let mut preferences = state.envelope.with(|envelope| envelope.preferences.clone());
        preferences.mode = state.mode().get();
        state.editor.with(|editor| {
            state.writer.with_value(|writer| {
                writer.permission == Permission::Ready
                    && writer.acknowledged.as_ref().is_none_or(
                        |(saved, document, saved_preferences)| {
                            *saved != epoch
                                || document != &editor.document
                                || saved_preferences != &preferences
                        },
                    )
            })
        })
    });
    let outcomes = Memo::new(move |_| {
        state.storage_epoch.get();
        state.writer.with_value(|writer| writer.outcomes.clone())
    });
    let tone = Memo::new(move |_| {
        state.storage_epoch.get();
        if !state.storage_issue.get().is_empty() {
            return "error";
        }
        if !crate::storage_browser::writable()
            || state.permission() != Permission::Ready
            || state.external.get().is_some()
        {
            return "warning";
        }
        let epoch = state.editor_epoch.get();
        state.writer.with_value(|writer| {
            if writer.paused {
                "warning"
            } else if writer.pending() > 0 {
                "pending"
            } else if writer
                .acknowledged
                .as_ref()
                .is_some_and(|(saved, _, _)| *saved == epoch)
            {
                "success"
            } else {
                "memory"
            }
        })
    });
    let upgraded = move || {
        outcomes.with(|items| {
            items
                .last()
                .is_some_and(|item| item.kind == "upgrade" && item.state == "saved")
        })
    };
    let download_current = move || {
        let result = rustify_ui::theme::export_json(
            &state
                .editor
                .with_untracked(|editor| editor.document.clone()),
            &state.context(),
        )
        .map_err(|error| error.to_string())
        .and_then(|json| {
            rustify_ui::files::export("theme.json", json.as_bytes(), "application/json")
                .map_err(|error| format!("{error:?}"))
        });
        download_status.set(match result {
            Ok(()) => state
                .tr(
                    "Download started. Check your browser downloads.",
                    "已发起下载，请查看浏览器下载记录。",
                )
                .into(),
            Err(_) => state
                .tr(
                    "Download failed. Open Export to select and copy the theme JSON.",
                    "下载失败，请打开导出，选择并复制主题 JSON。",
                )
                .into(),
        });
    };
    view! {
        <section class="studio-storage tools-stack" aria-label=move || state.tr("Local data safety", "本地数据保护")>
            <Show when=normal_state>
            <div class=move || format!("tools-storage-state tools-state-{}",tone.get()) role="status">
                <strong>{move || match tone.get(){
                    "success" if upgraded()=>state.tr("Library upgraded to v2","主题库已升级至 v2"),
                    "success"=>state.tr("Saved","已保存"),
                    "pending"=>state.tr("Saving","正在保存"),
                    "error" if state.storage_issue.get().contains("Quota")=>state.tr("Storage full · saving in memory","存储已满 · 暂存内存"),
                    "error"=>state.tr("Local save failed · saving in memory","本地保存失败 · 暂存内存"),
                    "warning" if !crate::storage_browser::writable()=>state.tr("Memory only · local saving unavailable","仅内存 · 无法保存到本机"),
                    "warning"=>state.tr("Memory only · saving paused","仅内存 · 保存已暂停"),
                    _=>state.tr("Memory only · not saved yet","仅内存 · 尚未保存"),
                }}</strong>
                <p data-testid="storage-status">{move || {
                    if !state.storage_issue.get().is_empty() {
                        if draft_saved(state) {
                            state.tr("The current draft is saved on this device.","当前草稿已保存到本机。").to_owned()
                        } else {state.tr("Your current draft stays in memory. Download or export a copy before leaving.","当前草稿保留在内存，离开前请下载或导出副本。").to_owned()}
                    }else if upgraded() {
                        let count = state.envelope.with(|envelope| envelope.saved_themes.len());
                        if state.locale().get() == rustify_ui::Locale::Chinese {format!("{count} 个命名主题与当前草稿已保留。")} else {format!("{count} saved theme{} and your current draft were retained.",if count == 1 {""} else {"s"})}
                    }else if !crate::storage_browser::writable() {
                        state.tr("You can edit in memory and download or export your theme.","可以在内存中编辑，并下载或导出主题。").to_owned()
                    }else{state.storage_status.get()}
                }}</p>
                <Show when=upgraded><p class="tools-status">{move || state.tr("Original data keeps a copy of the library from before the upgrade.","原始数据保存着升级前的主题库副本。")}</p></Show>
                <Show when=move || matches!(tone.get(),"warning"|"error"|"memory")>
                    <Button size=ButtonSize::One on_click=move || download_current() test_id="storage-export">{move || state.tr("Download current theme", "下载当前主题")}</Button>
                </Show>
                <Show when=move ||state.legacy_raw.with(Option::is_some) || state.envelope.with(|envelope|envelope.migrated_from_v1.is_some())><Button size=ButtonSize::One variant=ButtonVariant::Outline on_click=move ||state.download_legacy() test_id="legacy-download">{move ||state.tr("Download original data","下载原始数据")}</Button></Show>
                <Show when=move ||!state.storage_issue.get().is_empty()><details data-testid="storage-issue"><summary>{move ||state.tr("Technical details","技术详情")}</summary><code>{move ||state.storage_issue.get()}</code></details></Show>
                <p class="tools-status" role="status">{move ||download_status.get()}</p>
            </div>
            </Show>
            <Show when=upgrade>
                <div class="tools-notice tools-state-warning" data-testid="upgrade-notice" data-upgrade-state=move || format!("{:?}", state.permission())>
                    <h3>{move || if state.permission()==Permission::UpgradeCancelled{state.tr("Memory only · upgrade cancelled","仅内存 · 已取消升级")}else{state.tr("Review the library upgrade","检查主题库升级")}}</h3>
                    <p data-testid="storage-status">{move || if state.permission()==Permission::UpgradeCancelled {state.tr("Reopening restores the original draft. Download your current theme to keep these memory-only edits.","重新打开会恢复原草稿。请下载当前主题，以保留仅在内存中的改动。").to_owned()}else{state.storage_status.get()}}</p>
                    <Show when=move ||state.permission()!=Permission::UpgradeCancelled><p class="tools-status">{move ||state.tr("Editing, undo/redo and export remain available while confirmation is pending.","等待确认期间仍可编辑、撤销/重做与导出。")}</p></Show>
                    <div class="tools-actions">
                        <Button on_click=move || state.review_upgrade() test_id="upgrade-review-open">{move || if state.permission()==Permission::UpgradeCancelled{state.tr("Review upgrade again","重新检查升级")}else{state.tr("Review upgrade", "检查升级")}}</Button>
                        <Show when=move ||state.permission()==Permission::UpgradeCancelled><Button variant=ButtonVariant::Outline on_click=move ||download_current() test_id="storage-export">{move ||state.tr("Download current theme","下载当前主题")}</Button></Show>
                        <Button variant=ButtonVariant::Outline on_click=move || state.download_legacy() test_id="legacy-download">{move || state.tr("Download original data", "下载原始数据")}</Button>
                        <Show when=move ||state.permission()!=Permission::UpgradeCancelled && state.upgrade_review.with(Option::is_none)><Button variant=ButtonVariant::Ghost on_click=move || state.cancel_storage() test_id="upgrade-cancel">{move || state.tr("Cancel upgrade", "取消升级")}</Button></Show>
                    </div>
                </div>
            </Show>
            <Show when=move || !normal_state() && !state.storage_issue.get().is_empty()>
                <details class="tools-notice" data-testid="storage-issue"><summary>{move ||state.tr("Technical details","技术详情")}</summary><code>{move ||state.storage_issue.get()}</code></details>
            </Show>
            <Dialog open=Signal::derive(move || state.upgrade_review.with(Option::is_some))
                on_open_change=move |open|if !open{state.upgrade_review.set(None);}
                title=Signal::derive(move ||state.tr("Review the library upgrade","检查主题库升级").to_owned())
                class="tools-dialog tools-upgrade-dialog" test_id="upgrade-review">
                <div class="tools-stack">
                    <p>{move || state.tr("Review every saved theme, the current draft and preferences before upgrading. Confirmation preserves your editing history and preview mode.", "升级前请核对全部命名主题、当前草稿和偏好。确认会保留编辑历史与预览模式。")}</p>
                    {move ||state.upgrade_review.get().map(|review|{
                        let candidate=review.candidate;
                        let preferences=candidate.preferences;
                        let count=candidate.saved_themes.len();
                        let draft_status=state.editor.with(|editor| reference_status(&review.snapshot.draft, editor.checkpoint_document(), state.locale().get()));
                        view!{
                            <dl class="tools-summary tools-migration-summary">
                                <dt>{state.tr("Library version","主题库版本")}</dt><dd>"v1 → v2"</dd>
                                <dt>{state.tr("Original fingerprint","原始数据指纹")}</dt><dd><code>{crate::persistence::digest(&review.raw).chars().take(12).collect::<String>()}</code></dd>
                            </dl>
                            <h3>{format!("{} ({count})",state.tr("Saved themes retained","保留的命名主题"))}</h3>
                            <div class="tools-migration-theme-head"><span>{state.tr("Name","名称")}</span><span>{state.tr("ID","标识")}</span><span>{state.tr("Result","结果")}</span></div>
                            <ul class="tools-migration-themes">{candidate.saved_themes.into_iter().map(|theme|view!{
                                <li><strong>{theme.name}</strong><code>{theme.id}</code><span>{state.tr("Retained as v2","保留为 v2")}</span></li>
                            }).collect_view()}</ul>
                            <h3>{state.tr("Current draft to save","将保存的当前草稿")}</h3>
                            <dl class="tools-summary tools-migration-summary">
                                <dt>{state.tr("Name","名称")}</dt><dd>{candidate.draft.name}</dd>
                                <dt>{state.tr("Preview mode","预览模式")}</dt><dd>{if preferences.mode==rustify_ui::ThemeMode::Light{state.tr("Light","浅色")}else{state.tr("Dark","深色")}}</dd>
                                <dt>{state.tr("Draft state","草稿状态")}</dt><dd>{draft_status}</dd>
                            </dl>
                            <h3>{state.tr("Preferences to save","将保存的偏好")}</h3>
                            <dl class="tools-summary tools-migration-summary">
                                <dt>{state.tr("Language","语言")}</dt><dd>{if preferences.locale=="zh"{"中文"}else{"English"}}</dd>
                                <dt>{state.tr("Scene","场景")}</dt><dd>{preferences.scene}</dd>
                                <dt>{state.tr("Renderer","渲染器")}</dt><dd>{match preferences.renderer.as_str(){"compare"=>"DOM + GPU","gpu"=>"GPU",_=>"DOM"}}</dd>
                                <dt>{state.tr("Preview width","预览宽度")}</dt><dd>{match preferences.preview_width.as_str(){"responsive"=>state.tr("Responsive","自适应"),"desktop"=>"1024px","tablet"=>"768px",_=>"390px"}}</dd>
                                <dt>{state.tr("Favorites","收藏")}</dt><dd>{if preferences.favorites.is_empty(){state.tr("None","无").to_owned()}else{preferences.favorites.join(", ")}}</dd>
                            </dl>
                        }
                    })}
                    <details><summary>{move || state.tr("Complete library upgrade JSON","完整主题库升级 JSON")}</summary>
                        <textarea class="tools-code tools-raw" rows="12" readonly aria-label=move || state.tr("Complete upgrade candidate", "完整升级候选") data-testid="upgrade-candidate"
                            prop:value=move || state.upgrade_review.with(|review| review.as_ref().map(|review| serde_json::to_string_pretty(&review.candidate).unwrap_or_default()).unwrap_or_default())/>
                    </details>
                    <p class="tools-status" role="status" data-testid="upgrade-review-status" data-stale=move || state.upgrade_review.with(|review| review.as_ref().is_some_and(|review| review.snapshot.epoch != state.editor_epoch.get())).to_string()>
                        {move || if state.upgrade_review.with(|review| review.as_ref().is_some_and(|review| review.snapshot.epoch != state.editor_epoch.get())) {state.tr("The draft changed after review. Confirm will refresh this candidate for another explicit review.", "检查后草稿已变化。确认会刷新候选，需再次明确检查。")} else {state.tr("Ready to upgrade after your confirmation.", "可明确确认本次升级。")}}
                    </p>
                    <div class="tools-actions tools-dialog-footer">
                        <Button on_click=move || {
                            state.confirm_upgrade();
                            if state.permission()==Permission::Upgrading{state.upgrade_review.set(None);}
                        } loading=Signal::derive(move || state.permission() == Permission::Upgrading) test_id="upgrade-confirm">{move || state.tr("Upgrade and save current draft", "升级并保存当前草稿")}</Button>
                        <Button variant=ButtonVariant::Outline on_click=move ||state.download_legacy() test_id="upgrade-download-original">{move ||state.tr("Download original data","下载原始数据")}</Button>
                        <Button variant=ButtonVariant::Ghost on_click=move ||state.cancel_storage() test_id="upgrade-cancel">{move ||state.tr("Cancel upgrade","取消升级")}</Button>
                    </div>
                </div>
            </Dialog>
            <Show when=move || state.legacy_changed.get()>
                <div class="tools-notice" data-testid="legacy-changed">
                    <p>{move || state.tr("An older window changed the v1 library. It was not merged into v2. The original backup remains separate.", "旧版窗口修改了 v1 库，未合并到 v2；升级前备份仍独立保留。")}</p>
                    <Button size=ButtonSize::One variant=ButtonVariant::Outline on_click=move || state.download_changed_legacy() test_id="legacy-changed-download">{move || state.tr("Download the changed v1 data", "下载变化后的 v1 原文")}</Button>
                </div>
            </Show>
            {move || outcomes.with(|items|items.iter().rev().find(|item|matches!(item.kind,"save"|"copy"|"rename"|"delete"|"conflict-copy")).filter(|item|Some(item.id)!=dismissed_outcome.get()).cloned()).map(|outcome|{
                let action=match outcome.kind{"save"=>state.tr("Save","保存"),"rename"=>state.tr("Rename","重命名"),"delete"=>state.tr("Delete","删除"),_=>state.tr("Copy","复制")};
                let result=match outcome.state{
                    "saved"=>state.tr("completed","已完成"),
                    "failed"=>state.tr("failed; retained for retry. Export remains available","失败，已保留供重试，仍可导出"),
                    "rejected" if matches!(state.permission(),Permission::UpgradePending|Permission::UpgradeCancelled|Permission::Upgrading)=>state.tr("not saved; review the library upgrade before saving this named theme","未保存，请先检查主题库升级，再保存此命名主题"),
                    "rejected" if !crate::storage_browser::writable()=>state.tr("not saved; no local write was made","未保存，未写入本机"),
                    "rejected" if state.permission()==Permission::Ready && state.external.with(Option::is_none)=>state.tr("wasn't saved before the upgrade","升级前未保存"),
                    "rejected"=>state.tr("not saved; review the storage warning first","未保存，请先处理存储提示"),
                    "cancelled"=>state.tr("cancelled","已取消"),
                    _=>state.tr("waiting to save","等待保存")
                };
                let id=outcome.id;
                let retry_name=outcome.detail.clone();
                let can_retry=outcome.kind=="save" && matches!(outcome.state,"rejected"|"failed") && state.permission()==Permission::Ready && crate::storage_browser::writable() && state.external.with(Option::is_none);
                let review_upgrade=outcome.state=="rejected" && matches!(state.permission(),Permission::UpgradePending|Permission::UpgradeCancelled|Permission::Upgrading);
                let download_unavailable=outcome.state=="rejected" && !crate::storage_browser::writable();
                let retry_failed=outcome.state=="failed";
                let retry_label=if retry_failed {if state.locale().get()==rustify_ui::Locale::Chinese {format!("重试“{}”",outcome.detail)} else {format!("Retry “{}”",outcome.detail)}}else if state.locale().get()==rustify_ui::Locale::Chinese {format!("立即保存“{}”",outcome.detail)}else{format!("Save “{}” now",outcome.detail)};
                let retry_label=StoredValue::new(retry_label);
                view!{<div class=if outcome.state=="failed" {"tools-notice tools-state-error"} else if outcome.state=="rejected" && !can_retry {"tools-notice tools-state-warning"} else {"tools-notice"} role="status" data-testid="named-storage-feedback" data-result=outcome.state>
                    <p>{format!("{action} “{}” · {result}.",outcome.detail)}</p>
                    <div class="tools-actions"><Show when=move ||can_retry><Button size=ButtonSize::One on_click={let name=retry_name.clone();move ||if retry_failed {state.retry_storage()}else{state.save_named(name.clone(),None)}}>{retry_label.get_value()}</Button></Show>
                    <Show when=move ||review_upgrade><Button size=ButtonSize::One variant=ButtonVariant::Outline on_click=move ||state.review_upgrade() test_id="named-upgrade-review">{move ||state.tr("Review upgrade","检查升级")}</Button></Show>
                    <Show when=move ||download_unavailable><Button size=ButtonSize::One variant=ButtonVariant::Outline on_click=move ||download_current() test_id="named-storage-download">{move ||state.tr("Download current theme","下载当前主题")}</Button></Show>
                    <Button size=ButtonSize::One variant=ButtonVariant::Ghost on_click=move ||dismissed_outcome.set(Some(id))>{move ||state.tr("Dismiss","关闭提示")}</Button></div>
                </div>}
            })}
            <details data-testid="storage-operations">
                <summary>{move || format!("{} {} · {} {}",pending.get(),state.tr("pending","项待处理"),outcomes.with(Vec::len),if outcomes.with(Vec::len)==1 {state.tr("recent operation","项最近操作")}else{state.tr("recent operations","项最近操作")})}</summary>
                <div class="tools-operations-content">
                    <p class="tools-status">{move ||state.tr("Retry saves the listed actions in order. Cancel removes pending saves and keeps your draft and undo history.","重试会按原顺序保存已保留的操作。取消只丢弃待写操作，保留当前草稿与历史。")}{move ||state.external.with(Option::is_some).then(||state.tr(" Loading the other version removes all pending actions.","载入另一版本会放弃全部待处理操作。"))}</p>
                    <ol><For each=move || outcomes.get()
                        key=|outcome| (outcome.id, outcome.state) let:outcome>
                        <li data-op-id=outcome.id data-op-kind=outcome.kind data-op-state=outcome.state>
                            {move || {
                                let kind = match outcome.kind {"autosave" => state.tr("Draft", "草稿"), "save" => state.tr("Save", "保存"), "copy" | "conflict-copy" => state.tr("Copy", "副本"), "rename" => state.tr("Rename", "改名"), "delete" => state.tr("Delete", "删除"), "upgrade" => state.tr("Upgrade", "升级"), "clear" | "clear-backups" => state.tr("Clear", "清除"), "load" => state.tr("Load", "载入"), _ => state.tr("Keep", "保留")};
                                let status = match outcome.state {"saved" => state.tr("confirmed", "已确认"), "failed" => state.tr("failed; retained", "失败，已保留"), "superseded" => state.tr("merged into newer draft", "已合并到更新草稿"), "cancelled" => state.tr("cancelled", "已取消"), "rejected" => state.tr("not queued; storage protected", "未入队，存储受保护"), _ => state.tr("pending", "待处理")};
                                let name=if outcome.detail.is_empty(){String::new()}else{format!(" “{}”",outcome.detail)};
                                view!{<span>{format!("#{} · {kind}{name} · {status} ",outcome.id)}</span><span class="tools-op-revision">{format!("({} {})",state.tr("draft revision","草稿修订"),outcome.epoch)}</span>}
                            }}
                        </li>
                    </For></ol>
                </div>
            </details>
            <div class="tools-actions">
                <Button size=ButtonSize::One variant=ButtonVariant::Outline disabled=Signal::derive(move || !crate::storage_browser::writable() || state.external.with(Option::is_some) || pending.get()==0 && !unsaved_draft.get() && state.permission()!=Permission::Cleared) on_click=move || state.retry_storage() test_id="storage-retry">{move || if state.permission() == Permission::Cleared {state.tr("Save current draft again", "重新保存当前草稿")} else if pending.get()==0 && unsaved_draft.get() {state.tr("Save current draft", "保存当前草稿")} else {state.tr("Retry all retained saves", "重试全部已保留的保存")}}</Button>
                <Button size=ButtonSize::One variant=ButtonVariant::Outline disabled=Signal::derive(move || pending.get() == 0) on_click=move || state.cancel_storage() test_id="storage-cancel">{move || state.tr("Cancel pending writes", "取消待写操作")}</Button>
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
    let revealed_input = Memo::new(move |_| {
        if !overridden.get() {
            return None;
        }
        state.editor.with(|editor| {
            let mut document = editor.document.clone();
            let token = state.selected_token.get();
            let modes = if rustify_ui::theme::COMMON_TOKENS.contains(&token.as_str()) {
                vec![rustify_ui::ThemeMode::Light, rustify_ui::ThemeMode::Dark]
            } else {
                vec![editor.mode]
            };
            for mode in modes {
                document.overrides.values_mut(mode).0.remove(&token);
            }
            Some((
                document,
                token,
                editor.mode,
                state.rem.get(),
                state.reduce_motion.get(),
                state.locale().get(),
            ))
        })
    });
    let revealed = Memo::new(move |_| {
        let Some((document, token, mode, rem, reduce_motion, locale)) = revealed_input.get() else {
            return Vec::new();
        };
        let context = rustify_ui::theme::font_context(rem, reduce_motion);
        let modes = if rustify_ui::theme::COMMON_TOKENS.contains(&token.as_str()) {
            vec![rustify_ui::ThemeMode::Light, rustify_ui::ThemeMode::Dark]
        } else {
            vec![mode]
        };
        modes
            .into_iter()
            .filter_map(|mode| {
                rustify_ui::theme::resolve(&document, mode, &context)
                    .ok()
                    .and_then(|theme| theme.values.0.get(&token).cloned())
                    .map(|value| {
                        (
                            match (mode, locale) {
                                (rustify_ui::ThemeMode::Light, rustify_ui::Locale::English) => {
                                    "Light"
                                }
                                (rustify_ui::ThemeMode::Light, rustify_ui::Locale::Chinese) => {
                                    "浅色"
                                }
                                (rustify_ui::ThemeMode::Dark, rustify_ui::Locale::English) => {
                                    "Dark"
                                }
                                (rustify_ui::ThemeMode::Dark, rustify_ui::Locale::Chinese) => {
                                    "深色"
                                }
                            },
                            value,
                        )
                    })
            })
            .collect::<Vec<_>>()
    });
    view! {
        <section class="studio-token-source tools-stack" data-testid="token-source">
            <h3>{move || state.tr("Fine adjustment source", "精调来源")}</h3>
            <dl class="tools-summary tools-source-summary">
            <dt>"Token"</dt><dd><code data-testid="source-token">{move || state.selected_token.get()}</code></dd>
            <dt>{move ||state.tr("Effective value","有效值")}</dt><dd><code data-testid="source-effective">{move ||state.values().get().0.get(&state.selected_token.get()).cloned().unwrap_or_default()}</code></dd>
            <dt>{move ||state.tr("Source","来源")}</dt><dd data-testid="source-kind">{move || {
                use rustify_ui::ThemeValueOrigin;
                let token = state.selected_token.get();
                let origin = state.resolved().get().origins.get(token.as_str()).copied();
                match origin {Some(ThemeValueOrigin::DocumentOverride) => state.tr("Explicit document override", "文档显式覆盖"), Some(ThemeValueOrigin::Configuration(_)) => state.tr("Generated by base configuration", "基础配置生成"), Some(ThemeValueOrigin::BoundaryConfiguration {..}) => state.tr("Boundary configuration", "边界配置生成"), Some(ThemeValueOrigin::BoundaryOverride {..}) => state.tr("Boundary override", "边界覆盖"), _ => state.tr("Authored base value", "作者基础值")}
            }}</dd>
            <dt>{move ||state.tr("Path","路径")}</dt><dd><code class="tools-source-path" data-testid="source-path">{move ||{
                use rustify_ui::{ThemeValueOrigin, ThemeConfigGroup, ThemeMode};
                let token=state.selected_token.get();
                let theme=state.resolved().get();
                let mode=state.mode().get().as_str();
                match theme.origins.get(token.as_str()).copied(){
                    Some(ThemeValueOrigin::DocumentOverride)=>format!("overrides.{mode}.{token}"),
                    Some(ThemeValueOrigin::Configuration(group))=>{
                        let selected=match group{
                            ThemeConfigGroup::Accent=>{
                                let step=match token.as_str(){"ring"|"sidebar-ring"=>"8","primary-foreground"|"sidebar-primary-foreground"=>"contrast",_=>"9"};
                                format!("{} · {step}",theme.brand.family.name())
                            },
                            ThemeConfigGroup::Gray=>{
                                let step=match token.as_str(){"background"|"sidebar" if theme.mode==ThemeMode::Light=>"white","background"|"sidebar"=>"1","input"=>"2","secondary"|"muted"|"sidebar-accent"=>"3","border"|"sidebar-border"=>"7","muted-foreground"=>"11",_=>"12"};
                                format!("{} · {step}",theme.gray.family.name())
                            },
                            ThemeConfigGroup::Panel=>theme.config.selected.panel_background.map(|value|value.as_str()).unwrap_or("Custom").into(),
                            ThemeConfigGroup::Radius=>theme.config.selected.radius.map(|value|value.as_str()).unwrap_or("Custom").into(),
                        };
                        format!("config.{} · {selected} · {mode}",group.field())
                    },
                    _=>format!("base.{mode}.{token}"),
                }
            }}</code></dd>
            <dt>{move ||state.tr("If cleared","解除后")}</dt><dd><Show when=move ||overridden.get() fallback=move ||state.tr("This value has no explicit override to clear. Change its base configuration or authored base value to adjust it.","此值没有可解除的显式覆盖，可通过基础配置或作者基础值调整。")>
                {move ||revealed.get().into_iter().map(|(mode,value)|view!{<span class="tools-revealed-value">{mode}": "<code>{value}</code></span>}).collect_view()}
                <p class="tools-status">{move ||state.tr("Other tokens stay unchanged.","其他 token 保持不变。")}</p>
            </Show></dd>
            </dl>
            <Button size=ButtonSize::One variant=ButtonVariant::Outline disabled=Signal::derive(move || !overridden.get()) on_click=move || state.clear_override(&state.selected_token.get_untracked()) test_id="source-clear">
                {move || format!("{} {} · {}",state.tr("Clear","解除"),state.selected_token.get(),if rustify_ui::theme::COMMON_TOKENS.contains(&state.selected_token.get().as_str()){state.tr("both modes","两模式")}else if state.mode().get()==rustify_ui::ThemeMode::Light{state.tr("Light only","仅浅色")}else{state.tr("Dark only","仅深色")})}
            </Button>
        </section>
    }
}
