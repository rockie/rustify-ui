//! What each component category actually supports, in one table.
//!
//! R18 asks a catalogue for eighteen categories with no blank support cell.
//! All eighteen have a DOM component now; what differs between them is what a
//! GPU region can draw of one, and every row says which and why.
//!
//! Every entry answers the same six questions (properties, actions, theme,
//! input, accessibility, environment), so a reader never has to guess whether
//! a silence means "yes" or "nobody looked".
//!
//! The table lives here, in the crate that has the components, and the
//! capability document is printed from it. Two copies would drift.

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

    /// Whether a GPU region can draw this category at all.
    ///
    /// The interesting split now that all eighteen have a DOM component: six
    /// are drawn by a region as well, four partly, and the rest are DOM layers
    /// anchored to a region rather than something drawn inside one.
    pub fn drawn_by_region(&self) -> bool {
        self.presentation.gpu != Support::No
    }
}

/// The catalogue. Eighteen categories, every cell answered.
pub const CATALOG: [Entry; 18] = [
    Entry {
        category: Category::Button,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "label as children, variant, size, disabled; a disabled button leaves the tab order and calls nothing"),
        actions: cap(Support::Yes, "click; the DOM and the region button call the same application action"),
        theme: cap(Support::Yes, "seven variants and four sizes, all drawn from the scope's tokens"),
        input: cap(Support::Yes, "pointer and Enter/Space, from either half"),
        accessibility: cap(Support::Yes, "a native button; the region's has a DOM control with the same name and action"),
        environment: cap(Support::Yes, "macOS Chrome, strict CSP, WebGL2 region; no inline script or style"),
    },
    Entry {
        category: Category::Label,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "children as the text, and the id of the control it names"),
        actions: cap(Support::No, "a label has none; clicking one moves focus to the control it names"),
        theme: cap(Support::Yes, "foreground, at the size the scope's text token sets"),
        input: cap(Support::Yes, "selectable text in the DOM"),
        accessibility: cap(Support::Yes, "names the control it points at; a region's label is decoration and stays out of the tree"),
        environment: cap(Support::Yes, "macOS Chrome, strict CSP; a region's text needs that region's font"),
    },
    Entry {
        category: Category::Link,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::No,
        },
        properties: cap(Support::Yes, "href, exact or within matching, disabled; a disabled link keeps its place and loses its href"),
        actions: cap(Support::Yes, "follows the link, and marks itself aria-current=page while the path is at or below its own"),
        theme: cap(Support::Yes, "primary and the focus ring"),
        input: cap(Support::Yes, "pointer and Enter, plus everything the browser gives a link"),
        accessibility: cap(Support::Yes, "a native anchor; the current one says so rather than only looking different"),
        environment: cap(Support::Partial, "the current path comes from the application until the SDK router arrives in M4; a region reports a click and the application navigates, because a region never touches history (C-5)"),
    },
    Entry {
        category: Category::Icon,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "one of six glyphs, or any SVG the application passes as children; a label makes it an image and no label makes it decoration"),
        actions: cap(Support::No, "none; it is a drawing"),
        theme: cap(Support::Yes, "it takes the text colour around it, in both halves"),
        input: cap(Support::No, "none; an icon-only control is the button around it"),
        accessibility: cap(Support::Yes, "aria-hidden without a label and role=img with one - a second name after the control's own is noise"),
        environment: cap(Support::Yes, "the same six shapes on the same twenty-four unit grid in both halves; no icon font and no fetched asset"),
    },
    Entry {
        category: Category::TextField,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Partial,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "value, kind, placeholder, disabled, read-only, invalid, described-by; a refused value is replaced by the application's rather than left on screen"),
        actions: cap(Support::Yes, "input; the application owns the value"),
        theme: cap(Support::Yes, "the input surface, its border, the focus ring, and the destructive border when invalid"),
        input: cap(Support::Yes, "the browser's own control, so IME composition is the platform's"),
        accessibility: cap(Support::Yes, "aria-invalid and aria-describedby tie it to its error; a Label names it"),
        environment: cap(Support::Partial, "the region half displays the value and hands editing to a native control over its rectangle; it is not a GPU text editor"),
    },
    Entry {
        category: Category::TextArea,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Partial,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "value, rows, placeholder, disabled, read-only, invalid, described-by"),
        actions: cap(Support::Yes, "input; Enter is a newline and commits nothing"),
        theme: cap(Support::Yes, "the same tokens as the text field"),
        input: cap(Support::Yes, "the browser's own control; several lines"),
        accessibility: cap(Support::Yes, "the same error wiring as the text field"),
        environment: cap(Support::Partial, "the region half shows the first line only; editing is the native control's"),
    },
    Entry {
        category: Category::Checkbox,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "checked, disabled, read-only, invalid; read-only takes the click and puts the box back"),
        actions: cap(Support::Yes, "change; both halves call one application action"),
        theme: cap(Support::Yes, "the box, its border and the mark, with the focus ring on the box"),
        input: cap(Support::Yes, "pointer and Space, from either half"),
        accessibility: cap(Support::Yes, "the browser's own checkbox under a drawn box, so the state and the keyboard stay the platform's"),
        environment: cap(Support::Yes, "macOS Chrome, strict CSP, WebGL2 region"),
    },
    Entry {
        category: Category::Radio,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "the options, which one is chosen, disabled, read-only; an option can be disabled on its own"),
        actions: cap(Support::Yes, "change carrying the chosen value; a region's radio asks for itself and the application decides what that does to the rest"),
        theme: cap(Support::Yes, "the disc, its ring and the dot"),
        input: cap(Support::Yes, "one tab stop for the group and the arrows within it, which is the browser's own behaviour"),
        accessibility: cap(Support::Yes, "role=radiogroup over native radios, each option named by its own label"),
        environment: cap(Support::Yes, "macOS Chrome, strict CSP, WebGL2 region"),
    },
    Entry {
        category: Category::Switch,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "checked, disabled, read-only; it never flips itself"),
        actions: cap(Support::Yes, "change; both halves call one application action"),
        theme: cap(Support::Yes, "the track in two colours and the knob"),
        input: cap(Support::Yes, "pointer and Enter/Space"),
        accessibility: cap(Support::Yes, "role=switch with aria-checked; the region's has a DOM control with the same name and state"),
        environment: cap(Support::Yes, "macOS Chrome, strict CSP, WebGL2 region"),
    },
    Entry {
        category: Category::Select,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Partial,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "value, the options, open, placeholder, disabled, read-only, invalid; an option can be disabled on its own"),
        actions: cap(Support::Yes, "change and open-change, both the application's to grant"),
        theme: cap(Support::Yes, "the closed control, the list surface and the chosen row"),
        input: cap(Support::Yes, "Enter/Space and the arrows open it; the arrows and Home/End move within it, Enter chooses, Escape closes the top layer only"),
        accessibility: cap(Support::Yes, "a combobox with aria-expanded over a listbox of options, with aria-activedescendant saying which is current"),
        environment: cap(Support::Partial, "experimental in a region: it draws the closed control and reports its rectangle, and the list is a DOM layer anchored to that, because a popup cannot leave the canvas"),
    },
    Entry {
        category: Category::Slider,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "value, min, max, step, disabled, read-only; the value is clamped and snapped before the application sees it"),
        actions: cap(Support::Yes, "change; both halves call one application action"),
        theme: cap(Support::Yes, "the track, the filled part and the knob"),
        input: cap(Support::Yes, "pointer drag and arrow keys, from either half"),
        accessibility: cap(Support::Yes, "a native range control; the region's has a DOM control with the same name and value"),
        environment: cap(Support::Yes, "macOS Chrome, strict CSP, WebGL2 region"),
    },
    Entry {
        category: Category::Progress,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "value, max, and whether the quantity is known at all"),
        actions: cap(Support::No, "none; it is a reading"),
        theme: cap(Support::Yes, "the track and the filled part"),
        input: cap(Support::No, "none"),
        accessibility: cap(Support::Yes, "role=progressbar; with nothing known it carries no aria-valuenow, which says 'in progress' rather than 'nothing done'"),
        environment: cap(Support::Partial, "the region draws a known fraction; an unknown one is the DOM's, because animating it in a region would keep the page's GPU awake"),
    },
    Entry {
        category: Category::Loading,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "whether it is spinning, and the four states of Load in the SDK's own LoadView: loading, empty, ready, error"),
        actions: cap(Support::Yes, "one application-defined retry on the error state"),
        theme: cap(Support::Yes, "the arc takes the text colour around it"),
        input: cap(Support::Yes, "the retry control is an ordinary button"),
        accessibility: cap(Support::Yes, "role=status with a name, so the state is announced rather than only drawn"),
        environment: cap(Support::Yes, "a region's arc turns on the pass clock and asks for the next frame only while it is spinning; a scope that asks for less motion gets the same arc held still"),
    },
    Entry {
        category: Category::Tooltip,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "open and the text; the application owns whether it is showing"),
        actions: cap(Support::Yes, "open-change on pointer and on focus, which is what the imported one lacked and what made it invisible to a keyboard"),
        theme: cap(Support::Yes, "the layer surface and its text"),
        input: cap(Support::Yes, "hover and focus both open it; Escape closes the top layer only"),
        accessibility: cap(Support::Yes, "role=tooltip, attached to the control it describes rather than to a wrapper no reader announces"),
        environment: cap(Support::Partial, "a region's tooltip is a DOM layer anchored to a rectangle inside it; a popup drawn in the canvas cannot leave it (F16)"),
    },
    Entry {
        category: Category::Menu,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "open, the items, and a reason on any item that cannot run; a command that cannot run stays in the list"),
        actions: cap(Support::Yes, "activation and open-change; activating an item closes the menu once"),
        theme: cap(Support::Yes, "the panel surface, its border and the hovered row"),
        input: cap(Support::Yes, "the arrows and Home/End move within it, Enter and Space activate, Escape closes the top layer only"),
        accessibility: cap(Support::Yes, "role=menu over menuitems, the keyboard lands on the first reachable one, and focus returns to what opened it"),
        environment: cap(Support::Partial, "anchors to a rectangle inside a region within one CSS pixel, but does not flip or clamp to the viewport"),
    },
    Entry {
        category: Category::Dialog,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "open, title, description, modal, and whether the backdrop dismisses it"),
        actions: cap(Support::Yes, "open-change; Escape closes the top layer only"),
        theme: cap(Support::Yes, "the panel surface, its border and the backdrop"),
        input: cap(Support::Yes, "the keyboard cannot leave a modal one, and clicks do not reach what it covers, region included"),
        accessibility: cap(Support::Yes, "role=dialog with aria-modal, labelled by its own title and described by its own description; focus returns on close"),
        environment: cap(Support::Partial, "a modal covers the scope's regions as well as its DOM and clicks reach neither; only this scope is made inert, and a host page outside it is not"),
    },
    Entry {
        category: Category::Tabs,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Partial,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "the tabs, which is active, and whether one is disabled; the panels stay in the document so what is in them survives a look elsewhere"),
        actions: cap(Support::Yes, "activate; the application owns which tab is showing"),
        theme: cap(Support::Yes, "the strip, the active tab's face and the focus ring"),
        input: cap(Support::Yes, "one tab stop for the strip, the arrows and Home/End within it, wrapping and stepping over the unreachable"),
        accessibility: cap(Support::Yes, "a tablist over tabs and tabpanels, each panel labelled by its own tab"),
        environment: cap(Support::Partial, "experimental in a region: the strip is drawn and reports which tab was asked for, and the panel below it is the application's to draw"),
    },
    Entry {
        category: Category::ScrollArea,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "what it contains, and whether a wheel arriving at the edge scrolls the page behind it"),
        actions: cap(Support::No, "none of its own; the scrolling is the browser's"),
        theme: cap(Support::Yes, "the scrollbar takes the scope's border colour rather than the browser's"),
        input: cap(Support::Yes, "wheel, drag and the keyboard - it takes a tab stop, because a box only a mouse can scroll is unreachable without one"),
        accessibility: cap(Support::Yes, "role=group with a name, so a reader knows which box the keyboard landed in"),
        environment: cap(Support::Partial, "a region's is makepad's own ScrollBars on a View; the boundary a wheel is handed back at is P2 M6"),
    },
];

