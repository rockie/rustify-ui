//! Reference site B: the content the docs site generator is measured on.
//!
//! The PRD defines it as 500 pages, half English and half Chinese, whose
//! bodies follow the distribution of this repository's twelve guides. The
//! distribution is frozen once (`--profile`) into
//! `tests/docs-ssg/b-profile.json`: each guide's top-level blocks in order -
//! heading levels, paragraph and list sizes, table shapes, code fences with
//! their language and line count, alerts - and the guide's SHA-256. The
//! generator (`--out`) then writes page `i` with guide `i mod 12`'s skeleton,
//! filled with seeded filler of the same byte sizes, so the corpus does not
//! drift when a guide is edited and the same seed always writes the same
//! bytes. `b-manifest.json` in the output records the seed, the profile's
//! digest and every file's digest.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use pulldown_cmark::{BlockQuoteKind, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::option;

const USAGE: &str = "\
mbx xtask docs-corpus --profile                 refreeze tests/docs-ssg/b-profile.json from docs/*.md
mbx xtask docs-corpus --out <dir> [--seed <n>]  write reference site B into an empty or absent <dir>";

/// The twelve guides the PRD names, in its order.
pub const GUIDES: [&str; 12] = [
    "quickstart",
    "architecture",
    "navigation",
    "workspace",
    "forms",
    "i18n",
    "data",
    "themes",
    "utility",
    "compatibility",
    "components",
    "vellum",
];

