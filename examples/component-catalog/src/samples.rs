//! The twenty fixed texts of B5.
//!
//! Fixed, and in one place, because the whole point is comparing: the same
//! twenty strings are drawn by the browser and by the region, and a reviewer
//! judges them against a reference rendering. Two lists that had drifted apart
//! would make the comparison meaningless without making it look wrong.
//!
//! The categories the plan names are Chinese, English, Arabic (right to left),
//! combining marks and family emoji. Each is here more than once, because a
//! script gets a case right or wrong in more than one way: Arabic alone is not
//! the same test as Arabic with Western digits in it, and one combining mark
//! is not the same test as five stacked on one letter.

pub struct Sample {
    /// Stable, and used in a test id.
    pub id: &'static str,
    /// What is being tested, in English, for the reviewer's checklist.
    pub label: &'static str,
    pub text: &'static str,
    /// Whether this text's own direction is right to left. Written on the
    /// element rather than guessed from the characters: `dir="auto"` reads the
    /// first strong character, which is the wrong answer for a line that
    /// starts with a digit or a quotation mark.
    pub rtl: bool,
}

pub const SAMPLES: [Sample; 20] = [
    Sample {
        id: "en-plain",
        label: "English, plain",
        text: "The quick brown fox jumps over the lazy dog.",
        rtl: false,
    },
    Sample {
        id: "en-punctuation",
        label: "English with typographic punctuation",
        text: "\u{201c}Don\u{2019}t\u{2014}really\u{2014}don\u{2019}t,\u{201d} she said\u{2026}",
        rtl: false,
    },
    Sample {
        id: "zh-common",
        label: "Chinese, common characters",
        text: "这是一个用来检查常用汉字排版的句子。",
        rtl: false,
    },
    Sample {
        id: "zh-punctuation",
        label: "Chinese punctuation and brackets",
        text: "他说：「这样——真的可以吗？」（当然可以。）",
        rtl: false,
    },
    Sample {
        id: "zh-latin",
        label: "Chinese with Latin and digits mixed in",
        text: "属性工作台 v0.1.0 已加载 1,000 个对象。",
        rtl: false,
    },
    Sample {
        id: "zh-traditional",
        label: "Traditional Chinese",
        text: "這個介面同時繪製兩種字體的文字。",
        rtl: false,
    },
    Sample {
        id: "zh-vertical-forms",
        label: "Chinese with full-width Latin",
        text: "全角ＡＢＣ与半角ABC并排。",
        rtl: false,
    },
    Sample {
        id: "ar-plain",
        label: "Arabic, right to left",
        text: "النص العربي يُكتب من اليمين إلى اليسار.",
        rtl: true,
    },
    Sample {
        id: "ar-digits",
        label: "Arabic with Western digits (bidirectional numbers)",
        text: "الإصدار 0.1.0 صدر في 2026.",
        rtl: true,
    },
    Sample {
        id: "ar-latin",
        label: "Arabic with a Latin word inside it",
        text: "افتح الملف باسم objects.txt ثم أعد المحاولة.",
        rtl: true,
    },
    Sample {
        id: "he-plain",
        label: "Hebrew, right to left",
        text: "טקסט בעברית נכתב מימין לשמאל.",
        rtl: true,
    },
    Sample {
        id: "combining-acute",
        label: "Combining acute accent (decomposed) beside its composed form",
        text: "e\u{0301} vs \u{00e9}",
        rtl: false,
    },
    Sample {
        id: "combining-stack",
        label: "Five combining marks on one letter",
        text: "a\u{0300}\u{0301}\u{0302}\u{0303}\u{0308}",
        rtl: false,
    },
    Sample {
        id: "combining-devanagari",
        label: "Devanagari cluster",
        text: "क्षि त्र ज्ञ",
        rtl: false,
    },
    Sample {
        id: "combining-thai",
        label: "Thai, marks above and below",
        text: "ที่นี่มีสระและวรรณยุกต์",
        rtl: false,
    },
    Sample {
        id: "emoji-family",
        label: "Family emoji (one grapheme, four people, three joiners)",
        text: "👨‍👩‍👧‍👦 is one character",
        rtl: false,
    },
    Sample {
        id: "emoji-skin-tone",
        label: "Emoji with a skin-tone modifier",
        text: "👍🏽 👋🏿 🧑🏻‍💻",
        rtl: false,
    },
    Sample {
        id: "emoji-flags",
        label: "Flags (pairs of regional indicators)",
        text: "🇨🇳 🇬🇧 🇸🇦",
        rtl: false,
    },
    Sample {
        id: "emoji-in-text",
        label: "Emoji in the middle of a sentence",
        text: "保存成功 ✅，但有 2 个警告 ⚠️。",
        rtl: false,
    },
    Sample {
        id: "mixed-everything",
        label: "Everything at once",
        text: "Rustify 属性工作台 — نص عربي — é 👨‍👩‍👧‍👦 — 1,234.50",
        rtl: false,
    },
];

