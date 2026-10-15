//! The interface's built-in words and project message overrides.
//!
//! Built-ins use the primary language subtag: `zh…` gets Chinese and all
//! others English. A resolved catalog records each string's actual language,
//! including English fallback for missing third-language messages.

use std::collections::BTreeMap;
use std::fmt;

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use toml::Spanned;

use crate::diagnostic::Issue;
use crate::diagnostic_text::{code_span, literal};

#[derive(Serialize)]
pub struct Labels {
    /// The `lang` the strings are in, written on them when it differs from
    /// the page's.
    #[serde(skip)]
    pub lang: &'static str,
    pub note: &'static str,
    pub tip: &'static str,
    pub important: &'static str,
    pub warning: &'static str,
    pub caution: &'static str,
    pub copy: &'static str,
    pub copied: &'static str,
    pub copy_failed: &'static str,
    /// The copy button's own word for a refusal; `copy_failed` is what is
    /// announced.
    pub copy_failed_short: &'static str,
    pub plain_text: &'static str,
    pub footnotes: &'static str,
    pub back_to_reference: &'static str,
    pub skip_to_content: &'static str,
    pub on_this_page: &'static str,
    pub previous: &'static str,
    pub next: &'static str,
    pub menu: &'static str,
    /// The header's configured links.
    pub site_links: &'static str,
    pub start_reading: &'static str,
    pub footer_links: &'static str,
    pub theme: &'static str,
    pub theme_system: &'static str,
    pub theme_light: &'static str,
    pub theme_dark: &'static str,
    pub draft: &'static str,
    pub not_found_title: &'static str,
    pub not_found_body: &'static str,
    pub back_home: &'static str,
    pub last_updated: &'static str,
    pub edit_page: &'static str,
    pub language: &'static str,
    pub untranslated: &'static str,
    pub available_languages: &'static str,
    /// A task list's box: its accessible name, the item's text following.
    pub task: &'static str,
    /// What `dev` adds while an author writes; `{n}` is a number.
    pub dev: DevLabels,
    /// The interactive example's controls, outside its locked canvas.
    pub demo: DemoLabels,
    pub search: SearchLabels,
    pub ai: AiLabels,
}

#[derive(Serialize)]
pub struct AiLabels {
    pub home_description: &'static str,
    pub other_pages: &'static str,
    pub fallback: &'static str,
}

#[derive(Serialize)]
pub struct SearchLabels {
    pub open: &'static str,
    pub input: &'static str,
    pub close: &'static str,
    pub hint: &'static str,
    pub loading: &'static str,
    pub empty: &'static str,
    pub failed: &'static str,
    pub retry: &'static str,
    pub count: &'static str,
    pub more: &'static str,
    pub results: &'static str,
    pub fallback: &'static str,
    pub no_script: &'static str,
}

#[derive(Serialize)]
pub struct DemoLabels {
    pub example: &'static str,
    pub loading: &'static str,
    pub failed: &'static str,
    pub reload: &'static str,
    pub no_script: &'static str,
    pub function: &'static str,
    pub full_file: &'static str,
}

/// The words of `dev`'s problem panel and of the page shown when nothing
/// has built yet.
#[derive(Debug, Serialize)]
pub struct DevLabels {
    /// The panel's name.
    pub problems: &'static str,
    pub error_one: &'static str,
    pub error_other: &'static str,
    pub warning_one: &'static str,
    pub warning_other: &'static str,
    /// Each problem's severity, said before its rule.
    pub error: &'static str,
    pub warning: &'static str,
    /// After the count while an older version is shown.
    pub last_good: &'static str,
    /// After the count when no version has built.
    pub no_last_good: &'static str,
    pub revision: &'static str,
    pub hide: &'static str,
    /// Announced when the problems are gone.
    pub fixed: &'static str,
    pub shell_title: &'static str,
    pub shell_body: &'static str,
    pub building: &'static str,
    pub building_title: &'static str,
    pub building_body: &'static str,
    pub host: &'static str,
    pub wasm: &'static str,
    pub render: &'static str,
}