pub const PROFILE: &str = "tests/docs-ssg/b-profile.json";
pub const DEFAULT_SEED: u64 = 0x5eed_b500;
/// Changes whenever the generator's output for a given profile and seed does.
const GENERATOR_VERSION: u32 = 2;
const PAGES_PER_LANGUAGE: usize = 250;
const GROUPS: usize = 10;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Block {
    Heading {
        level: u8,
        bytes: usize,
    },
    Paragraph {
        bytes: usize,
        links: usize,
        code: usize,
    },
    List {
        ordered: bool,
        items: Vec<usize>,
        links: usize,
    },
    /// `rows` counts the body; the header row is always there.
    Table {
        columns: usize,
        rows: usize,
        cell_bytes: usize,
    },
    Code {
        lang: String,
        lines: usize,
    },
    Quote {
        bytes: usize,
    },
    Alert {
        alert: String,
        bytes: usize,
    },
    Html {
        bytes: usize,
    },
    Rule,
    Image,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Guide {
    pub name: String,
    pub sha256: String,
    pub bytes: usize,
    pub images: usize,
    pub blocks: Vec<Block>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub schema_version: u32,
    pub source: String,
    pub guides: Vec<Guide>,
}

pub fn run(args: &[String]) -> Result<(), String> {
    let root = crate::build::repo_root();
    if args.iter().any(|a| a == "--profile") {
        let profile = profile(&root)?;
        let json = serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())? + "\n";
        std::fs::create_dir_all(root.join("tests/docs-ssg")).map_err(|e| e.to_string())?;
        std::fs::write(root.join(PROFILE), json).map_err(|e| e.to_string())?;
        println!(
            "froze the distribution of {} guides into {PROFILE}",
            profile.guides.len()
        );
        return Ok(());
    }
    let out = option(args, "--out").ok_or(USAGE)?;
    let seed = match option(args, "--seed") {
        Some(seed) => seed
            .parse()
            .map_err(|_| format!("--seed {seed}: not a number"))?,
        None => DEFAULT_SEED,
    };
    let manifest = generate(&root, Path::new(out), seed)?;
    println!(
        "reference site B: {} pages, {} files, digest {} (seed {seed:#x}) in {out}",
        manifest.pages,
        manifest.files.len(),
        manifest.digest
    );
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Reads each guide's top-level structure.
pub fn profile(root: &Path) -> Result<Profile, String> {
    let mut guides = Vec::new();
    for name in GUIDES {
        let path = root.join("docs").join(format!("{name}.md"));
        let text =
            std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let blocks = blocks(&text);
        let images = blocks
            .iter()
            .filter(|block| matches!(block, Block::Image))
            .count();
        guides.push(Guide {
            name: name.to_owned(),
            sha256: sha256_hex(text.as_bytes()),
            bytes: text.len(),
            images,
            blocks,
        });
    }
    let head = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(root)
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_default();
    Ok(Profile {
        schema_version: 1,
        source: format!("docs/*.md at {head}; each guide's sha256 is below"),
        guides,
    })
}

fn blocks(text: &str) -> Vec<Block> {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_GFM;
    let mut blocks = Vec::new();
    let mut depth = 0usize;
    // The top-level block being measured.
    let mut current: Option<Block> = None;
    let mut list_item_bytes = 0usize;
    let mut table_cells = 0usize;
    let mut table_bytes = 0usize;
    let mut code_text = String::new();
    for event in Parser::new_ext(text, options) {
        match event {
            Event::Start(tag) => {
                depth += 1;
                if depth == 1 {
                    current = Some(match &tag {
                        Tag::Heading { level, .. } => Block::Heading {
                            level: *level as u8,
                            bytes: 0,
                        },
                        Tag::Paragraph => Block::Paragraph {
                            bytes: 0,
                            links: 0,
                            code: 0,
                        },
                        Tag::List(start) => Block::List {
                            ordered: start.is_some(),
                            items: Vec::new(),
                            links: 0,
                        },
                        Tag::Table(alignments) => {
                            table_cells = 0;
                            table_bytes = 0;
                            Block::Table {
                                columns: alignments.len(),
                                rows: 0,
                                cell_bytes: 0,
                            }
                        }
                        Tag::CodeBlock(kind) => {
                            code_text.clear();
                            let lang = match kind {
                                CodeBlockKind::Fenced(info) => {
                                    info.split_whitespace().next().unwrap_or("").to_owned()
                                }
                                CodeBlockKind::Indented => String::new(),
                            };
                            Block::Code { lang, lines: 0 }
                        }
                        Tag::BlockQuote(Some(kind)) => Block::Alert {
                            alert: match kind {
                                BlockQuoteKind::Note => "NOTE",
                                BlockQuoteKind::Tip => "TIP",
                                BlockQuoteKind::Important => "IMPORTANT",
                                BlockQuoteKind::Warning => "WARNING",
                                BlockQuoteKind::Caution => "CAUTION",
                            }
                            .to_owned(),
                            bytes: 0,
                        },
                        Tag::BlockQuote(None) => Block::Quote { bytes: 0 },
                        Tag::HtmlBlock => Block::Html { bytes: 0 },
                        _ => Block::Html { bytes: 0 },
                    });
                }
                match (&mut current, &tag) {
                    (Some(Block::Paragraph { links, .. }), Tag::Link { .. })
                    | (Some(Block::List { links, .. }), Tag::Link { .. }) => *links += 1,
                    (Some(_), Tag::Image { .. }) => blocks.push(Block::Image),
                    (Some(Block::List { .. }), Tag::Item) if depth == 2 => list_item_bytes = 0,
                    (Some(Block::Table { .. }), Tag::TableCell) => table_cells += 1,
                    _ => {}
                }
            }
            Event::End(end) => {
                if let (Some(Block::List { items, .. }), TagEnd::Item) = (&mut current, &end) {
                    if depth == 2 {
                        items.push(list_item_bytes);
                    }
                }
                if let (Some(Block::Table { rows, .. }), TagEnd::TableRow) = (&mut current, &end) {
                    *rows += 1;
                }
                depth -= 1;
                if depth == 0 {
                    if let Some(mut block) = current.take() {
                        match &mut block {
                            Block::Table { cell_bytes, .. } => {
                                *cell_bytes = table_bytes / table_cells.max(1);
                            }
                            Block::Code { lines, .. } => *lines = code_text.lines().count(),
                            _ => {}
                        }
                        blocks.push(block);
                    }
                }
            }
            Event::Text(ref text) | Event::Code(ref text) => {
                let length = text.len();
                let inline_code = matches!(event, Event::Code(_));
                match &mut current {
                    Some(Block::Heading { bytes, .. })
                    | Some(Block::Quote { bytes })
                    | Some(Block::Alert { bytes, .. })
                    | Some(Block::Html { bytes }) => *bytes += length,
                    Some(Block::Paragraph { bytes, code, .. }) => {
                        *bytes += length;
                        *code += usize::from(inline_code);
                    }
                    Some(Block::List { .. }) => list_item_bytes += length,
                    Some(Block::Table { .. }) => table_bytes += length,
                    Some(Block::Code { .. }) => code_text.push_str(text),
                    _ => {}
                }
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                if let Some(Block::Html { bytes }) = &mut current {
                    *bytes += html.len();
                }
            }
            Event::Rule if depth == 0 => blocks.push(Block::Rule),
            _ => {}
        }
    }
    blocks
}

/// SplitMix64: small, seedable and the same everywhere.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }
}

