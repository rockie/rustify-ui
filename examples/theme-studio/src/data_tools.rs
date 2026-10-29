use crate::persistence::{Envelope, SAVED_THEME_LIMIT};
use rustify_ui::theme::THEME_INPUT_LIMIT;
use rustify_ui::theme::{
    export_css, export_json, export_json_v1_lossy, import_css, import_json, ColorFormat,
    CssProfile, LegacyExportLoss, ResolveContext, ThemeDiagnostic, ThemeDocument, ThemeError,
    ThemeMode,
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
    ReviewClearCache,
    ReviewExternal,
    ReviewConflict,
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
    source_version: Option<u32>,
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
    for ((field, before), (_, after)) in before
        .config
        .fields()
        .into_iter()
        .zip(after.config.fields())
    {
        if before != after {
            changes.push(Change {
                mode: None,
                token: format!("config.{field}"),
                before: Some(before),
                after: Some(after),
            });
        }
    }
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        for (layer, before, after) in [
            (
                "base",
                &before.base_values(mode).0,
                &after.base_values(mode).0,
            ),
            (
                "overrides",
                &before.overrides.values(mode).0,
                &after.overrides.values(mode).0,
            ),
        ] {
            let keys: BTreeSet<_> = before.keys().chain(after.keys()).collect();
            for token in keys {
                if before.get(token) != after.get(token) {
                    changes.push(Change {
                        mode: Some(mode),
                        token: format!("{layer}.{token}"),
                        before: before.get(token).cloned(),
                        after: after.get(token).cloned(),
                    });
                }
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
    if kind == "json" && input.len() <= THEME_INPUT_LIMIT {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(input) {
            if let Some(version) = value.get("schema_version") {
                if !matches!(version.as_u64(), Some(1 | 2)) {
                    return Err(error("schema_version", &version.to_string()));
                }
            }
        }
    }
    let imported = match kind {
        "json" => import_json(input, context)?,
        "css" => import_css(input, base, context)?,
        _ => return Err(error("format", "choose JSON or CSS")),
    };
    Ok(ImportPreview {
        source_version: if kind == "json" {
            serde_json::from_str::<serde_json::Value>(input)
                .ok()
                .and_then(|value| value["schema_version"].as_u64())
                .map(|value| value as u32)
        } else {
            None
        },
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
    losses: Vec<LegacyExportLoss>,
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
            losses: Vec::new(),
        }),
        "json-v1" => {
            let legacy = export_json_v1_lossy(document, context)?;
            Ok(ExportArtifact {
                text: legacy.json,
                name: "theme-v1.json",
                mime: "application/json",
                losses: legacy.losses,
            })
        }
        "rust" => {
            export_json(document, context)?;
            Ok(ExportArtifact {
                text: format!(
                    "// Source SDK/CSS revision (source CLI only for Docs): {}\n{}",
                    env!("RUSTIFY_STUDIO_SOURCE_REVISION"),
                    rust_snippet()
                ),
                name: "theme.rs",
                mime: "text/plain",
                losses: Vec::new(),
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
                losses: Vec::new(),
            })
        }
        _ => Err(error("format", "choose JSON, CSS or Rust")),
    }
}

fn preview_call(context: &ResolveContext<'_>, mode: ThemeMode) -> String {
    format!(
        "mount_exported_preview(container, config, ThemeMode::{mode:?}, &font_context({:?}, {}))",
        context.rem_px, context.reduce_motion
    )
    .replace(".0,", ".,")
}

fn integration_notes(
    document: &ThemeDocument,
    context: &ResolveContext<'_>,
    preview_mode: ThemeMode,
) -> Result<ExportArtifact, ThemeError> {
    let static_id = document
        .id
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    let mut text = format!(
        "Rustify Theme setup / 主题接入说明\nSource revision / 源码修订: {}\n\n\
        JSON v2 / Rust: use the SDK and CSS from this source checkout. Published 0.2.1 SDK/CLI reject complete v2 documents.\n\
        JSON v2 / Rust：使用此修订的源码 SDK 与 CSS，已发布的 0.2.1 SDK/CLI 不支持完整 v2。\n\
        Only for Docs / 仅 Docs 路径需要：build the source CLI from the same checkout / 从同一 checkout 构建源码 CLI：\n\
        mbx build --locked -p rustify-docs-cli --profile docs-cli\n\
        Docs author dependencies rustify-docs, rustify-dom and rustify-components must use canonical paths in that checkout.\n\
        Docs 作者项目的上述三个依赖须使用同一 checkout 的规范路径。\n\n\
        CSS: both resolved modes; static CSS needs no wasm or CLI. Compile Tailwind v4 exports with Tailwind 4.1.13.\n\
        CSS：包含两模式解析结果；静态消费无需 wasm 或 CLI。Tailwind v4 输出使用 Tailwind 4.1.13 编译。\n\
        Rustify scoped CSS: import sdk.css, then theme-v4.css and the exported theme.css in one Tailwind input.\n\
        Rustify 局部 CSS：在同一 Tailwind input 中依次导入 sdk.css、theme-v4.css 和导出的 theme.css。\n\
        Static root / 静态根：\n\
        <section data-rustify-scope=\"theme-preview\" data-rustify-theme-css=\"{}\">...</section>\n\
        Both attributes are required on the same element. / 两个属性必须放在同一元素上。\n\
        Scoped dark: add data-theme=\"dark\" to that root; Tailwind standard dark: .dark. CSS does not preserve author intent; keep JSON v2.\n\
        局部深色给该根元素添加 data-theme=\"dark\"，标准深色使用 .dark；CSS 不保留作者意图，请保留 JSON v2。\n\
        Compatible JSON v1 is lossy; review its losses before using it. / 兼容 JSON v1 有损，使用前请核对损失。\n\n\
        Preview / 当前预览: {preview_mode:?}; author appearance / 作者外观: {:?}\n\
        Put theme.json beside the downloaded theme.rs. / 将 theme.json 放在下载的 theme.rs 旁边。\n\
        {}\n\
        container: your application's HtmlElement; config: MountConfig with a unique scope, url_owner and deployment base.\n\
        container 是应用挂载节点；config 是 MountConfig，设置独立 scope、url_owner 和部署 base。\n\
        Set MountConfig.base to your deployment path (e.g. /tools/demo/). Retain AppHandle while mounted.\n\
        MountConfig.base 使用实际部署路径（如 /tools/demo/），挂载期间保持 AppHandle 存活。\n\
        font_context uses the host rem size and motion preference; use load_exported_theme_with_context for a custom font resolver.\n\
        font_context 使用宿主 rem 大小与动效偏好；自定义字体解析使用 load_exported_theme_with_context。\n\
        mount_exported_theme honors author appearance; mount_exported_preview selects a mode without changing JSON/history.\n\
        mount_exported_theme 遵守作者外观；mount_exported_preview 显式预览，不改 JSON 或历史。\n\n\
        Fonts / 外部字体（requested / 请求 → resolved / 有效；resource / 资源）:\n",
        env!("RUSTIFY_STUDIO_SOURCE_REVISION"),
        static_id,
        document.config.appearance,
        preview_call(context, preview_mode),
    );
    let mut resources = BTreeSet::new();
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        let theme = rustify_ui::theme::resolve(document, mode, context)?;
        for (slot, font) in ["sans", "serif", "mono"].into_iter().zip(&theme.fonts) {
            text.push_str(&format!(
                "{mode:?} {slot}: {} → {}; {}\n",
                font.requested.join(", "),
                font.family,
                font.resource_path
                    .as_deref()
                    .unwrap_or("system / non-packaged font · 系统/非打包字体"),
            ));
            if let Some(path) = &font.resource_path {
                resources.insert(path.clone());
            }
        }
    }
    if !resources.is_empty() {
        for face in rustify_ui::theme::FONT_GLYPH_FALLBACKS {
            resources.insert(face.resource_path.into());
        }
    }
    text.push_str("\nCopy resources / 复制资源（deduplicated / 已去重）:\n");
    for path in resources {
        text.push_str(&format!("{path}\n"));
    }
    text.push_str("Font files are external; this download contains instructions, not fonts. Copy the listed files and distribute their original licenses.\n字体文件外置，本下载仅含说明；请复制所列文件并分发原许可。\nLicense source / 许可源文件: makepad/widgets/resources/FONT-LICENSES.md (in the source checkout, not a deployed URL / 位于源码 checkout，非部署 URL).\nFont URLs are relative to the deployed CSS file; preserve the resource paths or adjust URLs for your base/CDN.\n字体 URL 相对于部署后的 CSS 文件；保留资源路径或按实际 base/CDN 调整 URL。\nGuides / 指南: docs/themes.md, docs/dom-components-migration.md; full compiled consumer / 完整编译消费者: tests/theme-export/README.md.\n");
    Ok(ExportArtifact {
        text,
        name: "theme-setup.txt",
        mime: "text/plain",
        losses: Vec::new(),
    })
}