pub const EN: Labels = Labels {
    ai: AiLabels {
        home_description: "Homepage of {site}.",
        other_pages: "Other pages",
        fallback: "Not translated; content language: {lang}",
    },
    search: SearchLabels {
        open: "Search",
        input: "Search documentation",
        close: "Close",
        hint: "Type a word to search this language’s documentation.",
        loading: "Preparing search… You can keep typing.",
        empty: "No results. Try a different word or a shorter prefix.",
        failed: "Search is unavailable. Please try again.",
        retry: "Retry",
        count: "{n} results",
        more: "Show more",
        results: "Search results",
        fallback: "Not translated; content language: {lang}",
        no_script: "Enable JavaScript to search. You can still use the navigation links.",
    },
    lang: "en",
    note: "Note",
    tip: "Tip",
    important: "Important",
    warning: "Warning",
    caution: "Caution",
    copy: "Copy",
    copied: "Copied",
    copy_failed: "Copy failed: the code is selected, copy it by hand",
    copy_failed_short: "Copy failed",
    plain_text: "Text",
    footnotes: "Footnotes",
    back_to_reference: "Back to reference",
    skip_to_content: "Skip to content",
    on_this_page: "On this page",
    previous: "Previous",
    next: "Next",
    menu: "Menu",
    site_links: "Site links",
    start_reading: "Start reading",
    footer_links: "Footer links",
    theme: "Theme",
    theme_system: "System",
    theme_light: "Light",
    theme_dark: "Dark",
    draft: "Draft",
    not_found_title: "Page not found",
    not_found_body: "There is no page at this address.",
    back_home: "Back to the home page",
    last_updated: "Last updated",
    edit_page: "Edit this page",
    language: "Language",
    untranslated: "This page has not been translated yet.",
    available_languages: "Available languages",
    task: "Task",
    dev: DevLabels {
        problems: "Problems",
        error_one: "{n} error",
        error_other: "{n} errors",
        warning_one: "{n} warning",
        warning_other: "{n} warnings",
        error: "Error",
        warning: "Warning",
        last_good: "showing the last good version",
        no_last_good: "nothing has built yet",
        revision: "revision {n}",
        hide: "Hide problems",
        fixed: "Problems fixed; the page is up to date",
        shell_title: "The site does not build yet",
        shell_body: "Fix the problems listed and save: this page reloads by itself.",
        building: "Building",
        building_title: "Building your site",
        building_body: "The first dependency build takes longer. Keep this page open: it updates when the site is ready.",
        host: "Compiling native renderer",
        wasm: "Compiling browser module",
        render: "Rendering pages",
    },
    demo: DemoLabels {
        example: "Interactive example",
        loading: "Loading interactive example…",
        failed: "This example could not start. Reload the page to try again.",
        reload: "Reload page",
        no_script: "Enable JavaScript to interact",
        function: "Function",
        full_file: "Full file",
    },
};

