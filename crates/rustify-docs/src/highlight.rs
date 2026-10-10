//! Syntax highlighting at build time, into class names rather than colours.
//!
//! Every token becomes a `<span>` whose classes are the atoms of its scope,
//! each prefixed `hl-` (`source.rust` → `hl-source hl-rust`), the same shape
//! syntect's `ClassStyle::SpacedPrefixed { prefix: "hl-" }` produces. The
//! colours live in a stylesheet of their own ([`theme_css`]), so a page
//! carries no `style` attribute, follows the reader's light or dark mode
//! without a script, and the class names never reach the utility compiler.
//!
//! The output is split into lines, each closing the spans it opened and the
//! next reopening the ones still in force, so a line can be numbered or
//! marked on its own without a scope crossing its edge.

use std::cell::RefCell;
use std::collections::HashMap;

use syntect::parsing::{
    BasicScopeStackOp, ParseState, Scope, ScopeStack, SyntaxReference, SyntaxSet,
};
use syntect::util::LinesWithEndings;

use crate::escape::escape_html_into;

/// The class prefix that keeps highlighting apart from utility classes.
pub const CLASS_PREFIX: &str = "hl-";

/// Which grammars a fragment was highlighted with: part of every cache key,
/// so a grammar update re-highlights instead of serving old markup. Kept in
/// step with the locked versions by a test.
pub const GRAMMAR_VERSION: &str = "syntect-5.3.0+two-face-0.5.2+bat-0.26.1+fancy";

/// A language a code fence can name.
#[derive(Debug, PartialEq, Eq)]
pub struct Language {
    /// The canonical fence tag, which is also what `data-lang` carries.
    pub id: &'static str,
    /// Other tags that mean the same language.
    pub aliases: &'static [&'static str],
    /// The grammar's name in the syntax set.
    pub syntax: &'static str,
    /// What a reader sees in a code block's header.
    pub label: &'static str,
}

/// The languages a fence is highlighted in. Any other tag is shown as plain
/// text; it is not an error, because a fence's tag is also documentation.
pub const LANGUAGES: &[Language] = &[
    Language {
        id: "rust",
        aliases: &["rs"],
        syntax: "Rust",
        label: "Rust",
    },
    Language {
        id: "toml",
        aliases: &[],
        syntax: "TOML",
        label: "TOML",
    },
    Language {
        id: "sh",
        aliases: &["shell", "bash", "zsh"],
        syntax: "Bourne Again Shell (bash)",
        label: "Shell",
    },
    Language {
        id: "json",
        aliases: &[],
        syntax: "JSON",
        label: "JSON",
    },
    Language {
        id: "js",
        aliases: &["javascript", "mjs", "cjs"],
        syntax: "JavaScript",
        label: "JavaScript",
    },
    Language {
        id: "ts",
        aliases: &["typescript"],
        syntax: "TypeScript",
        label: "TypeScript",
    },
    Language {
        id: "html",
        aliases: &[],
        syntax: "HTML",
        label: "HTML",
    },
    Language {
        id: "css",
        aliases: &[],
        syntax: "CSS",
        label: "CSS",
    },
    Language {
        id: "md",
        aliases: &["markdown"],
        syntax: "Markdown",
        label: "Markdown",
    },
    Language {
        id: "yaml",
        aliases: &["yml"],
        syntax: "YAML",
        label: "YAML",
    },
    Language {
        id: "diff",
        aliases: &["patch"],
        syntax: "Diff",
        label: "Diff",
    },
];

/// Finds the language a fence tag names, ignoring case.
pub fn language(tag: &str) -> Option<&'static Language> {
    let tag = tag.trim();
    LANGUAGES.iter().find(|language| {
        language.id.eq_ignore_ascii_case(tag)
            || language
                .aliases
                .iter()
                .any(|alias| alias.eq_ignore_ascii_case(tag))
    })
}

