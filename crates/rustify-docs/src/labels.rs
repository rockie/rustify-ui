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
    copy_failed: "Copy failed: select the code to copy it",
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
    copy_failed: "复制失败：请选中代码后手动复制",
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