pub const ZH: Labels = Labels {
    ai: AiLabels {
        home_description: "{site}首页",
        other_pages: "其他页面",
        fallback: "尚未翻译，内容语言：{lang}",
    },
    search: SearchLabels {
        open: "搜索",
        input: "搜索文档",
        close: "关闭",
        hint: "输入词语，搜索当前语言的文档。",
        loading: "正在准备搜索… 你可以继续输入。",
        empty: "没有找到结果。试试其他词语或更短的前缀。",
        failed: "搜索暂时不可用，请重试。",
        retry: "重试",
        count: "{n} 个结果",
        more: "显示更多",
        results: "搜索结果",
        fallback: "尚未翻译，内容语言：{lang}",
        no_script: "启用 JavaScript 后可搜索。你仍可通过导航链接阅读文档。",
    },
    lang: "zh",
    note: "注意",
    tip: "提示",
    important: "重要",
    warning: "警告",
    caution: "小心",
    copy: "复制",
    copied: "已复制",
    copy_failed: "复制失败：代码已选中，请手动复制",
    copy_failed_short: "复制失败",
    plain_text: "文本",
    footnotes: "脚注",
    back_to_reference: "返回引用处",
    skip_to_content: "跳到正文",
    on_this_page: "本页目录",
    previous: "上一页",
    next: "下一页",
    menu: "菜单",
    site_links: "站点链接",
    start_reading: "开始阅读",
    footer_links: "页脚导航",
    theme: "主题",
    theme_system: "跟随系统",
    theme_light: "浅色",
    theme_dark: "深色",
    draft: "草稿",
    not_found_title: "页面不存在",
    not_found_body: "这个地址没有页面。",
    back_home: "返回首页",
    last_updated: "最后更新",
    edit_page: "编辑此页",
    language: "语言",
    untranslated: "此页面尚未翻译。",
    available_languages: "可用语言",
    task: "任务",
    dev: DevLabels {
        problems: "问题",
        error_one: "{n} 个错误",
        error_other: "{n} 个错误",
        warning_one: "{n} 个警告",
        warning_other: "{n} 个警告",
        error: "错误",
        warning: "警告",
        last_good: "显示上一个可用版本",
        no_last_good: "尚无可用版本",
        revision: "第 {n} 版",
        hide: "收起问题",
        fixed: "问题已修复，页面为最新",
        shell_title: "站点暂时无法构建",
        shell_body: "修正列出的问题并保存，本页会自动刷新。",
        building: "构建中",
        building_title: "正在构建站点",
        building_body: "首次构建依赖需要更长时间。保持本页打开，站点就绪后会自动更新。",
        host: "正在编译原生渲染器",
        wasm: "正在编译浏览器模块",
        render: "正在渲染页面",
    },
    demo: DemoLabels {
        example: "交互示例",
        loading: "正在加载交互示例…",
        failed: "此示例暂时无法运行。重新加载页面后再试。",
        reload: "重新加载页面",
        no_script: "启用 JavaScript 后可操作此示例",
        function: "函数",
        full_file: "完整文件",
    },
};

/// The labels for a page in `lang`, and whether they need a `lang` of their
/// own because the page is in neither of the interface's languages.
pub fn for_lang(lang: &str) -> (&'static Labels, bool) {
    let primary = lang.split(['-', '_']).next().unwrap_or("");
    if primary.eq_ignore_ascii_case("zh") {
        (&ZH, false)
    } else if primary.eq_ignore_ascii_case("en") {
        (&EN, false)
    } else {
        (&EN, true)
    }
}

/// A plain interface string and the language its words are actually in.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LabelText {
    pub text: String,
    pub lang: String,
}

/// One build's resolved interface words, shared by all pages in a language.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct Catalog {
    entries: BTreeMap<String, LabelText>,
    #[serde(skip)]
    missing: Vec<String>,
    #[serde(skip)]
    digest: String,
}

