use crate::document::Node;

/// Supplies browser font metrics and optionally browser locale/segmentation behavior.
pub trait Measure {
    /// Measures with this node's font and letter spacing already applied.
    fn width(&self, node: &Node, text: &str) -> f64;

    fn segments<'a>(&self, text: &'a str) -> Vec<&'a str> {
        use unicode_segmentation::UnicodeSegmentation;
        text.graphemes(true).collect()
    }

    fn display_text(&self, node: &Node) -> String {
        display_text(node)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextLayout {
    pub lines: Vec<String>,
    pub line_height: f64,
    pub height: f64,
    pub baseline: f64,
}

pub fn layout_text(node: &Node, measure: &(impl Measure + ?Sized)) -> TextLayout {
    let max_width = node.w.max(1.0);
    let mut lines = Vec::new();
    for paragraph in measure.display_text(node).split('\n') {
        if paragraph.is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut line = String::new();
        for word in words(paragraph) {
            let candidate = format!("{line}{word}");
            if measure.width(node, &candidate) <= max_width {
                line.push_str(word);
                continue;
            }
            if !line.trim_matches(is_whitespace).is_empty() {
                lines.push(line.trim_end_matches(is_whitespace).to_owned());
                line.clear();
            }
            let word = word.trim_start_matches(is_whitespace);
            if measure.width(node, word) <= max_width {
                line = word.to_owned();
                continue;
            }
            for segment in measure.segments(word) {
                if !line.is_empty() && measure.width(node, &format!("{line}{segment}")) > max_width
                {
                    lines.push(std::mem::take(&mut line));
                }
                line.push_str(segment);
            }
        }
        lines.push(line.trim_end_matches(is_whitespace).to_owned());
    }
    let font_size = nonzero(node.font_size, 16.0);
    let line_height = font_size * nonzero(node.line_height, 1.35);
    TextLayout {
        height: line_height.max(lines.len() as f64 * line_height),
        baseline: (line_height - font_size) / 2.0 + font_size * 0.82,
        lines,
        line_height,
    }
}

pub fn display_text(node: &Node) -> String {
    match node.text_case.as_str() {
        "upper" => node.text.to_uppercase(),
        "lower" => node.text.to_lowercase(),
        "title" => title_case(&node.text),
        _ => node.text.clone(),
    }
}

pub fn font_spec(node: &Node) -> String {
    let family = if node.font_family == "Inter" {
        "Inter, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Arial, sans-serif".to_owned()
    } else {
        format!(
            "\"{}\", sans-serif",
            node.font_family.replace(['"', '\\'], "")
        )
    };
    let style = if node.font_style.is_empty() {
        "normal"
    } else {
        &node.font_style
    };
    format!(
        "{style} {} {}px {family}",
        nonzero(node.font_weight, 400.0),
        nonzero(node.font_size, 16.0)
    )
}

pub fn character_segments(text: &str) -> Vec<&str> {
    text.char_indices()
        .map(|(index, ch)| &text[index..index + ch.len_utf8()])
        .collect()
}

fn nonzero(value: f64, default: f64) -> f64 {
    if value == 0.0 {
        default
    } else {
        value
    }
}

fn is_whitespace(ch: char) -> bool {
    matches!(ch, '\u{0009}'..='\u{000d}' | ' ' | '\u{00a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}

fn words(text: &str) -> impl Iterator<Item = &str> {
    let mut chars = text.char_indices().peekable();
    std::iter::from_fn(move || {
        let (start, first) = chars.next()?;
        while chars
            .peek()
            .is_some_and(|(_, ch)| is_whitespace(*ch) == is_whitespace(first))
        {
            chars.next();
        }
        let end = chars.peek().map_or(text.len(), |(index, _)| *index);
        Some(&text[start..end])
    })
}

fn title_case(text: &str) -> String {
    use unicode_general_category::{get_general_category, GeneralCategory};
    let is_letter = |ch| {
        matches!(
            get_general_category(ch),
            GeneralCategory::UppercaseLetter
                | GeneralCategory::LowercaseLetter
                | GeneralCategory::TitlecaseLetter
                | GeneralCategory::ModifierLetter
                | GeneralCategory::OtherLetter
        )
    };
    let is_mark = |ch| {
        matches!(
            get_general_category(ch),
            GeneralCategory::NonspacingMark
                | GeneralCategory::SpacingMark
                | GeneralCategory::EnclosingMark
        )
    };
    let mut output = String::with_capacity(text.len());
    let mut chars = text.char_indices().peekable();
    while let Some((index, ch)) = chars.next() {
        if !is_letter(ch) {
            output.push(ch);
            continue;
        }
        let tail = index + ch.len_utf8();
        while chars
            .peek()
            .is_some_and(|(_, next)| is_letter(*next) || is_mark(*next))
        {
            chars.next();
        }
        let end = chars.peek().map_or(text.len(), |(index, _)| *index);
        // The reference uppercases s[0], which is one UTF-16 code unit.
        if ch.len_utf16() == 2 {
            output.push(ch);
        } else {
            output.extend(ch.to_uppercase());
        }
        output.push_str(&text[tail..end].to_lowercase());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use unicode_segmentation::UnicodeSegmentation;

    struct FixedWidth;

    impl Measure for FixedWidth {
        fn width(&self, node: &Node, text: &str) -> f64 {
            text.graphemes(true).count() as f64 * (10.0 + node.letter_spacing)
        }
    }

    fn text(value: &str, width: f64) -> Node {
        let mut node = Node::new("text");
        node.text = value.into();
        node.w = width;
        node.font_size = 20.0;
        node.line_height = 1.5;
        node
    }

    #[test]
    fn preserves_empty_paragraphs_and_terminal_newline() {
        let layout = layout_text(&text("first\n\nlast\n", 200.0), &FixedWidth);
        assert_eq!(layout.lines, ["first", "", "last", ""]);
    }

    #[test]
    fn wraps_words_and_trims_only_at_line_boundaries() {
        let layout = layout_text(&text("  one  two  three  ", 100.0), &FixedWidth);
        assert_eq!(layout.lines, ["  one  two", "three"]);
    }

    #[test]
    fn breaks_oversized_words_at_grapheme_boundaries() {
        let layout = layout_text(&text("e\u{301}👩‍👩‍👧‍👦🇨🇳字", 20.0), &FixedWidth);
        assert_eq!(layout.lines, ["e\u{301}👩‍👩‍👧‍👦", "🇨🇳字"]);
    }

    #[test]
    fn falls_back_to_unicode_characters_when_segmenter_is_unavailable() {
        struct Characters;
        impl Measure for Characters {
            fn width(&self, _: &Node, text: &str) -> f64 {
                text.chars().count() as f64 * 10.0
            }
            fn segments<'a>(&self, text: &'a str) -> Vec<&'a str> {
                character_segments(text)
            }
        }
        let layout = layout_text(&text("e\u{301}😀", 10.0), &Characters);
        assert_eq!(layout.lines, ["e", "\u{301}", "😀"]);
    }

    #[test]
    fn never_splits_a_grapheme_even_when_it_exceeds_available_width() {
        assert_eq!(
            layout_text(&text("🧑‍💻字", 0.0), &FixedWidth).lines,
            ["🧑‍💻", "字"]
        );
    }

    #[test]
    fn includes_letter_spacing_in_the_injected_measurement() {
        let mut node = text("abcd", 40.0);
        node.letter_spacing = 2.0;
        assert_eq!(layout_text(&node, &FixedWidth).lines, ["abc", "d"]);
    }

    #[test]
    fn uses_javascript_whitespace_rules() {
        assert_eq!(
            layout_text(&text("a\u{feff}b\u{85}c", 30.0), &FixedWidth).lines,
            ["a", "b\u{85}c"]
        );
    }

    #[test]
    fn calculates_line_height_total_height_and_baseline() {
        let layout = layout_text(&text("one\ntwo", 100.0), &FixedWidth);
        assert_eq!(
            (layout.line_height, layout.height, layout.baseline),
            (30.0, 60.0, 21.4)
        );
    }

    #[test]
    fn empty_text_still_occupies_a_line_and_zero_metrics_use_defaults() {
        let mut node = text("", 100.0);
        node.font_size = 0.0;
        node.line_height = 0.0;
        let layout = layout_text(&node, &FixedWidth);
        assert_eq!(
            (layout.lines, layout.line_height, layout.height),
            (vec![String::new()], 21.6, 21.6)
        );
    }

    #[test]
    fn transforms_unicode_upper_lower_and_title_case() {
        let mut node = text("hÉLLo e\u{301}COLE 42wORLD straße", 100.0);
        node.text_case = "upper".into();
        assert_eq!(display_text(&node), "HÉLLO E\u{301}COLE 42WORLD STRASSE");
        node.text_case = "lower".into();
        assert_eq!(display_text(&node), "héllo e\u{301}cole 42world straße");
        node.text_case = "title".into();
        assert_eq!(display_text(&node), "Héllo E\u{301}cole 42World Straße");
    }

    #[test]
    fn allows_the_browser_measurement_provider_to_apply_its_locale() {
        struct Turkish;
        impl Measure for Turkish {
            fn width(&self, _: &Node, text: &str) -> f64 {
                text.len() as f64
            }
            fn display_text(&self, _: &Node) -> String {
                "İSTANBUL".into()
            }
        }
        assert_eq!(
            layout_text(&text("istanbul", 100.0), &Turkish).lines,
            ["İSTANBUL"]
        );
    }

    #[test]
    fn uses_inter_system_stack_and_sanitizes_custom_font_names() {
        let mut node = text("", 100.0);
        assert_eq!(font_spec(&node), "normal 400 20px Inter, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Arial, sans-serif");
        node.font_family = "Cus\"tom\\ Font".into();
        node.font_style = "italic".into();
        node.font_weight = 700.0;
        assert_eq!(
            font_spec(&node),
            "italic 700 20px \"Custom Font\", sans-serif"
        );
    }
}
