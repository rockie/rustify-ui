//! Markdown to an HTML fragment, through this crate's own writer.
//!
//! pulldown-cmark parses with source offsets; the writer below turns its
//! events into the markup the default theme styles. It is not
//! `pulldown_cmark::html::push_html`, which writes a `style` attribute for
//! table alignment that the site's CSP would refuse: alignment becomes a
//! class here, headings get ids that match GitHub's, GitHub's five alerts and
//! extended autolinks render as they do there, and a code block is
//! highlighted into classes with its raw text kept beside it for the copy
//! button. Two HTML comments GitHub shows as nothing add what Markdown
//! lacks: a code group, and code included from a source file.
//!
//! A fragment depends on its own source alone, so it can be cached by it.
//! What depends on the rest of the site (where a `guide.md#setup` link or an
//! image ends up) is left as written and recorded as a slot: the byte range
//! of the URL in the fragment's HTML, which the content graph fills in for
//! each generation. Raw HTML is checked against the site's policy here, and
//! its URLs become slots the same way.

use std::collections::BTreeMap;
use std::ops::Range;

use pulldown_cmark::{
    Alignment, BlockQuoteKind, CodeBlockKind, Event, HeadingLevel, LinkType, Options, Parser, Tag,
    TagEnd, TextMergeWithOffset,
};

use crate::autolink;
use crate::diagnostic::Issue;
use crate::escape::{escape_html_into, escape_url_into};
use crate::highlight::Highlighter;
use crate::html_check::{self, UrlRole};
use crate::include::{self, Directive};
use crate::labels::Labels;
use crate::slug::{slugify, Slugger};

/// The parser options every page is read with: GFM tables, task lists,
/// footnotes, strikethrough and alerts, and `{#id}` on headings.
pub fn options() -> Options {
    Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_GFM
        | Options::ENABLE_HEADING_ATTRIBUTES
}

/// A version for everything above that decides a fragment's bytes; part of
/// the fragment cache key.
pub const WRITER_VERSION: u32 = 4;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Heading {
    pub level: u8,
    pub id: String,
    /// The heading's text, without markup.
    pub text: String,
    /// Whether the id came from `{#id}`.
    pub explicit: bool,
    pub span: Range<usize>,
}

/// What a URL in the fragment is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefKind {
    /// A Markdown link, or `a href` in raw HTML.
    Link,
    /// A Markdown image, or an image a raw element loads.
    Image,
    /// Audio, video or a track a raw element loads.
    Media,
    /// One URL of a raw `srcset`; the slot holds the whole attribute.
    Srcset,
    /// An image a raw element's class loads through the stylesheet; the
    /// slot is the URL inside the class's `url(…)`.
    ClassImage,
}

/// A URL as written, with where it was written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference {
    pub destination: String,
    pub span: Range<usize>,
    pub kind: RefKind,
}

/// Where a reference's URL sits in the fragment's HTML: the escaped value of
/// its attribute, which a generation replaces with the resolved URL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    pub html: Range<usize>,
    pub reference: usize,
}

/// A fenced, indented or included code block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeBlock {
    /// The language tag as written, if any.
    pub tag: Option<String>,
    /// The block's text exactly as the copy button hands it over.
    pub text: String,
    pub span: Range<usize>,
}

/// A code group: its name and its tabs' labels, in order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeGroup {
    pub name: String,
    pub labels: Vec<String>,
    pub span: Range<usize>,
}

/// A rendered Markdown document.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fragment {
    pub html: String,
    pub headings: Vec<Heading>,
    /// The ids the page's footnotes give their references and notes, which
    /// a link may target as it targets a heading's.
    pub anchors: Vec<String>,
    /// The text of the first level-one heading.
    pub title: Option<String>,
    pub h1_count: usize,
    /// The first top-level paragraph's text, for a description.
    pub summary: Option<String>,
    /// Every URL the content wrote, Markdown and raw HTML alike.
    pub references: Vec<Reference>,
    pub slots: Vec<Slot>,
    /// External URLs a bare autolink made, which no slot holds.
    pub autolinks: Vec<String>,
    pub code_blocks: Vec<CodeBlock>,
    pub code_groups: Vec<CodeGroup>,
    /// The project-relative files `rustify:include` read.
    pub includes: Vec<String>,
    pub issues: Vec<Issue>,
}

/// The files a page's `rustify:include` directives name, read before the
/// page renders: the contents, or why they could not be read.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IncludeFiles(pub BTreeMap<String, Result<String, String>>);

/// The project-relative paths `source`'s include directives name, so they
/// can be read (and hashed into the fragment's cache key) before rendering.
pub fn include_paths(source: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut rest = source;
    while let Some(start) = rest.find("<!--") {
        let after = &rest[start..];
        let Some(end) = after.find("-->") else {
            break;
        };
        if let Some(Ok(directive)) = include::directive(&after[..end + 3]) {
            if let Ok(path) = crate::config::relative_path(&directive.path) {
                if !paths.contains(&path) {
                    paths.push(path);
                }
            }
        }
        rest = &after[end + 3..];
    }
    paths
}

/// What a page is rendered with.
pub struct Context<'a> {
    pub highlighter: &'a Highlighter,
    pub labels: &'a Labels,
    /// The page is in neither interface language, so interface strings say
    /// `lang="en"`.
    pub foreign: bool,
    pub includes: &'a IncludeFiles,
}

/// Renders `source`, whose first byte is at `offset` in its file (after any
/// frontmatter), so every span in the result is a file offset.
pub fn render(source: &str, offset: usize, context: &Context) -> Fragment {
    let events: Vec<(Event, Range<usize>)> =
        TextMergeWithOffset::new(Parser::new_ext(source, options()).into_offset_iter())
            .map(|(event, span)| (event, span.start + offset..span.end + offset))
            .collect();
    let mut writer = Writer::new(context);
    writer.reserve_explicit_ids(&events);
    for (event, span) in events {
        writer.event(event, span);
    }
    writer.finish()
}

/// HTML being written, and the slots in it.
#[derive(Default)]
struct Buffer {
    html: String,
    slots: Vec<Slot>,
}

impl Buffer {
    fn append(&mut self, other: Buffer) {
        let base = self.html.len();
        self.html.push_str(&other.html);
        self.slots.extend(other.slots.into_iter().map(|slot| Slot {
            html: slot.html.start + base..slot.html.end + base,
            reference: slot.reference,
        }));
    }

    /// Inserts `text` at `at`, moving the slots after it.
    fn insert(&mut self, at: usize, text: &str) {
        self.html.insert_str(at, text);
        for slot in &mut self.slots {
            if slot.html.start >= at {
                slot.html = slot.html.start + text.len()..slot.html.end + text.len();
            }
        }
    }
}

struct HeadingCapture {
    level: HeadingLevel,
    id: Option<String>,
    span: Range<usize>,
    text: String,
}