const EN_WORDS: &[&str] = &[
    "the",
    "region",
    "scope",
    "a",
    "page",
    "build",
    "of",
    "state",
    "to",
    "component",
    "and",
    "value",
    "is",
    "theme",
    "renders",
    "with",
    "action",
    "list",
    "in",
    "browser",
    "when",
    "site",
    "each",
    "layout",
    "that",
    "control",
    "for",
    "signal",
    "every",
    "event",
    "from",
    "frame",
    "on",
    "input",
    "keeps",
    "returns",
    "an",
    "focus",
    "table",
    "row",
    "it",
    "application",
    "props",
    "draws",
    "canvas",
    "pump",
    "listener",
    "token",
    "colour",
    "mode",
    "reader",
    "author",
    "content",
    "link",
    "heading",
    "anchor",
    "deploy",
    "static",
    "style",
    "class",
    "utility",
    "compiler",
    "cache",
    "version",
    "test",
    "check",
    "error",
    "warning",
    "path",
    "route",
    "base",
    "output",
    "file",
    "markdown",
    "code",
    "block",
    "language",
    "copy",
    "group",
    "menu",
    "navigation",
    "sidebar",
];

const ZH_WORDS: &[&str] = &[
    "区域",
    "作用域",
    "页面",
    "构建",
    "状态",
    "组件",
    "数值",
    "主题",
    "渲染",
    "动作",
    "列表",
    "浏览器",
    "站点",
    "布局",
    "控件",
    "信号",
    "事件",
    "画布",
    "输入",
    "焦点",
    "表格",
    "应用",
    "属性",
    "绘制",
    "监听",
    "令牌",
    "颜色",
    "模式",
    "读者",
    "作者",
    "内容",
    "链接",
    "标题",
    "锚点",
    "部署",
    "静态",
    "样式",
    "工具类",
    "编译器",
    "缓存",
    "版本",
    "测试",
    "检查",
    "错误",
    "警告",
    "路径",
    "路由",
    "基路径",
    "输出",
    "文件",
    "代码块",
    "语言",
    "复制",
    "分组",
    "菜单",
    "导航",
    "侧栏",
    "的",
    "在",
    "和",
    "会",
    "把",
    "从",
    "每个",
    "这个",
    "可以",
    "需要",
    "如果",
    "然后",
    "因为",
    "所以",
    "并且",
    "不会",
    "只有",
    "仍然",
    "已经",
];

/// Lines a code block of each language is filled from, in order and
/// repeated as needed.
fn code_lines(lang: &str) -> &'static [&'static str] {
    match lang {
        "rust" | "rs" => &[
            "use rustify_ui::prelude::*;",
            "",
            "#[component]",
            "pub fn Panel(title: String, count: Signal<u32>) -> impl IntoView {",
            "    let label = move || format!(\"{title}: {}\", count.get());",
            "    view! { <section class=\"p-4\">{label}</section> }",
            "}",
            "// Each region owns its own Cx; the application owns the state.",
            "let region = GpuRegion::new(Scene::default());",
            "region.apply(|scene| scene.zoom = 2.0);",
        ],
        "sh" | "bash" | "shell" => &[
            "# Build the release product, then serve it under the strict policy.",
            "mbx xtask build-web --example fusion-basic --release",
            "mbx xtask serve --example fusion-basic --release --port 4173",
            "npx playwright test --project=fusion-basic",
            "cargo install --locked wasm-bindgen-cli --version 0.2.128",
        ],
        "toml" => &[
            "[package]",
            "name = \"my-app\"",
            "version = \"0.1.0\"",
            "",
            "[dependencies]",
            "rustify-ui = { git = \"https://github.com/rockie/rustify-ui\", features = [\"gpu\"] }",
            "leptos = { version = \"=0.8.22\", features = [\"csr\"] }",
        ],
        "css" => &[
            "@import \"tailwindcss\" source(none);",
            ".panel {",
            "  border: 1px solid var(--border);",
            "  color: var(--foreground); /* a token, never a literal */",
            "}",
        ],
        "html" => &[
            "<!doctype html>",
            "<html lang=\"en\">",
            "  <head><link rel=\"stylesheet\" href=\"./rustify.css\"></head>",
            "  <body><div id=\"app\" data-rustify-scope></div></body>",
            "</html>",
        ],
        "js" => &[
            "import { boot } from \"./loader.js\";",
            "const app = await boot({ wasm: \"./app_bg.wasm\" });",
            "window.__app = { reset: () => app.reset() }; // driven by the specs",
        ],
        _ => &[
            "graph TD",
            "  Loader --> Instance",
            "  Instance --> Scope",
            "  Scope --> Region",
        ],
    }
}

