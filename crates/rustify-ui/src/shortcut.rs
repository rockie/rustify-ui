//! Keyboard shortcuts: written once as text, matched against what the browser
//! reports, and kept away from the keys a text field needs.
//!
//! A shortcut is written the way menus print it - `Mod+Shift+K` - and parsed
//! once. `Mod` is the command key of the platform the page is running on:
//! Meta on a Mac, Control everywhere else. It is decided once per page, so a
//! table of shortcuts means the same thing for as long as the page lives.
//!
//! Matching reads `KeyboardEvent.key`, the character the layout produced, so
//! `Mod+Z` is the key labelled Z on an AZERTY keyboard as much as on a QWERTY
//! one. Where the layout produced something else for a letter or a digit - a
//! Cyrillic layout, Option on a Mac composing a symbol, Shift turning `1` into
//! `!` - the physical key stands in, which is what a person pressing "Mod+K" on
//! those keyboards means.

use std::fmt;

/// Which key `Mod` stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Platform {
    /// macOS, and iPadOS with a keyboard: the command key is Meta.
    Mac,
    /// Everything else: the command key is Control.
    Other,
}

impl Platform {
    /// The platform this page runs on, decided on first use and never again.
    ///
    /// Outside a browser there is nothing to ask, and the answer is
    /// [`Platform::Other`]; code that needs a particular platform says so
    /// through [`Shortcut::parse_for`].
    pub fn current() -> Platform {
        #[cfg(target_arch = "wasm32")]
        {
            static CURRENT: std::sync::OnceLock<Platform> = std::sync::OnceLock::new();
            *CURRENT.get_or_init(|| {
                let platform = leptos::web_sys::window()
                    .and_then(|window| window.navigator().platform().ok())
                    .unwrap_or_default();
                // `MacIntel` on every Mac, and on an iPad asking for desktop
                // pages; `iPad`, `iPhone` or `iPod` otherwise. All of them
                // put the command key where Meta is.
                if platform.starts_with("Mac") || platform.starts_with("iP") {
                    Platform::Mac
                } else {
                    Platform::Other
                }
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Platform::Other
        }
    }
}

/// Which modifier keys are held.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}

/// A key press as plain data: what `KeyboardEvent` says, without the event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Keystroke<'a> {
    /// `KeyboardEvent.key`: what the layout produced.
    pub key: &'a str,
    /// `KeyboardEvent.code`: which physical key it was.
    pub code: &'a str,
    pub modifiers: Modifiers,
}

/// Why a shortcut's text does not describe one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShortcutError {
    /// Nothing but white space.
    Empty,
    /// Modifiers and no key, such as `Ctrl+Shift` or `Ctrl+`.
    MissingKey,
    /// A part before the key that is not a modifier.
    UnknownModifier(String),
    /// The same modifier twice.
    RepeatedModifier(String),
    /// A key name longer than one character that is not a named key.
    UnknownKey(String),
}

impl fmt::Display for ShortcutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "a shortcut needs a key"),
            Self::MissingKey => write!(f, "a shortcut needs a key after its modifiers"),
            Self::UnknownModifier(part) => write!(f, "`{part}` is not a modifier"),
            Self::RepeatedModifier(part) => write!(f, "`{part}` is named twice"),
            Self::UnknownKey(part) => write!(f, "`{part}` is not a key"),
        }
    }
}

impl std::error::Error for ShortcutError {}

/// One key and the modifiers held with it, with `Mod` already resolved.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Shortcut {
    /// The `KeyboardEvent.key` value it answers to, a letter in lower case.
    key: String,
    modifiers: Modifiers,
}

/// Named keys, by every name a person is likely to write, and the value
/// `KeyboardEvent.key` reports for each.
const NAMED_KEYS: &[(&str, &str)] = &[
    ("escape", "Escape"),
    ("esc", "Escape"),
    ("enter", "Enter"),
    ("return", "Enter"),
    ("tab", "Tab"),
    ("space", " "),
    ("backspace", "Backspace"),
    ("delete", "Delete"),
    ("del", "Delete"),
    ("insert", "Insert"),
    ("home", "Home"),
    ("end", "End"),
    ("pageup", "PageUp"),
    ("pagedown", "PageDown"),
    ("arrowup", "ArrowUp"),
    ("up", "ArrowUp"),
    ("arrowdown", "ArrowDown"),
    ("down", "ArrowDown"),
    ("arrowleft", "ArrowLeft"),
    ("left", "ArrowLeft"),
    ("arrowright", "ArrowRight"),
    ("right", "ArrowRight"),
    // `+` separates the parts, so the key itself is written by name - or as
    // the last of two, as in `Ctrl++`.
    ("plus", "+"),
];

