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
mbx xtask docs-corpus --out <dir> [--seed <n>]  write reference site B into an empty or absent <dir>
mbx xtask docs-corpus --p2 --out <dir> [--seed <n>]  add twelve four-island measurement candidates
mbx xtask docs-corpus --p4-search --out <dir> [--pages 100] [--seed <n>]  1000 searchable pages per language";

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
const P2_GENERATOR_VERSION: u32 = 1;
const P2_RNG_STREAM: u64 = 0x7032_5f69_736c_616e;
const P2_FIXTURE: &str = "tests/docs-ssg/fixtures/components";
const P2_TEMPLATE_FILES: [&str; 5] = [
    "src/catalog.rs",
    "src/demos.rs",
    "src/bin/docs-render.rs",
    "Cargo.toml",
    "Cargo.lock",
];

const P2_LIBRARY: &str = r##"#![recursion_limit = "256"]

use rustify_docs::SiteRegistry;

mod catalog;
pub mod demos;

pub fn site() -> SiteRegistry {
    rustify_docs::site! {
        components {
            Counter { initial: i32 = 2, label: String = "Counter".to_string() } =>
                |props, children| demos::counter(props.initial, props.label, children),
        }
        demos {
            CatalogProxy => crate::demos::catalog_proxy,
        }
    }
}

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn docs_hydrate() -> Result<(), wasm_bindgen::JsValue> {
    rustify_docs::hydrate_site(site())
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
"##;

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
    let p2 = args.iter().any(|a| a == "--p2");
    let search = args.iter().any(|a| a == "--p4-search");
    if search && (p2 || args.iter().any(|a| a == "--profile")) {
        return Err("--p4-search cannot be combined with --p2 or --profile".into());
    }
    if !search && option(args, "--pages").is_some() {
        return Err("--pages is only supported with --p4-search".into());
    }
    if args.iter().any(|a| a == "--profile") {
        if p2 {
            return Err(
                "--p2 uses the frozen P1 profile; it cannot be combined with --profile".into(),
            );
        }
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
    if search {
        let pages = option(args, "--pages")
            .unwrap_or("1000")
            .parse()
            .map_err(|_| "--pages must be 100 or 1000")?;
        if !matches!(pages, 100 | 1000) {
            return Err("--pages must be 100 or 1000".into());
        }
        generate_search(&root, Path::new(out), seed, pages)?;
        println!("search corpus: {pages} searchable pages per language in {out} (seed {seed:#x})");
    } else if p2 {
        let manifest = generate_p2(&root, Path::new(out), seed)?;
        println!(
            "reference site B P2: {} pages ({} baseline + {} demo candidates), {} files, digest {} (seed {seed:#x}) in {out}",
            manifest.pages, manifest.baseline_pages, manifest.demo_pages.len(), manifest.files.len(), manifest.digest,
        );
    } else {
        let manifest = generate(&root, Path::new(out), seed)?;
        println!(
            "reference site B: {} pages, {} files, digest {} (seed {seed:#x}) in {out}",
            manifest.pages,
            manifest.files.len(),
            manifest.digest
        );
    }
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

fn baseline_files(profile: &Profile, slots: &[PageSlot], seed: u64) -> BTreeMap<String, Vec<u8>> {
    let mut files = BTreeMap::new();
    let mut rng = Rng(seed);
    for slot in slots {
        let text = page(profile, slot, slots, &mut rng);
        files.insert(format!("content/{}", slot.path), text.into_bytes());
    }
    files.insert("rustify-docs.toml".into(), config().into_bytes());
    files.insert("static/logo.svg".into(), LOGO.as_bytes().to_vec());
    files
}

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
    let slots = slots();
    let files = baseline_files(&profile, &slots, seed);
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

/// One P2 candidate's source and canonical route; each contains four islands.
#[derive(Debug, Serialize)]
pub struct P2Page {
    pub path: String,
    pub route: String,
    pub guide: String,
}

/// The P2 addition has its own identity; it never rewrites the P1 manifest contract.
#[derive(Debug, Serialize)]
pub struct P2Manifest {
    pub generator_version: u32,
    pub baseline_generator_version: u32,
    pub baseline_pages: usize,
    pub baseline_digest: String,
    pub seed: u64,
    pub profile_sha256: String,
    pub pages: usize,
    pub languages: BTreeMap<String, usize>,
    pub demo_pages: Vec<P2Page>,
    /// Original measurement fixture inputs, before SDK dependency projection.
    pub template_sha256: BTreeMap<String, String>,
    /// The actual paired CLI/framework identities; absent for development paths.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_pair: Option<BTreeMap<String, String>>,
    pub lock_sha256: String,
    /// Every generated author input, excluding this manifest itself.
    pub files: BTreeMap<String, String>,
    /// SHA-256 over every `path sha256` line, in path order.
    pub digest: String,
}

fn fingerprints(files: &BTreeMap<String, Vec<u8>>) -> (BTreeMap<String, String>, String) {
    let mut digests = BTreeMap::new();
    let mut all = Sha256::new();
    for (path, bytes) in files {
        let digest = sha256_hex(bytes);
        all.update(format!("{path} {digest}\n").as_bytes());
        digests.insert(path.clone(), digest);
    }
    (digests, format!("{:x}", all.finalize()))
}

/// Writes an independent Rust author project for the B island measurements.
/// Its original 500 content files are identical to P1; twelve English guide
/// skeletons add the fixture's CatalogProxy and three matching Counter islands.
pub fn generate_p2(root: &Path, out: &Path, seed: u64) -> Result<P2Manifest, String> {
    if out.exists()
        && std::fs::read_dir(out)
            .map_err(|e| e.to_string())?
            .next()
            .is_some()
    {
        return Err(format!("{} is not empty", out.display()));
    }
    let profile_bytes = std::fs::read(root.join(PROFILE))
        .map_err(|e| format!("{PROFILE}: {e}; use the frozen P1 profile"))?;
    let profile: Profile =
        serde_json::from_slice(&profile_bytes).map_err(|e| format!("{PROFILE}: {e}"))?;
    if profile.guides.len() != GUIDES.len() {
        return Err(format!(
            "{PROFILE} has {} guides, not {}",
            profile.guides.len(),
            GUIDES.len()
        ));
    }
    let base_slots = slots();
    let mut files = baseline_files(&profile, &base_slots, seed);
    let baseline_digest = fingerprints(&files).1;
    let fixture = root.join(P2_FIXTURE);
    let mut template_sha256 = BTreeMap::new();
    for path in P2_TEMPLATE_FILES {
        let mut bytes =
            std::fs::read(fixture.join(path)).map_err(|e| format!("{P2_FIXTURE}/{path}: {e}"))?;
        let digest = sha256_hex(&bytes);
        template_sha256.insert(format!("{P2_FIXTURE}/{path}"), digest);
        if path == "Cargo.toml" {
            let mut cargo =
                String::from_utf8(bytes).map_err(|e| format!("{P2_FIXTURE}/{path}: {e}"))?;
            for library in ["rustify-ui", "rustify-components", "rustify-docs"] {
                let old = format!("path = \"../../../../crates/{library}\"");
                if cargo.matches(&old).count() != 1 {
                    return Err(format!(
                        "{P2_FIXTURE}/Cargo.toml must contain exactly one SDK path for {library}"
                    ));
                }
                let sdk = std::fs::canonicalize(root.join("crates").join(library))
                    .map_err(|e| format!("the local {library} SDK path cannot be resolved: {e}"))?;
                let sdk = sdk.to_str().ok_or("the local SDK path must be UTF-8")?;
                let quoted = serde_json::to_string(sdk).map_err(|e| e.to_string())?;
                cargo = cargo.replacen(&old, &format!("path = {quoted}"), 1);
            }
            bytes = cargo.into_bytes();
        }
        files.insert(path.to_string(), bytes);
    }
    files.insert("src/lib.rs".into(), P2_LIBRARY.as_bytes().to_vec());
    let proxy = std::fs::read_to_string(fixture.join("content/proxy.md"))
        .map_err(|e| format!("{P2_FIXTURE}/content/proxy.md: {e}"))?;
    template_sha256.insert(
        format!("{P2_FIXTURE}/content/proxy.md"),
        sha256_hex(proxy.as_bytes()),
    );
    let start = proxy
        .find("\n## All real categories\n")
        .ok_or("the measurement fixture proxy needs its All real categories section")?;
    let proxy = &proxy[start..];
    let mut rng = Rng(seed ^ P2_RNG_STREAM);
    let mut demo_pages = Vec::new();
    for (guide, name) in GUIDES.iter().enumerate() {
        let slot = PageSlot {
            path: format!("p2/{name}.md"),
            lang: "en",
            group: 0,
            order: guide + 1,
            guide,
        };
        let generated = page(&profile, &slot, &base_slots, &mut rng);
        let (_, body) = generated
            .split_once("\n---\n\n")
            .ok_or("generated B page has no frontmatter boundary")?;
        let text = format!(
            "---\ntitle: \"Reference B: {name} DOM measurement\"\ndescription: \"Frozen {name} content distribution, all 24 DOM categories and three matching islands.\"\nlang: en\nsidebar:\n  group: \"P2 demonstrations\"\n  order: {}\n---\n\n{body}{proxy}", guide + 1,
        );
        let path = format!("content/{}", slot.path);
        files.insert(path.clone(), text.into_bytes());
        demo_pages.push(P2Page {
            path,
            route: format!("/p2/{name}/"),
            guide: (*name).to_string(),
        });
    }
    let mut configuration = config();
    configuration.push_str("\n[rust]\nmanifest = \"Cargo.toml\"\nregistry = \"src/lib.rs\"\n\n[[nav.groups]]\nlabel = \"P2 demonstrations\"\nitems = [");
    for (index, name) in GUIDES.iter().enumerate() {
        if index > 0 {
            configuration.push_str(", ");
        }
        configuration.push_str(&format!("\"p2/{name}.md\""));
    }
    configuration.push_str("]\n");
    files.insert("rustify-docs.toml".into(), configuration.into_bytes());
    for (path, bytes) in &files {
        let target = out.join(path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        std::fs::write(&target, bytes).map_err(|e| format!("{}: {e}", target.display()))?;
    }
    let pair = crate::docs::pair_project(root, &out.join("Cargo.toml"), None)?;
    let framework_pair = if pair["framework_revision"].as_str() == Some("") {
        None
    } else {
        Some(BTreeMap::from([
            (
                "candidate_sha".to_string(),
                pair["candidate_sha"]
                    .as_str()
                    .ok_or("pair helper omitted C")?
                    .to_string(),
            ),
            (
                "framework_revision".to_string(),
                pair["framework_revision"]
                    .as_str()
                    .ok_or("pair helper omitted R")?
                    .to_string(),
            ),
        ]))
    };
    for path in ["Cargo.toml", "Cargo.lock"] {
        files.insert(
            path.to_string(),
            std::fs::read(out.join(path)).map_err(|e| format!("{path}: {e}"))?,
        );
    }
    let lock_sha256 = sha256_hex(&files["Cargo.lock"]);
    let (digests, digest) = fingerprints(&files);
    let manifest = P2Manifest {
        generator_version: P2_GENERATOR_VERSION,
        baseline_generator_version: GENERATOR_VERSION,
        baseline_pages: base_slots.len(),
        baseline_digest,
        seed,
        profile_sha256: sha256_hex(&profile_bytes),
        pages: base_slots.len() + demo_pages.len(),
        languages: BTreeMap::from([
            ("en".to_string(), PAGES_PER_LANGUAGE + demo_pages.len()),
            ("zh".to_string(), PAGES_PER_LANGUAGE),
        ]),
        demo_pages,
        template_sha256,
        framework_pair,
        lock_sha256,
        files: digests,
        digest,
    };
    let json = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())? + "\n";
    std::fs::write(out.join("b-p2-manifest.json"), json).map_err(|e| e.to_string())?;
    Ok(manifest)
}

/// A bilingual search consumer using the frozen guide distribution. Its
/// registered home counts toward capacity; excluded sentinels never do.
pub fn generate_search(root: &Path, out: &Path, seed: u64, pages: usize) -> Result<(), String> {
    if out.exists()
        && std::fs::read_dir(out)
            .map_err(|e| e.to_string())?
            .next()
            .is_some()
    {
        return Err(format!("{} is not empty", out.display()));
    }
    let profile_bytes = std::fs::read(root.join(PROFILE)).map_err(|e| e.to_string())?;
    let profile: Profile = serde_json::from_slice(&profile_bytes).map_err(|e| e.to_string())?;
    if profile.guides.len() != GUIDES.len() {
        return Err("search corpus needs all twelve frozen guide profiles".into());
    }
    let mut slots: Vec<PageSlot> = (1..pages)
        .map(|i| PageSlot {
            path: format!("page-{i:04}.md"),
            lang: "en",
            group: 0,
            order: i,
            guide: (i - 1) % GUIDES.len(),
        })
        .collect();
    slots.extend((1..=pages / 2).map(|i| PageSlot {
        path: format!("zh/page-{i:04}.md"),
        lang: "zh",
        group: 0,
        order: i,
        guide: (i - 1) % GUIDES.len(),
    }));
    let mut files = BTreeMap::new();
    let mut rng = Rng(seed ^ 0x7034_7365_6172_6368);
    for slot in &slots {
        let generated = page(&profile, slot, &slots, &mut rng);
        let (_, body) = generated
            .split_once("\n---\n\n")
            .ok_or("generated search page has no frontmatter boundary")?;
        let zh = slot.lang == "zh";
        let title = match (slot.order, zh) {
            (1, true) => "主题切换".into(),
            (1, false) => "Theme switching".into(),
            (2, true) => "使用ThemeScope".into(),
            (2, false) => "ThemeScope reference".into(),
            (_, true) => format!("文档示例 {}", slot.order),
            (_, false) => format!("Guide sample {}", slot.order),
        };
        let body = if let Some(rest) = body.strip_prefix("# ") {
            rest.split_once('\n').map_or(body, |(_, body)| body)
        } else {
            body
        };
        let marker = if slot.order == pages / 2 + 1 {
            "FallbackOnlyToken"
        } else {
            "SearchCorpusToken"
        };
        let distraction = if slot.order == 3 {
            "ThemeScope ".repeat(100)
        } else {
            String::new()
        };
        let text = format!("---\ntitle: \"{title}\"\ndescription: \"{}\"\nlang: {}\n---\n# {title}\n\n{marker}. {distraction}\n\n{body}", if zh { "按真实指南分布生成的搜索测试内容。" } else { "Search measurement content shaped like the frozen guides." }, slot.lang);
        files.insert(format!("content/{}", slot.path), text.into_bytes());
    }
    for (name, metadata, text) in [
        ("draft", "draft: true", "DraftSecretSentinel"),
        ("search-excluded", "search: false", "SearchExcludedSentinel"),
        (
            "ai-excluded",
            "search: false\nai: false",
            "AiExcludedSentinel",
        ),
    ] {
        files.insert(format!("content/{name}.md"), format!("---\ntitle: {name}\ndescription: Excluded sentinel\n{metadata}\n---\n# {name}\n\n{text}\n").into_bytes());
    }
    let items = (1..pages)
        .map(|i| format!("\"page-{i:04}.md\""))
        .collect::<Vec<_>>()
        .join(", ");
    let configuration = format!("schema_version=1\n[site]\nname=\"Search corpus\"\nlang=\"en\"\nurl=\"https://docs.example.net/\"\nbase=\"/\"\n[rust]\nmanifest=\"Cargo.toml\"\nregistry=\"src/lib.rs\"\n[[nav.groups]]\nlabel=\"Home\"\nroutes=[\"/\"]\n[[nav.groups]]\nlabel=\"Guides\"\nitems=[{items}]\n[[i18n.locales]]\nlang=\"en\"\nlabel=\"English\"\nprefix=\"\"\n[[i18n.locales]]\nlang=\"zh\"\nlabel=\"中文\"\nprefix=\"zh\"\n");
    files.insert("rustify-docs.toml".into(), configuration.into_bytes());
    // Use the existing consumer toolchain and helper, without any demo islands.
    for path in ["Cargo.toml", "Cargo.lock", "src/bin/docs-render.rs"] {
        let mut bytes =
            std::fs::read(root.join(P2_FIXTURE).join(path)).map_err(|e| e.to_string())?;
        if path == "Cargo.toml" {
            let mut cargo = String::from_utf8(bytes).map_err(|e| e.to_string())?;
            for library in ["rustify-ui", "rustify-components", "rustify-docs"] {
                let old = format!("path = \"../../../../crates/{library}\"");
                let sdk = std::fs::canonicalize(root.join("crates").join(library))
                    .map_err(|e| e.to_string())?;
                let quoted = serde_json::to_string(sdk.to_str().ok_or("SDK path must be UTF-8")?)
                    .map_err(|e| e.to_string())?;
                if cargo.matches(&old).count() != 1 {
                    return Err(format!("consumer template needs one {library} path"));
                }
                cargo = cargo.replacen(&old, &format!("path = {quoted}"), 1);
            }
            bytes = cargo.into_bytes();
        }
        files.insert(path.into(), bytes);
    }
    files.insert("src/lib.rs".into(), br##"use leptos::prelude::*;
use rustify_docs::{RenderContext, SiteRegistry};
pub fn site() -> SiteRegistry {
    rustify_docs::site! { pages {
        Home { route: "/", title: "Custom search home", description: "A real registered page inside the search capacity." } => home,
        Excluded { route: "/undescribed/", title: "Undescribed custom", search: false } => undescribed,
    }}
}
fn home(_: RenderContext) -> impl IntoView {
    leptos::view! { <h1>"Custom search home"</h1><p>"CustomOnlyToken from a registered DOM page."</p><h2 id="custom-section">"Custom section"</h2><p>"ReachableCustomToken"</p> }
}
fn undescribed(_: RenderContext) -> impl IntoView {
    leptos::view! { <h1>"Undescribed custom"</h1><p>"UndescribedSentinel"</p> }
}
"##.to_vec());
    for (path, bytes) in &files {
        let target = out.join(path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(target, bytes).map_err(|e| e.to_string())?;
    }
    let pair = crate::docs::pair_project(root, &out.join("Cargo.toml"), None)?;
    for path in ["Cargo.toml", "Cargo.lock"] {
        files.insert(
            path.into(),
            std::fs::read(out.join(path)).map_err(|e| e.to_string())?,
        );
    }
    let (digests, digest) = fingerprints(&files);
    let manifest = serde_json::json!({
        "generator_version": 1, "seed": seed, "profile_sha256": sha256_hex(&profile_bytes),
        "pages_per_language": pages, "translated_zh": pages / 2, "fallback_zh": pages / 2,
        "custom_route": "/", "files": digests, "digest": digest,
        "framework_pair": { "candidate_sha": pair["candidate_sha"], "framework_revision": pair["framework_revision"] },
    });
    std::fs::write(
        out.join("p4-search-manifest.json"),
        serde_json::to_vec_pretty(&manifest).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Scratch(PathBuf);

    impl Scratch {
        fn new() -> Self {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let stamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "rustify-b-corpus-{}-{stamp}-{}",
                std::process::id(),
                NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            ));
            std::fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn p4_search_cli_writes_independent_reproducible_bilingual_inputs() {
        let scratch = Scratch::new();
        let first = scratch.0.join("first");
        let second = scratch.0.join("second");
        for out in [&first, &second] {
            run(&[
                "--p4-search".into(),
                "--pages".into(),
                "100".into(),
                "--out".into(),
                out.to_str().unwrap().into(),
            ])
            .unwrap();
        }
        let name = "p4-search-manifest.json";
        let bytes = std::fs::read(first.join(name)).unwrap();
        assert_eq!(bytes, std::fs::read(second.join(name)).unwrap());
        let manifest: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(manifest["pages_per_language"], 100);
        assert_eq!(manifest["translated_zh"], 50);
        assert_eq!(manifest["fallback_zh"], 50);
        for path in manifest["files"].as_object().unwrap().keys() {
            assert_eq!(
                std::fs::read(first.join(path)).unwrap(),
                std::fs::read(second.join(path)).unwrap(),
                "{path}"
            );
        }
        assert!(!first.join("b-manifest.json").exists());
        assert!(
            std::fs::read_to_string(first.join("content/zh/page-0001.md"))
                .unwrap()
                .contains("主题切换")
        );
        assert!(std::fs::read_to_string(first.join("src/lib.rs"))
            .unwrap()
            .contains("search: false"));
    }

    #[test]
    fn p2_preserves_the_frozen_profile_and_all_original_content_bytes() {
        let root = crate::build::repo_root();
        let profile = std::fs::read(root.join(PROFILE)).unwrap();
        let scratch = Scratch::new();
        let base = scratch.0.join("p1");
        let extended = scratch.0.join("p2");
        let baseline = generate(&root, &base, DEFAULT_SEED).unwrap();
        let p2 = generate_p2(&root, &extended, DEFAULT_SEED).unwrap();
        assert_eq!(baseline.generator_version, 2);
        assert_eq!(baseline.pages, 500);
        assert_eq!(
            baseline.languages,
            BTreeMap::from([("en".to_string(), 250), ("zh".to_string(), 250)])
        );
        assert_eq!(p2.baseline_digest, baseline.digest);
        assert_eq!(p2.profile_sha256, baseline.profile_sha256);
        assert_eq!(std::fs::read(root.join(PROFILE)).unwrap(), profile);
        for path in baseline
            .files
            .keys()
            .filter(|path| path.starts_with("content/"))
        {
            assert_eq!(
                std::fs::read(base.join(path)).unwrap(),
                std::fs::read(extended.join(path)).unwrap(),
                "{path}"
            );
        }
        assert!(!base.join("Cargo.toml").exists());
        assert!(!base.join("src").exists());
        assert!(base.join("b-manifest.json").is_file());
        assert!(!extended.join("b-manifest.json").exists());
        assert!(extended.join("b-p2-manifest.json").is_file());
    }

    #[test]
    fn p2_candidates_have_four_real_islands_valid_links_and_the_exact_author_sources() {
        let root = crate::build::repo_root();
        let scratch = Scratch::new();
        let out = scratch.0.join("p2");
        let manifest = generate_p2(&root, &out, DEFAULT_SEED).unwrap();
        assert_eq!(manifest.pages, 512);
        assert_eq!(
            manifest.languages,
            BTreeMap::from([("en".to_string(), 262), ("zh".to_string(), 250)])
        );
        assert_eq!(manifest.demo_pages.len(), 12);
        assert_eq!(
            manifest
                .files
                .keys()
                .filter(|path| path.ends_with(".md"))
                .count(),
            512
        );
        for candidate in &manifest.demo_pages {
            let path = out.join(&candidate.path);
            let text = std::fs::read_to_string(&path).unwrap();
            let mut embeds = Vec::new();
            let mut capture = None::<(String, String)>;
            for event in Parser::new(&text) {
                match event {
                    Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info)))
                        if matches!(info.as_ref(), "rustify-demo" | "rustify-component") =>
                    {
                        capture = Some((info.to_string(), String::new()));
                    }
                    Event::Text(body) => {
                        if let Some((_, text)) = &mut capture {
                            text.push_str(&body);
                        }
                    }
                    Event::End(TagEnd::CodeBlock) => {
                        if let Some(block) = capture.take() {
                            embeds.push(block);
                        }
                    }
                    Event::Start(Tag::Link { dest_url, .. }) => {
                        let target = path
                            .parent()
                            .unwrap()
                            .join(dest_url.as_ref())
                            .canonicalize()
                            .unwrap();
                        assert!(
                            target.starts_with(out.canonicalize().unwrap().join("content")),
                            "{dest_url}"
                        );
                        assert!(target.is_file(), "{dest_url}");
                    }
                    _ => {}
                }
            }
            assert_eq!(embeds.len(), 4, "{}", candidate.path);
            assert_eq!(
                embeds[0],
                (
                    "rustify-demo".to_string(),
                    "name: CatalogProxy\n".to_string()
                )
            );
            assert!(embeds[1..]
                .iter()
                .all(|(kind, declaration)| kind == "rustify-component"
                    && declaration.contains("name: Counter\n")
                    && declaration.contains("  initial: 2\n")));
            assert!(embeds[1..].windows(2).all(|pair| pair[0] == pair[1]));
        }
        for path in P2_TEMPLATE_FILES
            .into_iter()
            .filter(|path| *path != "Cargo.toml" && *path != "Cargo.lock")
        {
            assert_eq!(
                std::fs::read(out.join(path)).unwrap(),
                std::fs::read(root.join(P2_FIXTURE).join(path)).unwrap(),
                "{path}"
            );
        }
        let cargo = std::fs::read_to_string(out.join("Cargo.toml")).unwrap();
        let lock = std::fs::read(out.join("Cargo.lock")).unwrap();
        let original_lock = std::fs::read(root.join(P2_FIXTURE).join("Cargo.lock")).unwrap();
        if manifest.framework_pair.is_none() {
            assert_eq!(lock, original_lock);
        }
        for library in ["rustify-ui", "rustify-components", "rustify-docs"] {
            if let Some(pair) = &manifest.framework_pair {
                assert!(cargo.contains(&format!(
                    "{library} = {{ git = \"https://github.com/rockie/rustify-ui\", rev = \"{}\", default-features = false }}",
                    pair["framework_revision"]
                )));
            } else {
                let sdk = root.join("crates").join(library).canonicalize().unwrap();
                let path = serde_json::to_string(sdk.to_str().unwrap()).unwrap();
                assert!(cargo.contains(&format!("path = {path}")));
            }
        }
        assert_eq!(
            manifest.template_sha256[&format!("{P2_FIXTURE}/Cargo.lock")],
            sha256_hex(&original_lock)
        );
        assert_eq!(manifest.lock_sha256, sha256_hex(&lock));
        let registry = std::fs::read_to_string(out.join("src/lib.rs")).unwrap();
        assert!(registry.contains("CatalogProxy => crate::demos::catalog_proxy"));
        assert!(!registry.contains("BrowserCounter"));
        assert!(!registry.contains("FailureDemo"));
        assert!(!registry.contains("slots {"));
        assert!(!out.join("theme.json").exists());
    }

    #[test]
    fn p2_is_reproducible_and_never_overwrites_a_nonempty_project() {
        let root = crate::build::repo_root();
        let scratch = Scratch::new();
        let first = scratch.0.join("first");
        let second = scratch.0.join("second");
        let one = generate_p2(&root, &first, 17).unwrap();
        let two = generate_p2(&root, &second, 17).unwrap();
        assert_eq!(one.digest, two.digest);
        assert_eq!(one.files, two.files);
        assert_eq!(
            std::fs::read(first.join("b-p2-manifest.json")).unwrap(),
            std::fs::read(second.join("b-p2-manifest.json")).unwrap()
        );
        let before = std::fs::read(first.join("content/index.md")).unwrap();
        assert!(generate_p2(&root, &first, 18)
            .unwrap_err()
            .contains("not empty"));
        assert_eq!(
            std::fs::read(first.join("content/index.md")).unwrap(),
            before
        );
    }

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
