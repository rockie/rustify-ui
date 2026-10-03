use crate::persistence::{Envelope, SAVED_THEME_LIMIT};
#[cfg(any(target_arch = "wasm32", test))]
use rustify_ui::theme::THEME_INPUT_LIMIT;
use rustify_ui::theme::{
    export_css, export_json, import_css, import_json, ColorFormat, CssProfile, ResolveContext,
    ThemeDiagnostic, ThemeDocument, ThemeError, ThemeMode,
};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq)]
pub enum ToolCommand {
    Import(ThemeDocument),
    LoadLocal(String),
    Save {
        name: String,
        overwrite: Option<String>,
    },
    Copy {
        id: String,
        name: String,
    },
    Rename {
        id: String,
        name: String,
    },
    Delete(String),
    ClearCache,
    LoadExternal,
    KeepCurrent,
    SaveConflictCopy {
        name: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
struct Change {
    mode: Option<ThemeMode>,
    token: String,
    before: Option<String>,
    after: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct ImportPreview {
    base: ThemeDocument,
    document: ThemeDocument,
    changes: Vec<Change>,
    diagnostics: Vec<ThemeDiagnostic>,
}

fn error(field: &str, message: &str) -> ThemeError {
    ThemeError {
        field: field.into(),
        message: message.into(),
    }
}

fn changes(before: &ThemeDocument, after: &ThemeDocument) -> Vec<Change> {
    let mut changes = Vec::new();
    for (token, before, after) in [
        ("id", &before.id, &after.id),
        ("name", &before.name, &after.name),
    ] {
        if before != after {
            changes.push(Change {
                mode: None,
                token: token.into(),
                before: Some(before.clone()),
                after: Some(after.clone()),
            });
        }
    }
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        let before = &before.values(mode).0;
        let after = &after.values(mode).0;
        let keys: BTreeSet<_> = before.keys().chain(after.keys()).collect();
        for token in keys {
            if before.get(token) != after.get(token) {
                changes.push(Change {
                    mode: Some(mode),
                    token: token.clone(),
                    before: before.get(token).cloned(),
                    after: after.get(token).cloned(),
                });
            }
        }
    }
    changes
}

fn review_import(
    input: &str,
    kind: &str,
    base: &ThemeDocument,
    context: &ResolveContext<'_>,
) -> Result<ImportPreview, ThemeError> {
    let imported = match kind {
        "json" => import_json(input, context)?,
        "css" => import_css(input, base, context)?,
        _ => return Err(error("format", "choose JSON or CSS")),
    };
    Ok(ImportPreview {
        base: base.clone(),
        changes: changes(base, &imported.document),
        document: imported.document,
        diagnostics: imported.diagnostics,
    })
}

fn apply_import(
    preview: &ImportPreview,
    current: &ThemeDocument,
    context: &ResolveContext<'_>,
) -> Result<ToolCommand, ThemeError> {
    if current != &preview.base {
        return Err(error(
            "draft",
            "the current theme changed; review this import again",
        ));
    }
    export_json(&preview.document, context)?;
    Ok(ToolCommand::Import(preview.document.clone()))
}

#[derive(Clone, Debug, PartialEq)]
enum NameAction {
    Save,
    Copy(String),
    Rename(String),
    ConflictCopy,
}

#[derive(Clone, Debug, PartialEq)]
enum NamedIntent {
    Command(ToolCommand),
    Overwrite { id: String, name: String },
}

fn named_intent(
    envelope: &Envelope,
    document: &ThemeDocument,
    action: &NameAction,
    name: &str,
    context: &ResolveContext<'_>,
) -> Result<NamedIntent, ThemeError> {
    let mut identity = document.clone();
    identity.name = name.into();
    identity.validate_identity()?;
    let name = name.trim().to_owned();
    let renamed_id = match action {
        NameAction::Rename(id) => Some(id.as_str()),
        _ => None,
    };
    if let Some(existing) = envelope
        .saved_themes
        .iter()
        .find(|theme| theme.name.trim() == name && Some(theme.id.as_str()) != renamed_id)
    {
        if action == &NameAction::Save {
            export_json(document, context)?;
            return Ok(NamedIntent::Overwrite {
                id: existing.id.clone(),
                name,
            });
        }
        return Err(error(
            "name",
            "a saved theme already has this name; choose another name",
        ));
    }
    if !matches!(action, NameAction::Rename(_)) && envelope.saved_themes.len() >= SAVED_THEME_LIMIT
    {
        return Err(error(
            "saved_themes",
            "100 themes are already saved; delete one or overwrite a saved theme",
        ));
    }
    let command = match action {
        NameAction::Save => {
            export_json(document, context)?;
            ToolCommand::Save {
                name,
                overwrite: None,
            }
        }
        NameAction::ConflictCopy => {
            export_json(document, context)?;
            ToolCommand::SaveConflictCopy { name }
        }
        NameAction::Copy(id) | NameAction::Rename(id) => {
            if !envelope.saved_themes.iter().any(|theme| &theme.id == id) {
                return Err(error("id", "the saved theme no longer exists"));
            }
            if matches!(action, NameAction::Copy(_)) {
                ToolCommand::Copy {
                    id: id.clone(),
                    name,
                }
            } else {
                ToolCommand::Rename {
                    id: id.clone(),
                    name,
                }
            }
        }
    };
    Ok(NamedIntent::Command(command))
}

#[derive(Clone, Debug, PartialEq)]
struct ExportArtifact {
    text: String,
    name: &'static str,
    mime: &'static str,
}

fn export_artifact(
    document: &ThemeDocument,
    context: &ResolveContext<'_>,
    kind: &str,
    profile: &str,
    format: &str,
) -> Result<ExportArtifact, ThemeError> {
    match kind {
        "json" => Ok(ExportArtifact {
            text: export_json(document, context)?,
            name: "theme.json",
            mime: "application/json",
        }),
        "rust" => {
            export_json(document, context)?;
            Ok(ExportArtifact {
                text: rust_snippet().into(),
                name: "theme.rs",
                mime: "text/plain",
            })
        }
        "css" => {
            let profile = match profile {
                "standard" => CssProfile::TailwindV4,
                "scoped" => CssProfile::RustifyScoped,
                _ => return Err(error("profile", "unknown CSS profile")),
            };
            let format = match format {
                "hex" => ColorFormat::Hex,
                "rgb" => ColorFormat::Rgb,
                "hsl" => ColorFormat::Hsl,
                "oklch" => ColorFormat::Oklch,
                _ => return Err(error("format", "unknown color format")),
            };
            Ok(ExportArtifact {
                text: export_css(document, profile, context, format)?,
                name: "theme.css",
                mime: "text/css",
            })
        }
        _ => Err(error("format", "choose JSON, CSS or Rust")),
    }
}

fn rust_snippet() -> &'static str {
    r#"use leptos::prelude::*;
use rustify_ui::{mount, AppHandle, MountConfig, ResolvedTheme, ThemeScope};
use rustify_ui::theme::{font_context, import_json, resolve, ThemeError, ThemeMode};
use std::sync::Arc;

// Put the downloaded theme.json beside this source file.
// Include sdk.css and then theme-v4.css in one Tailwind v4 build.
// Copy the listed same-origin font files; CSS does not embed font bytes.
pub fn load_exported_theme(mode: ThemeMode) -> Result<Arc<ResolvedTheme>, ThemeError> {
    let context = font_context(16., false);
    let document = import_json(include_str!("theme.json"), &context)?.document;
    Ok(Arc::new(resolve(&document, mode, &context)?))
}

// A GPU region receives this same Arc<ResolvedTheme> through its typed props.
// Retain the returned AppHandle for as long as the preview is mounted.
pub fn mount_exported_theme(
    container: leptos::web_sys::HtmlElement,
) -> Result<AppHandle, String> {
    let initial = load_exported_theme(ThemeMode::Light).map_err(|error| error.to_string())?;
    mount(container, MountConfig {
        scope: "theme-preview".into(), url_owner: false, base: "/".into(),
    }, move || {
        let theme = RwSignal::new(initial);
        view! {
            <ThemeScope resolved=theme />
            <main class="bg-background text-foreground font-sans p-4 rounded-lg">
                "Your themed application"
            </main>
        }
    }).map_err(|error| error.to_string())
}
"#
}

#[cfg(target_arch = "wasm32")]
pub use browser::DataTools;

#[cfg(target_arch = "wasm32")]
mod browser {
    use super::*;
    use leptos::prelude::*;
    use rustify_components::{
        Button, ButtonVariant, Dialog, Select, SelectOption, TextArea, TextField,
    };
    use rustify_ui::{clipboard, files, Locale};

