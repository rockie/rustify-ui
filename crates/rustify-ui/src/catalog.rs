//! What each component category actually supports, in one table.
//!
//! R18 asks a catalogue for eighteen categories with no blank support cell.
//! P1 ships nine of them; the other nine are in the table as well, saying
//! plainly that they are not here.
//!
//! Every entry answers the same six questions (properties, actions, theme,
//! input, accessibility, environment), so a reader never has to guess whether
//! a silence means "yes" or "nobody looked".

use std::fmt::Write as _;

/// The eighteen categories R18 names. The catalogue covers all of them; it
/// does not implement all of them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Button,
    Label,
    Link,
    Icon,
    TextField,
    TextArea,
    Checkbox,
    Radio,
    Switch,
    Select,
    Slider,
    Progress,
    Loading,
    Tooltip,
    Menu,
    Dialog,
    Tabs,
    ScrollArea,
}

impl Category {
    /// The name the catalogue prints, and the one the plan uses.
    pub fn name(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Label => "label",
            Self::Link => "link",
            Self::Icon => "icon",
            Self::TextField => "text field",
            Self::TextArea => "text area",
            Self::Checkbox => "checkbox",
            Self::Radio => "radio",
            Self::Switch => "switch",
            Self::Select => "select",
            Self::Slider => "slider",
            Self::Progress => "progress",
            Self::Loading => "loading",
            Self::Tooltip => "tooltip",
            Self::Menu => "menu",
            Self::Dialog => "dialog",
            Self::Tabs => "tabs",
            Self::ScrollArea => "scroll area",
        }
    }
}

/// How far one capability goes.
///
/// There is deliberately no `Unknown`: a question nobody answered is a gap in
/// the catalogue, not a state a component can be in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Support {
    /// Implemented and covered by a test in this release.
    Yes,
    /// Implemented, with a stated limit.
    Partial,
    /// Not in this release.
    No,
}

impl Support {
    pub fn name(self) -> &'static str {
        match self {
            Self::Yes => "yes",
            Self::Partial => "partial",
            Self::No => "no",
        }
    }
}

/// One answer, and the reason for it. The reason is never empty - a bare
/// "partial" tells a reader nothing they can act on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capability {
    pub support: Support,
    pub note: &'static str,
}

const fn cap(support: Support, note: &'static str) -> Capability {
    Capability { support, note }
}

/// Where a category is drawn, and whether one value can drive both halves.
///
/// `across_regions` is the interesting column: it says the component keeps one
/// value and one action path across the DOM/GPU boundary, either because both
/// halves render it or because it can be anchored to a rectangle inside a GPU
/// region.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Presentation {
    pub dom: Support,
    pub gpu: Support,
    pub across_regions: Support,
}

/// One row of the catalogue.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entry {
    pub category: Category,
    pub presentation: Presentation,
    /// Which values the component takes and what it does with them.
    pub properties: Capability,
    /// What the user can make it do, and what the application hears.
    pub actions: Capability,
    /// Whether it follows the scope's theme tokens.
    pub theme: Capability,
    /// Pointer, keyboard and text input.
    pub input: Capability,
    /// Name, role, value, state and a keyboard path to the same result.
    pub accessibility: Capability,
    /// Where it has been run: browser, strict CSP, and what it needs.
    pub environment: Capability,
}