struct CodeCapture {
    kind: CodeBlockKind<'static>,
    text: String,
    span: Range<usize>,
}

struct ImageCapture {
    destination: String,
    title: String,
    alt: String,
    span: Range<usize>,
}

struct TableState {
    alignments: Vec<Alignment>,
    cell: usize,
    in_head: bool,
}

struct GroupCapture {
    name: String,
    /// Where in the document buffer the group's wrapper goes.
    start: usize,
    labels: Vec<String>,
    span: Range<usize>,
    /// Something other than a code block was found inside.
    stray: bool,
}

/// A code block to write: its text, tag, title and options.
struct BlockSpec {
    text: String,
    tag: Option<String>,
    title: Option<String>,
    linenos: bool,
    highlight: Vec<std::ops::RangeInclusive<usize>>,
    span: Range<usize>,
}

struct Writer<'a> {
    context: &'a Context<'a>,
    buffers: Vec<Buffer>,
    slugger: Slugger,
    heading: Option<HeadingCapture>,
    code: Option<CodeCapture>,
    image: Option<ImageCapture>,
    /// Inside a link: text is not scanned for autolinks.
    links_open: usize,
    tables: Vec<TableState>,
    group: Option<GroupCapture>,
    /// Block containers open around the current event, so the first
    /// top-level paragraph can be told from one inside a list or quote.
    depth: usize,
    summary: Option<String>,
    summary_capture: Option<String>,
    footnote_order: Vec<String>,
    footnote_references: Vec<usize>,
    footnotes: Vec<(String, Buffer)>,
    out: Fragment,
}

impl<'a> Writer<'a> {
    fn new(context: &'a Context<'a>) -> Self {
        Self {
            context,
            buffers: vec![Buffer::default()],
            slugger: Slugger::default(),
            heading: None,
            code: None,
            image: None,
            links_open: 0,
            tables: Vec::new(),
            group: None,
            depth: 0,
            summary: None,
            summary_capture: None,
            footnote_order: Vec::new(),
            footnote_references: Vec::new(),
            footnotes: Vec::new(),
            out: Fragment::default(),
        }
    }

    /// `{#id}` ids are taken before any automatic id is handed out, so an
    /// automatic one never collides with an explicit one further down.
    fn reserve_explicit_ids(&mut self, events: &[(Event, Range<usize>)]) {
        let mut seen: Vec<&str> = Vec::new();
        for (event, span) in events {
            if let Event::Start(Tag::Heading { id: Some(id), .. }) = event {
                if seen.contains(&id.as_ref()) {
                    self.out.issues.push(
                        Issue::error(
                            span.clone(),
                            "duplicate-heading-id",
                            format!("the heading id `{id}` is used twice on this page"),
                        )
                        .suggest("give one of the headings another {#id}"),
                    );
                } else {
                    seen.push(id);
                }
            }
        }
        self.slugger = Slugger::new(seen);
    }

    fn buf(&mut self) -> &mut Buffer {
        self.buffers.last_mut().expect("the document buffer")
    }

    fn push(&mut self, html: &str) {
        self.buf().html.push_str(html);
    }

    /// Writes `destination` as an attribute value and records it.
    fn push_url(&mut self, destination: &str, prefix: &str, span: Range<usize>, kind: RefKind) {
        let reference = self.out.references.len();
        self.out.references.push(Reference {
            destination: destination.to_owned(),
            span,
            kind,
        });
        let buffer = self.buf();
        let start = buffer.html.len();
        buffer.html.push_str(prefix);
        escape_url_into(&mut buffer.html, destination);
        let end = buffer.html.len();
        buffer.slots.push(Slot {
            html: start..end,
            reference,
        });
    }

    fn capture_text(&mut self, text: &str) {
        if let Some(heading) = &mut self.heading {
            heading.text.push_str(text);
        }
        if let Some(summary) = &mut self.summary_capture {
            summary.push_str(text);
        }
    }

    fn text(&mut self, text: &str) {
        self.capture_text(text);
        let links = if self.links_open == 0 {
            autolink::find(text)
        } else {
            Vec::new()
        };
        let mut html = String::with_capacity(text.len());
        let mut at = 0;
        for link in links {
            let range = link.range().clone();
            escape_html_into(&mut html, &text[at..range.start]);
            html.push_str("<a href=\"");
            escape_url_into(&mut html, link.href());
            html.push('"');
            if let autolink::Autolink::Url { .. } = link {
                html.push_str(" rel=\"noopener\"");
                self.out.autolinks.push(link.href().to_owned());
            }
            html.push('>');
            escape_html_into(&mut html, &text[range.clone()]);
            html.push_str("</a>");
            at = range.end;
        }
        escape_html_into(&mut html, &text[at..]);
        self.push(&html);
    }

