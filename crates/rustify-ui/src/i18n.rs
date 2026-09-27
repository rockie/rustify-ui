//! Two languages, and the strings the SDK itself has to say.
//!
//! The division is the point: the SDK ships the words it puts on screen
//! without being asked - "retry", "close", the sentence a refused command
//! shows - and the application ships everything else. An SDK that shipped an
//! application's words would be translating a vocabulary it cannot see; an
//! application that had to translate the SDK's would find out which ones exist
//! by meeting them.
//!
//! Switching language changes one signal. Nothing is reloaded, no component is
//! rebuilt, and no string is cached anywhere that would keep the old one.

/// The languages this release ships.
///
/// Two, and named rather than open, because every string here exists in both:
/// a `Locale` that could be any tag would be one whose catalogue could have
/// holes, and a hole in a catalogue is a screen with a key on it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Locale {
    #[default]
    English,
    Chinese,
}

/// Which way the writing goes.
///
/// Neither language here is right-to-left, and the type exists anyway: the
/// components are written in logical properties and the scope root carries a
/// `dir`, so the direction is a value the layout already reads. A third
/// language would be a catalogue, not a rewrite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
}

impl Direction {
    /// The value for an element's `dir` attribute.
    pub fn attribute(self) -> &'static str {
        match self {
            Self::LeftToRight => "ltr",
            Self::RightToLeft => "rtl",
        }
    }
}

impl Locale {
    /// The BCP 47 tag, which is what `Intl` and the `lang` attribute want.
    pub fn tag(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Chinese => "zh-CN",
        }
    }

    pub fn direction(self) -> Direction {
        match self {
            Self::English | Self::Chinese => Direction::LeftToRight,
        }
    }

    /// The locale a tag names, or `None`.
    ///
    /// Matching is on the language subtag, so `zh`, `zh-CN` and `zh-Hans-CN`
    /// are all Chinese: a browser sends what it likes, and refusing `zh-Hans`
    /// for not being `zh-CN` would be refusing a reader their own language
    /// over punctuation.
    pub fn from_tag(tag: &str) -> Option<Self> {
        let language = tag.split(['-', '_']).next()?.to_ascii_lowercase();
        match language.as_str() {
            "en" => Some(Self::English),
            "zh" => Some(Self::Chinese),
            _ => None,
        }
    }

    /// The first of `tags` this SDK has, falling back to English.
    ///
    /// For `navigator.languages`, which is a list in preference order. A
    /// person who prefers a language nobody here speaks gets the fallback
    /// rather than the last thing in their list.
    pub fn best_of<'a>(tags: impl IntoIterator<Item = &'a str>) -> Self {
        tags.into_iter()
            .find_map(Self::from_tag)
            .unwrap_or(Self::English)
    }

    pub fn all() -> [Self; 2] {
        [Self::English, Self::Chinese]
    }

    /// What this language is called in itself. A language picker written in
    /// the language the reader does not have is a picker they cannot use.
    pub fn endonym(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Chinese => "中文",
        }
    }
}

/// Everything the SDK says on its own account.
///
/// One enum rather than free strings, so that adding a message means adding it
/// to every language: the compiler asks for the arm, and
/// `every_message_exists_in_every_language` checks that no arm was filled in
/// with the other language's words.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Message {
    /// A control that offers to try again after a failure.
    Retry,
    /// Dismissing a layer.
    Close,
    /// While something is on its way.
    Loading,
    /// An answer that is "there is nothing", which is not the same as a wait
    /// that never ends.
    Empty,
    /// A load that did not arrive.
    LoadFailed,
    /// A command that is visible and cannot run. The reason beside it is the
    /// application's; this is the label for the state.
    Unavailable,
    /// A field the application would not accept.
    Invalid,
    /// A form that is being saved.
    Saving,
    /// Work that would be lost by leaving.
    UnsavedChanges,
    /// A file the import would not read because of its size.
    FileTooLarge,
    /// A file the import would not read because of what it is.
    FileWrongKind,
    /// A picker that was dismissed.
    NothingChosen,
    /// The clipboard said no, and here is the path that always works.
    ClipboardDenied,
    /// A region that lost its GPU context and is coming back.
    RegionRecovering,
    /// A region that could not start at all.
    RegionFailed,
    /// Text drawn in a font that has no glyph for it.
    MissingGlyph,
    /// The default words on a drop zone.
    DropFileHere,
    /// Taking a status message down before it goes on its own.
    Dismiss,
}