/// The grammars, loaded once and shared by every page of a build.
pub struct Highlighter {
    syntaxes: SyntaxSet,
    /// The grammar for each entry of [`LANGUAGES`], by the same index.
    grammars: Vec<usize>,
}

/// A code block's lines, each one balanced HTML without its newline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Highlighted {
    pub language: Option<&'static Language>,
    pub lines: Vec<String>,
}

impl Highlighter {
    /// Loads the grammars. The set is deserialized lazily, context by
    /// context, so this is cheap; the regexes compile on first use.
    ///
    /// # Panics
    /// When a grammar [`LANGUAGES`] names is missing, which is a defect in
    /// this crate rather than in a site.
    pub fn new() -> Self {
        let syntaxes = two_face::syntax::extra_newlines();
        let grammars = LANGUAGES
            .iter()
            .map(|language| {
                syntaxes
                    .syntaxes()
                    .iter()
                    .position(|syntax| syntax.name == language.syntax)
                    .unwrap_or_else(|| panic!("no grammar named {:?}", language.syntax))
            })
            .collect();
        Self { syntaxes, grammars }
    }

    pub fn syntaxes(&self) -> &SyntaxSet {
        &self.syntaxes
    }

    /// The grammar a language is highlighted with.
    pub fn grammar(&self, language: &Language) -> &SyntaxReference {
        let index = LANGUAGES
            .iter()
            .position(|known| known == language)
            .expect("a language from LANGUAGES");
        &self.syntaxes.syntaxes()[self.grammars[index]]
    }