    /// `lang="en"` on an interface string inside foreign-language content.
    fn ui_lang(&self) -> &'static str {
        if self.context.foreign {
            " lang=\"en\""
        } else {
            ""
        }
    }

    /// Notes block content inside a code group, which may hold only code.
    fn block_in_group(&mut self) {
        if let Some(group) = &mut self.group {
            group.stray = true;
        }
    }

    fn event(&mut self, event: Event, span: Range<usize>) {
        if let Some(code) = &mut self.code {
            match event {
                Event::Text(text) => code.text.push_str(&text),
                Event::End(TagEnd::CodeBlock) => {
                    let code = self.code.take().expect("an open code block");
                    self.fenced_block(code);
                }
                _ => {}
            }
            return;
        }
        if let Some(image) = &mut self.image {
            match event {
                Event::Text(text) | Event::Code(text) => image.alt.push_str(&text),
                Event::End(TagEnd::Image) => {
                    let image = self.image.take().expect("an open image");
                    self.image(image);
                }
                _ => {}
            }
            return;
        }
        match event {
            Event::Start(tag) => self.start(tag, span),
            Event::End(tag) => self.end(tag),
            Event::Text(text) => self.text(&text),
            Event::Code(code) => {
                self.capture_text(&code);
                let mut html = String::from("<code>");
                escape_html_into(&mut html, &code);
                html.push_str("</code>");
                self.push(&html);
            }
            Event::Html(html) => self.html(&html, span, true),
            Event::InlineHtml(html) => self.html(&html, span, false),
            Event::FootnoteReference(label) => self.footnote_reference(&label),
            Event::SoftBreak => self.text("\n"),
            Event::HardBreak => self.push("<br>\n"),
            Event::Rule => {
                self.block_in_group();
                self.push("<hr>\n");
            }
            Event::TaskListMarker(checked) => {
                // Named, so the box is not an unlabelled control; its state
                // and the item's text follow.
                let task = self.context.labels.task;
                self.push(&format!(
                    "<input type=\"checkbox\" disabled{} aria-label=\"{task}\"> ",
                    if checked { " checked" } else { "" }
                ));
            }
            Event::InlineMath(_) | Event::DisplayMath(_) => {}
        }
    }

    /// Raw HTML: this crate's own comments, or author markup checked against
    /// the policy and passed through with its URLs as slots.
    fn html(&mut self, html: &str, span: Range<usize>, block: bool) {
        let trimmed = html.trim();
        if trimmed.starts_with("<!--") && trimmed.contains("rustify:") {
            if let Some(directive) = include::directive(trimmed) {
                if !block {
                    self.out.issues.push(Issue::error(
                        span,
                        "include-inline",
                        "an include must be on a line of its own",
                    ));
                    return;
                }
                match directive {
                    Ok(directive) => self.include(directive, span),
                    Err(message) => self.out.issues.push(Issue::error(span, "include", message)),
                }
                return;
            }
            if let Some(command) = group_marker(trimmed) {
                self.group_marker(command, span);
                return;
            }
        }
        if block && !trimmed.starts_with("<!--") {
            self.block_in_group();
        }
        let (issues, slots) = html_check::check(html);
        for issue in issues {
            let start = (span.start + issue.span.start).min(span.end);
            let end = (span.start + issue.span.end).min(span.end);
            self.out.issues.push(Issue {
                span: start..end,
                ..issue
            });
        }
        let base = self.buf().html.len();
        self.buf().html.push_str(html);
        for slot in slots {
            let kind = match slot.role {
                UrlRole::Link => RefKind::Link,
                UrlRole::Resource { image: true } => RefKind::Image,
                UrlRole::Resource { image: false } => RefKind::Media,
                UrlRole::Srcset => RefKind::Srcset,
                UrlRole::ClassImage => RefKind::ClassImage,
            };
            let reference = self.out.references.len();
            self.out.references.push(Reference {
                destination: slot.value,
                span: span.clone(),
                kind,
            });
            self.buf().slots.push(Slot {
                html: base + slot.span.start..base + slot.span.end,
                reference,
            });
        }
    }

    fn group_marker(&mut self, command: GroupMarker, span: Range<usize>) {
        match command {
            GroupMarker::Open(Ok(name)) => {
                if let Some(open) = &self.group {
                    self.out.issues.push(Issue::error(
                        span,
                        "code-group",
                        format!(
                            "code group `{}` is still open; groups do not nest",
                            open.name
                        ),
                    ));
                    return;
                }
                let start = self.buf().html.len();
                self.group = Some(GroupCapture {
                    name,
                    start,
                    labels: Vec::new(),
                    span,
                    stray: false,
                });
            }
            GroupMarker::Open(Err(message)) => {
                self.out
                    .issues
                    .push(Issue::error(span, "code-group", message));
            }
            GroupMarker::Close => {
                let Some(group) = self.group.take() else {
                    self.out.issues.push(Issue::error(
                        span,
                        "code-group",
                        "this closes a code group that was never opened",
                    ));
                    return;
                };
                self.close_group(group, span);
            }
        }
    }

    fn close_group(&mut self, group: GroupCapture, end: Range<usize>) {
        let span = group.span.start..end.end;
        if group.stray {
            self.out.issues.push(
                Issue::error(
                    span.clone(),
                    "code-group",
                    format!(
                        "code group `{}` holds something other than code blocks",
                        group.name
                    ),
                )
                .suggest("keep only fenced code blocks between the two group comments"),
            );
        }
        if group.labels.is_empty() {
            self.out.issues.push(Issue::error(
                span.clone(),
                "code-group",
                format!("code group `{}` has no code blocks", group.name),
            ));
        }
        for (i, label) in group.labels.iter().enumerate() {
            if group.labels[..i].contains(label) {
                self.out.issues.push(
                    Issue::error(
                        span.clone(),
                        "code-group",
                        format!(
                            "code group `{}` has two tabs labelled `{label}`",
                            group.name
                        ),
                    )
                    .suggest("give each block its own title=\"…\""),
                );
            }
        }
        let mut open = String::from("<div class=\"rd-code-group\" data-rd-group=\"");
        escape_html_into(&mut open, &group.name);
        open.push_str(
            "\">\n<div class=\"rd-code-tabs\" hidden><div class=\"rd-code-tablist\" role=\"tablist\">",
        );
        for (i, label) in group.labels.iter().enumerate() {
            open.push_str(&format!(
                "<button type=\"button\" role=\"tab\" aria-selected=\"{}\" data-rd-tab=\"{i}\">",
                i == 0
            ));
            escape_html_into(&mut open, label);
            open.push_str("</button>");
        }
        open.push_str("</div></div>\n");
        let start = group.start;
        self.buf().insert(start, &open);
        self.push("</div>\n");
        self.out.code_groups.push(CodeGroup {
            name: group.name,
            labels: group.labels,
            span,
        });
    }

    fn start(&mut self, tag: Tag, span: Range<usize>) {
        match tag {
            Tag::Paragraph => {
                self.block_in_group();
                if self.depth == 0
                    && self.summary.is_none()
                    && self.summary_capture.is_none()
                    && self.heading.is_none()
                {
                    self.summary_capture = Some(String::new());
                }
                self.push("<p>");
            }
            Tag::Heading {
                level,
                id,
                classes,
                attrs,
            } => {
                self.block_in_group();
                if !classes.is_empty() || !attrs.is_empty() {
                    self.out.issues.push(
                        Issue::error(
                            span.clone(),
                            "heading-attribute",
                            "a heading accepts only `{#id}`; classes and other attributes are not allowed",
                        )
                        .suggest("keep `{#id}` and remove the rest"),
                    );
                }
                self.heading = Some(HeadingCapture {
                    level,
                    id: id.map(|id| id.to_string()),
                    span,
                    text: String::new(),
                });
                self.buffers.push(Buffer::default());
            }
            Tag::BlockQuote(kind) => {
                self.block_in_group();
                self.depth += 1;
                match kind {
                    None => self.push("<blockquote>\n"),
                    Some(kind) => {
                        let (name, title) = self.alert(kind);
                        let lang = self.ui_lang();
                        self.push(&format!(
                            "<div class=\"markdown-alert markdown-alert-{name}\" role=\"note\">\n<p class=\"markdown-alert-title\"{lang}>{}{title}</p>\n",
                            crate::icons::alert(name)
                        ));
                    }
                }
            }
            Tag::CodeBlock(kind) => {
                self.code = Some(CodeCapture {
                    kind: kind.into_static(),
                    text: String::new(),
                    span,
                });
            }
            Tag::HtmlBlock => {}
            Tag::List(start) => {
                self.block_in_group();
                self.depth += 1;
                match start {
                    Some(1) => self.push("<ol>\n"),
                    Some(start) => self.push(&format!("<ol start=\"{start}\">\n")),
                    None => self.push("<ul>\n"),
                }
            }
            Tag::Item => self.push("<li>"),
            Tag::FootnoteDefinition(label) => {
                self.depth += 1;
                self.footnotes.push((label.to_string(), Buffer::default()));
                self.buffers.push(Buffer::default());
            }
            Tag::DefinitionList | Tag::DefinitionListTitle | Tag::DefinitionListDefinition => {}
            Tag::Table(alignments) => {
                self.block_in_group();
                self.depth += 1;
                self.tables.push(TableState {
                    alignments,
                    cell: 0,
                    in_head: false,
                });
                self.push("<div class=\"rd-table\" tabindex=\"0\"><table>\n");
            }
            Tag::TableHead => {
                if let Some(table) = self.tables.last_mut() {
                    table.in_head = true;
                    table.cell = 0;
                }
                self.push("<thead><tr>");
            }
            Tag::TableRow => {
                if let Some(table) = self.tables.last_mut() {
                    table.cell = 0;
                }
                self.push("<tr>");
            }
            Tag::TableCell => {
                let (element, class) = match self.tables.last() {
                    Some(table) => (
                        if table.in_head { "th" } else { "td" },
                        match table.alignments.get(table.cell) {
                            Some(Alignment::Left) => " class=\"text-left\"",
                            Some(Alignment::Center) => " class=\"text-center\"",
                            Some(Alignment::Right) => " class=\"text-right\"",
                            _ => "",
                        },
                    ),
                    None => ("td", ""),
                };
                self.push(&format!("<{element}{class}>"));
            }
            Tag::Emphasis => self.push("<em>"),
            Tag::Strong => self.push("<strong>"),
            Tag::Strikethrough => self.push("<del>"),
            Tag::Superscript => self.push("<sup>"),
            Tag::Subscript => self.push("<sub>"),
            Tag::Link {
                link_type,
                dest_url,
                title,
                ..
            } => {
                self.links_open += 1;
                self.push("<a href=\"");
                let prefix = if link_type == LinkType::Email {
                    "mailto:"
                } else {
                    ""
                };
                self.push_url(&dest_url, prefix, span, RefKind::Link);
                let mut html = String::from("\"");
                if !title.is_empty() {
                    html.push_str(" title=\"");
                    escape_html_into(&mut html, &title);
                    html.push('"');
                }
                if is_external(&dest_url) {
                    html.push_str(" rel=\"noopener\"");
                }
                html.push('>');
                self.push(&html);
            }
            Tag::Image {
                dest_url, title, ..
            } => {
                self.image = Some(ImageCapture {
                    destination: dest_url.to_string(),
                    title: title.to_string(),
                    alt: String::new(),
                    span,
                });
            }
            Tag::MetadataBlock(_) => {}
        }
    }

    fn end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => {
                self.push("</p>\n");
                if self.depth == 0 {
                    if let Some(summary) = self.summary_capture.take() {
                        let summary = summary.split_whitespace().collect::<Vec<_>>().join(" ");
                        if !summary.is_empty() {
                            self.summary = Some(summary);
                        }
                    }
                }
            }
            TagEnd::Heading(_) => self.heading_end(),
            TagEnd::BlockQuote(kind) => {
                self.depth = self.depth.saturating_sub(1);
                self.push(if kind.is_some() {
                    "</div>\n"
                } else {
                    "</blockquote>\n"
                });
            }
            TagEnd::CodeBlock | TagEnd::HtmlBlock => {}
            TagEnd::List(ordered) => {
                self.depth = self.depth.saturating_sub(1);
                self.push(if ordered { "</ol>\n" } else { "</ul>\n" });
            }
            TagEnd::Item => self.push("</li>\n"),
            TagEnd::FootnoteDefinition => {
                self.depth = self.depth.saturating_sub(1);
                let buffer = self.buffers.pop().unwrap_or_default();
                if let Some(last) = self.footnotes.last_mut() {
                    last.1 = buffer;
                }
            }
            TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition => {}
            TagEnd::Table => {
                self.depth = self.depth.saturating_sub(1);
                self.tables.pop();
                self.push("</tbody></table></div>\n");
            }
            TagEnd::TableHead => {
                if let Some(table) = self.tables.last_mut() {
                    table.in_head = false;
                }
                self.push("</tr></thead>\n<tbody>\n");
            }
            TagEnd::TableRow => self.push("</tr>\n"),
            TagEnd::TableCell => {
                let element = match self.tables.last_mut() {
                    Some(table) => {
                        table.cell += 1;
                        if table.in_head {
                            "th"
                        } else {
                            "td"
                        }
                    }
                    None => "td",
                };
                self.push(&format!("</{element}>"));
            }
            TagEnd::Emphasis => self.push("</em>"),
            TagEnd::Strong => self.push("</strong>"),
            TagEnd::Strikethrough => self.push("</del>"),
            TagEnd::Superscript => self.push("</sup>"),
            TagEnd::Subscript => self.push("</sub>"),
            TagEnd::Link => {
                self.links_open = self.links_open.saturating_sub(1);
                self.push("</a>");
            }
            TagEnd::Image | TagEnd::MetadataBlock(_) => {}
        }
    }

    fn heading_end(&mut self) {
        let Some(heading) = self.heading.take() else {
            return;
        };
        let inner = self.buffers.pop().unwrap_or_default();
        let level = heading_level(heading.level);
        let text = heading
            .text
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let explicit = heading.id.is_some();
        let id = match heading.id {
            Some(id) => id,
            None => self.slugger.next(&text),
        };
        if level == 1 {
            self.out.h1_count += 1;
            if self.out.title.is_none() {
                self.out.title = Some(text.clone());
            }
        }
        let mut open = format!("<h{level} id=\"");
        escape_html_into(&mut open, &id);
        open.push_str("\">");
        self.push(&open);
        self.buf().append(inner);
        self.push(&format!("</h{level}>\n"));
        self.out.headings.push(Heading {
            level,
            id,
            text,
            explicit,
            span: heading.span,
        });
    }

    fn alert(&self, kind: BlockQuoteKind) -> (&'static str, &'static str) {
        let labels = self.context.labels;
        match kind {
            BlockQuoteKind::Note => ("note", labels.note),
            BlockQuoteKind::Tip => ("tip", labels.tip),
            BlockQuoteKind::Important => ("important", labels.important),
            BlockQuoteKind::Warning => ("warning", labels.warning),
            BlockQuoteKind::Caution => ("caution", labels.caution),
        }
    }

    fn image(&mut self, image: ImageCapture) {
        if image.alt.trim().is_empty() {
            self.out.issues.push(
                Issue::warning(
                    image.span.clone(),
                    "missing-alt",
                    "this image has no alt text",
                )
                .suggest("describe it between the brackets: ![what it shows](…)"),
            );
        }
        self.push("<img src=\"");
        self.push_url(&image.destination, "", image.span, RefKind::Image);
        let mut html = String::from("\" alt=\"");
        escape_html_into(&mut html, &image.alt);
        html.push('"');
        if !image.title.is_empty() {
            html.push_str(" title=\"");
            escape_html_into(&mut html, &image.title);
            html.push('"');
        }
        html.push('>');
        if let Some(summary) = &mut self.summary_capture {
            summary.push_str(&image.alt);
        }
        self.push(&html);
    }

    fn footnote_reference(&mut self, label: &str) {
        let index = match self.footnote_order.iter().position(|known| known == label) {
            Some(index) => index,
            None => {
                self.footnote_order.push(label.to_owned());
                self.footnote_references.push(0);
                self.footnote_order.len() - 1
            }
        };
        self.footnote_references[index] += 1;
        let slug = slugify(label);
        let occurrence = self.footnote_references[index];
        let reference_id = if occurrence == 1 {
            format!("fnref-{slug}")
        } else {
            format!("fnref-{slug}-{occurrence}")
        };
        let number = index + 1;
        self.push(&format!(
            "<sup class=\"rd-footnote-ref\"><a href=\"#fn-{slug}\" id=\"{reference_id}\">{number}</a></sup>"
        ));
        self.out.anchors.push(reference_id);
    }

    fn fenced_block(&mut self, code: CodeCapture) {
        let info = match &code.kind {
            CodeBlockKind::Fenced(info) => info.to_string(),
            CodeBlockKind::Indented => String::new(),
        };
        let fence = match parse_info(&info) {
            Ok(fence) => fence,
            Err(message) => {
                self.out.issues.push(
                    Issue::error(code.span.clone(), "code-fence-info", message).suggest(
                        "write the language first, then title=\"…\", linenos and highlight=\"1,3-5\"",
                    ),
                );
                FenceInfo::default()
            }
        };
        self.code_block(BlockSpec {
            text: code.text,
            tag: fence.tag,
            title: fence.title,
            linenos: fence.linenos,
            highlight: fence.highlight,
            span: code.span,
        });
    }

    fn include(&mut self, directive: Directive, span: Range<usize>) {
        let path = match crate::config::relative_path(&directive.path) {
            Ok(path) => path,
            Err(message) => {
                self.out
                    .issues
                    .push(Issue::error(span, "include-path", message));
                return;
            }
        };
        if !self.out.includes.contains(&path) {
            self.out.includes.push(path.clone());
        }
        let source = match self.context.includes.0.get(&path) {
            Some(Ok(source)) => source.clone(),
            Some(Err(message)) => {
                self.out.issues.push(
                    Issue::error(span, "include-missing", format!("`{path}`: {message}"))
                        .suggest("the path is relative to the project root"),
                );
                return;
            }
            None => {
                self.out.issues.push(Issue::error(
                    span,
                    "include-missing",
                    format!("`{path}` was not read for this page"),
                ));
                return;
            }
        };
        let text = match &directive.region {
            Some(region) => match include::extract(&source, region) {
                Ok(text) => text,
                Err(message) => {
                    self.out.issues.push(
                        Issue::error(span, "include-region", format!("`{path}`: {message}")).suggest(format!(
                            "mark the lines with `rustify-region:{region}` and `rustify-endregion:{region}` comments"
                        )),
                    );
                    return;
                }
            },
            None => source,
        };
        let tag = directive.lang.or_else(|| {
            path.rsplit('.')
                .next()
                .filter(|ext| crate::highlight::language(ext).is_some())
                .map(str::to_owned)
        });
        self.code_block(BlockSpec {
            text,
            tag,
            title: Some(directive.title.unwrap_or(path)),
            linenos: false,
            highlight: Vec::new(),
            span,
        });
    }

    fn code_block(&mut self, block: BlockSpec) {
        let highlighted = self
            .context
            .highlighter
            .highlight(&block.text, block.tag.as_deref());
        let line_count = highlighted.lines.len();
        for range in &block.highlight {
            if *range.end() > line_count {
                self.out.issues.push(Issue::error(
                    block.span.clone(),
                    "code-highlight-range",
                    format!(
                        "highlight=\"{}-{}\" is outside the block's {line_count} lines",
                        range.start(),
                        range.end()
                    ),
                ));
            }
        }
        let lang_attr = highlighted
            .language
            .map(|language| language.id.to_owned())
            .or_else(|| block.tag.clone())
            .unwrap_or_default();
        let label = match (&block.title, highlighted.language) {
            (Some(title), _) => title.clone(),
            (None, Some(language)) => language.label.to_owned(),
            (None, None) => block
                .tag
                .clone()
                .unwrap_or_else(|| self.context.labels.plain_text.to_owned()),
        };
        let tab = self.group.as_mut().map(|group| {
            group.labels.push(label.clone());
            group.labels.len() - 1
        });
        let mut html = String::with_capacity(block.text.len() * 3 + 256);
        html.push_str("<figure class=\"rd-code\"");
        if !lang_attr.is_empty() {
            html.push_str(" data-lang=\"");
            escape_html_into(&mut html, &lang_attr);
            html.push('"');
        }
        if block.linenos {
            html.push_str(" data-linenos");
        }
        if let Some(tab) = tab {
            html.push_str(&format!(" data-rd-tab-panel=\"{tab}\""));
        }
        html.push_str(">\n<figcaption class=\"rd-code-bar\"><span class=\"rd-code-title\">");
        escape_html_into(&mut html, &label);
        html.push_str("</span><button type=\"button\" class=\"rd-copy\" data-rd-copy hidden");
        html.push_str(self.ui_lang());
        html.push('>');
        html.push_str(&crate::icons::copy());
        html.push_str("<span class=\"rd-copy-label\">");
        html.push_str(self.context.labels.copy);
        html.push_str(
            "</span></button></figcaption>\n<pre class=\"rd-code-pre\" tabindex=\"0\"><code>",
        );
        for (index, line) in highlighted.lines.iter().enumerate() {
            let number = index + 1;
            if block.highlight.iter().any(|range| range.contains(&number)) {
                html.push_str("<span class=\"rd-line rd-line-hl\">");
            } else {
                html.push_str("<span class=\"rd-line\">");
            }
            html.push_str(line);
            html.push_str("</span>\n");
        }
        html.push_str("</code></pre>\n<template class=\"rd-code-raw\">");
        escape_html_into(&mut html, &block.text);
        html.push_str("</template>\n</figure>\n");
        self.push(&html);
        self.out.code_blocks.push(CodeBlock {
            tag: block.tag,
            text: block.text,
            span: block.span,
        });
    }

    fn finish(mut self) -> Fragment {
        if let Some(group) = self.group.take() {
            self.out.issues.push(
                Issue::error(
                    group.span.clone(),
                    "code-group",
                    format!("code group `{}` is never closed", group.name),
                )
                .suggest("end it with <!-- rustify:end-code-group -->"),
            );
            let end = group.span.clone();
            self.close_group(group, end);
        }
        // Unbalanced input still yields one buffer per open capture; fold
        // whatever is left back into the document rather than drop it.
        while self.buffers.len() > 1 {
            let inner = self.buffers.pop().unwrap_or_default();
            self.buf().append(inner);
        }
        if !self.footnote_order.is_empty() {
            let lang = self.ui_lang();
            let labels = self.context.labels;
            self.push(&format!(
                "<section class=\"rd-footnotes\"><h2 class=\"sr-only\"{lang}>{}</h2>\n<ol>\n",
                labels.footnotes
            ));
            let order = std::mem::take(&mut self.footnote_order);
            let mut footnotes = std::mem::take(&mut self.footnotes);
            for label in &order {
                let slug = slugify(label);
                self.push(&format!("<li id=\"fn-{slug}\">"));
                self.out.anchors.push(format!("fn-{slug}"));
                if let Some(index) = footnotes.iter().position(|(defined, _)| defined == label) {
                    let (_, mut body) = footnotes.swap_remove(index);
                    let trimmed = body.html.trim_end().len();
                    body.html.truncate(trimmed);
                    self.buf().append(body);
                }
                self.push(&format!(
                    " <a href=\"#fnref-{slug}\" class=\"rd-footnote-back\" aria-label=\"{}\"{lang}>{}</a></li>\n",
                    labels.back_to_reference,
                    crate::icons::back()
                ));
            }
            self.push("</ol>\n</section>\n");
        }
        self.out.summary = self.summary;
        let buffer = self.buffers.pop().unwrap_or_default();
        self.out.html = buffer.html;
        self.out.slots = buffer.slots;
        self.out
    }
}