struct Filler<'a> {
    rng: &'a mut Rng,
    zh: bool,
}

impl Filler<'_> {
    /// Prose of about `bytes` bytes, ending in a full stop.
    fn prose(&mut self, bytes: usize) -> String {
        let words = if self.zh { ZH_WORDS } else { EN_WORDS };
        let mut out = String::new();
        let mut sentence = 0;
        while out.len() < bytes.saturating_sub(1).max(1) {
            let word = words[self.rng.below(words.len())];
            if self.zh {
                out.push_str(word);
                sentence += 1;
                if sentence > 8 + self.rng.below(10) {
                    out.push('。');
                    sentence = 0;
                } else if self.rng.below(7) == 0 {
                    out.push('，');
                }
            } else {
                if sentence == 0 && !out.is_empty() {
                    out.push(' ');
                }
                if sentence > 0 {
                    out.push(' ');
                }
                if sentence == 0 {
                    let mut chars = word.chars();
                    if let Some(first) = chars.next() {
                        out.extend(first.to_uppercase());
                        out.push_str(chars.as_str());
                    }
                } else {
                    out.push_str(word);
                }
                sentence += 1;
                if sentence > 6 + self.rng.below(12) {
                    out.push('.');
                    sentence = 0;
                }
            }
        }
        if self.zh {
            if !out.ends_with('。') {
                out.push('。');
            }
        } else if !out.ends_with('.') {
            out.push('.');
        }
        out
    }

    fn title(&mut self, bytes: usize) -> String {
        let text = self.prose(bytes.clamp(6, 60));
        text.trim_end_matches(['.', '。']).to_owned()
    }
}

/// One page's place in the site.
struct PageSlot {
    path: String,
    lang: &'static str,
    group: usize,
    order: usize,
    guide: usize,
}

fn slots() -> Vec<PageSlot> {
    let mut slots = vec![PageSlot {
        path: "index.md".into(),
        lang: "en",
        group: 0,
        order: 0,
        guide: 0,
    }];
    let mut index = 1;
    for (lang, count) in [("en", PAGES_PER_LANGUAGE - 1), ("zh", PAGES_PER_LANGUAGE)] {
        let per_group = count.div_ceil(GROUPS);
        for n in 0..count {
            let group = n / per_group;
            let order = n % per_group + 1;
            slots.push(PageSlot {
                path: format!("{lang}/g{:02}/page-{:03}.md", group + 1, order),
                lang,
                group,
                order,
                guide: index % GUIDES.len(),
            });
            index += 1;
        }
    }
    slots
}

fn group_label(lang: &str, group: usize) -> String {
    if lang == "zh" {
        format!("第 {} 组", group + 1)
    } else {
        format!("Group {}", group + 1)
    }
}

