//! The interface's own words, in the two languages it ships.
//!
//! A page's `lang` picks them by its primary subtag: `zh…` gets Chinese,
//! everything else English. A page in a third language keeps its own `lang`
//! on its content, and the interface strings inside it say `lang="en"`, so a
//! screen reader does not read English with a French voice.

pub struct Labels {
    /// The `lang` the strings are in, written on them when it differs from
    /// the page's.
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
    /// A task list's box: its accessible name, the item's text following.
    pub task: &'static str,
    /// What `dev` adds while an author writes; `{n}` is a number.
    pub dev: DevLabels,
}

/// The words of `dev`'s problem panel and of the page shown when nothing
/// has built yet.
#[derive(Debug, serde::Serialize)]
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
}

pub const EN: Labels = Labels {
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
    },
};

pub const ZH: Labels = Labels {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