enum GroupMarker {
    Open(Result<String, String>),
    Close,
}

/// `<!-- rustify:code-group name="…" -->` and `<!-- rustify:end-code-group -->`.
fn group_marker(html: &str) -> Option<GroupMarker> {
    let inner = html.strip_prefix("<!--")?.strip_suffix("-->")?.trim();
    if inner == "rustify:end-code-group" {
        return Some(GroupMarker::Close);
    }
    let rest = inner.strip_prefix("rustify:code-group")?;
    if !rest.is_empty() && !rest.starts_with(char::is_whitespace) {
        return None;
    }
    Some(GroupMarker::Open(include::parse_attributes(rest).and_then(
        |attributes| match attributes.as_slice() {
            [(name, value)] if name == "name" && !value.trim().is_empty() => Ok(value.clone()),
            _ => Err("a code group takes exactly name=\"…\"".into()),
        },
    )))
}

fn heading_level(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// A destination that leaves the site.
pub fn is_external(destination: &str) -> bool {
    let lower = destination.trim_start().to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("//")
}

/// A code fence's info string: the language, then `title="…"`, `linenos`
/// and `highlight="1,3-5"` in any order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FenceInfo {
    pub tag: Option<String>,
    pub title: Option<String>,
    pub linenos: bool,
    pub highlight: Vec<std::ops::RangeInclusive<usize>>,
}