impl Entry {
    /// The six classes in the order the catalogue prints them.
    pub fn capabilities(&self) -> [(&'static str, Capability); 6] {
        [
            ("properties", self.properties),
            ("actions", self.actions),
            ("theme", self.theme),
            ("input", self.input),
            ("accessibility", self.accessibility),
            ("environment", self.environment),
        ]
    }

    /// Whether this release ships anything for the category at all.
    pub fn shipped(&self) -> bool {
        self.presentation.dom != Support::No || self.presentation.gpu != Support::No
    }
}

/// Why the nine categories P1 does not ship are absent. One sentence, used by
/// every one of them, so the reason cannot drift between rows.
const LATER: &str = "not in P1; the eighteen-category catalogue is P2 M2";

const fn absent(category: Category) -> Entry {
    Entry {
        category,
        presentation: Presentation {
            dom: Support::No,
            gpu: Support::No,
            across_regions: Support::No,
        },
        properties: cap(Support::No, LATER),
        actions: cap(Support::No, LATER),
        theme: cap(Support::No, LATER),
        input: cap(Support::No, LATER),
        accessibility: cap(Support::No, LATER),
        environment: cap(Support::No, LATER),
    }
}

/// The catalogue. Nine categories this release ships, nine it does not.
pub const CATALOG: [Entry; 18] = [
    Entry {
        category: Category::Button,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "label, disabled; a disabled button emits nothing"),
        actions: cap(
            Support::Yes,
            "click; the DOM and the GPU button call the same application action",
        ),
        theme: cap(Support::Yes, "scope tokens: primary, foreground, radius"),
        input: cap(Support::Yes, "pointer and Enter/Space, from either half"),
        accessibility: cap(
            Support::Yes,
            "native button in the DOM; the GPU button has a DOM control with the same name and action",
        ),
        environment: cap(
            Support::Yes,
            "macOS Chrome, strict CSP, WebGL2 region; no inline script or style",
        ),
    },
    Entry {
        category: Category::Label,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "text; optional association with a control"),
        actions: cap(Support::No, "a label has none; clicking one focuses its control"),
        theme: cap(Support::Yes, "scope tokens: foreground, muted"),
        input: cap(Support::Yes, "selectable text in the DOM"),
        accessibility: cap(
            Support::Yes,
            "names the control it wraps; GPU text is decoration and stays out of the tree",
        ),
        environment: cap(Support::Yes, "macOS Chrome, strict CSP; GPU text needs the region's font"),
    },
    absent(Category::Link),
    absent(Category::Icon),
    Entry {
        category: Category::TextField,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Partial,
            across_regions: Support::Yes,
        },
        properties: cap(
            Support::Yes,
            "value, disabled, read-only; a refused value is shown as the application's, not the keystrokes'",
        ),
        actions: cap(Support::Yes, "input and commit; the application owns the value"),
        theme: cap(Support::Yes, "scope tokens: input, border, ring, radius"),
        input: cap(
            Support::Yes,
            "the browser's own control, so IME composition is the platform's",
        ),
        accessibility: cap(Support::Yes, "native input with a label; five keyboard journeys"),
        environment: cap(
            Support::Partial,
            "the GPU half displays the value and hands editing to a native control over its rectangle; it is not a GPU text editor",
        ),
    },
    Entry {
        category: Category::TextArea,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Partial,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "value, rows, disabled, read-only"),
        actions: cap(
            Support::Yes,
            "input and commit on leaving; Enter is a newline, not a commit",
        ),
        theme: cap(Support::Yes, "scope tokens: input, border, ring, radius"),
        input: cap(Support::Yes, "the browser's own control; several lines"),
        accessibility: cap(Support::Yes, "native textarea with a label"),
        environment: cap(
            Support::Partial,
            "the GPU half shows the first line only; editing is the native control's",
        ),
    },
    Entry {
        category: Category::Checkbox,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(
            Support::Yes,
            "checked, disabled, read-only; read-only reverts the browser's own toggle",
        ),
        actions: cap(Support::Yes, "change; both halves call one application action"),
        theme: cap(Support::Yes, "scope tokens: primary, border, ring"),
        input: cap(Support::Yes, "pointer and Space, from either half"),
        accessibility: cap(
            Support::Yes,
            "native checkbox in the DOM; the GPU one has a DOM control with the same name, state and action",
        ),
        environment: cap(Support::Yes, "macOS Chrome, strict CSP, WebGL2 region"),
    },
    absent(Category::Radio),
    absent(Category::Switch),
    absent(Category::Select),
    Entry {
        category: Category::Slider,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(
            Support::Yes,
            "value, min, max, step, disabled, read-only; the value is clamped and snapped before the application sees it",
        ),
        actions: cap(Support::Yes, "change; both halves call one application action"),
        theme: cap(Support::Yes, "scope tokens: primary, border, ring"),
        input: cap(Support::Yes, "pointer drag and arrow keys, from either half"),
        accessibility: cap(
            Support::Yes,
            "native range in the DOM; the GPU one has a DOM control with the same name and value",
        ),
        environment: cap(Support::Yes, "macOS Chrome, strict CSP, WebGL2 region"),
    },
    absent(Category::Progress),
    Entry {
        category: Category::Loading,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Yes,
        },
        properties: cap(
            Support::Yes,
            "the four states of `Load`: loading, empty, ready, error; empty is an answer, not a missing one",
        ),
        actions: cap(Support::Yes, "one application-defined retry on the error state"),
        theme: cap(Support::Yes, "scope tokens: muted, destructive"),
        input: cap(Support::Yes, "the retry control is an ordinary button"),
        accessibility: cap(
            Support::Yes,
            "the state is a live status; the error is an alert with the retry beside it",
        ),
        environment: cap(
            Support::Yes,
            "a region can be covered by one, but the view itself is DOM",
        ),
    },
    absent(Category::Tooltip),
    Entry {
        category: Category::Menu,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "anchor, items, open state owned by the application"),
        actions: cap(Support::Yes, "item activation and close; Escape closes the top layer only"),
        theme: cap(Support::Yes, "scope tokens: background, border, radius"),
        input: cap(Support::Yes, "pointer and keyboard; the layer owns Escape while it is open"),
        accessibility: cap(
            Support::Yes,
            "menu/menuitem roles; focus returns to the control it was opened from",
        ),
        environment: cap(
            Support::Partial,
            "anchors to a rectangle inside a GPU region within one CSS pixel, but does not flip or clamp to the viewport",
        ),
    },
    Entry {
        category: Category::Dialog,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "modal, anchor, open state owned by the application"),
        actions: cap(Support::Yes, "close; Escape closes the top layer only"),
        theme: cap(Support::Yes, "scope tokens: background, border, radius"),
        input: cap(Support::Yes, "clicks do not reach what a modal covers, region included"),
        accessibility: cap(
            Support::Yes,
            "the scope's own content is inert while a modal is open; focus returns on close",
        ),
        environment: cap(
            Support::Partial,
            "only this scope is made inert; a host page outside it is not",
        ),
    },
    absent(Category::Tabs),
    absent(Category::ScrollArea),
];