fn rust_snippet() -> &'static str {
    r#"use leptos::prelude::*;
use rustify_ui::{mount, AppHandle, MountConfig, ResolvedTheme, ThemeScope};
use rustify_ui::theme::{font_context, import_json, resolve, ResolveContext, ThemeError, ThemeMode};
use std::sync::Arc;

// Put the downloaded theme.json beside this source file.
// Include sdk.css and then theme-v4.css in one Tailwind v4 build.
// Copy the listed same-origin font files and their licenses; fonts are external.
// Use the matching source SDK/CSS. The published 0.2.1 SDK and CLI reject v2.
pub fn load_exported_theme(mode: ThemeMode) -> Result<Arc<ResolvedTheme>, ThemeError> {
    load_exported_theme_with_context(mode, &font_context(16., false))
}

// Use your host's root rem size, motion preference and font context.
pub fn load_exported_theme_with_context(
    mode: ThemeMode,
    context: &ResolveContext<'_>,
) -> Result<Arc<ResolvedTheme>, ThemeError> {
    let document = import_json(include_str!("theme.json"), context)?.document;
    Ok(Arc::new(resolve(&document, mode, context)?))
}

// A GPU region receives this same Arc<ResolvedTheme> through its typed props.
// Retain the returned AppHandle for as long as the application is mounted.
// This ordinary provider honors the document's authored appearance.
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