fn page(profile: &Profile, slot: &PageSlot, slots: &[PageSlot], rng: &mut Rng) -> String {
    let guide = &profile.guides[slot.guide];
    let zh = slot.lang == "zh";
    let mut filler = Filler { rng, zh };
    let same_language: Vec<&PageSlot> = slots
        .iter()
        .filter(|other| other.lang == slot.lang && other.path != slot.path)
        .collect();
    let depth = slot.path.matches('/').count();
    let link = |filler: &mut Filler| {
        let target = same_language[filler.rng.below(same_language.len())];
        format!("{}{}", "../".repeat(depth), target.path)
    };
    let mut body = String::new();
    let mut title = None;
    for block in &guide.blocks {
        match block {
            Block::Heading { level, bytes } => {
                let text = filler.title(*bytes);
                if *level == 1 && title.is_none() {
                    title = Some(text.clone());
                }
                body.push_str(&format!("{} {text}\n\n", "#".repeat(usize::from(*level))));
            }
            Block::Paragraph { bytes, links, code } => {
                let mut text = filler.prose(*bytes);
                for _ in 0..*code {
                    let word = filler.title(8).to_lowercase().replace(' ', "_");
                    text.push_str(&format!(" `{word}`"));
                }
                for _ in 0..*links {
                    let target = link(&mut filler);
                    let label = filler.title(12);
                    text.push_str(&format!(" [{label}]({target})"));
                }
                body.push_str(&text);
                body.push_str("\n\n");
            }
            Block::List {
                ordered,
                items,
                links,
            } => {
                for (i, bytes) in items.iter().enumerate() {
                    let marker = if *ordered {
                        format!("{}.", i + 1)
                    } else {
                        "-".into()
                    };
                    let mut text = filler.prose(*bytes);
                    if i < *links {
                        let target = link(&mut filler);
                        text = format!("[{}]({target}) {text}", filler.title(10));
                    }
                    body.push_str(&format!("{marker} {text}\n"));
                }
                body.push('\n');
            }
            Block::Table {
                columns,
                rows,
                cell_bytes,
            } => {
                let columns = (*columns).max(1);
                let header: Vec<String> = (0..columns).map(|_| filler.title(*cell_bytes)).collect();
                body.push_str(&format!("| {} |\n", header.join(" | ")));
                body.push_str(&format!("|{}\n", "---|".repeat(columns)));
                for _ in 0..*rows {
                    let cells: Vec<String> = (0..columns)
                        .map(|_| filler.prose(*cell_bytes).replace('|', "/"))
                        .collect();
                    body.push_str(&format!("| {} |\n", cells.join(" | ")));
                }
                body.push('\n');
            }
            Block::Code { lang, lines } => {
                let source = code_lines(lang);
                body.push_str(&format!("```{lang}\n"));
                for line in 0..*lines {
                    body.push_str(source[line % source.len()]);
                    body.push('\n');
                }
                body.push_str("```\n\n");
            }
            Block::Quote { bytes } => body.push_str(&format!("> {}\n\n", filler.prose(*bytes))),
            Block::Alert { alert, bytes } => {
                body.push_str(&format!("> [!{alert}]\n> {}\n\n", filler.prose(*bytes)));
            }
            Block::Html { bytes } => body.push_str(&format!("<p>{}</p>\n\n", filler.prose(*bytes))),
            Block::Rule => body.push_str("---\n\n"),
            Block::Image => {}
        }
    }
    let title = title.unwrap_or_else(|| filler.title(24));
    let description = filler.prose(120).replace('"', "'");
    let mut front = String::from("---\n");
    front.push_str(&format!("title: \"{title}\"\n"));
    front.push_str(&format!("description: \"{description}\"\n"));
    front.push_str(&format!("lang: {}\n", slot.lang));
    if slot.path == "index.md" {
        front.push_str("layout: home\n");
        front.push_str("tagline: \"Reference site B for Rustify Docs\"\n");
    } else {
        front.push_str(&format!(
            "sidebar:\n  group: \"{}\"\n  order: {}\n",
            group_label(slot.lang, slot.group),
            slot.order
        ));
    }
    front.push_str("---\n\n");
    front + &body
}

#[derive(Serialize)]
pub struct Manifest {
    pub generator_version: u32,
    pub seed: u64,
    pub profile_sha256: String,
    pub pages: usize,
    pub languages: BTreeMap<String, usize>,
    pub files: BTreeMap<String, String>,
    /// SHA-256 over every `path sha256` line, in path order.
    pub digest: String,
}

fn config() -> String {
    let mut groups = String::new();
    for lang in ["en", "zh"] {
        for group in 0..GROUPS {
            groups.push_str(&format!(
                "\n[[nav.groups]]\nlabel = \"{}\"\ndir = \"{lang}/g{:02}\"\nscope = \"/{lang}/\"\n",
                group_label(lang, group),
                group + 1
            ));
        }
    }
    format!(
        "schema_version = 1\n\n[site]\nname = \"Reference site B\"\ndescription = \"500 pages in English and Chinese, shaped like the Rustify UI guides\"\nlang = \"en\"\nurl = \"https://example.invalid/\"\nbase = \"/\"\n\n[theme]\nlogo = \"static/logo.svg\"\n{groups}"
    )
}

const LOGO: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 32 32\"><rect width=\"32\" height=\"32\" rx=\"6\" fill=\"#175cd3\"/><path d=\"M9 22V10h8a4 4 0 0 1 0 8h-8\" fill=\"none\" stroke=\"#fff\" stroke-width=\"3\"/></svg>\n";