    #[derive(Clone, Copy, PartialEq)]
    enum Panel {
        Import,
        Export,
        Library,
    }

    #[derive(Clone, Debug)]
    enum Confirmation {
        Overwrite { id: String, name: String },
        Delete { id: String, name: String },
        Clear,
        LoadExternal,
    }

    #[derive(Clone, Copy, Default)]
    enum Feedback {
        #[default]
        None,
        Copying,
        Copied,
        CopyFailed,
        DownloadStarted,
    }

    fn tr(locale: Locale, en: &'static str, zh: &'static str) -> &'static str {
        match locale {
            Locale::English => en,
            Locale::Chinese => zh,
        }
    }

    fn download(artifact: &ExportArtifact, feedback: RwSignal<Feedback>, issue: RwSignal<String>) {
        match files::export(artifact.name, artifact.text.as_bytes(), artifact.mime) {
            Ok(()) => {
                issue.set(String::new());
                feedback.set(Feedback::DownloadStarted);
            }
            Err(error) => {
                feedback.set(Feedback::None);
                issue.set(format!("{error:?}"));
            }
        }
    }

    #[component]
    pub fn DataTools(
        #[prop(into)] document: Signal<ThemeDocument>,
        #[prop(into)] envelope: Signal<Envelope>,
        #[prop(into)] locale: Signal<Locale>,
        #[prop(into)] context: Signal<ResolveContext<'static>>,
        #[prop(into)] corrupt: Signal<Option<String>>,
        #[prop(into)] conflict: Signal<bool>,
        #[prop(into)] status: Signal<String>,
        on_command: impl Fn(ToolCommand) + Clone + Send + Sync + 'static,
    ) -> impl IntoView {
        let command = StoredValue::new(on_command);
        let panel = RwSignal::new(None::<Panel>);
        let source = RwSignal::new(String::new());
        let import_kind = RwSignal::new("json".to_owned());
        let import_kind_open = RwSignal::new(false);
        let preview = RwSignal::new(None::<ImportPreview>);
        let issue = RwSignal::new(String::new());
        let reading = RwSignal::new(false);
        let read_generation = RwSignal::new(0u64);
        let export_kind = RwSignal::new("json".to_owned());
        let export_kind_open = RwSignal::new(false);
        let profile = RwSignal::new("scoped".to_owned());
        let profile_open = RwSignal::new(false);
        let color_format = RwSignal::new("rgb".to_owned());
        let color_format_open = RwSignal::new(false);
        let output_node = NodeRef::<leptos::html::Textarea>::new();
        let feedback = RwSignal::new(Feedback::None);
        let name_action = RwSignal::new(None::<NameAction>);
        let theme_name = RwSignal::new(String::new());
        let confirmation = RwSignal::new(None::<Confirmation>);
        let output = Memo::new(move |_| {
            export_artifact(
                &document.get(),
                &context.get(),
                &export_kind.get(),
                &profile.get(),
                &color_format.get(),
            )
        });
        let open_panel = move |next| {
            issue.set(String::new());
            feedback.set(Feedback::None);
            preview.set(None);
            panel.set(Some(next));
        };
        let open_name = move |action, name| {
            issue.set(String::new());
            feedback.set(Feedback::None);
            theme_name.set(name);
            name_action.set(Some(action));
        };
        let review = move || {
            preview.set(None);
            issue.set(String::new());
            match review_import(
                &source.get_untracked(),
                &import_kind.get_untracked(),
                &document.get_untracked(),
                &context.get_untracked(),
            ) {
                Ok(candidate) => preview.set(Some(candidate)),
                Err(error) => issue.set(error.to_string()),
            }
        };
        let pick = move || {
            issue.set(String::new());
            preview.set(None);
            reading.set(true);
            read_generation.update(|value| *value += 1);
            let generation = read_generation.get_untracked();
            files::pick(
                files::Limits {
                    max_bytes: THEME_INPUT_LIMIT as u64,
                    kinds: &[".json", ".css"],
                },
                move |answer| {
                    if read_generation.try_get_untracked() != Some(generation) {
                        return;
                    }
                    reading.set(false);
                    if panel.get_untracked() != Some(Panel::Import) {
                        return;
                    }
                    match answer {
                        files::Import::Loaded { name, bytes } => match String::from_utf8(bytes) {
                            Ok(text) => {
                                import_kind.set(
                                    if name.to_ascii_lowercase().ends_with(".css") {
                                        "css"
                                    } else {
                                        "json"
                                    }
                                    .into(),
                                );
                                source.set(text);
                                review();
                            }
                            Err(_) => issue.set(
                                tr(
                                    locale.get_untracked(),
                                    "The file is not valid UTF-8. Choose a CSS or JSON text file.",
                                    "文件不是有效 UTF-8。请选择 CSS 或 JSON 文本文件。",
                                )
                                .into(),
                            ),
                        },
                        files::Import::Refused { why, .. } => issue.set(match why {
                            files::Refusal::TooLarge { .. } => tr(
                                locale.get_untracked(),
                                "The file exceeds 256 KiB. Choose a smaller file.",
                                "文件超过 256 KiB。请选择较小的文件。",
                            )
                            .into(),
                            files::Refusal::WrongKind { .. } => tr(
                                locale.get_untracked(),
                                "Choose a .json or .css file.",
                                "请选择 .json 或 .css 文件。",
                            )
                            .into(),
                            files::Refusal::Unreadable => tr(
                                locale.get_untracked(),
                                "The file could not be read. Try another file or paste its text.",
                                "无法读取文件。请换一个文件，或粘贴文本。",
                            )
                            .into(),
                        }),
                        files::Import::Aborted => {}
                    }
                },
            );
        };
        let apply = move || {
            if let Some(candidate) = preview.get_untracked() {
                match apply_import(
                    &candidate,
                    &document.get_untracked(),
                    &context.get_untracked(),
                ) {
                    Ok(next) => {
                        command.with_value(|callback| callback(next));
                        panel.set(None);
                        preview.set(None);
                    }
                    Err(error) => issue.set(error.to_string()),
                }
            }
        };
        let submit_name = move || {
            if let Some(action) = name_action.get_untracked() {
                match named_intent(
                    &envelope.get_untracked(),
                    &document.get_untracked(),
                    &action,
                    &theme_name.get_untracked(),
                    &context.get_untracked(),
                ) {
                    Ok(NamedIntent::Command(next)) => {
                        command.with_value(|callback| callback(next));
                        name_action.set(None);
                    }
                    Ok(NamedIntent::Overwrite { id, name }) => {
                        name_action.set(None);
                        confirmation.set(Some(Confirmation::Overwrite { id, name }));
                    }
                    Err(error) => issue.set(error.to_string()),
                }
            }
        };
        let confirm = move || {
            let next = match confirmation.get_untracked() {
                Some(Confirmation::Overwrite { id, name }) => ToolCommand::Save {
                    name,
                    overwrite: Some(id),
                },
                Some(Confirmation::Delete { id, .. }) => ToolCommand::Delete(id),
                Some(Confirmation::Clear) => ToolCommand::ClearCache,
                Some(Confirmation::LoadExternal) => ToolCommand::LoadExternal,
                None => return,
            };
            command.with_value(|callback| callback(next));
            confirmation.set(None);
        };

        view! {
            <section class="data-tools" data-testid="data-tools">
                <div class="tools-actions">
                    <Button on_click=move || open_name(NameAction::Save, document.get_untracked().name) test_id="theme-save">
                        {move || tr(locale.get(), "Save locally", "保存到本机")}
                    </Button>
                    <Button variant=ButtonVariant::Outline on_click=move || open_panel(Panel::Import) test_id="theme-import">
                        {move || tr(locale.get(), "Import", "导入")}
                    </Button>
                    <Button variant=ButtonVariant::Outline on_click=move || open_panel(Panel::Export) test_id="theme-export">
                        {move || tr(locale.get(), "Export", "导出")}
                    </Button>
                    <Button variant=ButtonVariant::Ghost on_click=move || open_panel(Panel::Library) test_id="theme-library">
                        {move || format!("{} ({})", tr(locale.get(), "Local themes", "本地主题"), envelope.get().saved_themes.len())}
                    </Button>
                </div>
                <p role="status" class="tools-status" data-testid="storage-status">{move || status.get()}</p>
                <Show when=move || corrupt.get().is_some() fallback=|| ()>
                    <div class="tools-notice" data-testid="corrupt-cache">
                        <p>{move || tr(locale.get(), "Stored data could not be restored. It is preserved; download it before clearing this site's cache.", "无法恢复缓存。原数据已保留；清除本站缓存前可以先下载。")}</p>
                        <div class="tools-actions">
                            <Button variant=ButtonVariant::Outline on_click=move || {
                                if let Some(raw) = corrupt.get_untracked() { download(&ExportArtifact { text: raw, name: "theme-studio-cache.txt", mime: "text/plain" }, feedback, issue); }
                            } test_id="cache-download">{move || tr(locale.get(), "Download original data", "下载原数据")}</Button>
                            <Button variant=ButtonVariant::Ghost on_click=move || { issue.set(String::new()); confirmation.set(Some(Confirmation::Clear)); } test_id="cache-clear">
                                {move || tr(locale.get(), "Clear this site's cache", "清除本站缓存")}
                            </Button>
                        </div>
                    </div>
                </Show>
                <Show when=move || conflict.get() fallback=|| ()>
                    <div class="tools-notice" data-testid="storage-conflict">
                        <p>{move || tr(locale.get(), "Another tab saved a different version. Your current draft is unchanged.", "另一个标签页保存了不同版本。当前草稿保持不变。")}</p>
                        <div class="tools-actions">
                            <Button variant=ButtonVariant::Outline on_click=move || { issue.set(String::new()); confirmation.set(Some(Confirmation::LoadExternal)); } test_id="conflict-load">
                                {move || tr(locale.get(), "Load other version", "载入另一版本")}
                            </Button>
                            <Button variant=ButtonVariant::Ghost on_click=move || command.with_value(|callback| callback(ToolCommand::KeepCurrent)) test_id="conflict-keep">
                                {move || tr(locale.get(), "Keep current draft", "保留当前草稿")}
                            </Button>
                            <Button variant=ButtonVariant::Outline on_click=move || open_name(NameAction::ConflictCopy, String::new()) test_id="conflict-copy">
                                {move || tr(locale.get(), "Save current as a copy", "将当前草稿另存为副本")}
                            </Button>
                        </div>
                    </div>
                </Show>
                <Show when=move || !issue.get().is_empty() && panel.get().is_none() && name_action.get().is_none() fallback=|| ()>
                    <p class="tools-error" role="alert" data-testid="tools-error">{move || format!("{} {}", tr(locale.get(), "Action failed:", "操作失败："), issue.get())}</p>
                </Show>
                <p class="tools-status" role="status" data-testid="tools-feedback">{move || match feedback.get() {
                    Feedback::None => "",
                    Feedback::Copying => tr(locale.get(), "Copying…", "正在复制…"),
                    Feedback::Copied => tr(locale.get(), "Copied to clipboard.", "已复制到剪贴板。"),
                    Feedback::CopyFailed => tr(locale.get(), "Clipboard unavailable or refused. Select the text or download it.", "剪贴板不可用或被拒绝。请选择文本复制，或下载文件。"),
                    Feedback::DownloadStarted => tr(locale.get(), "Download started. Check your browser's downloads.", "已发起下载。请查看浏览器下载记录。"),
                }}</p>
            </section>

            <Dialog open=Signal::derive(move || panel.get().is_some()) on_open_change=move |open| {
                if !open { panel.set(None); preview.set(None); read_generation.update(|value| *value += 1); reading.set(false); }
            } title=Signal::derive(move || match panel.get() {
                Some(Panel::Import) => tr(locale.get(), "Import theme", "导入主题"),
                Some(Panel::Export) => tr(locale.get(), "Export theme", "导出主题"),
                _ => tr(locale.get(), "Local themes", "本地主题"),
            }.to_owned()) class="tools-dialog" test_id="data-tools-dialog">
                <Show when=move || panel.get()==Some(Panel::Import) fallback=|| ()>
                    <div class="tools-stack">
                        <p>{move || tr(locale.get(), "Paste or choose one CSS / JSON file, up to 256 KiB. Review changes before applying.", "粘贴或选择一个 CSS / JSON 文件，上限 256 KiB。应用前请核对变更。")}</p>
                        <div class="tools-form-row">
                            <label for="tools-import-format">{move || tr(locale.get(), "Input format", "输入格式")}</label>
                            <Select id="tools-import-format" value=import_kind options=Signal::derive(|| vec![SelectOption::new("json","Theme JSON"), SelectOption::new("css","CSS")])
                                open=import_kind_open on_open_change=move |open| import_kind_open.set(open)
                                on_change=move |kind| { import_kind.set(kind); preview.set(None); issue.set(String::new()); read_generation.update(|value| *value += 1); reading.set(false); } test_id="import-format" />
                            <Button variant=ButtonVariant::Outline on_click=pick disabled=reading test_id="import-file">
                                {move || if reading.get() { tr(locale.get(), "Reading file…", "正在读取…") } else { tr(locale.get(), "Choose file", "选择文件") }}
                            </Button>
                        </div>
                        <label class="tools-stack"><span>{move || tr(locale.get(), "Theme source", "主题源码")}</span>
                            <TextArea value=source on_change=move |text| { source.set(text); preview.set(None); issue.set(String::new()); read_generation.update(|value| *value += 1); reading.set(false); }
                                rows=10 class="tools-code" test_id="import-source" />
                        </label>
                        <p class="tools-status">{move || format!("{} / {} {}", source.get().len(), THEME_INPUT_LIMIT, tr(locale.get(),"bytes","字节"))}</p>
                        <Show when=move || !issue.get().is_empty() fallback=|| ()>
                            <p class="tools-error" role="alert" data-testid="import-error">{move || format!("{} {}", tr(locale.get(), "Import failed:", "导入失败："), issue.get())}</p>
                        </Show>
                        <Button variant=ButtonVariant::Outline on_click=review disabled=Signal::derive(move || reading.get() || source.get().is_empty()) test_id="import-review">
                            {move || tr(locale.get(), "Review changes", "预览变更")}
                        </Button>
                        {move || preview.get().map(|candidate| {
                        let no_changes = candidate.changes.is_empty();
                            let has_diagnostics = !candidate.diagnostics.is_empty();
                            view! {
                            <div class="tools-stack" data-testid="import-preview">
                                <p>{format!("{} · {} {}", candidate.document.name, candidate.changes.len(), tr(locale.get(), "changes", "项变更"))}</p>
                                <Show when=move || no_changes fallback=|| ()>
                                    <p>{move || tr(locale.get(), "No author values differ from the current theme.", "作者值与当前主题相同。")}</p>
                                </Show>
                                <div class="tools-scroll">
                                    <table class="tools-diff"><thead><tr>
                                        <th>{move || tr(locale.get(), "Mode / token", "模式 / token")}</th>
                                        <th>{move || tr(locale.get(), "Current", "当前")}</th>
                                        <th>{move || tr(locale.get(), "Imported", "导入值")}</th>
                                    </tr></thead><tbody>
                                        {candidate.changes.iter().map(|change| view! {
                                            <tr><th scope="row">{format!("{} / {}", change.mode.map(ThemeMode::as_str).unwrap_or("identity"), change.token)}</th>
                                                <td>{change.before.clone().unwrap_or_else(|| "—".into())}</td><td>{change.after.clone().unwrap_or_else(|| "—".into())}</td></tr>
                                        }).collect_view()}
                                    </tbody></table>
                                </div>
                                <Show when=move || has_diagnostics fallback=|| ()>
                                    <div class="tools-notice" data-testid="import-diagnostics">
                                        <p>{move || tr(locale.get(), "Import notes", "导入提示")}</p>
                                        <ul>{candidate.diagnostics.iter().map(|note| view! { <li>{format!("{}: {}", note.field, note.message)}</li> }).collect_view()}</ul>
                                    </div>
                                </Show>
                                <div class="tools-actions">
                                    <Button on_click=apply test_id="import-apply">{move || tr(locale.get(), "Apply import", "应用导入")}</Button>
                                    <Button variant=ButtonVariant::Ghost on_click=move || { panel.set(None); preview.set(None); } test_id="import-cancel">
                                        {move || tr(locale.get(), "Cancel", "取消")}
                                    </Button>
                                </div>
                            </div>
                        }})}
                    </div>
                </Show>
                <Show when=move || panel.get()==Some(Panel::Export) fallback=|| ()>
                    <div class="tools-stack">
                        <div class="tools-form-row">
                            <label for="tools-export-kind">{move || tr(locale.get(), "Output", "导出格式")}</label>
                            <Select id="tools-export-kind" value=export_kind options=Signal::derive(move || vec![SelectOption::new("json","Theme JSON"),SelectOption::new("css","Tailwind v4 CSS"),SelectOption::new("rust",tr(locale.get(),"Rust integration","Rust 接入片段"))])
                                open=export_kind_open on_open_change=move |open| export_kind_open.set(open)
                                on_change=move |kind| { export_kind.set(kind); feedback.set(Feedback::None); issue.set(String::new()); } test_id="export-kind" />
                        </div>
                        <Show when=move || export_kind.get()=="css" fallback=|| ()>
                            <div class="tools-form-row">
                                <label for="tools-export-profile">{move || tr(locale.get(), "CSS profile", "CSS 用途")}</label>
                                <Select id="tools-export-profile" value=profile options=Signal::derive(move || vec![SelectOption::new("scoped",tr(locale.get(),"Rustify scoped","Rustify 局部作用域")),SelectOption::new("standard",tr(locale.get(),"Standard Tailwind v4","标准 Tailwind v4"))])
                                    open=profile_open on_open_change=move |open| profile_open.set(open) on_change=move |value| profile.set(value) test_id="export-profile" />
                                <label for="tools-export-color">{move || tr(locale.get(), "Color notation", "颜色表达")}</label>
                                <Select id="tools-export-color" value=color_format options=Signal::derive(|| vec![SelectOption::new("hex","HEX"),SelectOption::new("rgb","RGB"),SelectOption::new("hsl","HSL"),SelectOption::new("oklch","OKLCH")])
                                    open=color_format_open on_open_change=move |open| color_format_open.set(open) on_change=move |value| color_format.set(value) test_id="export-color-format" />
                            </div>
                            <p class="tools-status">{move || tr(locale.get(), "CSS colors are mapped to sRGB. JSON retains author values. Fonts need the listed files; they are not embedded.", "CSS 颜色已映射到 sRGB。JSON 保留作者原值。字体需要列出的资源文件，未嵌入 CSS。")}</p>
                        </Show>
                        <Show when=move || export_kind.get()=="rust" fallback=|| ()>
                            <p class="tools-status">{move || tr(locale.get(), "Download the matching theme.json beside theme.rs. Keep AppHandle alive and pass the same resolved snapshot to your GPU region.", "将配套 theme.json 放在 theme.rs 旁。保留 AppHandle，并把同一解析快照传给 GPU region。")}</p>
                            <Button variant=ButtonVariant::Outline on_click=move || match export_artifact(&document.get_untracked(),&context.get_untracked(),"json","scoped","rgb") {
                                Ok(artifact) => download(&artifact,feedback,issue), Err(error) => issue.set(error.to_string())
                            } test_id="export-matching-json">{move || tr(locale.get(), "Download matching JSON", "下载配套 JSON")}</Button>
                        </Show>
                        <label class="tools-stack"><span>{move || tr(locale.get(), "Exported code", "导出代码")}</span>
                            <textarea node_ref=output_node class="tools-code tools-output" rows="14" readonly
                                prop:value=move || output.get().map(|artifact|artifact.text).unwrap_or_else(|error|error.to_string())
                                data-testid="export-source" />
                        </label>
                        <Show when=move || !issue.get().is_empty() fallback=|| ()>
                            <p class="tools-error" role="alert" data-testid="export-error">{move || format!("{} {}", tr(locale.get(), "Export failed:", "导出失败："), issue.get())}</p>
                        </Show>
                        <p class="tools-status" role="status" data-testid="export-feedback">{move || match feedback.get() {
                            Feedback::Copying => tr(locale.get(), "Copying…", "正在复制…"),
                            Feedback::Copied => tr(locale.get(), "Copied to clipboard.", "已复制到剪贴板。"),
                            Feedback::CopyFailed => tr(locale.get(), "Clipboard unavailable or refused. Select the text or download it.", "剪贴板不可用或被拒绝。请选择文本复制，或下载文件。"),
                            Feedback::DownloadStarted => tr(locale.get(), "Download started. Check your browser's downloads.", "已发起下载。请查看浏览器下载记录。"),
                            Feedback::None => "",
                        }}</p>
                        <div class="tools-actions">
                            <Button disabled=Signal::derive(move || output.get().is_err() || matches!(feedback.get(),Feedback::Copying))
                                on_click=move || if let Ok(artifact)=output.get_untracked() {
                                    feedback.set(Feedback::Copying);
                                    clipboard::copy(&artifact.text,move |result| feedback.set(if result.is_ok(){Feedback::Copied}else{Feedback::CopyFailed}));
                                } test_id="export-copy">{move || tr(locale.get(), "Copy code", "复制代码")}</Button>
                            <Button variant=ButtonVariant::Outline on_click=move || if let Some(node)=output_node.get_untracked(){ let _=node.focus(); node.select(); }
                                test_id="export-select">{move || tr(locale.get(), "Select text", "选择文本")}</Button>
                            <Button variant=ButtonVariant::Outline disabled=Signal::derive(move || output.get().is_err())
                                on_click=move || if let Ok(artifact)=output.get_untracked(){download(&artifact,feedback,issue);}
                                test_id="export-download">{move || tr(locale.get(), "Download", "下载")}</Button>
                        </div>
                    </div>
                </Show>
                <Show when=move || panel.get()==Some(Panel::Library) fallback=|| ()>
                    <div class="tools-stack">
                        <p class="tools-status">{move || tr(locale.get(), "Up to 100 themes on this browser. Data stays local; JSON downloads are your portable backup.", "此浏览器最多保存 100 个主题。数据仅保存在本机；JSON 下载可作为便携备份。")}</p>
                        <Show when=move || envelope.get().saved_themes.is_empty() fallback=|| ()>
                            <p data-testid="library-empty">{move || tr(locale.get(), "No local themes yet. Save the current theme to create your first one.", "暂无本地主题。保存当前主题即可创建第一个。")}</p>
                        </Show>
                        <ul class="tools-library">{move || envelope.get().saved_themes.into_iter().map(|theme| {
                            let name=theme.name.clone(); let copy_id=theme.id.clone(); let rename_id=theme.id.clone();
                            let selected=theme.id.clone();
                            let delete_id=theme.id.clone(); let rename_name=theme.name.clone(); let delete_name=theme.name.clone();
                            view! { <li data-theme-id=theme.id><strong>{name}</strong><div class="tools-actions">
                                <Button variant=ButtonVariant::Outline on_click=move || {
                                    command.with_value(|callback|callback(ToolCommand::LoadLocal(selected.clone())));
                                    panel.set(None);
                                } test_id="local-load">{move || tr(locale.get(),"Use theme","使用主题")}</Button>
                                <Button variant=ButtonVariant::Outline disabled=Signal::derive(move || envelope.get().saved_themes.len()>=SAVED_THEME_LIMIT)
                                    on_click=move || open_name(NameAction::Copy(copy_id.clone()),String::new()) test_id="local-copy">
                                    {move || tr(locale.get(),"Copy","复制")}
                                </Button>
                                <Button variant=ButtonVariant::Ghost on_click=move || open_name(NameAction::Rename(rename_id.clone()),rename_name.clone()) test_id="local-rename">
                                    {move || tr(locale.get(),"Rename","重命名")}
                                </Button>
                                <Button variant=ButtonVariant::Ghost on_click=move || confirmation.set(Some(Confirmation::Delete{id:delete_id.clone(),name:delete_name.clone()})) test_id="local-delete">
                                    {move || tr(locale.get(),"Delete","删除")}
                                </Button>
                            </div></li> }
                        }).collect_view()}</ul>
                        <Show when=move || {envelope.get().saved_themes.len() >= SAVED_THEME_LIMIT} fallback=|| ()>
                            <p class="tools-notice" data-testid="library-limit">{move || tr(locale.get(), "The 100-theme limit is reached. Rename or overwrite an existing theme, or delete one before creating a copy.", "已达到 100 个主题上限。可以重命名、覆盖现有主题，或删除一个后再复制。")}</p>
                        </Show>
                    </div>
                </Show>
                <p class="tools-status" role="status" data-testid="dialog-storage-status">{move || status.get()}</p>
            </Dialog>

            <Dialog open=Signal::derive(move || name_action.get().is_some()) on_open_change=move |open| if !open{name_action.set(None);}
                title=Signal::derive(move || match name_action.get() {
                    Some(NameAction::Copy(_)) => tr(locale.get(),"Copy local theme","复制本地主题"),
                    Some(NameAction::Rename(_)) => tr(locale.get(),"Rename local theme","重命名本地主题"),
                    Some(NameAction::ConflictCopy) => tr(locale.get(),"Save current draft as a copy","将当前草稿另存为副本"),
                    _ => tr(locale.get(),"Save theme locally","保存主题到本机"),
                }.to_owned()) test_id="theme-name-dialog">
                <label class="tools-stack"><span>{move || tr(locale.get(),"Theme name · 1–80 characters","主题名称 · 1–80 个字符")}</span>
                    <TextField value=theme_name on_change=move |value|{theme_name.set(value);issue.set(String::new());}
                        invalid=Signal::derive(move || !issue.get().is_empty()) test_id="theme-name" />
                </label>
                <p class="tools-status">{move || tr(locale.get(), "A matching name asks before overwriting. Copies need a new name.", "名称相同会先询问是否覆盖。副本需要新名称。")}</p>
                <Show when=move || !issue.get().is_empty() fallback=|| ()>
                    <p class="tools-error" role="alert" data-testid="name-error">{move || format!("{} {}",tr(locale.get(),"Cannot save:","无法保存："),issue.get())}</p>
                </Show>
                <div class="tools-actions">
                    <Button on_click=submit_name test_id="theme-name-confirm">{move || tr(locale.get(),"Continue","继续")}</Button>
                    <Button variant=ButtonVariant::Ghost on_click=move || name_action.set(None) test_id="theme-name-cancel">{move || tr(locale.get(),"Cancel","取消")}</Button>
                </div>
            </Dialog>

            <Dialog open=Signal::derive(move || confirmation.get().is_some()) on_open_change=move |open| if !open{confirmation.set(None);}
                title=Signal::derive(move || match confirmation.get() {
                    Some(Confirmation::Overwrite{..}) => tr(locale.get(),"Overwrite saved theme?","覆盖已保存主题？"),
                    Some(Confirmation::Delete{..}) => tr(locale.get(),"Delete saved theme?","删除已保存主题？"),
                    Some(Confirmation::LoadExternal) => tr(locale.get(),"Load another tab's version?","载入另一标签页的版本？"),
                    _ => tr(locale.get(),"Clear this site's cache?","清除本站缓存？"),
                }.to_owned()) test_id="theme-confirm-dialog">
                <p>{move || match confirmation.get() {
                    Some(Confirmation::Overwrite{name,..}) => format!("{}: {name}. {}",tr(locale.get(),"Replace","替换"),tr(locale.get(),"Its id is kept; its author values will be replaced by the current theme.","保留原 id，作者值将被当前主题替换。")),
                    Some(Confirmation::Delete{name,..}) => format!("{}: {name}. {}",tr(locale.get(),"Delete","删除"),tr(locale.get(),"The current draft stays available.","当前草稿仍可继续使用。")),
                    Some(Confirmation::LoadExternal) => tr(locale.get(),"This replaces your current draft with the stored version. Export your draft first if you need a backup.","此操作用缓存版本替换当前草稿。如需备份，请先导出草稿。").into(),
                    _ => tr(locale.get(),"Only this site's Theme Studio storage key is removed. Download the original data first if you need it.","仅删除本站 Theme Studio 的存储键。如需原数据，请先下载。").into(),
                }}</p>
                <div class="tools-actions">
                    <Button on_click=confirm test_id="theme-confirm">{move || tr(locale.get(),"Confirm","确认")}</Button>
                    <Button variant=ButtonVariant::Ghost on_click=move || confirmation.set(None) test_id="theme-confirm-cancel">{move || tr(locale.get(),"Cancel","取消")}</Button>
                </div>
            </Dialog>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::Preferences;
    use rustify_ui::{theme::font_context, Theme};

    fn base() -> ThemeDocument {
        ThemeDocument::from_legacy(Theme::light(), Theme::dark())
    }
    fn envelope() -> Envelope {
        Envelope {
            envelope_version: 1,
            draft: base(),
            saved_themes: Vec::new(),
            preferences: Preferences::default(),
        }
    }

    #[test]
    fn import_review_lists_identity_added_removed_and_both_mode_changes_without_mutating() {
        let context = font_context(16., false);
        let mut original = base();
        original
            .styles
            .light
            .0
            .insert("removed".into(), "author value".into());
        let mut candidate = base();
        candidate.id = "imported".into();
        candidate
            .styles
            .dark
            .0
            .insert("future-token".into(), "url(https://example.test)".into());
        candidate
            .styles
            .light
            .0
            .insert("primary".into(), "#abc".into());
        let preview = review_import(
            &export_json(&candidate, &context).unwrap(),
            "json",
            &original,
            &context,
        )
        .unwrap();
        assert_eq!(preview.document, candidate);
        assert_eq!(preview.base, original);
        assert_eq!(preview.changes.len(), 4);
        assert!(preview
            .changes
            .iter()
            .any(|change| change.token == "removed" && change.after.is_none()));
        assert!(preview
            .changes
            .iter()
            .any(|change| change.mode == Some(ThemeMode::Dark) && change.token == "future-token"));
        assert_eq!(
            apply_import(&preview, &original, &context).unwrap(),
            ToolCommand::Import(candidate)
        );
        let mut newer = original.clone();
        newer.name = "Another edit".into();
        assert!(apply_import(&preview, &newer, &context).is_err());
        assert_eq!(preview.base, original);
    }

    #[test]
    fn invalid_and_oversize_imports_never_produce_an_apply_candidate() {
        let context = font_context(16., false);
        let base = base();
        for input in [
            ":root {--primary: url(https://example.test)}".into(),
            ":root {--primary: #abc; } .dark {--radius: calc(1px)}".into(),
            " ".repeat(THEME_INPUT_LIMIT + 1),
        ] {
            assert!(review_import(&input, "css", &base, &context).is_err());
        }
        assert!(review_import("{}", "json", &base, &context).is_err());
        let preview = review_import(
            ":root {--primary:#abc;--future-token:#fff}",
            "css",
            &base,
            &context,
        )
        .unwrap();
        assert!(preview
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("future-token")));
        assert!(!preview.document.styles.light.0.contains_key("future-token"));
    }

    #[test]
    fn save_requires_separate_overwrite_intent_and_copy_rename_conflicts_are_rejected() {
        let context = font_context(16., false);
        let original = envelope();
        let saved = original
            .save_named(&original.draft, "local-a", "Alpha", None, &context)
            .unwrap();
        assert_eq!(
            named_intent(&saved, &base(), &NameAction::Save, " Alpha ", &context).unwrap(),
            NamedIntent::Overwrite {
                id: "local-a".into(),
                name: "Alpha".into()
            }
        );
        assert!(named_intent(
            &saved,
            &base(),
            &NameAction::Copy("local-a".into()),
            "Alpha",
            &context
        )
        .is_err());
        assert_eq!(
            named_intent(
                &saved,
                &base(),
                &NameAction::Rename("local-a".into()),
                "Alpha",
                &context
            )
            .unwrap(),
            NamedIntent::Command(ToolCommand::Rename {
                id: "local-a".into(),
                name: "Alpha".into()
            })
        );
        assert!(named_intent(
            &saved,
            &base(),
            &NameAction::Rename("missing".into()),
            "Beta",
            &context
        )
        .is_err());
        for name in ["", "   ", "bad\nname", &"中".repeat(81)] {
            assert!(named_intent(&saved, &base(), &NameAction::Save, name, &context).is_err());
        }
        assert_eq!(saved.saved_themes.len(), 1);
    }

    #[test]
    fn full_library_permits_rename_and_confirmed_overwrite_but_rejects_new_copy() {
        let context = font_context(16., false);
        let mut full = envelope();
        for index in 0..SAVED_THEME_LIMIT {
            let mut theme = base();
            theme.id = format!("local-{index}");
            theme.name = format!("Name {index}");
            full.saved_themes.push(theme);
        }
        assert!(matches!(
            named_intent(&full, &base(), &NameAction::Save, "Name 0", &context).unwrap(),
            NamedIntent::Overwrite { .. }
        ));
        assert!(named_intent(&full, &base(), &NameAction::Save, "New", &context).is_err());
        assert!(named_intent(&full, &base(), &NameAction::ConflictCopy, "New", &context).is_err());
        assert!(named_intent(
            &full,
            &base(),
            &NameAction::Rename("local-0".into()),
            "New",
            &context
        )
        .is_ok());
    }

    #[test]
    fn exports_route_profiles_color_formats_and_lossless_json_to_actual_sdk() {
        let context = font_context(16., false);
        let original = base();
        let json = export_artifact(&original, &context, "json", "scoped", "rgb").unwrap();
        assert_eq!(json.name, "theme.json");
        assert_eq!(
            import_json(&json.text, &context).unwrap().document,
            original
        );
        let standard = export_artifact(&original, &context, "css", "standard", "hsl").unwrap();
        assert!(standard.text.contains(":root"));
        assert!(standard.text.contains("--primary: hsl("));
        let scoped = export_artifact(&original, &context, "css", "scoped", "oklch").unwrap();
        assert!(scoped.text.contains("[data-rustify-scope]"));
        assert!(scoped.text.contains("--rustify-unit:"));
        let rust = export_artifact(&original, &context, "rust", "scoped", "rgb").unwrap();
        assert!(rust.text.contains("include_str!(\"theme.json\")"));
        assert!(rust.text.contains("<ThemeScope resolved=theme />"));
        assert!(!rust.text.contains("Theme {"));
        assert!(export_artifact(&original, &context, "css", "unknown", "rgb").is_err());
    }
}