// To match Studio's explicit preview, pass its selected mode here. This does
// not change the document's appearance or either mode's author values.
// Set MountConfig.base to the deployment base, including any subpath.
pub fn mount_exported_preview(
    container: leptos::web_sys::HtmlElement,
    config: MountConfig,
    mode: ThemeMode,
    context: &ResolveContext<'_>,
) -> Result<AppHandle, String> {
    let initial = load_exported_theme_with_context(mode, context)
        .map_err(|error| error.to_string())?;
    mount(container, config, move || {
        let theme = RwSignal::new(initial);
        view! {
            <ThemeScope resolved=theme preview_mode=Signal::derive(move || mode) />
            <main class="bg-background text-foreground font-sans p-4 rounded-lg">
                "Your themed preview"
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
    use leptos::{portal::Portal, prelude::*};
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

    fn config_label(field: &str, locale: Locale) -> &'static str {
        match field {
            "appearance" => tr(locale, "Appearance", "作者外观"),
            "accent_color" => tr(locale, "Brand color", "品牌色"),
            "gray_color" => tr(locale, "Neutral color", "中性色"),
            "panel_background" => tr(locale, "Panel background", "面板背景"),
            "radius" => tr(locale, "Radius", "圆角"),
            _ => tr(locale, "Scaling", "缩放"),
        }
    }

    fn config_value(field: &str, value: &str, locale: Locale) -> String {
        if field == "scaling" {
            return format!("{value}%");
        }
        if locale == Locale::Chinese {
            let color = match value {
                "gray" => Some("灰色"),
                "gold" => Some("金色"),
                "bronze" => Some("青铜色"),
                "brown" => Some("棕色"),
                "yellow" => Some("黄色"),
                "amber" => Some("琥珀色"),
                "orange" => Some("橙色"),
                "tomato" => Some("番茄红"),
                "red" => Some("红色"),
                "ruby" => Some("宝石红"),
                "crimson" => Some("绯红色"),
                "pink" => Some("粉色"),
                "plum" => Some("梅紫色"),
                "purple" => Some("紫色"),
                "violet" => Some("紫罗兰色"),
                "iris" => Some("鸢尾紫"),
                "indigo" => Some("靛蓝色"),
                "blue" => Some("蓝色"),
                "cyan" => Some("青色"),
                "teal" => Some("青绿色"),
                "jade" => Some("翡翠绿"),
                "green" => Some("绿色"),
                "grass" => Some("草绿色"),
                "lime" => Some("青柠色"),
                "mint" => Some("薄荷色"),
                "sky" => Some("天蓝色"),
                "mauve" => Some("紫灰色"),
                "slate" => Some("蓝灰色"),
                "sage" => Some("灰绿色"),
                "olive" => Some("橄榄灰"),
                "sand" => Some("沙灰色"),
                _ => None,
            };
            if let Some(color) = color {
                return color.into();
            }
        }
        match value {
            "null" => tr(locale, "Custom", "自定义").into(),
            "inherit" => tr(locale, "Inherit", "继承").into(),
            "light" => tr(locale, "Light", "浅色").into(),
            "dark" => tr(locale, "Dark", "深色").into(),
            "auto" => tr(locale, "Auto", "自动").into(),
            "solid" => tr(locale, "Solid", "纯色").into(),
            "translucent" => tr(locale, "Translucent", "半透明").into(),
            "none" => tr(locale, "None", "无").into(),
            "small" => tr(locale, "Small", "小").into(),
            "medium" => tr(locale, "Medium", "中").into(),
            "large" => tr(locale, "Large", "大").into(),
            "full" => tr(locale, "Full", "全圆角").into(),
            _ => {
                let mut chars = value.chars();
                chars
                    .next()
                    .map(|first| first.to_uppercase().collect::<String>() + chars.as_str())
                    .unwrap_or_default()
            }
        }
    }

    fn byte_count(value: usize) -> String {
        let digits = value.to_string();
        let mut formatted = String::new();
        for (index, ch) in digits.chars().enumerate() {
            if index > 0 && (digits.len() - index).is_multiple_of(3) {
                formatted.push(',');
            }
            formatted.push(ch);
        }
        formatted
    }

    #[component]
    fn DiagnosticPath(field: String, locale: Locale) -> impl IntoView {
        let parts = field.splitn(3, '.').map(str::to_owned).collect::<Vec<_>>();
        match parts.as_slice() {
            [layer, mode, token] if matches!(mode.as_str(), "light" | "dark") => {
                let mode = if mode == "light" {
                    tr(locale, "Light", "浅色")
                } else {
                    tr(locale, "Dark", "深色")
                };
                view! {<span>{mode}" · "{layer.clone()}" · "<code class="tools-inline-token">{token.clone()}</code></span>}.into_any()
            }
            _ => view! {<span>{field}</span>}.into_any(),
        }
    }

    #[component]
    fn DocumentDiffRow(
        change: Change,
        #[prop(into)] prefix: String,
        identity: bool,
        locale: Locale,
        #[prop(into)] before_label: String,
        #[prop(into)] after_label: String,
    ) -> impl IntoView {
        let unchanged = change.before == change.after;
        let change_label = if unchanged {
            tr(locale, "Unchanged", "未变更")
        } else {
            match (&change.before, &change.after) {
                (None, _) => tr(locale, "Added", "新增"),
                (_, None) => tr(locale, "Removed", "删除"),
                _ => tr(locale, "Changed", "修改"),
            }
        };
        let field = if identity {
            match change.token.as_str() {
                "source" => tr(locale, "Source", "来源").to_owned(),
                "name" => tr(locale, "Name", "名称").to_owned(),
                "id" => "id".into(),
                _ => change
                    .token
                    .strip_prefix("config.")
                    .map(|field| config_label(field, locale).to_owned())
                    .unwrap_or_else(|| change.token.clone()),
            }
        } else {
            change
                .token
                .strip_prefix(&prefix)
                .unwrap_or(&change.token)
                .to_owned()
        };
        let plain_value = identity && change.token != "id";
        let source_cell = identity && change.token == "source";
        let before = change.before.unwrap_or_else(|| "—".into());
        let after = change.after.unwrap_or_else(|| "—".into());
        view! {
            <tr class="tools-diff-row">
                <th scope="row">{if identity {view!{<span>{field}</span>}.into_any()}else{view!{<code>{field}</code>}.into_any()}}</th>
                <td class="tools-diff-kind">{if unchanged {view!{<span class="tools-change-unchanged">{change_label}</span>}.into_any()}else{view!{<span class="tools-change-label">{change_label}</span>}.into_any()}}</td>
                <td data-label=before_label>{if plain_value {view!{<span>{before}</span>}.into_any()}else{view!{<code>{before}</code>}.into_any()}}</td>
                <td data-label=after_label data-testid=move || if source_cell { "import-version" } else { "" }>{if plain_value {view!{<span>{after}</span>}.into_any()}else{view!{<code>{after}</code>}.into_any()}}</td>
            </tr>
        }
    }

    #[component]
    fn DocumentDiff(
        before: ThemeDocument,
        after: ThemeDocument,
        locale: Locale,
        #[prop(into)] before_label: String,
        #[prop(into)] after_label: String,
        #[prop(optional)] import_source: Option<(String, String)>,
    ) -> impl IntoView {
        let difference = changes(&before, &after);
        let import_identity = import_source.map(|(current, imported)| {
            let mut rows = vec![Change {
                mode: None,
                token: "source".into(),
                before: Some(current),
                after: Some(imported),
            }];
            for (token, before, after) in [
                ("id", &before.id, &after.id),
                ("name", &before.name, &after.name),
            ] {
                rows.push(Change {
                    mode: None,
                    token: token.into(),
                    before: Some(before.clone()),
                    after: Some(after.clone()),
                });
            }
            for ((field, before), (_, after)) in before
                .config
                .fields()
                .into_iter()
                .zip(after.config.fields())
            {
                rows.push(Change {
                    mode: None,
                    token: format!("config.{field}"),
                    before: Some(config_value(field, &before, locale)),
                    after: Some(config_value(field, &after, locale)),
                });
            }
            rows
        });
        let has_import = import_identity.is_some();
        let groups = [
            (None, "", tr(locale, "Identity", "名称与标识")),
            (None, "config.", tr(locale, "Configuration", "基础配置")),
            (
                Some(ThemeMode::Light),
                "base.",
                tr(locale, "Light · base", "浅色 · 基础值"),
            ),
            (
                Some(ThemeMode::Light),
                "overrides.",
                tr(locale, "Light · overrides", "浅色 · 覆盖"),
            ),
            (
                Some(ThemeMode::Dark),
                "base.",
                tr(locale, "Dark · base", "深色 · 基础值"),
            ),
            (
                Some(ThemeMode::Dark),
                "overrides.",
                tr(locale, "Dark · overrides", "深色 · 覆盖"),
            ),
        ];
        let left_label = before_label.clone();
        let right_label = after_label.clone();
        view! {
            <div class="tools-diff-content">
                <table class="tools-diff"><thead><tr>
                    <th>{if has_import {tr(locale, "Field", "字段")} else {tr(locale, "Field / token", "字段 / token")}}</th>
                    <th>{tr(locale, "Change", "变更")}</th>
                    <th>{before_label}</th><th>{after_label}</th>
                </tr></thead>
                {import_identity.map(|rows| view! {
                    <tbody data-testid="import-config"><tr class="tools-diff-group"><th colspan="4">{format!("{} ({})",tr(locale,"Identity & configuration","标识与配置"),rows.len())}</th></tr>
                        {rows.into_iter().map(|change| view! {<DocumentDiffRow change=change prefix="" identity=true locale=locale before_label=left_label.clone() after_label=right_label.clone()/>}).collect_view()}
                    </tbody>
                })}
                {groups.into_iter().filter_map(|(mode, prefix, label)| {
                    if has_import && mode.is_none() { return None; }
                    let rows = difference.iter().filter(|change| change.mode == mode && if prefix.is_empty() { !change.token.contains('.') } else { change.token.starts_with(prefix) }).collect::<Vec<_>>();
                    if rows.is_empty() { return None; }
                    Some(view! {
                        <tbody><tr class="tools-diff-group"><th colspan="4">{format!("{label} ({})", rows.len())}</th></tr>
                            {rows.into_iter().map(|change| view! {<DocumentDiffRow change=change.clone() prefix=prefix identity=false locale=locale before_label=left_label.clone() after_label=right_label.clone()/>}).collect_view()}
                        </tbody>
                    })
                }).collect_view()}
                </table>
            </div>
        }
    }

    fn import_error_text(error: &ThemeError, locale: Locale) -> String {
        if error.field == "schema_version" {
            return format!(
                "{} {}. {}",
                tr(
                    locale,
                    "Unsupported Theme JSON version",
                    "不支持的 Theme JSON 版本"
                ),
                error.message,
                tr(
                    locale,
                    "This Studio reads v1 and v2.",
                    "此 Studio 支持 v1 和 v2。"
                )
            );
        }
        if error.field == "json" {
            return tr(locale, "This is not valid Theme JSON. Check its syntax and required fields; technical details are below.", "这不是有效的 Theme JSON。请检查语法和必填字段；技术详情见下方。").into();
        }
        tr(locale, "This theme could not be imported. Check its size and supported color values; the current theme is unchanged.", "无法导入此主题。请检查体积和支持的颜色值；当前主题保持不变。").into()
    }

    fn legacy_loss_text(
        loss: &LegacyExportLoss,
        locale: Locale,
        config: rustify_ui::ThemeConfig,
    ) -> String {
        let selected = |field, value: &str| config_value(field, value, locale);
        match loss.field {
            "config.accent_color" => format!("{} “{}”. {}", tr(locale, "Brand color", "品牌色族"), selected("accent_color", config.accent_color.map(|color|color.as_str()).unwrap_or("null")), tr(locale, "v1 keeps final colors, without palette selection.", "v1 只保留最终颜色，不保留色族选择。")),
            "config.gray_color" => format!("{} “{}”. {}", tr(locale, "Neutral color", "中性色族"), {let chosen=selected("gray_color",config.gray_color.map(|color|color.as_str()).unwrap_or("null"));if config.gray_color==Some(rustify_ui::GrayColor::Auto){let neutral=rustify_ui::GrayColor::Auto.family(config.accent_color.unwrap_or(rustify_ui::AccentColor::Indigo));format!("{chosen} ({})",config_value("gray_color",neutral.name(),locale))}else{chosen}}, tr(locale, "v1 cannot retain Auto pairing or the selected neutral family.", "v1 无法保留自动配灰关系或中性色族选择。")),
            "config.panel_background" => format!("{} “{}”. {}", tr(locale, "Panel background", "面板背景"), selected("panel_background",config.panel_background.map(|panel|panel.as_str()).unwrap_or("null")), tr(locale, "Independent panel and surface roles are flattened.", "独立面板与表面角色将被压平。")),
            "config.appearance" => format!("{} “{}”. {}", tr(locale, "Appearance intent", "外观意图"), selected("appearance",config.appearance.as_str()), tr(locale, "v1 stores two mode tables, without a mode-selection instruction.", "v1 仅保留两张模式表，不保留模式选择意图。")),
            "config.radius" => format!("{} “{}”. {}", tr(locale, "Radius", "圆角"), selected("radius", config.radius.map(|radius|radius.as_str()).unwrap_or("null")), tr(locale, "Full, thumb and component radius roles cannot be recovered.", "全圆角、thumb 与组件圆角角色无法恢复。")),
            "config.scaling" => format!("{} “{}%”. {}", tr(locale, "Scaling intent", "缩放意图"), config.scaling.percent(), tr(locale, "The scale is baked into resolved metrics; future scaling intent is not stored.", "缩放固化为最终度量，不再保留后续缩放意图。")),
            "sources" => tr(locale, "Override sources: final values replace base/configuration/override origins, original notation and requested font fallback intent.", "覆盖来源：最终值取代 base/config/overrides 来源、原始写法及请求的字体回退意图。").into(),
            "derived_roles" => tr(locale, "Derived roles: the legacy 49-token table cannot retain palette/alpha steps, panel roles, component scales or the complete v2 appearance.", "派生角色：旧 49-token 表无法保留色阶与透明阶、面板角色、组件尺度或完整 v2 外观。").into(),
            "shadows" => tr(locale, "Secondary shadow layers cannot be fully represented by v1's single set of shadow parameters.", "v1 的单组阴影参数无法完整表示次级阴影层。").into(),
            _ => loss.message.clone(),
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
        toolbar: NodeRef<leptos::html::Div>,
        library_toolbar: NodeRef<leptos::html::Div>,
        #[prop(into)] document: Signal<ThemeDocument>,
        #[prop(into)] envelope: Signal<Envelope>,
        #[prop(into)] locale: Signal<Locale>,
        #[prop(into)] context: Signal<ResolveContext<'static>>,
        #[prop(into)] corrupt: Signal<Option<String>>,
        #[prop(into)] conflict: Signal<bool>,
        #[prop(into)] external: Signal<Option<Option<String>>>,
        #[prop(into)] pending: Signal<usize>,
        #[prop(optional, into)] editor_epoch: Signal<u64>,
        #[prop(optional, into)] preview_mode: Signal<ThemeMode>,
        on_command: impl Fn(ToolCommand) + Clone + Send + Sync + 'static,
    ) -> impl IntoView {
        let command = StoredValue::new(on_command);
        let panel = RwSignal::new(None::<Panel>);
        let source = RwSignal::new(String::new());
        let import_kind = RwSignal::new("json".to_owned());
        let import_kind_open = RwSignal::new(false);
        let preview = RwSignal::new(None::<ImportPreview>);
        let review_epoch = RwSignal::new(0u64);
        let issue = RwSignal::new(String::new());
        let import_issue = RwSignal::new(None::<ThemeError>);
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
        let external_envelope = Memo::new(move |_| {
            external.get().flatten().and_then(|raw| {
                match crate::persistence::load(Some(&raw), &context.get()) {
                    crate::persistence::LoadResult::Valid(envelope) => Some(*envelope),
                    _ => None,
                }
            })
        });
        let open_panel = move |next| {
            issue.set(String::new());
            import_issue.set(None);
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
            import_issue.set(None);
            review_epoch.set(editor_epoch.get_untracked());
            match review_import(
                &source.get_untracked(),
                &import_kind.get_untracked(),
                &document.get_untracked(),
                &context.get_untracked(),
            ) {
                Ok(candidate) => preview.set(Some(candidate)),
                Err(error) => {
                    issue.set(error.to_string());
                    import_issue.set(Some(error));
                }
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
                                "The file exceeds 256 KiB. Choose a smaller file.",
                                "文件超过 256 KiB。请选择较小的文件。",
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
            if review_epoch.get_untracked() != editor_epoch.get_untracked() {
                issue.set(
                    tr(
                        locale.get(),
                        "The editor changed after review. Review this import again.",
                        "检查后编辑器已变化，请重新检查本次导入。",
                    )
                    .into(),
                );
                return;
            }
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

        let actions = move || {
            view! {
                    <div class="tools-actions">
                        <Button variant=Signal::derive(move ||if crate::storage_browser::writable(){ButtonVariant::Solid}else{ButtonVariant::Outline}) on_click=move || open_name(NameAction::Save, document.get_untracked().name) test_id="theme-save">
                            {move || tr(locale.get(), "Save locally", "保存到本机")}
                        </Button>
                        <Button variant=ButtonVariant::Outline on_click=move || open_panel(Panel::Import) test_id="theme-import">
                            {move || tr(locale.get(), "Import", "导入")}
                        </Button>
                        <Button variant=ButtonVariant::Outline on_click=move || open_panel(Panel::Export) test_id="theme-export">
                            {move || tr(locale.get(), "Export", "导出")}
                        </Button>
                    </div>
            }
        };
        let body_visible = move || {
            corrupt.get().is_some()
                || conflict.get()
                || (!issue.get().is_empty() && panel.get().is_none() && name_action.get().is_none())
                || !matches!(feedback.get(), Feedback::None)
        };
        view! {
            <section class="data-tools" data-testid="data-tools" hidden=move || !body_visible()>
                {move || toolbar.get().map(move |target| {
                    let target: leptos::web_sys::Element = target.into();
                    view! {<Portal mount=target>{actions()}</Portal>}
                })}
                {move || library_toolbar.get().map(move |target| {let target:leptos::web_sys::Element=target.into();view!{<Portal mount=target>
                        <Button variant=ButtonVariant::Ghost on_click=move || open_panel(Panel::Library) test_id="theme-library">
                            {move || format!("{} ({})", tr(locale.get(), "Local themes", "本地主题"), envelope.get().saved_themes.len())}
                        </Button>
                </Portal>}})}
                <Show when=move || corrupt.get().is_some() fallback=|| ()>
                    <div class="tools-notice" data-testid="corrupt-cache">
                        <p role="status" data-testid="storage-status">{move || tr(locale.get(), "Stored data could not be restored. It is preserved; download it before clearing this site's cache.", "无法恢复缓存。原数据已保留；清除本站缓存前可以先下载。")}</p>
                        <div class="tools-actions">
                            <Button variant=ButtonVariant::Outline on_click=move || {
                                if let Some(raw) = corrupt.get_untracked() { download(&ExportArtifact { text: raw, name: "theme-studio-cache.txt", mime: "text/plain", losses: Vec::new() }, feedback, issue); }
                            } test_id="cache-download">{move || tr(locale.get(), "Download original data", "下载原数据")}</Button>
                            <Button variant=ButtonVariant::Ghost on_click=move || { issue.set(String::new()); command.with_value(|callback| callback(ToolCommand::ReviewClearCache)); confirmation.set(Some(Confirmation::Clear)); } test_id="cache-clear">
                                {move || tr(locale.get(), "Clear this site's cache", "清除本站缓存")}
                            </Button>
                        </div>
                    </div>
                </Show>
                <Show when=move || conflict.get() fallback=|| ()>
                    <div class="tools-notice tools-state-warning" data-testid="storage-conflict">
                        <h3>{move || tr(locale.get(), "Choose which version to keep", "选择要保留的版本")}</h3>
                        <p data-testid="storage-status">{move || tr(locale.get(), "Another tab saved a different version; your draft is unchanged. Load replaces your draft and pending saves, Keep overwrites the other version, and Save a copy retains both.", "另一个标签页保存了不同版本；当前草稿保持不变。载入会替换当前草稿与待写操作，保留会覆盖另一版本，另存副本会保留两个主题。")}</p>
                        <div data-testid="storage-external">
                            <dl class="tools-summary" data-testid="storage-external-summary"
                                data-external-kind=move || if external_envelope.get().is_some(){"library"} else if external.get()==Some(None){"removed"} else {"invalid"}
                                data-external-revision=move || external_envelope.get().map(|value|value.revision.to_string())>
                                <dt>{move || tr(locale.get(), "This tab", "此标签页")}</dt>
                                <dd>{move || format!("{} {} · {}",tr(locale.get(),"Revision","修订"),envelope.get().revision, document.get().name)}</dd>
                                <dt>{move || tr(locale.get(), "Other tab", "另一标签页")}</dt>
                                <dd>{move || external_envelope.get().map(|value|format!("{} {} · {}",tr(locale.get(),"Revision","修订"),value.revision,value.draft.name)).unwrap_or_else(||if external.get()==Some(None){tr(locale.get(),"Another tab removed the v2 library. Keeping this draft creates a new library.","另一标签页移除了 v2 主题库。保留当前草稿会创建新库。").into()}else{tr(locale.get(),"Invalid external v2 library. Download the original data before deciding.","外部 v2 主题库无效，请先下载原始数据再决定。").into()})}</dd>
                                <dt>{move || tr(locale.get(), "Saved themes", "命名主题")}</dt>
                                <dd>{move || {let current=envelope.get().saved_themes.len();let other=external_envelope.get().map_or(0,|value|value.saved_themes.len());if current==other {format!("{current} ({})",tr(locale.get(),"unchanged","未变化"))}else{format!("{current} → {other}")}}}</dd>
                            </dl>
                            {move || external_envelope.get().map(|other|view! {
                                <details data-testid="storage-conflict-diff"><summary>{tr(locale.get(),"Draft differences","草稿差异")}</summary>
                                    <DocumentDiff before=document.get() after=other.draft locale=locale.get() before_label=tr(locale.get(),"This tab","此标签页") after_label=tr(locale.get(),"Other tab","另一标签页") />
                                </details>
                            })}
                        </div>
                        <Show when=move || pending.get() != 0><p class="tools-status">{move || format!("{} {}",pending.get(),tr(locale.get(),"pending actions retained. Loading the other version discards them.","项待处理操作已保留。载入另一版本将放弃这些操作。"))}</p></Show>
                        <div class="tools-actions">
                            <Button variant=ButtonVariant::Outline on_click=move || { issue.set(String::new()); command.with_value(|callback| callback(ToolCommand::ReviewExternal)); confirmation.set(Some(Confirmation::LoadExternal)); } test_id="conflict-load">
                                {move || tr(locale.get(), "Load other version", "载入另一版本")}
                            </Button>
                            <Button on_click=move || command.with_value(|callback| {callback(ToolCommand::ReviewConflict); callback(ToolCommand::KeepCurrent);}) test_id="conflict-keep">
                                {move || tr(locale.get(), "Keep current draft", "保留当前草稿")}
                            </Button>
                            <Button variant=ButtonVariant::Outline on_click=move || {command.with_value(|callback| callback(ToolCommand::ReviewConflict)); open_name(NameAction::ConflictCopy, String::new());} test_id="conflict-copy">
                                {move || tr(locale.get(), "Save current as a copy", "将当前草稿另存为副本")}
                            </Button>
                        </div>
                        <Show when=move || external.get().flatten().is_some()>
                            <details data-testid="storage-external-raw"><summary>{move || tr(locale.get(),"View stored text","查看存储原文")}</summary>
                                <textarea class="tools-code tools-raw" rows="8" readonly data-testid="storage-external-source" aria-label=move || tr(locale.get(),"Displayed external stored text","当前展示的外部存储原文") prop:value=move || {
                                    let raw=external.get().flatten().unwrap_or_default();
                                    serde_json::from_str::<serde_json::Value>(&raw).ok().and_then(|value|serde_json::to_string_pretty(&value).ok()).unwrap_or(raw)
                                }/>
                            </details>
                            <Button variant=ButtonVariant::Outline test_id="storage-external-download" on_click=move || {
                                if let Some(raw)=external.get_untracked().flatten(){download(&ExportArtifact{text:raw,name:"theme-studio-other-version.json",mime:"application/json",losses:Vec::new()},feedback,issue);}
                            }>{move || tr(locale.get(),"Download other version","下载另一版本原文")}</Button>
                        </Show>
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
                        <p>{move || tr(locale.get(), "Paste or choose one CSS / JSON file, up to 262,144 bytes (256 KiB). Review changes before applying.", "粘贴或选择一个 CSS / JSON 文件，上限 262,144 字节（256 KiB）。应用前请核对变更。")}</p>
                        <div class="tools-form-row">
                            <label for="tools-import-format">{move || tr(locale.get(), "Input format", "输入格式")}</label>
                            <Select id="tools-import-format" value=import_kind options=Signal::derive(|| vec![SelectOption::new("json","Theme JSON"), SelectOption::new("css","CSS")])
                                open=import_kind_open on_open_change=move |open| import_kind_open.set(open)
                                on_change=move |kind| { import_kind.set(kind); preview.set(None); issue.set(String::new()); import_issue.set(None); read_generation.update(|value| *value += 1); reading.set(false); } test_id="import-format" />
                            <Button variant=ButtonVariant::Outline on_click=pick disabled=reading test_id="import-file">
                                {move || if reading.get() { tr(locale.get(), "Reading file…", "正在读取…") } else { tr(locale.get(), "Choose file", "选择文件") }}
                            </Button>
                        </div>
                        <label class="tools-stack"><span>{move || tr(locale.get(), "Theme source", "主题源码")}</span>
                            <TextArea value=source on_change=move |text| { source.set(text); preview.set(None); issue.set(String::new()); import_issue.set(None); read_generation.update(|value| *value += 1); reading.set(false); }
                                invalid=Signal::derive(move || !issue.get().is_empty()) described_by=Signal::derive(move || if issue.get().is_empty(){String::new()}else{"tools-import-error".into()})
                                rows=10 class="tools-code" test_id="import-source" />
                        </label>
                        <p class="tools-status">{move || format!("{} / {} {}", byte_count(source.get().len()),byte_count(THEME_INPUT_LIMIT),tr(locale.get(),"bytes","字节"))}</p>
                        <Show when=move || !issue.get().is_empty() fallback=|| ()>
                            <p class="tools-error" role="alert" id="tools-import-error" data-testid="import-error">{move || import_issue.get().as_ref().map(|error|import_error_text(error,locale.get())).unwrap_or_else(||issue.get())}</p>
                            <details class="tools-status"><summary>{move || tr(locale.get(),"Technical details","技术详情")}</summary><code>{move || issue.get()}</code></details>
                        </Show>
                        <Button variant=ButtonVariant::Outline on_click=review disabled=Signal::derive(move || reading.get() || source.get().is_empty()) test_id="import-review">
                            {move || tr(locale.get(), "Review changes", "预览变更")}
                        </Button>
                        {move || preview.get().map(|candidate| {
                            let no_changes = candidate.changes.is_empty();
                            let has_diagnostics = !candidate.diagnostics.is_empty();
                            let imported_source = candidate.source_version.map(|version| if version == 2 { "Theme JSON v2".into() } else { format!("Theme JSON v{version} · {} v2",tr(locale.get(),"saved as","保存为")) }).unwrap_or_else(|| format!("CSS · {} Custom v2",tr(locale.get(),"saved as","保存为")));
                            view! {
                            <div class="tools-stack" data-testid="import-preview">
                                <h3>{tr(locale.get(),"Import candidate","导入候选")}</h3>
                                <Show when=move || no_changes fallback=|| ()>
                                    <p>{move || tr(locale.get(), "No author values differ from the current theme.", "作者值与当前主题相同。")}</p>
                                </Show>
                                <DocumentDiff before=candidate.base.clone() after=candidate.document.clone() locale=locale.get() before_label=tr(locale.get(),"Current","当前") after_label=tr(locale.get(),"Imported","导入值") import_source=("Theme JSON v2".into(),imported_source)/>
                                <Show when=move || has_diagnostics fallback=|| ()>
                                    <div class="tools-notice" data-testid="import-diagnostics">
                                        <h3>{move || tr(locale.get(), "Import notes", "导入提示")}</h3>
                                        <ul>{candidate.diagnostics.iter().map(|note| {
                                            let message=if let Some((requested,resolved))=note.message.strip_prefix("Requested ").and_then(|message|message.split_once("; preview uses ")) {
                                                view!{<span>{tr(locale.get(),"Requested","请求值")}" "<code class="tools-font-value">{requested.to_owned()}</code>"; "{tr(locale.get(),"preview uses","预览使用")}" "<code class="tools-font-value">{resolved.to_owned()}</code></span>}.into_any()
                                            }else{note.message.clone().into_any()};
                                            view! {<li><DiagnosticPath field=note.field.clone() locale=locale.get()/>": "{message}</li>}
                                        }).collect_view()}</ul>
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
                            <Select id="tools-export-kind" value=export_kind options=Signal::derive(move || vec![SelectOption::new("json","Theme JSON v2"),SelectOption::new("json-v1",tr(locale.get(),"Compatible JSON v1 · lossy","兼容 JSON v1 · 有损")),SelectOption::new("css",tr(locale.get(),"CSS · resolved values","CSS · 解析后的值")),SelectOption::new("rust",tr(locale.get(),"Rust integration","Rust 接入片段"))])
                                open=export_kind_open on_open_change=move |open| export_kind_open.set(open)
                                on_change=move |kind| { export_kind.set(kind); feedback.set(Feedback::None); issue.set(String::new()); } test_id="export-kind" />
                        </div>
                        <dl class="tools-summary" data-testid="export-setup-summary">
                            <dt>{move || tr(locale.get(),"Source revision","源码修订")}</dt><dd><code>{env!("RUSTIFY_STUDIO_SOURCE_REVISION")}</code></dd>
                            <dt>{move || tr(locale.get(),"Integration","接入方式")}</dt><dd>{move || if export_kind.get()=="css" {
                                tr(locale.get(),"Two resolved modes. Static CSS needs no wasm or CLI; compile Tailwind profiles with Tailwind 4.1.13. New SDK components/source Docs need matching source assets.","两模式解析值。静态 CSS 无需 wasm 或 CLI；Tailwind 输出使用 Tailwind 4.1.13 编译。新 SDK 组件/源码 Docs 使用同源资产。")
                            } else {
                                tr(locale.get(),"JSON v2 / Rust require matching source SDK and CSS. Published 0.2.1 SDK/CLI reject complete v2. Compatible v1 is lossy; retain v2. Only Docs needs the source CLI from this checkout.","JSON v2 / Rust 需要匹配修订的源码 SDK 和 CSS。已发布的 0.2.1 SDK/CLI 不支持完整 v2。兼容 v1 有损，请保留 v2。仅 Docs 路径需要同一 checkout 的源码 CLI。")
                            }}</dd>
                            <dt>{move || tr(locale.get(),"External fonts","外部字体")}</dt><dd>{move || tr(locale.get(),"Copy the resolved font files and original licenses listed in the setup notes. This download contains instructions, not font files.","请按接入说明复制实际解析的字体文件及原许可。本下载仅含说明，不含字体文件。")}</dd>
                        </dl>
                        <details data-testid="export-setup-details">
                            <summary>{move || tr(locale.get(),"Font resources and setup steps","字体资源与接入步骤")}</summary>
                            <pre class="tools-status tools-setup" data-testid="export-setup-notes">{move || integration_notes(&document.get(), &context.get(), preview_mode.get()).map(|artifact|artifact.text).unwrap_or_else(|error|error.to_string())}</pre>
                        </details>
                        <p class="tools-status" data-testid="export-preview-mode">{move || tr(locale.get(),"To match this preview, call","匹配当前预览时调用")}" "<code>{move || preview_call(&context.get(), preview_mode.get())}</code>". "{move || tr(locale.get(),"container is your mount node; config is MountConfig with your deployment base. Ordinary mount_exported_theme honors author appearance.","container 是挂载节点，config 是包含部署 base 的 MountConfig。普通 mount_exported_theme 遵守作者外观。")}</p>
                        <Show when=move || export_kind.get()=="json-v1" fallback=|| ()>
                            <div class="tools-notice" data-testid="export-v1-losses" role="note">
                                <p>{move || tr(locale.get(), "Review these losses before copying or downloading v1. It cannot restore the complete configured component appearance.", "复制或下载 v1 前请核对以下损失。v1 无法恢复配置后组件的完整外观。")}</p>
                                <ul>{move || output.get().ok().map(|artifact| {
                                    let mut losses=artifact.losses;
                                    for field in ["config.accent_color","config.gray_color","config.appearance","config.radius","config.scaling","sources","derived_roles"] {
                                        if !losses.iter().any(|loss|loss.field==field){losses.push(LegacyExportLoss{field,message:String::new()});}
                                    }
                                    losses.into_iter().map(|loss| view! {
                                    <li data-loss=loss.field>{legacy_loss_text(&loss,locale.get(),document.with(|document|document.config))}</li>
                                }).collect_view()})}</ul>
                            </div>
                        </Show>
                        <Show when=move || export_kind.get()=="css" fallback=|| ()>
                            <div class="tools-form-row">
                                <label for="tools-export-profile">{move || tr(locale.get(), "CSS profile", "CSS 用途")}</label>
                                <Select id="tools-export-profile" value=profile options=Signal::derive(move || vec![SelectOption::new("scoped",tr(locale.get(),"Rustify scoped · data-theme","Rustify 局部作用域 · data-theme")),SelectOption::new("standard",tr(locale.get(),"Tailwind v4 · .dark","Tailwind v4 · .dark"))])
                                    open=profile_open on_open_change=move |open| profile_open.set(open) on_change=move |value| profile.set(value) test_id="export-profile" />
                                <label for="tools-export-color">{move || tr(locale.get(), "Color notation", "颜色表达")}</label>
                                <Select id="tools-export-color" value=color_format options=Signal::derive(|| vec![SelectOption::new("hex","HEX"),SelectOption::new("rgb","RGB"),SelectOption::new("hsl","HSL"),SelectOption::new("oklch","OKLCH")])
                                    open=color_format_open on_open_change=move |open| color_format_open.set(open) on_change=move |value| color_format.set(value) test_id="export-color-format" />
                            </div>
                            <p class="tools-status">{move ||tr(locale.get(),"CSS contains both resolved sRGB modes. Scoped dark selector:","CSS 包含 sRGB 下两种模式的解析结果。局部深色选择器：")}" "<code>"[data-theme=\"dark\"]"</code>"; "{move ||tr(locale.get(),"standard selector:","标准选择器：")}" "<code>".dark"</code>". "{move ||tr(locale.get(),"Author configuration remains in JSON. Copy the external font files and their licenses.","作者配置保留在 JSON 中。请复制外部字体文件及其许可。")}</p>
                        </Show>
                        <Show when=move || export_kind.get()=="rust" fallback=|| ()>
                            <p class="tools-status">{move ||tr(locale.get(),"Put","将")}" "<code>"theme.json"</code>" "{move ||tr(locale.get(),"next to","放在")}" "<code>"theme.rs"</code>{move ||tr(locale.get(),". "," 旁边。")}{move ||tr(locale.get(),"Use the matching source SDK and CSS; the published 0.2.1 SDK/CLI reject v2. Keep the returned","使用匹配的源码 SDK 和 CSS；已发布的 0.2.1 SDK/CLI 不支持 v2。UI 挂载期间须保持返回的")}" "<code>"AppHandle"</code>" "{move ||tr(locale.get(),"alive while the UI is mounted. Fonts remain external.","存活。字体仍使用外部资源。")}</p>
                            <p class="tools-status">{move || tr(locale.get(),"For Docs only, build the CLI from that revision:","仅 Docs 路径：从同一修订构建 CLI：")}" "<code>"mbx build --locked -p rustify-docs-cli --profile docs-cli"</code></p>
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
                        <div class="tools-actions tools-export-actions">
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
                            <Button variant=ButtonVariant::Outline on_click=move || match integration_notes(&document.get_untracked(), &context.get_untracked(), preview_mode.get_untracked()) {
                                Ok(artifact) => download(&artifact,feedback,issue), Err(error) => issue.set(error.to_string())
                            } test_id="export-setup-download">{move || tr(locale.get(), "Download setup notes", "下载接入说明")}</Button>
                            <p class="tools-status tools-feedback" role="status" data-testid="export-feedback" data-feedback=move || match feedback.get(){Feedback::Copied|Feedback::DownloadStarted=>"success",Feedback::CopyFailed=>"error",_=>"idle"}>{move || match feedback.get() {
                                Feedback::Copying => tr(locale.get(), "Copying…", "正在复制…"),
                                Feedback::Copied => tr(locale.get(), "Copied to clipboard.", "已复制到剪贴板。"),
                                Feedback::CopyFailed => tr(locale.get(), "Clipboard unavailable or refused. Select the text or download it.", "剪贴板不可用或被拒绝。请选择文本复制，或下载文件。"),
                                Feedback::DownloadStarted => tr(locale.get(), "Download started. Check your browser's downloads.", "已发起下载。请查看浏览器下载记录。"),
                                Feedback::None => "",
                            }}</p>
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
                    Some(Confirmation::LoadExternal) => tr(locale.get(),"Load the reviewed stored version and cancel all pending writes. Export the current draft first if you need a backup. A later external change requires a new decision.","载入已检查的缓存版本，并取消全部待写操作。如需备份当前草稿，请先导出；外部版本再次变化时须重新决策。").into(),
                    _ => tr(locale.get(),"Remove only the reviewed v2 library and cancel all preceding pending writes. Keep the current draft, v1 source and original backups. Saving resumes only by explicit request.","只删除已检查的 v2 库，并取消此前全部待写操作。保留当前草稿、v1 原文与原始备份；之后须明确请求才恢复保存。").into(),
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
            envelope_version: 2,
            revision: 0,
            migrated_from_v1: None,
            draft: base(),
            saved_themes: Vec::new(),
            preferences: Preferences::default(),
        }
    }

    #[test]
    fn setup_notes_preserve_json_and_explicit_preview_context() {
        let context = font_context(20., true);
        let mut document = base();
        document.config.appearance = rustify_ui::Appearance::Dark;
        let before = export_json(&document, &context).unwrap();
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            let notes = integration_notes(&document, &context, mode).unwrap();
            assert_eq!(notes.name, "theme-setup.txt");
            assert_eq!(notes.mime, "text/plain");
            assert!(notes.text.contains(env!("RUSTIFY_STUDIO_SOURCE_REVISION")));
            assert!(notes.text.contains(&format!("mount_exported_preview(container, config, ThemeMode::{mode:?}, &font_context(20., true))")));
            assert!(notes.text.contains("0.2.1"));
            assert!(notes.text.contains("MountConfig.base"));
            assert!(notes.text.contains("FONT-LICENSES.md"));
            for mode in [ThemeMode::Light, ThemeMode::Dark] {
                let resolved = rustify_ui::theme::resolve(&document, mode, &context).unwrap();
                for font in &resolved.fonts {
                    assert!(notes.text.contains(&font.requested.join(", ")));
                    assert!(notes.text.contains(&font.family));
                    if let Some(path) = &font.resource_path {
                        assert!(notes.text.contains(path));
                    }
                }
            }
            for face in rustify_ui::theme::FONT_GLYPH_FALLBACKS {
                assert!(notes.text.contains(face.resource_path));
            }
        }
        assert_eq!(export_json(&document, &context).unwrap(), before);
        document.id.clear();
        let invalid = document.clone();
        assert!(integration_notes(&document, &context, ThemeMode::Light).is_err());
        assert_eq!(document, invalid);
    }

    #[test]
    fn setup_notes_do_not_invent_packaged_files_for_css_fonts() {
        let context = ResolveContext {
            font_resolution: rustify_ui::theme::FontResolution::Css,
            ..font_context(16., false)
        };
        let notes = integration_notes(&base(), &context, ThemeMode::Light).unwrap();
        assert!(notes.text.contains("system / non-packaged font"));
        assert!(!notes.text.contains(".ttf"));
    }

    #[test]
    fn import_review_lists_identity_added_removed_and_both_mode_changes_without_mutating() {
        let context = font_context(16., false);
        let mut original = base();
        original
            .base
            .light
            .0
            .insert("removed".into(), "author value".into());
        let mut candidate = base();
        candidate.id = "imported".into();
        candidate
            .base
            .dark
            .0
            .insert("future-token".into(), "url(https://example.test)".into());
        candidate
            .overrides
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
        assert_eq!(preview.source_version, Some(2));
        assert_eq!(preview.base, original);
        assert_eq!(preview.changes.len(), 4);
        assert!(preview
            .changes
            .iter()
            .any(|change| change.token == "base.removed" && change.after.is_none()));
        assert!(preview
            .changes
            .iter()
            .any(|change| change.mode == Some(ThemeMode::Dark)
                && change.token == "base.future-token"));
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
        assert!(!preview.document.base.light.0.contains_key("future-token"));
    }

    #[test]
    fn unsupported_json_version_is_reported_before_missing_document_fields() {
        let original = base();
        let error = review_import(
            r#"{"schema_version":999}"#,
            "json",
            &original,
            &font_context(16., false),
        )
        .unwrap_err();
        assert_eq!(error.field, "schema_version");
        assert_eq!(error.message, "999");
        assert_eq!(original, base());
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
        assert!(rust.text.contains("pub fn mount_exported_preview("));
        assert!(rust
            .text
            .contains("preview_mode=Signal::derive(move || mode)"));
        assert!(!rust.text.contains("Theme {"));
        assert!(export_artifact(&original, &context, "css", "unknown", "rgb").is_err());
    }

    #[test]
    fn compatible_v1_is_explicit_and_carries_the_reviewed_losses_with_its_bytes() {
        let context = font_context(16., false);
        let mut original = ThemeDocument::standard("configured", "Configured");
        original.config.scaling = rustify_ui::ThemeScaling::P110;
        original.config.appearance = rustify_ui::Appearance::Dark;
        let full = export_artifact(&original, &context, "json", "scoped", "rgb").unwrap();
        assert!(full.losses.is_empty());
        assert_eq!(
            import_json(&full.text, &context).unwrap().document,
            original
        );
        let legacy = export_artifact(&original, &context, "json-v1", "scoped", "rgb").unwrap();
        assert_eq!(legacy.name, "theme-v1.json");
        let wire: serde_json::Value = serde_json::from_str(&legacy.text).unwrap();
        assert_eq!(wire["schema_version"], 1);
        assert!(wire.get("config").is_none());
        for field in [
            "config.accent_color",
            "config.gray_color",
            "config.appearance",
            "config.radius",
            "config.scaling",
            "sources",
            "derived_roles",
        ] {
            assert!(
                legacy.losses.iter().any(|loss| loss.field == field),
                "{field}"
            );
        }
        let preview = review_import(&legacy.text, "json", &original, &context).unwrap();
        assert_eq!(preview.source_version, Some(1));
        assert_eq!(preview.document.config, rustify_ui::ThemeConfig::custom());
        assert_eq!(original.config.scaling, rustify_ui::ThemeScaling::P110);
    }
}