    /// Highlights `code` as the language `tag` names, or as plain text when
    /// it names none of [`LANGUAGES`].
    pub fn highlight(&self, code: &str, tag: Option<&str>) -> Highlighted {
        let language = tag.and_then(language);
        let lines = language
            .and_then(|language| classed_lines(&self.syntaxes, self.grammar(language), code).ok())
            .unwrap_or_else(|| plain_lines(code));
        Highlighted { language, lines }
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

/// The lines of `code`, without their line endings. A final newline ends the
/// last line rather than starting an empty one.
fn split_lines(code: &str) -> impl Iterator<Item = (&str, &str)> {
    LinesWithEndings::from(code).map(|line| {
        let text = line
            .strip_suffix('\n')
            .map(|text| text.strip_suffix('\r').unwrap_or(text))
            .unwrap_or(line);
        (line, text)
    })
}

fn plain_lines(code: &str) -> Vec<String> {
    split_lines(code)
        .map(|(_, text)| {
            let mut html = String::with_capacity(text.len());
            escape_html_into(&mut html, text);
            html
        })
        .collect()
}

fn classed_lines(
    syntaxes: &SyntaxSet,
    grammar: &SyntaxReference,
    code: &str,
) -> Result<Vec<String>, syntect::Error> {
    let mut state = ParseState::new(grammar);
    let mut stack = ScopeStack::new();
    let mut lines = Vec::new();
    for (line, text) in split_lines(code) {
        let ops = state.parse_line(line, syntaxes)?;
        let mut html = String::with_capacity(text.len() * 2);
        // Where each open span starts, so one that ends up holding no text
        // can be taken back out.
        let mut starts: Vec<usize> = Vec::with_capacity(stack.len() + 4);
        // The end of the last text written: a span that opened at or after
        // it and closes now is empty.
        let mut text_end = 0;
        for scope in stack.as_slice() {
            starts.push(html.len());
            open_span(&mut html, *scope);
        }
        let mut cursor = 0;
        for (index, op) in &ops {
            // An operation after the newline still opens or closes a scope,
            // but the newline itself is not part of the line's markup.
            let index = (*index).min(text.len());
            if index > cursor {
                escape_html_into(&mut html, &text[cursor..index]);
                text_end = html.len();
                cursor = index;
            }
            stack.apply_with_hook(op, |basic, _| match basic {
                BasicScopeStackOp::Push(scope) => {
                    starts.push(html.len());
                    open_span(&mut html, scope);
                }
                BasicScopeStackOp::Pop => match starts.pop() {
                    Some(start) if start >= text_end => html.truncate(start),
                    Some(_) => html.push_str("</span>"),
                    None => {}
                },
            })?;
        }
        if cursor < text.len() {
            escape_html_into(&mut html, &text[cursor..]);
            text_end = html.len();
        }
        while let Some(start) = starts.pop() {
            if start >= text_end {
                html.truncate(start);
            } else {
                html.push_str("</span>");
            }
        }
        lines.push(html);
    }
    Ok(lines)
}

thread_local! {
    /// A scope's class list, once built. Building it reads syntect's global
    /// scope repository behind a lock, which the build's worker threads
    /// would otherwise all queue on.
    static CLASSES: RefCell<HashMap<Scope, Box<str>>> = RefCell::new(HashMap::new());
}

fn open_span(html: &mut String, scope: Scope) {
    html.push_str("<span class=\"");
    CLASSES.with(|classes| {
        let mut classes = classes.borrow_mut();
        let list = classes.entry(scope).or_insert_with(|| {
            let mut list = String::new();
            for (i, atom) in scope.build_string().split('.').enumerate() {
                if i > 0 {
                    list.push(' ');
                }
                list.push_str(CLASS_PREFIX);
                list.push_str(atom);
            }
            list.into_boxed_str()
        });
        html.push_str(list);
    });
    html.push_str("\">");
}

/// One colour for the scopes a selector names, in each mode.
struct Rule {
    /// Scope selectors, as dotted scope prefixes.
    scopes: &'static [&'static str],
    light: &'static str,
    dark: &'static str,
    bold: bool,
    italic: bool,
}

/// The palette. Each colour keeps at least 4.5:1 against its mode's code
/// background ([`CODE_BACKGROUND`]), which a test checks. Later rules win
/// over earlier ones of the same specificity.
const RULES: &[Rule] = &[
    Rule {
        scopes: &["comment", "punctuation.definition.comment"],
        light: "#57606a",
        dark: "#9198a1",
        bold: false,
        italic: true,
    },
    Rule {
        scopes: &[
            "keyword",
            "storage",
            "storage.type",
            "storage.modifier",
            "punctuation.definition.keyword",
        ],
        light: "#b42318",
        dark: "#ff7b72",
        bold: false,
        italic: false,
    },
    Rule {
        scopes: &["string", "punctuation.definition.string"],
        light: "#0a3069",
        dark: "#a5d6ff",
        bold: false,
        italic: false,
    },
    Rule {
        scopes: &["constant", "support.constant", "variable.other.constant"],
        light: "#0550ae",
        dark: "#79c0ff",
        bold: false,
        italic: false,
    },
    Rule {
        scopes: &[
            "entity.name.function",
            "support.function",
            "meta.function-call",
        ],
        light: "#6639ba",
        dark: "#d2a8ff",
        bold: false,
        italic: false,
    },
    Rule {
        scopes: &[
            "entity.name.type",
            "entity.name.class",
            "entity.name.struct",
            "entity.name.enum",
            "entity.name.trait",
            "support.type",
            "support.class",
            "entity.name.tag",
        ],
        light: "#116329",
        dark: "#7ee787",
        bold: false,
        italic: false,
    },
    Rule {
        scopes: &[
            "entity.other.attribute-name",
            "variable.parameter",
            "variable.language",
            "support.type.property-name",
            "meta.mapping.key",
        ],
        light: "#953800",
        dark: "#ffa657",
        bold: false,
        italic: false,
    },
    Rule {
        scopes: &["markup.heading", "entity.name.section"],
        light: "#0550ae",
        dark: "#79c0ff",
        bold: true,
        italic: false,
    },
    Rule {
        scopes: &["markup.bold"],
        light: "inherit",
        dark: "inherit",
        bold: true,
        italic: false,
    },
    Rule {
        scopes: &["markup.italic"],
        light: "inherit",
        dark: "inherit",
        bold: false,
        italic: true,
    },
    Rule {
        scopes: &["markup.inserted", "meta.diff.header.to-file"],
        light: "#116329",
        dark: "#7ee787",
        bold: false,
        italic: false,
    },
    Rule {
        scopes: &["markup.deleted", "meta.diff.header.from-file"],
        light: "#a40e26",
        dark: "#ffa198",
        bold: false,
        italic: false,
    },
    Rule {
        scopes: &["meta.diff.range", "meta.diff.header"],
        light: "#6639ba",
        dark: "#d2a8ff",
        bold: false,
        italic: false,
    },
    Rule {
        scopes: &["invalid"],
        light: "#a40e26",
        dark: "#ffa198",
        bold: false,
        italic: false,
    },
];

/// The code background each palette is checked against, light then dark.
/// The default theme's stylesheet paints code blocks with these.
pub const CODE_BACKGROUND: (&str, &str) = ("#f2f4f7", "#161b26");

/// The selector for a dotted scope: every atom as a class.
fn selector(scope: &str) -> String {
    let mut selector = String::new();
    for atom in scope.split('.') {
        selector.push('.');
        selector.push_str(CLASS_PREFIX);
        selector.push_str(atom);
    }
    selector
}

fn declarations(color: &str, rule: &Rule) -> String {
    let mut body = format!("color:{color}");
    if rule.bold {
        body.push_str(";font-weight:600");
    }
    if rule.italic {
        body.push_str(";font-style:italic");
    }
    body
}

/// The highlighting stylesheet: the light palette, then the dark one for a
/// page in dark mode and for a page following a system in dark mode.
///
/// `dark_root` and `system_root` are the selectors of those two roots.
pub fn theme_css(dark_root: &str, system_root: &str) -> String {
    let mut css = String::new();
    for rule in RULES {
        let selectors: Vec<String> = rule.scopes.iter().map(|scope| selector(scope)).collect();
        css.push_str(&selectors.join(","));
        css.push('{');
        css.push_str(&declarations(rule.light, rule));
        css.push_str("}\n");
    }
    let dark = |root: &str| {
        let mut css = String::new();
        for rule in RULES {
            let selectors: Vec<String> = rule
                .scopes
                .iter()
                .map(|scope| format!("{root} {}", selector(scope)))
                .collect();
            css.push_str(&selectors.join(","));
            css.push('{');
            css.push_str(&declarations(rule.dark, rule));
            css.push_str("}\n");
        }
        css
    };
    css.push_str(&dark(dark_root));
    css.push_str("@media (prefers-color-scheme: dark){\n");
    css.push_str(&dark(system_root));
    css.push_str("}\n");
    css
}

#[cfg(test)]
mod tests {
    use super::*;
    use syntect::parsing::syntax_definition::Pattern;
    use syntect::parsing::Regex;

