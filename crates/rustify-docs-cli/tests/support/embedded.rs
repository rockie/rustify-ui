//! Every embedded rejection a project without Rust can make, written the
//! ways authors write: plain, after Chinese text, with CRLF line ends, and
//! repeated with the same values. Each refusal knows its exact location and
//! the exact repair that `check`, `build` and `dev` must all report.

use serde_json::Value;

pub const REFERENCE: &str =
    "https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md#static-builtins";

pub struct Variant {
    pub file: &'static str,
    pub prefix: &'static str,
    /// A flow-mapping entry written before the repeated key on its line.
    pub note: &'static str,
    pub newline: &'static str,
    pub repeat: usize,
}

pub const VARIANTS: [Variant; 4] = [
    Variant {
        file: "content/plain.md",
        prefix: "Plain prose before the examples.",
        note: "note: wrong",
        newline: "\n",
        repeat: 1,
    },
    Variant {
        file: "content/zh.md",
        prefix: "中文前缀：以下示例都有错误。",
        note: "说明: 中文前缀",
        newline: "\n",
        repeat: 1,
    },
    Variant {
        file: "content/crlf.md",
        prefix: "Prose with CRLF line ends.",
        note: "note: wrong",
        newline: "\r\n",
        repeat: 1,
    },
    Variant {
        file: "content/same.md",
        prefix: "Every refused value is the same word, twice.",
        note: "note: wrong",
        newline: "\n",
        repeat: 2,
    },
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expected {
    pub rule: String,
    pub file: String,
    pub line: u64,
    pub column: u64,
    pub suggestion: String,
}

enum At {
    /// The fence's opening line: builtin and registration errors.
    Fence,
    /// The start of the line beginning with this text.
    Line(&'static str),
    /// The flow mapping that repeats a key, where YAML reports it.
    Mapping,
}

/// The page's Markdown and the refusals it must produce, in report order.
pub fn page(variant: &Variant) -> (String, Vec<Expected>) {
    let duplicate = format!("props: {{{}, items: [], items: []}}", variant.note);
    let blocks: Vec<(Vec<String>, &str, At, String)> = vec![
        (
            fence("rustify-component", &["name: Steps", "props:", "  items: wrong"]),
            "embed-props",
            At::Fence,
            format!("Write `items` as `Vec<StepSpec>` (required); see {REFERENCE} for its object fields. Reported props path: `items`."),
        ),
        (
            fence("rustify-component", &["name: Steps"]),
            "embed-props",
            At::Fence,
            format!("Add or correct `props` using the declared fields `items` (`Vec<StepSpec>`; required); see {REFERENCE} for its object fields."),
        ),
        (
            fence("rustify-component", &["name: CardGrid", "props:", "  cards: []", "  columns: 4"]),
            "embed-props",
            At::Fence,
            "Set `columns` to 1, 2 or 3, or omit it for the default expression `2`.".to_owned(),
        ),
        (
            fence(
                "rustify-component",
                &["name: Tabs", "props:", "  tabs:", "    - label: wrong", "      content: wrong", "      extra: wrong"],
            ),
            "embed-props",
            At::Fence,
            format!("Write `tabs` as `Vec<TabSpec>` (required); see {REFERENCE} for its object fields. Reported props path: `tabs[0].extra`."),
        ),
        (
            fence("rustify-component", &["name: Steps", &duplicate]),
            "embed-payload",
            At::Mapping,
            "Keep each YAML mapping key once; remove or rename the duplicate key.".to_owned(),
        ),
        (
            fence("rustify-component", &["name: Steps", "unknown: wrong"]),
            "embed-payload",
            At::Line("unknown: wrong"),
            "Write valid YAML for `rustify-component` (`name`, optional `props` mapping and `children` string) or `rustify-demo` (`name` only); correct the reported payload error.".to_owned(),
        ),
        (
            fence("rustify-component", &["name: MissingWidget"]),
            "unregistered-component",
            At::Fence,
            "Use a component from `CardGrid`, `Steps`, `Tabs`; for an author component, enable `[rust]` and register its name in `site!`.".to_owned(),
        ),
        (
            fence("rustify-demo", &["name: MissingDemo"]),
            "unregistered-demo",
            At::Fence,
            "Enable `[rust]` and register a demo in `site!` before using a `rustify-demo` block."
                .to_owned(),
        ),
    ];
    let mut lines: Vec<String> = [
        "---",
        "title: Embedded errors",
        "description: Every refused embed",
        "---",
        "# Embedded errors",
        "",
        variant.prefix,
        "",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    let mut expected = Vec::new();
    for _ in 0..variant.repeat {
        for (block, rule, at, suggestion) in &blocks {
            let first = lines.len();
            lines.extend(block.iter().cloned());
            lines.push(String::new());
            let (line, column) = match at {
                At::Fence => (first, 1),
                At::Line(start) => (
                    first
                        + block
                            .iter()
                            .position(|line| line.starts_with(start))
                            .unwrap(),
                    1,
                ),
                At::Mapping => {
                    let index = first + block.iter().position(|line| *line == duplicate).unwrap();
                    let text = &lines[index];
                    let at = text.find('{').unwrap();
                    (index, text[..at].chars().count() + 1)
                }
            };
            expected.push(Expected {
                rule: rule.to_string(),
                file: variant.file.to_owned(),
                line: line as u64 + 1,
                column: column as u64,
                suggestion: suggestion.clone(),
            });
        }
    }
    let mut text = lines.join(variant.newline);
    text.push_str(variant.newline);
    (text, expected)
}

fn fence(info: &str, body: &[&str]) -> Vec<String> {
    std::iter::once(format!("```{info}"))
        .chain(body.iter().map(|line| line.to_string()))
        .chain(std::iter::once("```".to_owned()))
        .collect()
}

/// A content-only project with a good home page and every variant's page.
pub fn files() -> Vec<(String, String)> {
    let mut files = vec![
        (
            "rustify-docs.toml".to_owned(),
            "schema_version=1\n[site]\nname='D'\nurl='https://example.com/'\n".to_owned(),
        ),
        (
            "content/index.md".to_owned(),
            "---\ndescription: d\n---\n# Home\n".to_owned(),
        ),
    ];
    for variant in &VARIANTS {
        files.push((variant.file.to_owned(), page(variant).0));
    }
    files
}

pub fn expected() -> Vec<Expected> {
    let mut all: Vec<Expected> = VARIANTS
        .iter()
        .flat_map(|variant| page(variant).1)
        .collect();
    all.sort_by(|a, b| {
        (&a.file, a.line, a.column, &a.rule).cmp(&(&b.file, b.line, b.column, &b.rule))
    });
    all
}

/// The embedded refusals in a report or a dev state, in the same shape.
pub fn reported(diagnostics: &Value) -> Vec<Expected> {
    let mut all: Vec<Expected> = diagnostics
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| {
            d["rule"]
                .as_str()
                .is_some_and(|rule| rule.starts_with("embed-") || rule.starts_with("unregistered-"))
        })
        .map(|d| {
            assert_eq!(d["severity"], "error", "{d}");
            let message = d["message"].as_str().unwrap();
            let suggestion = d["suggestion"].as_str().unwrap_or("");
            assert!(
                !message.is_empty() && (suggestion.is_empty() || !message.contains(suggestion)),
                "the repair adds to the original message: {d}"
            );
            Expected {
                rule: d["rule"].as_str().unwrap().to_owned(),
                file: d["file"].as_str().unwrap().to_owned(),
                line: d["line"].as_u64().unwrap(),
                column: d["column"].as_u64().unwrap(),
                suggestion: suggestion.to_owned(),
            }
        })
        .collect();
    all.sort_by(|a, b| {
        (&a.file, a.line, a.column, &a.rule).cmp(&(&b.file, b.line, b.column, &b.rule))
    });
    all
}