#[derive(Clone, Copy)]
enum Part {
    Mod,
    Ctrl,
    Alt,
    Shift,
    Meta,
}

fn modifier(part: &str) -> Option<Part> {
    Some(match part.to_ascii_lowercase().as_str() {
        "mod" => Part::Mod,
        "ctrl" | "control" => Part::Ctrl,
        "alt" | "option" | "opt" => Part::Alt,
        "shift" => Part::Shift,
        "meta" | "cmd" | "command" => Part::Meta,
        _ => return None,
    })
}

/// The `KeyboardEvent.key` value a key name stands for.
fn key_value(name: &str) -> Result<String, ShortcutError> {
    let mut chars = name.chars();
    if let (Some(only), None) = (chars.next(), chars.next()) {
        return Ok(only.to_lowercase().collect());
    }
    let lower = name.to_ascii_lowercase();
    if let Some((_, value)) = NAMED_KEYS.iter().find(|(alias, _)| *alias == lower) {
        return Ok((*value).to_owned());
    }
    // Function keys, F1 to F24.
    if let Some(number) = lower
        .strip_prefix('f')
        .and_then(|digits| digits.parse::<u8>().ok())
        .filter(|number| (1..=24).contains(number))
    {
        return Ok(format!("F{number}"));
    }
    Err(ShortcutError::UnknownKey(name.to_owned()))
}

/// The physical key for a letter or a digit, in `KeyboardEvent.code` terms.
fn physical(key: &str) -> Option<String> {
    let mut chars = key.chars();
    let (Some(only), None) = (chars.next(), chars.next()) else {
        return None;
    };
    if only.is_ascii_lowercase() {
        Some(format!("Key{}", only.to_ascii_uppercase()))
    } else if only.is_ascii_digit() {
        Some(format!("Digit{only}"))
    } else {
        None
    }
}

fn single_char(text: &str) -> Option<char> {
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(only), None) => Some(only),
        _ => None,
    }
}

impl Shortcut {
    /// Parses `text` for the platform this page runs on.
    pub fn parse(text: &str) -> Result<Shortcut, ShortcutError> {
        Self::parse_for(text, Platform::current())
    }

    /// Parses `text` with `Mod` standing for `platform`'s command key.
    ///
    /// Modifier and key names are case-insensitive. The last part is the key:
    /// a single character, or a named key such as `Escape`, `ArrowUp`, `F2`
    /// or `Space`.
    pub fn parse_for(text: &str, platform: Platform) -> Result<Shortcut, ShortcutError> {
        let text = text.trim();
        if text.is_empty() {
            return Err(ShortcutError::Empty);
        }
        let (parts, key) = if text == "+" {
            (None, "+")
        } else if let Some(parts) = text.strip_suffix("++") {
            (Some(parts), "+")
        } else {
            match text.rsplit_once('+') {
                Some((parts, key)) => (Some(parts), key.trim()),
                None => (None, text),
            }
        };
        if key.is_empty() || modifier(key).is_some() {
            return Err(ShortcutError::MissingKey);
        }

        let mut modifiers = Modifiers::default();
        // By name rather than by effect: `Mod+Ctrl` is two names for Control
        // on one platform and two different keys on the other, and a table of
        // shortcuts must not parse on one and fail on the other.
        let mut named = [false; 5];
        for part in parts.into_iter().flat_map(|parts| parts.split('+')) {
            let part = part.trim();
            let which =
                modifier(part).ok_or_else(|| ShortcutError::UnknownModifier(part.into()))?;
            if std::mem::replace(&mut named[which as usize], true) {
                return Err(ShortcutError::RepeatedModifier(part.into()));
            }
            match which {
                Part::Mod if platform == Platform::Mac => modifiers.meta = true,
                Part::Mod | Part::Ctrl => modifiers.ctrl = true,
                Part::Alt => modifiers.alt = true,
                Part::Shift => modifiers.shift = true,
                Part::Meta => modifiers.meta = true,
            }
        }
        Ok(Shortcut {
            key: key_value(key)?,
            modifiers,
        })
    }