    /// A sample of each language, written to reach its common scopes.
    fn sample(id: &str) -> &'static str {
        match id {
            "rust" => "fn main() {\n    let s = \"hi\"; // greet\n    println!(\"{s}\");\n}\n",
            "toml" => "[package]\nname = \"demo\" # a name\nversion = 1\n",
            "sh" => "# build it\nmbx build --release && echo \"done\"\n",
            "json" => "{\"name\": \"demo\", \"count\": 2, \"ok\": true}\n",
            "js" => "const answer = 42; // the answer\nexport function f(x) { return `${x}`; }\n",
            "ts" => "interface Props { name: string }\nconst p: Props = { name: \"x\" };\n",
            "html" => "<!-- page -->\n<p class=\"lead\">Hello <b>there</b></p>\n",
            "css" => "/* tokens */\n.card { color: #fff; margin: 0 4px; }\n",
            "md" => "# Title\n\nSome *text* and `code`.\n",
            "yaml" => "# config\nname: demo\nitems:\n  - 1\n",
            "diff" => "--- a/x\n+++ b/x\n@@ -1 +1 @@\n-old\n+new\n",
            other => panic!("no sample for {other}"),
        }
    }

    #[test]
    fn every_language_has_its_grammar_and_highlights() {
        let highlighter = Highlighter::new();
        assert_eq!(LANGUAGES.len(), 11);
        for language in LANGUAGES {
            let highlighted = highlighter.highlight(sample(language.id), Some(language.id));
            assert_eq!(highlighted.language, Some(language));
            let html = highlighted.lines.join("\n");
            assert!(
                html.contains("<span class=\"hl-"),
                "{} produced no classed span: {html}",
                language.id
            );
            assert!(!html.contains("style="), "{}", language.id);
            for alias in language.aliases {
                assert_eq!(super::language(alias), Some(language));
            }
            assert_eq!(super::language(&language.id.to_uppercase()), Some(language));
        }
    }