pub fn generate(root: &Path, out: &Path, seed: u64) -> Result<Manifest, String> {
    let profile_text = std::fs::read_to_string(root.join(PROFILE))
        .map_err(|e| format!("{PROFILE}: {e}; run `mbx xtask docs-corpus --profile` first"))?;
    let profile: Profile =
        serde_json::from_str(&profile_text).map_err(|e| format!("{PROFILE}: {e}"))?;
    if profile.guides.len() != GUIDES.len() {
        return Err(format!(
            "{PROFILE} has {} guides, not {}",
            profile.guides.len(),
            GUIDES.len()
        ));
    }
    if out.exists()
        && std::fs::read_dir(out)
            .map_err(|e| e.to_string())?
            .next()
            .is_some()
    {
        return Err(format!("{} is not empty", out.display()));
    }
    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    let slots = slots();
    let mut rng = Rng(seed);
    for slot in &slots {
        let text = page(&profile, slot, &slots, &mut rng);
        files.insert(format!("content/{}", slot.path), text.into_bytes());
    }
    files.insert("rustify-docs.toml".into(), config().into_bytes());
    files.insert("static/logo.svg".into(), LOGO.as_bytes().to_vec());
    let mut digests = BTreeMap::new();
    let mut all = Sha256::new();
    for (path, bytes) in &files {
        let target: PathBuf = out.join(path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        std::fs::write(&target, bytes).map_err(|e| format!("{}: {e}", target.display()))?;
        let digest = sha256_hex(bytes);
        all.update(format!("{path} {digest}\n").as_bytes());
        digests.insert(path.clone(), digest);
    }
    let mut languages = BTreeMap::new();
    for slot in &slots {
        *languages.entry(slot.lang.to_owned()).or_insert(0) += 1;
    }
    let manifest = Manifest {
        generator_version: GENERATOR_VERSION,
        seed,
        profile_sha256: sha256_hex(profile_text.as_bytes()),
        pages: slots.len(),
        languages,
        files: digests,
        digest: all.finalize().iter().map(|b| format!("{b:02x}")).collect(),
    };
    let json = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())? + "\n";
    std::fs::write(out.join("b-manifest.json"), json).map_err(|e| e.to_string())?;
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_hundred_pages_half_in_each_language() {
        let slots = slots();
        assert_eq!(slots.len(), 500);
        assert_eq!(slots.iter().filter(|slot| slot.lang == "zh").count(), 250);
        let mut paths: Vec<&str> = slots.iter().map(|slot| slot.path.as_str()).collect();
        paths.sort_unstable();
        paths.dedup();
        assert_eq!(paths.len(), 500);
        // Each guide shapes about a twelfth of the pages.
        for guide in 0..GUIDES.len() {
            let count = slots.iter().filter(|slot| slot.guide == guide).count();
            assert!((41..=42).contains(&count), "guide {guide}: {count}");
        }
    }

    #[test]
    fn profile_reads_structure() {
        let blocks = blocks("# T\n\nA [b](c.md) `d`.\n\n- x\n- y\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n```rust\nfn a() {}\nfn b() {}\n```\n\n> [!NOTE]\n> n\n");
        assert_eq!(blocks[0], Block::Heading { level: 1, bytes: 1 });
        assert!(matches!(blocks[1], Block::Paragraph { links: 1, .. }));
        assert_eq!(
            blocks[2],
            Block::List {
                ordered: false,
                items: vec![1, 1],
                links: 0
            }
        );
        assert_eq!(
            blocks[3],
            Block::Table {
                columns: 2,
                rows: 1,
                cell_bytes: 1
            }
        );
        assert_eq!(
            blocks[4],
            Block::Code {
                lang: "rust".into(),
                lines: 2
            }
        );
        assert_eq!(
            blocks[5],
            Block::Alert {
                alert: "NOTE".into(),
                bytes: 1
            }
        );
    }

    #[test]
    fn the_same_seed_writes_the_same_bytes() {
        let profile = Profile {
            schema_version: 1,
            source: String::new(),
            guides: (0..12)
                .map(|i| Guide {
                    name: format!("g{i}"),
                    sha256: String::new(),
                    bytes: 0,
                    images: 0,
                    blocks: vec![
                        Block::Heading {
                            level: 1,
                            bytes: 20,
                        },
                        Block::Paragraph {
                            bytes: 300,
                            links: 2,
                            code: 0,
                        },
                    ],
                })
                .collect(),
        };
        let slots = slots();
        let render = |seed| {
            let mut rng = Rng(seed);
            slots
                .iter()
                .take(20)
                .map(|slot| page(&profile, slot, &slots, &mut rng))
                .collect::<Vec<_>>()
        };
        assert_eq!(render(7), render(7));
        assert_ne!(render(7), render(8));
        let zh = page(&profile, &slots[300], &slots, &mut Rng(1));
        assert!(zh.contains("lang: zh") && zh.contains("。"));
    }
}