impl Message {
    /// This message in `locale`.
    pub fn text(self, locale: Locale) -> &'static str {
        match locale {
            Locale::English => self.english(),
            Locale::Chinese => self.chinese(),
        }
    }

    fn english(self) -> &'static str {
        match self {
            Self::Retry => "try again",
            Self::Close => "close",
            Self::Loading => "loading\u{2026}",
            Self::Empty => "nothing to show",
            Self::LoadFailed => "that did not load",
            Self::Unavailable => "not available",
            Self::Invalid => "not accepted",
            Self::Saving => "saving\u{2026}",
            Self::UnsavedChanges => "there is unsaved work here",
            Self::FileTooLarge => "that file is too large",
            Self::FileWrongKind => "that is not a kind we read",
            Self::NothingChosen => "nothing was chosen",
            Self::ClipboardDenied => "the browser would not let us copy; the text is selected",
            Self::RegionRecovering => "redrawing",
            Self::RegionFailed => "this view could not start",
            Self::DropFileHere => "drop a file here",
            Self::Dismiss => "dismiss this message",
            // The character, and what it means, because a lone box tells a
            // reader nothing about whose fault it is.
            Self::MissingGlyph => "□ (no glyph in this font)",
        }
    }

    fn chinese(self) -> &'static str {
        match self {
            Self::Retry => "重试",
            Self::Close => "关闭",
            Self::Loading => "加载中\u{2026}",
            Self::Empty => "没有内容",
            Self::LoadFailed => "没有加载成功",
            Self::Unavailable => "当前不可用",
            Self::Invalid => "未被接受",
            Self::Saving => "保存中\u{2026}",
            Self::UnsavedChanges => "这里有未保存的修改",
            Self::FileTooLarge => "文件太大",
            Self::FileWrongKind => "不是可读取的类型",
            Self::NothingChosen => "没有选择任何文件",
            Self::ClipboardDenied => "浏览器不允许复制；文本已选中",
            Self::RegionRecovering => "正在重绘",
            Self::RegionFailed => "这个视图没有启动",
            Self::MissingGlyph => "□（当前字体没有这个字形）",
            Self::DropFileHere => "把文件拖到这里",
            Self::Dismiss => "关闭这条消息",
        }
    }

    /// Every message there is, for a catalogue page and for the test that
    /// keeps the two languages the same size.
    pub fn all() -> [Self; 18] {
        [
            Self::Retry,
            Self::Close,
            Self::Loading,
            Self::Empty,
            Self::LoadFailed,
            Self::Unavailable,
            Self::Invalid,
            Self::Saving,
            Self::UnsavedChanges,
            Self::FileTooLarge,
            Self::FileWrongKind,
            Self::NothingChosen,
            Self::ClipboardDenied,
            Self::RegionRecovering,
            Self::RegionFailed,
            Self::MissingGlyph,
            Self::DropFileHere,
            Self::Dismiss,
        ]
    }

    /// A stable name, for a test id and for the catalogue page's table.
    pub fn key(self) -> &'static str {
        match self {
            Self::Retry => "retry",
            Self::Close => "close",
            Self::Loading => "loading",
            Self::Empty => "empty",
            Self::LoadFailed => "load-failed",
            Self::Unavailable => "unavailable",
            Self::Invalid => "invalid",
            Self::Saving => "saving",
            Self::UnsavedChanges => "unsaved-changes",
            Self::FileTooLarge => "file-too-large",
            Self::FileWrongKind => "file-wrong-kind",
            Self::NothingChosen => "nothing-chosen",
            Self::ClipboardDenied => "clipboard-denied",
            Self::RegionRecovering => "region-recovering",
            Self::RegionFailed => "region-failed",
            Self::MissingGlyph => "missing-glyph",
            Self::DropFileHere => "drop-file-here",
            Self::Dismiss => "dismiss",
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use browser::{
    browser_languages, format_date, format_number, provide_locale, use_locale, LocaleHandle,
};

#[cfg(target_arch = "wasm32")]
mod browser {
    use super::{Locale, Message};
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsValue;

    /// The scope's language.
    #[derive(Clone, Copy)]
    pub struct LocaleHandle {
        locale: RwSignal<Locale>,
    }

    /// Puts a language in context and returns it.
    ///
    /// `initial` is the application's decision. The browser's preference is
    /// available through [`browser_languages`] and [`Locale::best_of`], and it
    /// is deliberately not read here: an application that remembers what its
    /// user chose last time must not be overruled by a browser setting.
    pub fn provide_locale(initial: Locale) -> LocaleHandle {
        let handle = LocaleHandle {
            locale: RwSignal::new(initial),
        };
        provide_context(handle);
        handle
    }

    /// The scope's language, or English where no scope provided one.
    ///
    /// Unlike the drag session this does not panic. A component that says
    /// "Close" in English because nobody chose a language is still doing
    /// something useful; a drag with no session is not.
    pub fn use_locale() -> LocaleHandle {
        use_context::<LocaleHandle>().unwrap_or_else(|| LocaleHandle {
            locale: RwSignal::new(Locale::English),
        })
    }

    impl LocaleHandle {
        /// The current language, tracked.
        pub fn get(&self) -> Locale {
            self.locale.get()
        }

        /// The current language, without subscribing to it. For the handler
        /// that switches it: reading the old value there would make the
        /// handler depend on the thing it sets.
        pub fn get_untracked(&self) -> Locale {
            self.locale.get_untracked()
        }

        pub fn set(&self, locale: Locale) {
            self.locale.set(locale);
        }

        /// A message in the current language, tracked - so a component that
        /// reads one re-reads it when the language changes, and there is no
        /// cache anywhere to hold the old words.
        pub fn text(&self, message: Message) -> &'static str {
            message.text(self.locale.get())
        }
    }

    /// A number, formatted by the browser.
    ///
    /// `options` is an `Intl.NumberFormat` options object built by the
    /// application: currency, grouping, how many decimals. Which of those a
    /// number wants is a property of what the number *means*, and only the
    /// application knows that - so the SDK passes it through rather than
    /// guessing, and formats nothing of its own.
    ///
    /// The options have to be ones `Intl` accepts; an object it rejects throws
    /// out of here, exactly as an invalid argument does anywhere else.
    pub fn format_number(locale: Locale, value: f64, options: &js_sys::Object) -> String {
        let locales = js_sys::Array::of1(&JsValue::from_str(locale.tag()));
        let format = js_sys::Intl::NumberFormat::new(&locales, options);
        let formatter = format.format();
        formatter
            .call1(format.as_ref(), &JsValue::from_f64(value))
            .ok()
            .and_then(|text| text.as_string())
            .unwrap_or_else(|| value.to_string())
    }

    /// A date, formatted by the browser.
    ///
    /// `millis` is a Unix timestamp in milliseconds and `options` is an
    /// `Intl.DateTimeFormat` options object from the application - for the
    /// same reason as above, and more so: whether a date needs its year is
    /// something only the screen it is on can say.
    pub fn format_date(locale: Locale, millis: f64, options: &js_sys::Object) -> String {
        let locales = js_sys::Array::of1(&JsValue::from_str(locale.tag()));
        let date = js_sys::Date::new(&JsValue::from_f64(millis));
        let format = js_sys::Intl::DateTimeFormat::new(&locales, options);
        let formatter = format.format();
        formatter
            .call1(format.as_ref(), date.as_ref())
            .ok()
            .and_then(|text| text.as_string())
            .unwrap_or_else(|| date.to_iso_string().into())
    }

    /// Every language tag the browser says its user prefers, in order.
    pub fn browser_languages() -> Vec<String> {
        leptos::web_sys::window()
            .map(|window| window.navigator().languages())
            .map(|languages| {
                languages
                    .iter()
                    .filter_map(|tag| tag.as_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Locale, Message};
    use std::collections::HashSet;

    #[test]
    fn every_message_exists_in_every_language_and_is_its_own() {
        for locale in Locale::all() {
            let texts: Vec<&str> = Message::all()
                .into_iter()
                .map(|message| message.text(locale))
                .collect();
            assert!(
                texts.iter().all(|text| !text.is_empty()),
                "{} has an empty message",
                locale.tag()
            );
            // Two messages with the same words are two screens a reader cannot
            // tell apart, and usually a copied arm that was never edited.
            let unique: HashSet<&&str> = texts.iter().collect();
            assert_eq!(unique.len(), texts.len(), "{} repeats itself", locale.tag());
        }
    }

    #[test]
    fn no_message_was_left_in_the_other_language() {
        // Every arm was filled in for both, and a Chinese catalogue whose
        // entry is still the English sentence is the mistake this catches.
        for message in Message::all() {
            assert_ne!(
                message.text(Locale::English),
                message.text(Locale::Chinese),
                "{} is the same in both languages",
                message.key()
            );
        }
    }

    #[test]
    fn every_message_has_a_name_of_its_own() {
        let keys: HashSet<&str> = Message::all().into_iter().map(Message::key).collect();
        assert_eq!(keys.len(), Message::all().len());
    }

    #[test]
    fn a_language_is_matched_on_its_language_and_not_its_punctuation() {
        assert_eq!(Locale::from_tag("zh-CN"), Some(Locale::Chinese));
        assert_eq!(Locale::from_tag("zh"), Some(Locale::Chinese));
        // What a browser might actually send. Refusing this would be refusing
        // a reader their own language over a subtag.
        assert_eq!(Locale::from_tag("zh-Hans-CN"), Some(Locale::Chinese));
        assert_eq!(Locale::from_tag("zh_TW"), Some(Locale::Chinese));
        assert_eq!(Locale::from_tag("EN-GB"), Some(Locale::English));
        assert_eq!(Locale::from_tag("fr-FR"), None);
        assert_eq!(Locale::from_tag(""), None);
    }

    #[test]
    fn the_first_language_we_have_wins_and_english_is_the_fallback() {
        assert_eq!(
            Locale::best_of(["fr-FR", "zh-CN", "en"]),
            Locale::Chinese,
            "preference order is the browser's, not ours"
        );
        assert_eq!(Locale::best_of(["de", "it"]), Locale::English);
        assert_eq!(Locale::best_of([]), Locale::English);
    }

    #[test]
    fn a_language_says_its_own_name_in_itself() {
        assert_eq!(Locale::Chinese.endonym(), "中文");
        assert_eq!(Locale::English.endonym(), "English");
        assert_eq!(Locale::Chinese.tag(), "zh-CN");
        assert_eq!(Locale::English.direction(), Direction::LeftToRight);
        assert_eq!(Direction::RightToLeft.attribute(), "rtl");
    }
}
