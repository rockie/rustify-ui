//! A colour the application owns, typed as hex.
//!
//! Built on the SDK's `Draft` for the same reason as the number field: `#1f`
//! is on its way to a colour without being one, and a field that put the
//! application's value back after every keystroke would never let anyone type
//! a colour at all. Beside the text is a swatch of the colour the application
//! holds, painted through the CSS object model - a strict policy refuses a
//! `style` attribute, and the value is not one a class could name.

/// The colour `text` spells, as lower-case `#rrggbb`, or `#rrggbbaa` when
/// `alpha` allows an opacity.
///
/// The `#` is optional and blanks around the digits are ignored, along with a
/// byte-order mark a paste can carry. Three digits are shorthand for six and,
/// with `alpha`, four for eight, the way CSS reads them.
pub fn parse_hex(text: &str, alpha: bool) -> Option<String> {
    let text = text.trim_matches(|ch: char| ch.is_whitespace() || ch == '\u{feff}');
    let digits = text.strip_prefix('#').unwrap_or(text);
    if !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    let digits = digits.to_ascii_lowercase();
    match (digits.len(), alpha) {
        (6, _) | (8, true) => Some(format!("#{digits}")),
        (3, _) | (4, true) => Some(format!(
            "#{}",
            digits.chars().flat_map(|ch| [ch, ch]).collect::<String>()
        )),
        _ => None,
    }
}

#[cfg(target_arch = "wasm32")]
pub use dom::ColorField;

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::parse_hex;
    use leptos::prelude::*;
    use rustify_ui::Draft;

    const FIELD: &str = "flex items-center gap-2";
    const INPUT: &str = "flex h-9 w-full min-w-0 rounded-md border border-border bg-input text-foreground px-3 py-1 font-mono text-sm transition-colors outline-none placeholder:text-muted-foreground focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 read-only:bg-muted aria-invalid:border-destructive aria-invalid:ring-destructive/40";
    const SWATCH: &str = "size-9 shrink-0 rounded-md border border-border";

    /// A field for one colour, with a swatch of it.
    ///
    /// The swatch is decoration: the text is the value, and the text is what
    /// a reader hears. A disabled or read-only field asks for nothing.
    #[component]
    pub fn ColorField(
        /// The colour as the application holds it: what the field shows and
        /// the swatch paints.
        #[prop(into)]
        value: Signal<String>,
        /// Asked for the colour an edit ended on, as `parse_hex` writes it.
        on_change: impl Fn(String) + Send + Sync + 'static,
        /// Asked for every colour typed while the edit goes on.
        #[prop(optional, into)]
        on_preview: Option<Callback<String>>,
        /// Asked to take back the previews of an edit Escape abandoned.
        #[prop(optional, into)]
        on_cancel: Option<Callback<()>>,
        /// Whether eight digits, an opacity after the colour, are a colour.
        #[prop(optional)]
        alpha: bool,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        /// The application's own verdict on the colour it holds. A draft that
        /// does not parse is marked invalid without being asked.
        #[prop(optional, into)]
        invalid: Signal<bool>,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        let id = if id.is_empty() {
            crate::id::next("color")
        } else {
            id
        };
        let aria_label = (!aria_label.is_empty()).then_some(aria_label);
        let swatch_id = format!("{test_id}-swatch");
        let draft = Draft::new(value, String::clone, move |text| parse_hex(text, alpha))
            .with_commit(on_change)
            .with_preview(move |value| {
                if let Some(preview) = on_preview {
                    preview.run(value);
                }
            })
            .with_cancel(move || {
                if let Some(cancel) = on_cancel {
                    cancel.run(());
                }
            });
        let still = move || disabled.get_untracked() || read_only.get_untracked();
        let keydown = draft.on_keydown();
        let input = draft.on_input();
        let invalid_draft = draft.invalid();
        view! {
            <div class=crate::macros::merge(FIELD, &class) data-name="ColorField">
                <span
                    class=SWATCH
                    data-name="ColorSwatch"
                    data-testid=swatch_id
                    aria-hidden="true"
                    style:background-color=move || value.get()
                />
                <input
                    type="text"
                    autocomplete="off"
                    spellcheck="false"
                    id=id
                    class=INPUT
                    data-testid=test_id
                    aria-label=aria_label
                    aria-invalid=move || (invalid.get() || invalid_draft.get()).then_some("true")
                    aria-readonly=move || read_only.get().then_some("true")
                    aria-describedby=move || {
                        let described_by = described_by.get();
                        (!described_by.is_empty()).then_some(described_by)
                    }
                    prop:value=draft.text()
                    prop:disabled=move || disabled.get()
                    prop:readOnly=move || read_only.get()
                    on:focus=draft.on_focus()
                    on:blur=draft.on_blur()
                    on:keydown=move |event| {
                        if !still() {
                            keydown(event);
                        }
                    }
                    on:input=move |event| {
                        if !still() {
                            input(event);
                        }
                    }
                />
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_hex;

    #[test]
    fn six_digits_are_a_colour_with_or_without_the_hash() {
        assert_eq!(parse_hex("#1F6FEB", false).as_deref(), Some("#1f6feb"));
        assert_eq!(parse_hex("1f6feb", false).as_deref(), Some("#1f6feb"));
        assert_eq!(
            parse_hex(" \u{feff}#1f6feb\n", false).as_deref(),
            Some("#1f6feb")
        );
    }

    #[test]
    fn three_digits_are_shorthand_for_six() {
        assert_eq!(parse_hex("#abc", false).as_deref(), Some("#aabbcc"));
        assert_eq!(parse_hex("F0A", false).as_deref(), Some("#ff00aa"));
    }

    #[test]
    fn an_opacity_is_a_colour_only_where_alpha_is_allowed() {
        assert_eq!(parse_hex("#1f6feb80", false), None);
        assert_eq!(parse_hex("#abcd", false), None);
        assert_eq!(parse_hex("#1f6feb80", true).as_deref(), Some("#1f6feb80"));
        assert_eq!(parse_hex("#abcd", true).as_deref(), Some("#aabbccdd"));
        // And alpha does not stop an opaque colour from being one.
        assert_eq!(parse_hex("#1f6feb", true).as_deref(), Some("#1f6feb"));
    }

    #[test]
    fn text_on_its_way_to_a_colour_is_not_one_yet() {
        for text in [
            "", "#", "#1", "#1f", "#1f6fe", "#1f6febx", "#ggg", "red", "# 1f6feb",
        ] {
            assert_eq!(parse_hex(text, true), None, "{text:?}");
            assert_eq!(parse_hex(text, false), None, "{text:?}");
        }
    }
}