impl Catalog {
    /// Reads a key from the fixed built-in directory.
    ///
    /// # Panics
    /// Panics when a caller uses a key absent from the built-in labels.
    pub fn get(&self, key: &str) -> &LabelText {
        &self.entries[key]
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = (&str, &LabelText)> {
        self.entries
            .iter()
            .map(|(key, value)| (key.as_str(), value))
    }

    pub fn keys(&self) -> impl ExactSizeIterator<Item = &str> {
        self.entries.keys().map(String::as_str)
    }

    /// Third-language keys using English fallback; built-in languages have none.
    pub fn missing_keys(&self) -> &[String] {
        &self.missing
    }

    /// A deterministic digest of all keys, words and their actual languages.
    pub fn digest(&self) -> &str {
        &self.digest
    }
}

/// Resolves overrides already validated by [`parse_messages`].
pub fn resolve(lang: &str, overrides: &BTreeMap<String, String>) -> Catalog {
    let (labels, foreign) = for_lang(lang);
    let mut missing = Vec::new();
    let entries: BTreeMap<_, _> = builtin_entries(labels)
        .into_iter()
        .map(|(key, builtin)| {
            let value = if let Some(text) = overrides.get(&key) {
                LabelText {
                    text: text.clone(),
                    lang: lang.to_owned(),
                }
            } else {
                if foreign {
                    missing.push(key.clone());
                }
                LabelText {
                    text: builtin,
                    lang: labels.lang.to_owned(),
                }
            };
            (key, value)
        })
        .collect();
    let mut hash = Sha256::new();
    for (key, value) in &entries {
        for part in [key.as_str(), value.text.as_str(), value.lang.as_str()] {
            hash.update((part.len() as u64).to_le_bytes());
            hash.update(part.as_bytes());
        }
    }
    Catalog {
        entries,
        missing,
        digest: format!("{:x}", hash.finalize()),
    }
}

fn builtin_entries(labels: &Labels) -> BTreeMap<String, String> {
    fn flatten(prefix: &str, value: serde_json::Value, entries: &mut BTreeMap<String, String>) {
        match value {
            serde_json::Value::String(text) => {
                entries.insert(prefix.to_owned(), text);
            }
            serde_json::Value::Object(fields) => {
                for (key, value) in fields {
                    let key = if prefix.is_empty() {
                        key
                    } else {
                        format!("{prefix}.{key}")
                    };
                    flatten(&key, value, entries);
                }
            }
            _ => unreachable!("Labels only contain strings and nested label structs"),
        }
    }
    let value = match serde_json::to_value(labels) {
        Ok(value) => value,
        Err(error) => unreachable!("Labels have no fallible serialization: {error}"),
    };
    let mut entries = BTreeMap::new();
    flatten("", value, &mut entries);
    entries
}

type RawMessages = BTreeMap<Spanned<String>, Spanned<MessageValue>>;

enum MessageValue {
    String(String),
    Table(RawMessages),
}

impl<'de> Deserialize<'de> for MessageValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MessageVisitor;

        impl<'de> Visitor<'de> for MessageVisitor {
            type Value = MessageValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a message string or a table of message strings")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                Ok(MessageValue::String(value.to_owned()))
            }

            fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
                Ok(MessageValue::String(value))
            }

            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut values = BTreeMap::new();
                while let Some((key, value)) = map.next_entry()? {
                    values.insert(key, value);
                }
                Ok(MessageValue::Table(values))
            }
        }

        deserializer.deserialize_any(MessageVisitor)
    }
}

/// Reads project messages, keeping each invalid key or value's original span.
pub fn parse_messages(text: &str) -> Result<BTreeMap<String, String>, Vec<Issue>> {
    let raw: RawMessages = toml::from_str(text).map_err(|error| {
        vec![Issue::error(
            error.span().unwrap_or(0..0),
            "i18n-messages",
            literal(error.message()),
        )]
    })?;
    let mut entries = BTreeMap::new();
    let mut issues = Vec::new();
    flatten_messages("", raw, &builtin_entries(&EN), &mut entries, &mut issues);
    if issues.is_empty() {
        Ok(entries)
    } else {
        Err(issues)
    }
}