pub fn parse_info(info: &str) -> Result<FenceInfo, String> {
    let mut fence = FenceInfo::default();
    let mut rest = info.trim();
    let mut first = true;
    while !rest.is_empty() {
        let name_end = rest
            .find(|c: char| c == '=' || c.is_whitespace())
            .unwrap_or(rest.len());
        let name = &rest[..name_end];
        rest = &rest[name_end..];
        let value = if let Some(after) = rest.strip_prefix('=') {
            let after = after
                .strip_prefix('"')
                .ok_or_else(|| format!("the value of `{name}` must be in double quotes"))?;
            let end = after
                .find('"')
                .ok_or_else(|| format!("the value of `{name}` has no closing quote"))?;
            rest = &after[end + 1..];
            Some(&after[..end])
        } else {
            None
        };
        rest = rest.trim_start();
        match (name, value) {
            (tag, None) if first && tag != "linenos" => fence.tag = Some(tag.to_owned()),
            ("linenos", None) => fence.linenos = true,
            ("title", Some(title)) => fence.title = Some(title.to_owned()),
            ("highlight", Some(lines)) => fence.highlight = parse_ranges(lines)?,
            (name, _) => return Err(format!("`{name}` is not a code fence attribute")),
        }
        first = false;
    }
    Ok(fence)
}