    /// Whether `stroke` is this shortcut.
    ///
    /// Modifiers must match exactly, with one exception: a shortcut on a
    /// symbol or a digit does not care about Shift unless it names it, because
    /// whether typing `?` or `1` takes Shift is up to the layout.
    pub fn matches_keystroke(&self, stroke: &Keystroke) -> bool {
        let wanted = self.modifiers;
        let held = stroke.modifiers;
        if (held.ctrl, held.alt, held.meta) != (wanted.ctrl, wanted.alt, wanted.meta) {
            return false;
        }
        let by_value = match single_char(&self.key) {
            Some(_) => stroke.key.to_lowercase() == self.key,
            None => stroke.key == self.key,
        };
        if by_value {
            let layout_decides = single_char(&self.key)
                .is_some_and(|key| !key.is_alphabetic() && !key.is_whitespace());
            return held.shift == wanted.shift || (layout_decides && !wanted.shift);
        }
        // The layout produced something other than a letter or a digit for
        // this key, so the key's position is the best evidence left.
        let produced_alphanumeric =
            single_char(stroke.key).is_some_and(|key| key.is_ascii_alphanumeric());
        !produced_alphanumeric
            && held.shift == wanted.shift
            && physical(&self.key).is_some_and(|code| code == stroke.code)
    }
}