fn flatten_messages(
    prefix: &str,
    raw: RawMessages,
    builtins: &BTreeMap<String, String>,
    entries: &mut BTreeMap<String, String>,
    issues: &mut Vec<Issue>,
) {
    for (key, value) in raw {
        let full = if prefix.is_empty() {
            key.get_ref().clone()
        } else {
            format!("{prefix}.{}", key.get_ref())
        };
        let span = value.span();
        let namespace = format!("{full}.");
        let value = match value.into_inner() {
            MessageValue::Table(table)
                if builtins.keys().any(|key| key.starts_with(&namespace)) =>
            {
                flatten_messages(&full, table, builtins, entries, issues);
                continue;
            }
            value => value,
        };
        let Some(builtin) = builtins.get(&full) else {
            issues.push(Issue::error(
                key.span(),
                "i18n-messages",
                format!("unknown message key {}", code_span(&full)),
            ));
            continue;
        };
        let MessageValue::String(text) = value else {
            issues.push(Issue::error(
                span,
                "i18n-messages",
                format!("message {} must be a string", code_span(&full)),
            ));
            continue;
        };
        if text
            .chars()
            .any(|ch| ch.is_control() && !matches!(ch, '\n' | '\r' | '\t'))
        {
            issues.push(Issue::error(
                span,
                "i18n-messages",
                format!("message {} contains a control character", code_span(&full)),
            ));
        } else if placeholders(&text) != placeholders(builtin) {
            issues.push(
                Issue::error(
                    span,
                    "i18n-messages",
                    format!(
                        "message {} must keep the built-in format placeholders",
                        code_span(&full)
                    ),
                )
                .suggest(format!(
                    "preserve the placeholders in {}",
                    code_span(builtin)
                )),
            );
        } else if entries.insert(full.clone(), text).is_some() {
            issues.push(Issue::error(
                key.span(),
                "i18n-messages",
                format!("message {} is declared more than once", code_span(&full)),
            ));
        }
    }
}