#[cfg(test)]
mod tests {
    use super::SAMPLES;
    use std::collections::HashSet;

    #[test]
    fn there_are_twenty_of_them() {
        // B5 says twenty. The number is the requirement, not a convenience,
        // because the reference rendering a reviewer compares against has
        // twenty rows in it.
        assert_eq!(SAMPLES.len(), 20);
    }

    #[test]
    fn every_sample_has_a_name_of_its_own_and_something_to_draw() {
        let ids: HashSet<&str> = SAMPLES.iter().map(|sample| sample.id).collect();
        assert_eq!(ids.len(), SAMPLES.len(), "two samples share an id");
        for sample in &SAMPLES {
            assert!(!sample.text.is_empty(), "{} has no text", sample.id);
            assert!(!sample.label.is_empty(), "{} has no label", sample.id);
            // One line each: the two columns are compared row against row, and
            // a sample that wrapped would put the halves out of step.
            assert!(!sample.text.contains('\n'), "{} spans lines", sample.id);
        }
    }

    #[test]
    fn every_category_the_plan_names_is_here_more_than_once() {
        // Chinese, English, right-to-left, combining marks, emoji. More than
        // once each, because a script gets a case right or wrong in more than
        // one way: Arabic alone is not the same test as Arabic with Western
        // digits in it.
        for prefix in ["en-", "zh-", "ar-", "combining-", "emoji-"] {
            let found = SAMPLES
                .iter()
                .filter(|sample| sample.id.starts_with(prefix))
                .count();
            assert!(found >= 2, "only {found} sample(s) for {prefix}");
        }
    }

    #[test]
    fn direction_is_stated_rather_than_guessed() {
        // `dir="auto"` reads the first strong character, which is the wrong
        // answer for a line beginning with a digit or a quotation mark. Every
        // sample says which way it goes, and the right-to-left ones are the
        // ones written in a right-to-left script.
        let rtl: Vec<&str> = SAMPLES
            .iter()
            .filter(|sample| sample.rtl)
            .map(|sample| sample.id)
            .collect();
        assert_eq!(rtl, vec!["ar-plain", "ar-digits", "ar-latin", "he-plain"]);
        // And one that starts with a Latin word but is mostly Chinese is still
        // left to right.
        let mixed = SAMPLES
            .iter()
            .find(|sample| sample.id == "mixed-everything")
            .expect("the last sample");
        assert!(!mixed.rtl);
    }

    #[test]
    fn the_emoji_samples_really_are_the_hard_ones() {
        let family = SAMPLES
            .iter()
            .find(|sample| sample.id == "emoji-family")
            .expect("the family sample");
        // Four people and three zero-width joiners: one grapheme, seven code
        // points. A renderer that draws four faces has failed the test, and
        // this is what makes it a test of that.
        assert_eq!(family.text.matches('\u{200d}').count(), 3);

        let stack = SAMPLES
            .iter()
            .find(|sample| sample.id == "combining-stack")
            .expect("the combining sample");
        assert_eq!(stack.text.chars().count(), 6, "one letter, five marks");
    }
}