fn parse_ranges(lines: &str) -> Result<Vec<std::ops::RangeInclusive<usize>>, String> {
    let invalid = || {
        format!("highlight=\"{lines}\" is not a list of line numbers and ranges such as \"1,3-5\"")
    };
    lines
        .split(',')
        .map(|part| {
            let part = part.trim();
            let (start, end) = part.split_once('-').unwrap_or((part, part));
            let start: usize = start.trim().parse().map_err(|_| invalid())?;
            let end: usize = end.trim().parse().map_err(|_| invalid())?;
            if start == 0 || end < start {
                return Err(invalid());
            }
            Ok(start..=end)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostic::Severity;
    use crate::labels;

    fn render_with(
        source: &str,
        labels: &Labels,
        foreign: bool,
        includes: &IncludeFiles,
    ) -> Fragment {
        let highlighter = Highlighter::new();
        let context = Context {
            highlighter: &highlighter,
            labels,
            foreign,
            includes,
        };
        render(source, 0, &context)
    }

    fn md(source: &str) -> Fragment {
        render_with(source, &labels::EN, false, &IncludeFiles::default())
    }

    fn rules(fragment: &Fragment) -> Vec<&'static str> {
        fragment.issues.iter().map(|issue| issue.rule).collect()
    }

    /// The HTML with every slot replaced by `[n]`, its reference index.
    fn with_slots(fragment: &Fragment) -> String {
        let mut out = String::new();
        let mut at = 0;
        let mut slots = fragment.slots.clone();
        slots.sort_by_key(|slot| slot.html.start);
        for slot in slots {
            out.push_str(&fragment.html[at..slot.html.start]);
            out.push_str(&format!("[{}]", slot.reference));
            at = slot.html.end;
        }
        out.push_str(&fragment.html[at..]);
        out
    }

    #[test]
    fn headings_get_github_ids_and_explicit_ids_win() {
        let page =
            md("# Title\n\n## Setup {#install}\n\n## Install\n\n## Install\n\n### `code` & more\n");
        let ids: Vec<&str> = page.headings.iter().map(|h| h.id.as_str()).collect();
        assert_eq!(
            ids,
            ["title", "install", "install-1", "install-2", "code--more"]
        );
        assert_eq!(page.title.as_deref(), Some("Title"));
        assert_eq!(page.h1_count, 1);
        assert!(page.html.contains("<h2 id=\"install\">Setup</h2>"));
        assert!(page
            .html
            .contains("<h3 id=\"code--more\"><code>code</code> &amp; more</h3>"));
        assert!(page.headings[1].explicit && !page.headings[2].explicit);
    }

    #[test]
    fn heading_classes_and_duplicate_explicit_ids_are_errors() {
        let source = "## A {.wide}\n\n## B {#x}\n\n## C {#x}\n";
        let page = md(source);
        assert_eq!(rules(&page), ["duplicate-heading-id", "heading-attribute"]);
        assert_eq!(&source[page.issues[1].span.clone()], "## A {.wide}\n");
    }

    #[test]
    fn tables_align_with_classes_not_styles() {
        let page = md("| a | b | c |\n|:--|:-:|--:|\n| 1 | 2 | 3 |\n");
        assert!(!page.html.contains("style"));
        assert!(page.html.contains("<th class=\"text-left\">a</th><th class=\"text-center\">b</th><th class=\"text-right\">c</th>"));
        assert!(page.html.contains("<td class=\"text-left\">1</td><td class=\"text-center\">2</td><td class=\"text-right\">3</td>"));
    }

    #[test]
    fn the_five_alerts() {
        for (marker, name, title) in [
            ("NOTE", "note", "Note"),
            ("TIP", "tip", "Tip"),
            ("IMPORTANT", "important", "Important"),
            ("WARNING", "warning", "Warning"),
            ("CAUTION", "caution", "Caution"),
        ] {
            let page = md(&format!("> [!{marker}]\n> Body.\n"));
            assert_eq!(
                page.html,
                format!(
                    "<div class=\"markdown-alert markdown-alert-{name}\" role=\"note\">\n<p class=\"markdown-alert-title\">{}{title}</p>\n<p>Body.</p>\n</div>\n",
                    crate::icons::alert(name)
                )
            );
        }
        assert_eq!(
            md("> plain\n").html,
            "<blockquote>\n<p>plain</p>\n</blockquote>\n"
        );
    }

    #[test]
    fn interface_strings_carry_english_inside_foreign_content() {
        let page = render_with(
            "> [!TIP]\n> Astuce.\n\n```rust\nfn main() {}\n```\n",
            &labels::EN,
            true,
            &IncludeFiles::default(),
        );
        assert!(page.html.contains(&format!(
            "<p class=\"markdown-alert-title\" lang=\"en\">{}Tip</p>",
            crate::icons::alert("tip")
        )));
        assert!(page.html.contains("data-rd-copy hidden lang=\"en\"><svg"));
        assert!(page
            .html
            .contains("<span class=\"rd-copy-label\">Copy</span></button>"));
        let zh = render_with(
            "> [!TIP]\n> 提示内容\n",
            &labels::ZH,
            false,
            &IncludeFiles::default(),
        );
        assert!(zh.html.contains(&format!(
            "<p class=\"markdown-alert-title\">{}提示</p>",
            crate::icons::alert("tip")
        )));
    }

    #[test]
    fn code_blocks_keep_raw_text_and_mark_lines() {
        let page = md("```rust title=\"main.rs\" linenos highlight=\"2\"\nfn main() {\n    let s = \"<&>\";\n}\n```\n");
        assert_eq!(
            page.code_blocks[0].text,
            "fn main() {\n    let s = \"<&>\";\n}\n"
        );
        let html = &page.html;
        assert!(html.starts_with("<figure class=\"rd-code\" data-lang=\"rust\" data-linenos>\n<figcaption class=\"rd-code-bar\"><span class=\"rd-code-title\">main.rs</span>"));
        assert_eq!(html.matches("<span class=\"rd-line\">").count(), 2);
        assert_eq!(
            html.matches("<span class=\"rd-line rd-line-hl\">").count(),
            1
        );
        assert!(html.contains("<template class=\"rd-code-raw\">fn main() {\n    let s = &quot;&lt;&amp;&gt;&quot;;\n}\n</template>"));
        assert!(!html.contains("style="));
        assert!(page.issues.is_empty(), "{:?}", page.issues);
    }

    #[test]
    fn fence_info_errors_are_located() {
        let page =
            md("text\n\n```rust highlight=\"9\"\nx\n```\n\n```rust colour=\"red\"\ny\n```\n");
        assert_eq!(rules(&page), ["code-highlight-range", "code-fence-info"]);
        assert_eq!(page.issues[0].span.start, 6);
        assert_eq!(
            parse_info("ts linenos").unwrap(),
            FenceInfo {
                tag: Some("ts".into()),
                linenos: true,
                ..Default::default()
            }
        );
        assert_eq!(
            parse_info("rust highlight=\"1,3-5\"").unwrap().highlight,
            vec![1..=1, 3..=5]
        );
        assert!(parse_info("rust highlight=\"3-1\"").is_err());
        assert!(parse_info("rust title=main").is_err());
    }

    #[test]
    fn links_and_images_become_slots() {
        let page = md("# T\n\nSee [the guide](guide.md#setup \"Guide\"), [site](https://example.com) and <me@example.com>.\n\n![A *cat*](cat.png)\n");
        assert_eq!(
            page.summary.as_deref(),
            Some("See the guide, site and me@example.com.")
        );
        assert_eq!(
            with_slots(&page),
            "<h1 id=\"t\">T</h1>\n<p>See <a href=\"[0]\" title=\"Guide\">the guide</a>, <a href=\"[1]\" rel=\"noopener\">site</a> and <a href=\"[2]\">me@example.com</a>.</p>\n<p><img src=\"[3]\" alt=\"A cat\"></p>\n"
        );
        let kinds: Vec<(&str, RefKind)> = page
            .references
            .iter()
            .map(|r| (r.destination.as_str(), r.kind))
            .collect();
        assert_eq!(
            kinds,
            [
                ("guide.md#setup", RefKind::Link),
                ("https://example.com", RefKind::Link),
                ("me@example.com", RefKind::Link),
                ("cat.png", RefKind::Image)
            ]
        );
        // The slot holds the written URL until a generation fills it.
        assert_eq!(
            &page.html[page.slots[2].html.clone()],
            "mailto:me@example.com"
        );
    }

    #[test]
    fn bare_urls_link_like_on_github_but_not_inside_links_or_code() {
        let page = md("Go to www.example.com/a_b, https://x.example/p?q=1. or `https://not.linked` and [https://kept.example](other.md).\n");
        assert!(
            page.html.contains(
                "<a href=\"http://www.example.com/a_b\" rel=\"noopener\">www.example.com/a_b</a>"
            ),
            "{}",
            page.html
        );
        assert!(page.html.contains(
            "<a href=\"https://x.example/p?q=1\" rel=\"noopener\">https://x.example/p?q=1</a>."
        ));
        assert!(page.html.contains("<code>https://not.linked</code>"));
        assert_eq!(page.html.matches("<a ").count(), 3);
        assert_eq!(
            page.autolinks,
            ["http://www.example.com/a_b", "https://x.example/p?q=1"]
        );
    }

    #[test]
    fn raw_html_is_checked_and_its_urls_become_slots() {
        let source = "Text <a href=\"other.md#x\">o</a>.\n\n<div><img src=\"pic.png\" alt=\"p\"></div>\n\n<script>alert(1)</script>\n\n<p onclick=\"x()\">c</p>\n";
        let page = md(source);
        assert_eq!(rules(&page), ["inline-script", "event-handler"]);
        assert_eq!(&source[page.issues[0].span.clone()], "<script>");
        let destinations: Vec<(&str, RefKind)> = page
            .references
            .iter()
            .map(|r| (r.destination.as_str(), r.kind))
            .collect();
        assert_eq!(
            destinations,
            [("other.md#x", RefKind::Link), ("pic.png", RefKind::Image)]
        );
        assert!(with_slots(&page).contains("<a href=\"[0]\">o</a>"));
        assert!(with_slots(&page).contains("<img src=\"[1]\" alt=\"p\">"));
    }

    #[test]
    fn missing_alt_is_a_warning() {
        let page = md("![](a.png)\n\n<img src=\"b.png\">\n");
        let warnings: Vec<(&str, Severity)> =
            page.issues.iter().map(|i| (i.rule, i.severity)).collect();
        assert_eq!(
            warnings,
            [
                ("missing-alt", Severity::Warning),
                ("missing-alt", Severity::Warning)
            ]
        );
    }

    #[test]
    fn code_groups_wrap_their_blocks_and_keep_them_readable_without_script() {
        let source = "<!-- rustify:code-group name=\"install\" -->\n\n```sh title=\"Cargo\"\ncargo add x\n```\n\n```toml title=\"Manifest\"\nx = \"1\"\n```\n\n<!-- rustify:end-code-group -->\n";
        let page = md(source);
        assert!(page.issues.is_empty(), "{:?}", page.issues);
        let html = &page.html;
        assert!(html.starts_with("<div class=\"rd-code-group\" data-rd-group=\"install\">\n<div class=\"rd-code-tabs\" hidden><div class=\"rd-code-tablist\" role=\"tablist\"><button type=\"button\" role=\"tab\" aria-selected=\"true\" data-rd-tab=\"0\">Cargo</button><button type=\"button\" role=\"tab\" aria-selected=\"false\" data-rd-tab=\"1\">Manifest</button></div></div>\n<figure class=\"rd-code\" data-lang=\"sh\" data-rd-tab-panel=\"0\">"), "{html}");
        assert!(html.contains("data-rd-tab-panel=\"1\""));
        assert!(html.trim_end().ends_with("</figure>\n</div>"));
        assert!(!html.contains("rustify:"));
        assert_eq!(
            page.code_groups,
            vec![CodeGroup {
                name: "install".into(),
                labels: vec!["Cargo".into(), "Manifest".into()],
                span: 0..source.len()
            }]
        );
    }

    #[test]
    fn code_group_mistakes_are_located() {
        let page = md("<!-- rustify:code-group name=\"g\" -->\n\nNot code.\n\n```sh\na\n```\n\n```sh\nb\n```\n\n<!-- rustify:end-code-group -->\n");
        assert_eq!(
            rules(&page),
            ["code-group", "code-group"],
            "{:?}",
            page.issues
        );
        assert_eq!(
            rules(&md("<!-- rustify:end-code-group -->\n")),
            ["code-group"]
        );
        assert_eq!(
            rules(&md(
                "<!-- rustify:code-group name=\"g\" -->\n\n```sh\na\n```\n"
            )),
            ["code-group"]
        );
        assert_eq!(
            rules(&md("<!-- rustify:code-group label=\"g\" -->\n")),
            ["code-group"]
        );
    }

    #[test]
    fn includes_render_regions_as_code_and_report_what_is_missing() {
        let mut files = IncludeFiles::default();
        files.0.insert(
            "src/demo.rs".into(),
            Ok(
                "fn a() {}\n// rustify-region:main\nfn main() {}\n// rustify-endregion:main\n"
                    .into(),
            ),
        );
        files
            .0
            .insert("src/gone.rs".into(), Err("the file does not exist".into()));
        let source = "<!-- rustify:include path=\"src/demo.rs\" region=\"main\" -->\n\n<!-- rustify:include path=\"src/demo.rs\" region=\"nope\" -->\n\n<!-- rustify:include path=\"src/gone.rs\" -->\n";
        let page = render_with(source, &labels::EN, false, &files);
        assert_eq!(rules(&page), ["include-region", "include-missing"]);
        assert!(page.issues[0].message.contains("there is no region `nope`"));
        assert_eq!(page.code_blocks[0].text, "fn main() {}\n");
        assert_eq!(page.code_blocks[0].tag.as_deref(), Some("rs"));
        assert!(page
            .html
            .contains("<span class=\"rd-code-title\">src/demo.rs</span>"));
        assert_eq!(page.includes, ["src/demo.rs", "src/gone.rs"]);
        assert_eq!(include_paths(source), ["src/demo.rs", "src/gone.rs"]);
    }

    #[test]
    fn footnotes_tasks_strikethrough() {
        let page = md("A[^n] and B[^n].\n\n- [x] done\n- [ ] todo\n\n~~old~~\n\n[^n]: The note.\n");
        assert!(page.html.contains(
            "<sup class=\"rd-footnote-ref\"><a href=\"#fn-n\" id=\"fnref-n\">1</a></sup>"
        ));
        assert!(page.html.contains("id=\"fnref-n-2\">1</a>"));
        assert!(page.html.contains(
            "<li id=\"fn-n\"><p>The note.</p> <a href=\"#fnref-n\" class=\"rd-footnote-back\""
        ));
        assert!(page.html.contains(
            "<li><input type=\"checkbox\" disabled checked aria-label=\"Task\"> done</li>"
        ));
        assert!(page.html.contains("<del>old</del>"));
    }

    #[test]
    fn slots_survive_headings_and_footnotes() {
        let page = md("## See [a](a.md)\n\nText[^f].\n\n[^f]: Note with [b](b.md).\n");
        let slotted = with_slots(&page);
        assert!(
            slotted.contains("<h2 id=\"see-a\">See <a href=\"[0]\">a</a></h2>"),
            "{slotted}"
        );
        assert!(
            slotted.contains("<p>Note with <a href=\"[1]\">b</a>.</p>"),
            "{slotted}"
        );
    }

    #[test]
    fn offsets_are_file_offsets() {
        let highlighter = Highlighter::new();
        let includes = IncludeFiles::default();
        let context = Context {
            highlighter: &highlighter,
            labels: &labels::EN,
            foreign: false,
            includes: &includes,
        };
        let page = render("## A\n", 10, &context);
        assert_eq!(page.headings[0].span, 10..15);
    }
}