    /// Every pattern of every grammar in the set compiles with the pure-Rust
    /// engine: the eleven languages and every grammar they can embed are a
    /// subset of it. Patterns that splice in a capture from their context's
    /// opening match are compiled with a stand-in for it.
    #[test]
    fn every_grammar_regex_compiles() {
        let highlighter = Highlighter::new();
        let builder = highlighter.syntaxes().clone().into_builder();
        let named: Vec<&str> = LANGUAGES.iter().map(|language| language.syntax).collect();
        let mut compiled = 0;
        let mut of_the_eleven = 0;
        let mut failures = Vec::new();
        for definition in builder.syntaxes() {
            for context in definition.contexts.values() {
                for pattern in &context.patterns {
                    let Pattern::Match(pattern) = pattern else {
                        continue;
                    };
                    let source = if pattern.has_captures {
                        substitute_backreferences(pattern.regex.regex_str())
                    } else {
                        pattern.regex.regex_str().to_owned()
                    };
                    if let Some(error) = Regex::try_compile(&source) {
                        failures.push(format!(
                            "{}: {}: {error}",
                            definition.name,
                            pattern.regex.regex_str()
                        ));
                    }
                    compiled += 1;
                    if named.contains(&definition.name.as_str()) {
                        of_the_eleven += 1;
                    }
                }
            }
        }
        assert!(
            failures.is_empty(),
            "{} of {compiled} failed: {failures:#?}",
            failures.len()
        );
        assert!(
            of_the_eleven > 1000,
            "only {of_the_eleven} patterns in the eleven grammars"
        );
        eprintln!("{compiled} patterns compiled, {of_the_eleven} of them in the eleven grammars");
    }