/// The shortcut in `aria-keyshortcuts` form, such as `Control+Shift+K`, which
/// parses back to the same shortcut on any platform.
impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Modifiers {
            ctrl,
            alt,
            shift,
            meta,
        } = self.modifiers;
        for (held, name) in [
            (ctrl, "Control"),
            (alt, "Alt"),
            (shift, "Shift"),
            (meta, "Meta"),
        ] {
            if held {
                write!(f, "{name}+")?;
            }
        }
        match self.key.as_str() {
            " " => write!(f, "Space"),
            "+" => write!(f, "Plus"),
            key => match single_char(key) {
                Some(only) => write!(f, "{}", only.to_uppercase()),
                None => write!(f, "{key}"),
            },
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use dom::is_text_entry;

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::{Keystroke, Modifiers, Shortcut};
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys::{EventTarget, HtmlElement, KeyboardEvent};

    impl Shortcut {
        /// Whether `event` is this shortcut. A key pressed while an input
        /// method is composing belongs to the composition, and is never one.
        pub fn matches(&self, event: &KeyboardEvent) -> bool {
            if event.is_composing() {
                return false;
            }
            let key = event.key();
            let code = event.code();
            self.matches_keystroke(&Keystroke {
                key: &key,
                code: &code,
                modifiers: Modifiers {
                    ctrl: event.ctrl_key(),
                    alt: event.alt_key(),
                    shift: event.shift_key(),
                    meta: event.meta_key(),
                },
            })
        }
    }

    /// Whether `target` is somewhere a person types: an input, a text area, a
    /// select, or editable content.
    ///
    /// A shortcut without a modifier is text there - `v` is the letter v, and
    /// Backspace deletes a character, not the selection on a canvas - so a
    /// page-level handler asks this before it acts on one.
    pub fn is_text_entry(target: &EventTarget) -> bool {
        target.dyn_ref::<HtmlElement>().is_some_and(|element| {
            element.is_content_editable()
                || matches!(element.tag_name().as_str(), "INPUT" | "TEXTAREA" | "SELECT")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Keystroke, Modifiers, Platform, Shortcut, ShortcutError};

    const NONE: Modifiers = Modifiers {
        ctrl: false,
        alt: false,
        shift: false,
        meta: false,
    };
    const CTRL: Modifiers = Modifiers { ctrl: true, ..NONE };
    const META: Modifiers = Modifiers { meta: true, ..NONE };
    const SHIFT: Modifiers = Modifiers {
        shift: true,
        ..NONE
    };
    const ALT: Modifiers = Modifiers { alt: true, ..NONE };
    const CTRL_SHIFT: Modifiers = Modifiers {
        ctrl: true,
        shift: true,
        ..NONE
    };
    const META_SHIFT: Modifiers = Modifiers {
        meta: true,
        shift: true,
        ..NONE
    };
    const META_ALT: Modifiers = Modifiers {
        meta: true,
        alt: true,
        ..NONE
    };

    fn parse(text: &str, platform: Platform) -> Shortcut {
        Shortcut::parse_for(text, platform)
            .unwrap_or_else(|error| panic!("{text:?} should parse: {error}"))
    }

    #[test]
    fn mod_is_the_command_key_of_the_platform_it_was_parsed_for() {
        assert_eq!(
            parse("Mod+K", Platform::Mac),
            parse("Meta+K", Platform::Mac)
        );
        assert_eq!(
            parse("Mod+K", Platform::Other),
            parse("Ctrl+K", Platform::Other)
        );
        assert_ne!(
            parse("Mod+K", Platform::Mac),
            parse("Mod+K", Platform::Other)
        );
        // Only `Mod` moves: a shortcut that names its key means that key.
        assert_eq!(
            parse("Ctrl+K", Platform::Mac),
            parse("Ctrl+K", Platform::Other)
        );
    }

    #[test]
    fn names_are_read_without_regard_to_case_or_which_alias_was_used() {
        let same = [
            ("Mod+Shift+K", "mod+shift+k"),
            ("Ctrl+K", "Control+K"),
            ("Ctrl+K", "CTRL+k"),
            ("Meta+K", "Cmd+K"),
            ("Meta+K", "Command+K"),
            ("Alt+K", "Option+K"),
            ("Alt+K", "opt+K"),
            ("Shift+Ctrl+K", "Ctrl+Shift+K"),
            (" Ctrl + K ", "Ctrl+K"),
            ("Escape", "Esc"),
            ("Escape", "escape"),
            ("Enter", "Return"),
            ("Delete", "Del"),
            ("ArrowUp", "Up"),
            ("ArrowLeft", "left"),
            ("Space", "space"),
            ("Plus", "+"),
            ("Ctrl+Plus", "Ctrl++"),
            ("f2", "F2"),
        ];
        for (left, right) in same {
            assert_eq!(
                parse(left, Platform::Other),
                parse(right, Platform::Other),
                "{left:?} and {right:?}"
            );
        }
    }

    #[test]
    fn text_that_is_not_a_shortcut_says_why() {
        let refused = [
            ("", ShortcutError::Empty),
            ("   ", ShortcutError::Empty),
            ("Ctrl+", ShortcutError::MissingKey),
            ("Shift", ShortcutError::MissingKey),
            ("Mod+Shift", ShortcutError::MissingKey),
            ("Hyper+K", ShortcutError::UnknownModifier("Hyper".into())),
            ("K+L", ShortcutError::UnknownModifier("K".into())),
            ("Ctrl++K", ShortcutError::UnknownModifier(String::new())),
            (
                "Shift+shift+K",
                ShortcutError::RepeatedModifier("shift".into()),
            ),
            ("Mod+Mod+K", ShortcutError::RepeatedModifier("Mod".into())),
            ("Ctrl+Foo", ShortcutError::UnknownKey("Foo".into())),
            ("F25", ShortcutError::UnknownKey("F25".into())),
            ("F0", ShortcutError::UnknownKey("F0".into())),
        ];
        for (text, error) in refused {
            assert_eq!(
                Shortcut::parse_for(text, Platform::Other),
                Err(error),
                "{text:?}"
            );
        }
    }

    #[test]
    fn a_shortcut_matches_the_keys_the_browser_reports_for_it() {
        // (shortcut, platform, key, code, modifiers, matches)
        let table: &[(&str, Platform, &str, &str, Modifiers, bool)] = &[
            // The command key, per platform.
            ("Mod+K", Platform::Other, "k", "KeyK", CTRL, true),
            ("Mod+K", Platform::Other, "k", "KeyK", META, false),
            ("Mod+K", Platform::Mac, "k", "KeyK", META, true),
            ("Mod+K", Platform::Mac, "k", "KeyK", CTRL, false),
            // Modifiers are exact for a letter: Shift is part of the shortcut.
            ("Mod+K", Platform::Other, "K", "KeyK", CTRL_SHIFT, false),
            (
                "Mod+Shift+K",
                Platform::Other,
                "K",
                "KeyK",
                CTRL_SHIFT,
                true,
            ),
            ("Mod+Shift+K", Platform::Mac, "k", "KeyK", META_SHIFT, true),
            ("Mod+Shift+K", Platform::Other, "k", "KeyK", CTRL, false),
            ("K", Platform::Other, "k", "KeyK", NONE, true),
            ("K", Platform::Other, "k", "KeyK", CTRL, false),
            ("K", Platform::Other, "k", "KeyK", ALT, false),
            // The layout decides the letter: AZERTY's A sits where QWERTY's
            // Q is, and `Mod+A` is the key that types `a`.
            ("Mod+A", Platform::Other, "a", "KeyQ", CTRL, true),
            ("Mod+Q", Platform::Other, "a", "KeyQ", CTRL, false),
            // A layout with no Latin letters, and Option composing a symbol
            // on a Mac: the physical key stands in.
            ("Mod+K", Platform::Other, "л", "KeyK", CTRL, true),
            ("Mod+Alt+K", Platform::Mac, "˚", "KeyK", META_ALT, true),
            ("Mod+Alt+J", Platform::Mac, "˚", "KeyK", META_ALT, false),
            // Shift on a digit or a symbol is the layout's business...
            ("?", Platform::Other, "?", "Slash", SHIFT, true),
            ("?", Platform::Other, "?", "Minus", NONE, true),
            ("Mod+1", Platform::Other, "1", "Digit1", CTRL_SHIFT, true),
            ("Mod+1", Platform::Other, "&", "Digit1", CTRL, true),
            // ...unless the shortcut names it, and then it is the key's
            // position: US Shift+1 types `!`.
            ("Shift+1", Platform::Other, "!", "Digit1", SHIFT, true),
            ("1", Platform::Other, "!", "Digit1", SHIFT, false),
            ("1", Platform::Other, "1", "Digit1", NONE, true),
            // Named keys, exactly.
            ("Escape", Platform::Other, "Escape", "Escape", NONE, true),
            ("Escape", Platform::Other, "Escape", "Escape", SHIFT, false),
            ("Space", Platform::Other, " ", "Space", NONE, true),
            ("Space", Platform::Other, " ", "Space", SHIFT, false),
            (
                "Shift+ArrowUp",
                Platform::Other,
                "ArrowUp",
                "ArrowUp",
                SHIFT,
                true,
            ),
            ("F2", Platform::Other, "F2", "F2", NONE, true),
            ("Ctrl+Plus", Platform::Other, "+", "Equal", CTRL_SHIFT, true),
            (
                "Delete",
                Platform::Other,
                "Backspace",
                "Backspace",
                NONE,
                false,
            ),
        ];
        for &(text, platform, key, code, modifiers, expected) in table {
            let stroke = Keystroke {
                key,
                code,
                modifiers,
            };
            assert_eq!(
                parse(text, platform).matches_keystroke(&stroke),
                expected,
                "{text:?} on {platform:?} against {stroke:?}"
            );
        }
    }

    #[test]
    fn a_shortcut_prints_in_aria_form_and_reads_back_the_same_anywhere() {
        let printed = [
            ("Mod+Shift+K", Platform::Mac, "Shift+Meta+K"),
            ("Mod+Shift+K", Platform::Other, "Control+Shift+K"),
            ("alt+space", Platform::Other, "Alt+Space"),
            ("Ctrl++", Platform::Other, "Control+Plus"),
            ("esc", Platform::Other, "Escape"),
            ("?", Platform::Other, "?"),
        ];
        for (text, platform, aria) in printed {
            let shortcut = parse(text, platform);
            assert_eq!(shortcut.to_string(), aria);
            for elsewhere in [Platform::Mac, Platform::Other] {
                assert_eq!(
                    parse(aria, elsewhere),
                    shortcut,
                    "{aria:?} on {elsewhere:?}"
                );
            }
        }
    }

    #[test]
    fn outside_a_browser_the_platform_is_the_ordinary_one() {
        assert_eq!(Platform::current(), Platform::Other);
        assert_eq!(
            Shortcut::parse("Mod+K"),
            Shortcut::parse_for("Mod+K", Platform::Other)
        );
    }
}