/// The row for one category.
pub fn entry(category: Category) -> &'static Entry {
    CATALOG
        .iter()
        .find(|entry| entry.category == category)
        .expect("the catalogue covers every category")
}

/// Fences the generated table inside the document, so the prose around it
/// survives a regeneration.
pub const MARKER: &str = "<!-- catalogue -->";

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
    fn every_category_has_a_dom_component_and_a_way_to_reach_it() {
        // The DOM is where a control is named, focused and read, including the
        // ones a region draws: R18 asks for eighteen, and eighteen that a
        // keyboard cannot reach would not be eighteen.
        for entry in &CATALOG {
            assert_ne!(
                entry.presentation.dom,
                Support::No,
                "{} has no DOM component",
                entry.category.name()
            );
            assert_ne!(
                entry.accessibility.support,
                Support::No,
                "{} ships without a name, role or state",
                entry.category.name()
            );
        }
    }

    #[test]
    fn the_categories_a_region_draws_are_the_declared_subset() {
        let drawn: Vec<&'static str> = CATALOG
            .iter()
            .filter(|entry| entry.drawn_by_region())
            .map(|entry| entry.category.name())
            .collect();
        assert_eq!(
            drawn,
            [
                "button",
                "label",
                "icon",
                "text field",
                "text area",
                "checkbox",
                "radio",
                "switch",
                "select",
                "slider",
                "progress",
                "loading",
                "tabs",
                "scroll area"
            ]
        );
    }

    #[test]
    fn a_category_no_region_draws_says_where_it_is_drawn_instead() {
        // link, tooltip, menu and dialog. Three of them are layers anchored to
        // a region rather than drawn inside one, and a cell saying only "no"
        // would leave a reader thinking they cannot be used with a region at
        // all - which is the opposite of true.
        for entry in CATALOG.iter().filter(|entry| !entry.drawn_by_region()) {
            assert_ne!(
                entry.environment.support,
                Support::Yes,
                "{} claims a clean environment and has no GPU half",
                entry.category.name()
            );
            assert!(
                entry.environment.note.contains("region"),
                "{} does not say what a region does instead",
                entry.category.name()
            );
        }
    }

    #[test]
    fn a_partial_answer_is_never_a_bare_one() {
        // "Partial" is the answer that costs a reader the most if it is not
        // explained, so it is the one held to a length.
        for entry in &CATALOG {
            for (class, capability) in entry.capabilities() {
                if capability.support == Support::Partial {
                    assert!(
                        capability.note.len() > 40,
                        "{}'s {class} is partial and says almost nothing",
                        entry.category.name()
                    );
                }
            }
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

    /// The capability document is the same table, printed.
    ///
    /// `cargo xtask catalog --write docs/components.md` is what writes it;
    /// this is what fails when somebody edits the table and forgets.
    #[test]
    fn the_capability_document_holds_the_table_the_code_holds() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/components.md");
        let document = std::fs::read_to_string(path).unwrap_or_default();
        assert!(
            document.contains(&markdown()),
            "docs/components.md is out of step with the catalogue; \
             run `cargo xtask catalog --write docs/components.md`"
        );
    }

    #[test]
    fn a_category_can_be_looked_up() {
        assert_eq!(entry(Category::Slider).category, Category::Slider);
        assert_eq!(entry(Category::Tabs).presentation.gpu, Support::Partial);
    }
}