fn placeholders(mut text: &str) -> BTreeMap<&str, usize> {
    let mut names = BTreeMap::new();
    while let Some(start) = text.find('{') {
        text = &text[start + 1..];
        let Some(end) = text.find('}') else { break };
        *names.entry(&text[..end]).or_default() += 1;
        text = &text[end + 1..];
    }
    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn primary_subtag_picks_the_language() {
        let pick = |lang: &str| {
            let (labels, foreign) = for_lang(lang);
            (labels.lang, foreign)
        };
        assert_eq!(pick("zh-CN"), ("zh", false));
        assert_eq!(pick("zh-Hant-TW"), ("zh", false));
        assert_eq!(pick("en-GB"), ("en", false));
        assert_eq!(pick("EN"), ("en", false));
        assert_eq!(pick("fr"), ("en", true));
    }

    #[test]
    fn builtin_catalogs_allow_partial_overrides() {
        let overrides = BTreeMap::from([("copy".into(), "拷貝".into())]);
        let catalog = resolve("ZH-Hant", &overrides);
        assert_eq!(
            catalog.get("copy"),
            &LabelText {
                text: "拷貝".into(),
                lang: "ZH-Hant".into()
            }
        );
        assert_eq!(catalog.get("tip").text, ZH.tip);
        assert_eq!(catalog.get("tip").lang, "zh");
        assert!(catalog.missing_keys().is_empty());
    }

    #[test]
    fn third_languages_keep_each_strings_actual_language() {
        let overrides = BTreeMap::from([("copy".into(), "Copier".into())]);
        let catalog = resolve("fr-CA", &overrides);
        assert_eq!(
            catalog.get("copy"),
            &LabelText {
                text: "Copier".into(),
                lang: "fr-CA".into()
            }
        );
        assert_eq!(
            catalog.get("tip"),
            &LabelText {
                text: EN.tip.into(),
                lang: "en".into()
            }
        );
        assert!(!catalog.missing_keys().iter().any(|key| key == "copy"));
        assert_eq!(catalog.missing_keys().len(), catalog.keys().count() - 1);
    }

    #[test]
    fn catalog_keys_come_from_the_label_fields() {
        let catalog = resolve("en", &BTreeMap::new());
        let keys: Vec<_> = catalog.keys().collect();
        assert!(keys.contains(&"language"));
        assert!(keys.contains(&"untranslated"));
        assert!(keys.contains(&"available_languages"));
        assert!(keys.contains(&"demo.loading"));
        assert!(keys.contains(&"dev.error_one"));
        assert!(!keys.contains(&"lang"));
        assert_eq!(catalog.get("demo.loading").text, EN.demo.loading);
    }

    #[test]
    fn catalog_digest_depends_on_words_and_their_languages() {
        let base = resolve("en", &BTreeMap::new());
        let same = resolve("fr", &BTreeMap::new());
        let translated = resolve("fr", &BTreeMap::from([("copy".into(), EN.copy.into())]));
        let changed = resolve("en", &BTreeMap::from([("copy".into(), "Copy code".into())]));
        assert_eq!(base.digest(), same.digest());
        assert_ne!(base.digest(), translated.digest());
        assert_ne!(base.digest(), changed.digest());
    }

    #[test]
    fn messages_accept_root_dotted_and_nested_strings() {
        let messages = parse_messages("note = 'Remarque'\n\"demo.loading\" = 'Chargement…'\n[dev]\nerror_one = '{n} erreur'\n").unwrap();
        assert_eq!(
            messages,
            BTreeMap::from([
                ("note".into(), "Remarque".into()),
                ("demo.loading".into(), "Chargement…".into()),
                ("dev.error_one".into(), "{n} erreur".into()),
            ])
        );
    }

    #[test]
    fn messages_preserve_normal_newlines_and_plain_text() {
        let messages =
            parse_messages("note = \"\"\"First line\n<strong>second</strong>\"\"\"\n").unwrap();
        assert_eq!(messages["note"], "First line\n<strong>second</strong>");
    }

    #[test]
    fn unknown_message_keys_keep_the_source_key_span() {
        let text = "[demo]\nunknown = 'no'\n";
        let issues = parse_messages(text).unwrap_err();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule, "i18n-messages");
        assert_eq!(&text[issues[0].span.clone()], "unknown");
    }

    #[test]
    fn non_string_messages_keep_the_source_value_span() {
        let text = "[demo]\nloading = 42\n";
        let issues = parse_messages(text).unwrap_err();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule, "i18n-messages");
        assert_eq!(&text[issues[0].span.clone()], "42");
    }

    #[test]
    fn all_non_string_toml_values_are_message_errors() {
        for value in ["true", "1.5", "[1, 2]", "{ nested = 'text' }", "2026-10-15"] {
            let text = format!("note = {value}\n");
            let issues = parse_messages(&text).unwrap_err();
            assert_eq!(issues.len(), 1, "{value}: {issues:?}");
            assert_eq!(issues[0].rule, "i18n-messages", "{value}");
            assert_eq!(&text[issues[0].span.clone()], value, "{value}");
        }
    }

    #[test]
    fn unknown_empty_message_tables_are_errors() {
        let text = "[unknown]\n";
        let issues = parse_messages(text).unwrap_err();
        assert_eq!(issues[0].rule, "i18n-messages");
        assert_eq!(&text[issues[0].span.clone()], "unknown");
    }

    #[test]
    fn message_control_characters_keep_the_source_value_span() {
        let text = "note = \"bad\\u0007text\"\n";
        let issues = parse_messages(text).unwrap_err();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule, "i18n-messages");
        assert_eq!(&text[issues[0].span.clone()], "\"bad\\u0007text\"");
    }

    #[test]
    fn missing_message_placeholders_are_errors() {
        let text = "[dev]\nerror_one = 'erreur'\n";
        let issues = parse_messages(text).unwrap_err();
        assert_eq!(issues[0].rule, "i18n-messages");
        assert_eq!(&text[issues[0].span.clone()], "'erreur'");
    }

    #[test]
    fn added_message_placeholders_are_errors() {
        let issues = parse_messages("copy = 'Copier {n}'\n").unwrap_err();
        assert_eq!(issues[0].rule, "i18n-messages");
    }

    #[test]
    fn duplicate_message_keys_are_errors() {
        let issues = parse_messages("copy = 'Copier'\ncopy = 'Encore'\n").unwrap_err();
        assert_eq!(issues[0].rule, "i18n-messages");
        assert!(issues[0].span.start > 0);
    }

    #[test]
    fn flat_and_nested_aliases_cannot_repeat_a_message() {
        let issues = parse_messages("\"demo.loading\" = 'Premier'\n[demo]\nloading = 'Deuxième'\n")
            .unwrap_err();
        assert_eq!(issues[0].rule, "i18n-messages");
    }
}