/// The row for one category.
pub fn entry(category: Category) -> &'static Entry {
    CATALOG
        .iter()
        .find(|entry| entry.category == category)
        .expect("the catalogue covers every category")
}

/// The catalogue as the capability document prints it.
pub fn markdown() -> String {
    let mut out = String::new();
    out.push_str("| category | DOM | GPU | across regions | properties | actions | theme | input | accessibility | environment |\n");
    out.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for entry in &CATALOG {
        let _ = write!(
            out,
            "| {} | {} | {} | {} |",
            entry.category.name(),
            entry.presentation.dom.name(),
            entry.presentation.gpu.name(),
            entry.presentation.across_regions.name(),
        );
        for (_, capability) in entry.capabilities() {
            let _ = write!(
                out,
                " {} — {} |",
                capability.support.name(),
                capability.note
            );
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Category; 18] = [
        Category::Button,
        Category::Label,
        Category::Link,
        Category::Icon,
        Category::TextField,
        Category::TextArea,
        Category::Checkbox,
        Category::Radio,
        Category::Switch,
        Category::Select,
        Category::Slider,
        Category::Progress,
        Category::Loading,
        Category::Tooltip,
        Category::Menu,
        Category::Dialog,
        Category::Tabs,
        Category::ScrollArea,
    ];

    #[test]
    fn every_one_of_the_eighteen_categories_appears_exactly_once() {
        assert_eq!(CATALOG.len(), 18);
        for category in ALL {
            let found = CATALOG
                .iter()
                .filter(|entry| entry.category == category)
                .count();
            assert_eq!(found, 1, "{} appears {found} times", category.name());
        }
    }

    #[test]
    fn no_cell_is_blank_and_every_answer_carries_its_reason() {
        for entry in &CATALOG {
            assert!(
                !entry.category.name().is_empty(),
                "a category with no name is a blank cell"
            );
            for (class, capability) in entry.capabilities() {
                assert!(
                    !capability.note.trim().is_empty(),
                    "{} has a bare {class}",
                    entry.category.name()
                );
            }
        }
    }

    #[test]
    fn the_nine_this_release_does_not_ship_say_so_in_every_column() {
        let absent: Vec<&'static str> = CATALOG
            .iter()
            .filter(|entry| !entry.shipped())
            .map(|entry| entry.category.name())
            .collect();
        assert_eq!(
            absent,
            [
                "link",
                "icon",
                "radio",
                "switch",
                "select",
                "progress",
                "tooltip",
                "tabs",
                "scroll area"
            ]
        );
        for entry in CATALOG.iter().filter(|entry| !entry.shipped()) {
            assert_eq!(entry.presentation.across_regions, Support::No);
            for (class, capability) in entry.capabilities() {
                assert_eq!(
                    capability.support,
                    Support::No,
                    "{} claims {class} without shipping anything",
                    entry.category.name()
                );
                assert_eq!(capability.note, LATER);
            }
        }
    }

    #[test]
    fn the_nine_it_does_ship_are_the_subset_the_plan_named() {
        let shipped: Vec<&'static str> = CATALOG
            .iter()
            .filter(|entry| entry.shipped())
            .map(|entry| entry.category.name())
            .collect();
        assert_eq!(
            shipped,
            [
                "button",
                "label",
                "text field",
                "text area",
                "checkbox",
                "slider",
                "loading",
                "menu",
                "dialog"
            ]
        );
        // A shipped category is reachable from the DOM: that is where the
        // accessible entry for a GPU control lives.
        for entry in CATALOG.iter().filter(|entry| entry.shipped()) {
            assert_ne!(entry.presentation.dom, Support::No);
            assert_ne!(entry.accessibility.support, Support::No);
        }
    }

    #[test]
    fn every_row_is_printed_with_ten_filled_cells() {
        let table = markdown();
        let rows: Vec<&str> = table.lines().skip(2).collect();
        assert_eq!(rows.len(), 18);
        for row in rows {
            let cells: Vec<&str> = row.trim_matches('|').split('|').map(str::trim).collect();
            assert_eq!(cells.len(), 10, "{row}");
            for cell in cells {
                assert!(!cell.is_empty(), "blank cell in {row}");
            }
        }
    }

    /// The capability document is the same table, printed. Run with
    /// `RUSTIFY_UPDATE_DOCS=1` to write it instead of comparing it.
    #[test]
    fn the_capability_document_holds_the_table_the_code_holds() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/components.md");
        let table = markdown();
        let document = std::fs::read_to_string(path).unwrap_or_default();
        if std::env::var_os("RUSTIFY_UPDATE_DOCS").is_some() {
            let (head, tail) = document
                .split_once(MARKER)
                .map(|(head, rest)| {
                    let tail = rest.split_once(MARKER).map(|(_, tail)| tail).unwrap_or("");
                    (head.to_string(), tail.to_string())
                })
                .unwrap_or_default();
            std::fs::write(path, format!("{head}{MARKER}\n{table}{MARKER}{tail}"))
                .expect("cannot write the capability document");
            return;
        }
        assert!(
            document.contains(&table),
            "docs/components.md is out of step with the catalogue; \
             rerun with RUSTIFY_UPDATE_DOCS=1"
        );
    }

    /// Fences the generated table inside the document, so the prose around it
    /// survives a regeneration.
    const MARKER: &str = "<!-- catalogue -->";

    #[test]
    fn a_category_can_be_looked_up() {
        assert_eq!(entry(Category::Slider).category, Category::Slider);
        assert_eq!(entry(Category::Tabs).presentation.dom, Support::No);
    }
}