    fn substitute_backreferences(regex: &str) -> String {
        let mut out = String::with_capacity(regex.len());
        let mut chars = regex.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.peek() {
                    Some(d) if d.is_ascii_digit() => {
                        chars.next();
                        out.push('x');
                        continue;
                    }
                    Some(_) => {
                        out.push(c);
                        out.push(chars.next().unwrap());
                        continue;
                    }
                    None => {}
                }
            }
            out.push(c);
        }
        out
    }

    #[test]
    fn unknown_and_missing_tags_are_plain_escaped_text() {
        let highlighter = Highlighter::new();
        for tag in [None, Some("brainfuck"), Some("")] {
            let highlighted = highlighter.highlight("a < b\n&c\n", tag);
            assert_eq!(highlighted.language, None);
            assert_eq!(highlighted.lines, vec!["a &lt; b", "&amp;c"]);
        }
    }

    #[test]
    fn lines_are_balanced_and_carry_no_newline() {
        let highlighter = Highlighter::new();
        // A block comment spans lines: each line reopens what is in force.
        let highlighted =
            highlighter.highlight("/* one\n two */ fn f() {}\r\nlet x = 1;", Some("rust"));
        assert_eq!(highlighted.lines.len(), 3);
        for line in &highlighted.lines {
            assert!(!line.contains('\n') && !line.contains('\r'));
            assert_eq!(
                line.matches("<span").count(),
                line.matches("</span>").count(),
                "{line}"
            );
            assert!(!line.contains("\"></span>"), "empty span in {line}");
        }
        assert!(highlighted.lines[1]
            .starts_with("<span class=\"hl-source hl-rust\"><span class=\"hl-comment"));
    }

    #[test]
    fn text_survives_highlighting() {
        let highlighter = Highlighter::new();
        let code = "fn main() { let s = \"<&>\"; }\n// done\n";
        let highlighted = highlighter.highlight(code, Some("rust"));
        let text: Vec<String> = highlighted
            .lines
            .iter()
            .map(|line| strip_tags(line))
            .collect();
        assert_eq!(
            text,
            vec![
                "fn main() { let s = &quot;&lt;&amp;&gt;&quot;; }",
                "// done"
            ]
        );
    }

    /// Unfinished, odd and hostile input in every language: never a panic,
    /// and the text that comes out is exactly the text that went in.
    #[test]
    fn malformed_input_keeps_its_text_in_every_language() {
        let highlighter = Highlighter::new();
        let long = "x".repeat(20_000);
        let samples = [
            "\"unterminated string\n",
            "/* unclosed comment\nstill open\n",
            "<<<EOF\nheredoc without end\n",
            "\u{0}\u{7}\u{1F600} 中文 \u{200B}\n",
            "line one\r\nline two\r\n",
            "{{{{[[[(((\n)))]]]}}}}\n",
            "</script><script>alert(1)</script>\n",
            long.as_str(),
            "",
            "\n\n\n",
        ];
        for language in LANGUAGES {
            for sample in samples {
                let highlighted = highlighter.highlight(sample, Some(language.id));
                let text: Vec<String> = highlighted
                    .lines
                    .iter()
                    .map(|line| unescape(&strip_tags(line)))
                    .collect();
                let expected: Vec<&str> = split_lines(sample).map(|(_, text)| text).collect();
                assert_eq!(text, expected, "{} on {sample:?}", language.id);
                assert!(
                    highlighted
                        .lines
                        .iter()
                        .all(|line| !line.contains("<script")),
                    "{}",
                    language.id
                );
            }
        }
    }

    fn unescape(html: &str) -> String {
        html.replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
            .replace("&amp;", "&")
    }

    fn strip_tags(html: &str) -> String {
        let mut out = String::new();
        let mut inside = false;
        for c in html.chars() {
            match c {
                '<' => inside = true,
                '>' => inside = false,
                c if !inside => out.push(c),
                _ => {}
            }
        }
        out
    }

    #[test]
    fn grammar_version_matches_the_lock() {
        let lock = include_str!("../../../Cargo.lock");
        for (name, version) in [("syntect", "5.3.0"), ("two-face", "0.5.2+bat-0.26.1")] {
            let entry = format!("name = \"{name}\"\nversion = \"{version}\"");
            assert!(
                lock.contains(&entry),
                "Cargo.lock no longer has {name} {version}"
            );
            assert!(GRAMMAR_VERSION.contains(&format!("{name}-{version}")));
        }
    }

    #[test]
    fn palette_keeps_contrast_on_code_backgrounds() {
        for rule in RULES {
            for (color, background) in [
                (rule.light, CODE_BACKGROUND.0),
                (rule.dark, CODE_BACKGROUND.1),
            ] {
                if color == "inherit" {
                    continue;
                }
                let ratio = crate::color::contrast(color, background).unwrap();
                assert!(ratio >= 4.5, "{color} on {background} is {ratio:.2}:1");
            }
        }
    }

    #[test]
    fn theme_css_has_both_dark_roots_and_no_bare_colours_elsewhere() {
        let css = theme_css(":root[data-theme=\"dark\"]", ":root[data-theme=\"system\"]");
        assert!(css.contains(
            ".hl-comment,.hl-punctuation.hl-definition.hl-comment{color:#57606a;font-style:italic}"
        ));
        assert!(css.contains(":root[data-theme=\"dark\"] .hl-comment"));
        assert!(css.contains(
            "@media (prefers-color-scheme: dark){\n:root[data-theme=\"system\"] .hl-comment"
        ));
    }
}
