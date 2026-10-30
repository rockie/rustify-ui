//! Capability facts for the current source checkout.
//!
//! [`CATALOG`] preserves the category table used by existing applications.
//! [`COMPONENTS`], [`STABLE_REFERENCES`] and [`SUB_CAPABILITIES`] distinguish
//! public Rust entries, complete upstream inventories and their remaining
//! limits. Verification entries point to repeatable checks, not permanent
//! browser or accessibility certification.

use std::fmt::Write as _;

/// The legacy navigation categories. Their count is not a component or
/// public-symbol count; current entries are listed separately in [`COMPONENTS`].
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
    DataTable,
    Tree,
    Toast,
    NumberField,
    ColorField,
    ToggleGroup,
}

impl Category {
    /// The stable name existing catalogue applications print.
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
            Self::DataTable => "data table",
            Self::Tree => "tree",
            Self::Toast => "toast",
            Self::NumberField => "number field",
            Self::ColorField => "color field",
            Self::ToggleGroup => "toggle group",
        }
    }
}

/// How far one capability goes.
///
/// There is deliberately no `Unknown`: a question nobody answered is a gap in
/// the catalogue, not a state a component can be in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Support {
    /// Available under the listed contract, with a verification entry.
    Yes,
    /// Implemented, with a stated limit.
    Partial,
    /// No public implementation of the listed capability.
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
    /// Runtime requirements and limits. Repeatable checks are listed on the
    /// corresponding [`ComponentEntry`], rather than implying certification.
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
    /// This legacy category answer does not promise that a DOM component's
    /// newer visual properties have matching region implementations.
    pub fn drawn_by_region(&self) -> bool {
        self.presentation.gpu != Support::No
    }
}

/// The legacy category catalogue, retained for existing application navigation.
pub const CATALOG: [Entry; 24] = [
    Entry {
        category: Category::Button,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::Yes,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "DOM label, seven variants, responsive four-size scale, local color, loading and disabled; region buttons retain their own smaller property set"),
        actions: cap(Support::Yes, "click; the DOM and the region button call the same application action"),
        theme: cap(Support::Yes, "DOM visual choices consume scoped roles and geometry; this category does not promise matching region variants or size scales"),
        input: cap(Support::Yes, "pointer and Enter/Space, from either half"),
        accessibility: cap(Support::Yes, "a native button; the region's has a DOM control with the same name and action"),
        environment: cap(Support::Yes, "DOM native input and optional WebGL2 region; verification entries define browser coverage, with external rules or controlled CSSOM updates"),
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
        environment: cap(Support::Yes, "browser entry with strict CSP; a region's text needs that region's font"),
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
        environment: cap(Support::Partial, "CurrentPath is supplied by the application or router; the native DOM anchor navigates, while a region reports an action without touching browser history"),
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
        environment: cap(Support::Yes, "browser entry with strict CSP, WebGL2 region"),
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
        environment: cap(Support::Yes, "browser entry with strict CSP, WebGL2 region"),
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
        environment: cap(Support::Yes, "browser entry with strict CSP, WebGL2 region"),
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
        environment: cap(Support::Yes, "browser entry with strict CSP, WebGL2 region"),
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
        environment: cap(Support::Partial, "a region's tooltip is a DOM layer anchored to a rectangle inside it; a popup drawn in the canvas cannot leave it"),
    },
    Entry {
        category: Category::Menu,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "open, the items, group headings, separators, the application's shortcut beside a command, and a reason on any item that cannot run; a command that cannot run stays in the list"),
        actions: cap(Support::Yes, "activation and open-change; activating an item closes the menu once"),
        theme: cap(Support::Yes, "the panel surface, its border, the hovered row, the headings, the separators and the shortcut column"),
        input: cap(Support::Yes, "the arrows and Home/End move within it over the headings and separators, Enter and Space activate, Escape closes the top layer only"),
        accessibility: cap(Support::Yes, "role=menu over menuitems, groups named by their headings, separators, aria-keyshortcuts for every shortcut shown; the keyboard lands on the first reachable item, and focus returns to what opened it"),
        environment: cap(Support::Partial, "anchors to an element, a point, or a rectangle inside a region within one CSS pixel, and moves left or opens above rather than leave the viewport; it is a DOM layer, because a popup drawn in the canvas cannot leave it"),
    },
    Entry {
        category: Category::Dialog,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Yes,
        },
        properties: cap(Support::Yes, "open, title, description, modal, whether the backdrop dismisses it, and a title bar whose close button is named in the scope's language unless the application names it"),
        actions: cap(Support::Yes, "open-change; the close button and Escape send the same request, and Escape closes the top layer only"),
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
        environment: cap(Support::Partial, "a region uses Makepad ScrollBars; DOM Boundary selects browser overscroll behavior, while a region answers wheel handling from its previous drawing report"),
    },
    Entry {
        category: Category::DataTable,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Partial,
        },
        properties: cap(Support::Yes, "how many rows, which columns, a callback per cell and a callback per row identity; it holds no data, so a sorted, filtered or edited view is the same table asked different questions"),
        actions: cap(Support::Yes, "select a row, open a row, and ask to be shown a row; scrolling is the browser's"),
        theme: cap(Support::Yes, "border, muted header and the accent a selected row is drawn in"),
        input: cap(Support::Yes, "pointer, the arrows, PageUp/PageDown, Home/End and Ctrl+Home/End, Space to select and Enter or F2 to open"),
        accessibility: cap(Support::Yes, "role=grid with aria-rowcount over every row, not only the drawn ones; each row carries its real aria-rowindex and its business id, and one cell at a time takes the tab stop"),
        environment: cap(Support::Partial, "browser entry with strict CSP; no region draws a table - the GPU half of this load is the selection strip beside it - and the row elements are a pool, so a reader meets the rows on screen plus the overscan and reaches the rest by scrolling"),
    },
    Entry {
        category: Category::Tree,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Partial,
        },
        properties: cap(Support::Partial, "two fixed levels of groups, which are open, and which one is chosen; no arbitrary depth and no virtualisation"),
        actions: cap(Support::Yes, "open or close a group, and choose one"),
        theme: cap(Support::Yes, "accent for the chosen group and the scope's focus ring"),
        input: cap(Support::Yes, "pointer; the arrows walk what is showing, right opens a group and steps into it, left closes one and steps out of a child"),
        accessibility: cap(Support::Yes, "role=tree and role=treeitem with aria-expanded, aria-level and aria-selected; one tab stop for the whole tree"),
        environment: cap(Support::Partial, "browser entry with strict CSP, external rules or controlled CSSOM; no region draws a tree - a region takes the group a person chose as a projection, like any other value"),
    },
    Entry {
        category: Category::Toast,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::No,
        },
        properties: cap(Support::Yes, "the message, how long it stays and a tone; a newer message replaces the one showing rather than queueing behind it"),
        actions: cap(Support::Yes, "show, dismiss and hide on its own; a timer set for a message that was replaced leaves its successor alone"),
        theme: cap(Support::Yes, "the popover surface, its border, and a coloured edge per tone; the tone changes how it looks and nothing else"),
        input: cap(Support::Yes, "the dismiss button, by pointer or keyboard; the message takes no focus and answers no Escape"),
        accessibility: cap(Support::Yes, "role=status with aria-live=polite, present while empty so a reader knows it before it speaks; the dismiss button is named in the scope's language"),
        environment: cap(Support::Partial, "browser entry with strict CSP; drawn in the scope's overlay plane but not a layer, so a modal does not make it inert; it shows over a region and no region draws one"),
    },
    Entry {
        category: Category::NumberField,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Partial,
        },
        properties: cap(Support::Yes, "value, min, max, step, disabled, read-only, invalid; the bounds are announced, and clamping is the application's"),
        actions: cap(Support::Yes, "a preview for every number typed, one commit when the edit ends, a cancel when Escape abandons one; the application may refuse any of them"),
        theme: cap(Support::Yes, "the text field's tokens, with the destructive border while the draft is not a number"),
        input: cap(Support::Yes, "typing keeps a draft while the field has focus; Enter or leaving commits, Escape reverts, Up/Down ask for one step and Page Up/Page Down for ten"),
        accessibility: cap(Support::Yes, "role=spinbutton with aria-valuenow, aria-valuemin and aria-valuemax; aria-invalid while the draft does not parse"),
        environment: cap(Support::Partial, "browser entry with strict CSP; the draft is the one exception to controlled values and lives only inside the focused field; no region draws one - a region takes the number as a projection"),
    },
    Entry {
        category: Category::ColorField,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Partial,
        },
        properties: cap(Support::Yes, "value as hex, whether an opacity is allowed, disabled, read-only, invalid; #rgb and #rrggbb, and #rgba and #rrggbbaa with an opacity"),
        actions: cap(Support::Yes, "a preview for every colour typed, one commit when the edit ends, a cancel when Escape abandons one"),
        theme: cap(Support::Yes, "the text field's tokens, and a swatch of the application's colour painted through the CSS object model"),
        input: cap(Support::Yes, "typing keeps a draft while the field has focus; Enter or leaving commits, Escape reverts"),
        accessibility: cap(Support::Yes, "a named text field with aria-invalid while the draft is not a colour; the swatch is decoration and stays out of the tree"),
        environment: cap(Support::Partial, "browser entry with strict CSP and controlled CSSOM color updates; no region draws one - a region takes the colour as a projection, like any other value"),
    },
    Entry {
        category: Category::ToggleGroup,
        presentation: Presentation {
            dom: Support::Yes,
            gpu: Support::No,
            across_regions: Support::Partial,
        },
        properties: cap(Support::Yes, "which are pressed, one at a time or several, the buttons, disabled, and whether it is a toolbar; a button can be disabled on its own"),
        actions: cap(Support::Yes, "a press asks for the set the group would have afterwards; the application owns which are pressed"),
        theme: cap(Support::Yes, "the pressed face in the primary colour, the toolbar's bar and the focus ring"),
        input: cap(Support::Yes, "one tab stop for the group, the arrows and Home/End within it, wrapping and stepping over the unreachable; Enter and Space press"),
        accessibility: cap(Support::Yes, "role=group or role=toolbar with a name, each button aria-pressed"),
        environment: cap(Support::Partial, "browser entry with strict CSP; no region draws one - a region takes what is pressed as a projection, like any other value"),
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

    const ALL: [Category; 24] = [
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
        Category::DataTable,
        Category::Tree,
        Category::Toast,
        Category::NumberField,
        Category::ColorField,
        Category::ToggleGroup,
    ];

    #[test]
    fn every_category_appears_exactly_once() {
        assert_eq!(CATALOG.len(), 24);
        assert_eq!(ALL.len(), CATALOG.len());
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
        // ones a region draws: the category still needs an accessible DOM path.
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
        assert_eq!(rows.len(), CATALOG.len());
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
    /// `mbx xtask catalog --write docs/components.md` is what writes it;
    /// this is what fails when somebody edits the table and forgets.
    #[test]
    #[cfg(rustify_repo_tests)]
    fn the_capability_document_holds_the_table_the_code_holds() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/components.md");
        let document = std::fs::read_to_string(path).unwrap_or_default();
        assert!(
            document.contains(&markdown()),
            "docs/components.md is out of step with the catalogue; \
             run `mbx xtask catalog --write docs/components.md`"
        );
    }

    #[test]
    fn a_category_can_be_looked_up() {
        assert_eq!(entry(Category::Slider).category, Category::Slider);
        assert_eq!(entry(Category::Tabs).presentation.gpu, Support::Partial);
    }
}

/// A local reference snapshot. Versions identify the provided snapshots only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceLibrary {
    Themes,
    Primitives,
}
impl ReferenceLibrary {
    pub const ALL: [Self; 2] = [Self::Themes, Self::Primitives];
    pub fn name(self) -> &'static str {
        match self {
            Self::Themes => "Themes 3.3.0",
            Self::Primitives => "Primitives 1.8.1",
        }
    }
    pub fn source(self) -> &'static str {
        match self {
            Self::Themes => "ref/themes-main/packages/radix-ui-themes/src/components/index.tsx",
            Self::Primitives => "ref/primitives-main/packages/react/radix-ui/src/index.ts",
        }
    }
}

/// One stable runtime export in the fixed reference inventory.
///
/// Mapping a name to an entry does not imply complete upstream compatibility.
/// Missing composition and behavior are recorded in [`SUB_CAPABILITIES`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReferenceEntry {
    pub library: ReferenceLibrary,
    pub name: &'static str,
    pub component: Option<&'static str>,
    pub support: Support,
    pub limitation: &'static str,
}

/// A component, subcomponent, data surface or helper in the public Rust API.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentKind {
    Component,
    Subcomponent,
    Helper,
    Data,
}
impl ComponentKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Component => "component",
            Self::Subcomponent => "subcomponent",
            Self::Helper => "helper",
            Self::Data => "data",
        }
    }
}

/// A source-backed public entry, with its supported field subset and checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentEntry {
    pub name: &'static str,
    pub kind: ComponentKind,
    pub public_path: &'static str,
    pub source: &'static str,
    pub support: Support,
    pub presentation: Presentation,
    pub fields: &'static [&'static str],
    pub property_groups: &'static [&'static str],
    pub limitation: &'static str,
    pub verification: &'static [&'static str],
}

/// Fields shared only by components that explicitly opt into this group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PropertyGroup {
    pub name: &'static str,
    pub fields: &'static [&'static str],
    pub contract: &'static str,
    pub verification: &'static [&'static str],
}

/// A separately tracked behavior or composition boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubCapability {
    pub name: &'static str,
    pub component: &'static str,
    pub support: Support,
    pub public_path: Option<&'static str>,
    pub limitation: &'static str,
    pub verification: &'static [&'static str],
}

/// A non-stable reference export, excluded from stable and implementation counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Candidate {
    pub library: ReferenceLibrary,
    pub name: &'static str,
    pub source: &'static str,
    pub limitation: &'static str,
}

const DOM_ONLY: Presentation = Presentation {
    dom: Support::Yes,
    gpu: Support::No,
    across_regions: Support::No,
};
const PROJECTED: Presentation = Presentation {
    dom: Support::Yes,
    gpu: Support::No,
    across_regions: Support::Partial,
};
const BUTTON_CHECKS: &[&str] = &[
    "crates/rustify-components/src/button.rs",
    "crates/rustify-components/src/icon_button.rs",
    "tests/browser/dom-buttons.spec.ts",
];
const LAYOUT_CHECKS: &[&str] = &[
    "crates/rustify-components/src/layout.rs",
    "crates/rustify-components/src/responsive.rs",
    "tests/browser/dom-layout.spec.ts",
];
const TYPE_CHECKS: &[&str] = &[
    "crates/rustify-components/src/typography.rs",
    "tests/browser/dom-layout.spec.ts",
];
const LEGACY_CHECKS: &[&str] = &[
    "examples/component-catalog/src/main.rs",
    "tests/browser/m6-components.spec.ts",
    "tests/browser/catalog-inputs.spec.ts",
];
const THEME_CHECKS: &[&str] = &[
    "crates/rustify-dom/src/theme/author_tests.rs",
    "examples/component-catalog/src/theme_fixture.rs",
    "tests/browser/theme-compat.spec.ts",
];

const FORM_CHECKS: &[&str] = &[
    "tests/dom-only/primitives.spec.ts",
    "tests/dom-only/forms.spec.ts",
    "crates/rustify-components/tests/forms_ssr.rs",
    "tests/browser/theme-forms.spec.ts",
    "tests/browser/catalog-inputs.spec.ts",
    "tests/browser/p2-form.spec.ts",
];

/// Complete field groups used by the listed source APIs.
pub const PROPERTY_GROUPS: &[PropertyGroup] = &[
    PropertyGroup { name: "Element", fields: &["class", "test_id", "children"], contract: "Native attributes use Leptos passthrough; caller utilities retain precedence.", verification: LAYOUT_CHECKS },
    PropertyGroup { name: "Margin", fields: &["m", "mx", "my", "mt", "mr", "mb", "ml"], contract: "Responsive theme steps -9..9 or validated CSS lengths; axes and edges take precedence.", verification: LAYOUT_CHECKS },
    PropertyGroup { name: "Padding", fields: &["p", "px", "py", "pt", "pr", "pb", "pl"], contract: "Responsive theme steps 0..9 or validated nonnegative CSS lengths; records nearest inset padding.", verification: LAYOUT_CHECKS },
    PropertyGroup { name: "Gap", fields: &["gap", "gap_x", "gap_y"], contract: "Responsive theme steps 0..9 or validated CSS gap values; axis values take precedence.", verification: LAYOUT_CHECKS },
    PropertyGroup { name: "Layout", fields: &["padding", "width", "min_width", "max_width", "height", "min_height", "max_height", "position", "inset", "top", "right", "bottom", "left", "overflow", "overflow_x", "overflow_y", "flex_basis", "flex_grow", "flex_shrink", "grid_area", "grid_column", "grid_column_start", "grid_column_end", "grid_row", "grid_row_start", "grid_row_end", "align_self", "justify_self"], contract: "Sparse responsive fields; padding uses Padding. CSS is validated identically for SSR/CSR; explicit CSS sizes are not theme-scaled.", verification: LAYOUT_CHECKS },
    PropertyGroup { name: "InsetPadding", fields: &["p", "px", "py", "pt", "pr", "pb", "pl"], contract: "Responsive Current or Zero only; this group does not accept arbitrary negative margins or Layout.", verification: LAYOUT_CHECKS },
    PropertyGroup { name: "ButtonVisual", fields: &["variant", "size", "color", "color_role", "radius", "high_contrast"], contract: "Seven variants, responsive sizes 1..4, optional local brand color/radius; unselected color consumes semantic roles.", verification: BUTTON_CHECKS },
    PropertyGroup { name: "ButtonNative", fields: &["on_click", "aria_label", "loading", "disabled", "unavailable", "r#type", "controls", "expanded"], contract: "Native button/submit/reset; loading blocks activation and native defaults. Unavailable retains focus and delivers requests.", verification: BUTTON_CHECKS },
    PropertyGroup { name: "TextVisual", fields: &["size", "weight", "align", "trim", "wrap", "truncate", "color", "high_contrast"], contract: "Size 1..9, weight/align/trim/wrap responsive; truncate is a nonresponsive boolean. Omitted Text size inherits.", verification: TYPE_CHECKS },
    PropertyGroup { name: "QuoteVisual", fields: &["size", "weight", "wrap", "truncate", "color", "high_contrast"], contract: "Blockquote/Code omit align and trim; the native semantic element remains intact.", verification: TYPE_CHECKS },
    PropertyGroup { name: "InlineText", fields: &["wrap", "truncate"], contract: "Em/Quote/Strong inherit typography and expose no independent size, color or layout group.", verification: TYPE_CHECKS },
];

macro_rules! component {
    ($name:literal, $kind:ident, $path:literal, $source:literal, $support:ident, $presentation:expr, [$($field:literal),*], [$($group:literal),*], $limit:literal, $checks:expr) => {
        ComponentEntry { name: $name, kind: ComponentKind::$kind, public_path: $path, source: $source, support: Support::$support, presentation: $presentation,
            fields: &[$($field),*], property_groups: &[$($group),*], limitation: $limit, verification: $checks }
    };
}

/// Public source entries. Count ComponentKind values separately from helpers and data.
pub const COMPONENTS: &[ComponentEntry] = &[
    component!("Button", Component, "rustify_components::Button", "crates/rustify-components/src/button.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Partial }, ["on_click", "variant", "size", "color", "color_role", "radius", "high_contrast", "loading", "disabled", "unavailable", "r#type", "aria_label", "controls", "expanded", "class", "test_id", "children"], ["ButtonVisual", "ButtonNative", "Element"], "DOM visual API only. Existing region buttons share actions and theme projections, with a smaller visual property set.", BUTTON_CHECKS),
    component!("IconButton", Component, "rustify_components::IconButton", "crates/rustify-components/src/icon_button.rs", Yes, DOM_ONLY, ["on_click", "aria_label", "variant", "size", "color", "color_role", "radius", "high_contrast", "loading", "disabled", "unavailable", "r#type", "controls", "expanded", "class", "test_id", "children"], ["ButtonVisual", "ButtonNative", "Element"], "Required nonempty accessible name; decorative children. Reuses Button behavior; no separate GPU icon button.", BUTTON_CHECKS),
    component!("Link", Component, "rustify_components::Link", "crates/rustify-components/src/link.rs", Yes, DOM_ONLY, ["href", "match_", "disabled", "size", "weight", "trim", "wrap", "underline", "color", "high_contrast", "truncate", "class", "test_id", "children"], ["Element"], "Native anchor, CurrentPath matching and Leptos target/rel attributes; no unrelated Layout group or GPU navigation.", &["crates/rustify-components/src/link.rs", "tests/browser/dom-layout.spec.ts"]),
    component!("Box", Component, "rustify_components::Box", "crates/rustify-components/src/layout.rs", Yes, DOM_ONLY, ["as_", "display", "layout", "margin", "class", "test_id", "children"], ["Layout", "Margin", "Element"], "Native div/span; component display defaults survive missing responsive initial values. Caller utilities retain precedence.", LAYOUT_CHECKS),
    component!("Flex", Component, "rustify_components::Flex", "crates/rustify-components/src/layout.rs", Yes, DOM_ONLY, ["as_", "display", "direction", "align", "justify", "wrap", "gap", "layout", "margin", "class", "test_id", "children"], ["Layout", "Margin", "Gap", "Element"], "Native div/span; component display defaults survive missing responsive initial values. Caller utilities retain precedence.", LAYOUT_CHECKS),
    component!("Grid", Component, "rustify_components::Grid", "crates/rustify-components/src/layout.rs", Yes, DOM_ONLY, ["as_", "display", "areas", "columns", "rows", "flow", "align", "justify", "align_content", "justify_items", "gap", "layout", "margin", "class", "test_id", "children"], ["Layout", "Margin", "Gap", "Element"], "Native div/span; component display defaults survive missing responsive initial values. Caller utilities retain precedence.", LAYOUT_CHECKS),
    component!("Container", Component, "rustify_components::Container", "crates/rustify-components/src/layout.rs", Yes, DOM_ONLY, ["size", "align", "display", "layout", "margin", "class", "test_id", "children"], ["Layout", "Margin", "Element"], "Two layers: width/min/max and height/min/max target the inner content; size selects its maximum width.", LAYOUT_CHECKS),
    component!("Section", Component, "rustify_components::Section", "crates/rustify-components/src/layout.rs", Yes, DOM_ONLY, ["size", "display", "layout", "margin", "class", "test_id", "children"], ["Layout", "Margin", "Element"], "Native section; size consumes resolved theme spacing. Explicit padding takes precedence.", LAYOUT_CHECKS),
    component!("Inset", Component, "rustify_components::Inset", "crates/rustify-components/src/layout.rs", Yes, DOM_ONLY, ["side", "clip", "padding", "margin", "class", "test_id", "children"], ["Margin", "InsetPadding", "Element"], "Consumes only nearest container padding. Restricted padding Current/Zero; does not expose Layout.", LAYOUT_CHECKS),
    component!("Reset", Component, "rustify_components::Reset", "crates/rustify-components/src/layout.rs", Yes, DOM_ONLY, ["element"], [], "Typed single-native-element decorator; no wrapper, no Children/Slot API; preserves existing attributes and events.", LAYOUT_CHECKS),
    component!("Text", Component, "rustify_components::Text", "crates/rustify-components/src/typography.rs", Yes, DOM_ONLY, ["as_", "size", "weight", "align", "trim", "wrap", "truncate", "color", "high_contrast", "margin", "class", "test_id", "children"], ["TextVisual", "Margin", "Element"], "Native span/div/label/p; omitted size inherits.", TYPE_CHECKS),
    component!("Heading", Component, "rustify_components::Heading", "crates/rustify-components/src/typography.rs", Yes, DOM_ONLY, ["as_", "size", "weight", "align", "trim", "wrap", "truncate", "color", "high_contrast", "margin", "class", "test_id", "children"], ["TextVisual", "Margin", "Element"], "Native h1..h6 independent of visual size; default h1 with visual size 6.", TYPE_CHECKS),
    component!("Blockquote", Component, "rustify_components::Blockquote", "crates/rustify-components/src/typography.rs", Yes, DOM_ONLY, ["size", "weight", "wrap", "truncate", "color", "high_contrast", "margin", "class", "test_id", "children"], ["QuoteVisual", "Margin", "Element"], "Native blockquote; no independent align/trim properties.", TYPE_CHECKS),
    component!("Code", Component, "rustify_components::Code", "crates/rustify-components/src/typography.rs", Yes, DOM_ONLY, ["variant", "size", "weight", "wrap", "truncate", "color", "high_contrast", "margin", "class", "test_id", "children"], ["QuoteVisual", "Margin", "Element"], "Native code, four variants and theme mono sizing; no align/trim properties.", TYPE_CHECKS),
    component!("Kbd", Component, "rustify_components::Kbd", "crates/rustify-components/src/typography.rs", Yes, DOM_ONLY, ["variant", "size", "margin", "class", "test_id", "children"], ["Margin", "Element"], "Native kbd; classic/soft, optional size; no color/weight/trim properties.", TYPE_CHECKS),
    component!("Em", Component, "rustify_components::Em", "crates/rustify-components/src/typography.rs", Yes, DOM_ONLY, ["wrap", "truncate", "class", "test_id", "children"], ["InlineText", "Element"], "Native em; inherited size, no Margin/Layout/color group.", TYPE_CHECKS),
    component!("Quote", Component, "rustify_components::Quote", "crates/rustify-components/src/typography.rs", Yes, DOM_ONLY, ["wrap", "truncate", "class", "test_id", "children"], ["InlineText", "Element"], "Native q keeps browser quotation marks; no Margin/Layout/color group.", TYPE_CHECKS),
    component!("Strong", Component, "rustify_components::Strong", "crates/rustify-components/src/typography.rs", Yes, DOM_ONLY, ["wrap", "truncate", "class", "test_id", "children"], ["InlineText", "Element"], "Native strong; inherited size, no Margin/Layout/color group.", TYPE_CHECKS),
    component!("Label", Component, "rustify_components::Label", "crates/rustify-components/src/label.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Yes }, ["control", "class", "test_id", "children"], [], "Existing native controlled component contract; no complete upstream visual or primitive part API is implied.", LEGACY_CHECKS),
    component!("Icon", Component, "rustify_components::Icon", "crates/rustify-components/src/icon.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Yes }, ["glyph", "label", "class", "test_id", "children"], [], "Six Glyph shapes or caller SVG children; named image/decorative mode. No cloned Radix Icons product.", LEGACY_CHECKS),
    component!("TextField", Component, "rustify_components::TextField", "crates/rustify-components/src/input.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Partial }, ["value", "on_change", "kind", "id", "name", "form", "aria_label", "labelled_by", "described_by", "placeholder", "autocomplete", "inputmode", "required", "disabled", "read_only", "invalid", "pattern", "minlength", "maxlength", "size", "variant", "color", "radius", "class", "input_class", "test_id", "children"], [], "Controlled native editor with Root/Input/Slot; class styles the boundary, input_class the editor. Size 1/2/3 and classic/surface/soft; DOM parts share behavior.", FORM_CHECKS),
    component!("TextArea", Component, "rustify_components::TextArea", "crates/rustify-components/src/textarea.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Partial }, ["value", "on_change", "rows", "id", "name", "form", "aria_label", "labelled_by", "described_by", "placeholder", "autocomplete", "inputmode", "required", "disabled", "read_only", "invalid", "pattern", "minlength", "maxlength", "size", "variant", "color", "radius", "resize", "class", "input_class", "test_id"], [], "Native textarea with controlled value, composition completion, native form fields and responsive resize. Visual options apply to DOM only.", FORM_CHECKS),
    component!("Checkbox", Component, "rustify_components::Checkbox", "crates/rustify-components/src/checkbox.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Yes }, ["checked", "on_change", "indeterminate", "id", "aria_label", "labelled_by", "name", "value", "form", "required", "disabled", "read_only", "invalid", "described_by", "size", "variant", "color", "high_contrast", "class", "test_id"], [], "Boolean convenience plus indeterminate; parts use CheckedState. Only accepted checked values submit; native required, readonly and reset follow the shared form contract.", FORM_CHECKS),
    component!("RadioGroup", Component, "rustify_components::RadioGroup", "crates/rustify-components/src/radio/group.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Yes }, ["value", "options", "on_change", "name", "form", "aria_label", "described_by", "disabled", "read_only", "invalid", "required", "orientation", "r#loop", "size", "variant", "color", "high_contrast", "class", "test_id"], [], "Array convenience over radio_group Root/Item. Anonymous groups retain independent identity without native FormData keys; named groups honor actual form owners.", FORM_CHECKS),
    component!("Switch", Component, "rustify_components::Switch", "crates/rustify-components/src/switch.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Yes }, ["checked", "on_change", "id", "aria_label", "labelled_by", "name", "value", "form", "required", "disabled", "read_only", "invalid", "described_by", "size", "variant", "color", "high_contrast", "class", "test_id", "radius"], [], "Boolean convenience over native Root/Control/Thumb. Controlled requests, native form participation and component-local visual roles; no separate GPU part family.", FORM_CHECKS),
    component!("Select", Component, "rustify_components::Select", "crates/rustify-components/src/select.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Partial }, ["value", "options", "on_change", "open", "on_open_change", "placeholder", "id", "name", "form", "aria_label", "labelled_by", "disabled", "read_only", "invalid", "required", "described_by", "size", "variant", "color", "radius", "content_variant", "high_contrast", "class", "test_id"], [], "Convenience over public select Root/Trigger/Content and other parts. Trigger keeps focus; explicit entries supply SSR label and native accepted-value mirror. No list virtualization.", FORM_CHECKS),
    component!("Slider", Component, "rustify_components::Slider", "crates/rustify-components/src/slider.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Yes }, ["value", "on_change", "on_commit", "on_cancel", "min", "max", "step", "orientation", "inverted", "id", "aria_label", "labelled_by", "value_text", "name", "form", "disabled", "read_only", "invalid", "described_by", "size", "variant", "color", "high_contrast", "radius", "class", "test_id"], [], "Scalar convenience over public slider parts; stable thumb identity, accepted-value normalization, preview requests and commit/cancel. Existing GPU scalar counterpart is unchanged.", FORM_CHECKS),
    component!("Progress", Component, "rustify_components::Progress", "crates/rustify-components/src/progress.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Partial }, ["value", "max", "indeterminate", "aria_label", "class", "test_id"], [], "Value/max or indeterminate DOM progress; region rendering covers a known fraction only.", LEGACY_CHECKS),
    component!("Spinner", Component, "rustify_components::Spinner", "crates/rustify-components/src/spinner.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Yes }, ["label", "class", "test_id"], [], "Named status spinner; not a Skeleton component.", LEGACY_CHECKS),
    component!("Tooltip", Component, "rustify_components::Tooltip", "crates/rustify-components/src/tooltip.rs", Yes, PROJECTED, ["open", "on_open_change", "label", "class", "test_id", "children"], [], "Controlled scoped DOM layer with pointer/focus behavior; no HoverCard or Popover API.", LEGACY_CHECKS),
    component!("Menu", Component, "rustify_components::Menu", "crates/rustify-components/src/menu.rs", Yes, PROJECTED, ["open", "on_open_change", "anchor", "items", "on_activate", "aria_label", "class", "test_id"], [], "Command Menu with data items and scoped anchoring; not separate DropdownMenu, ContextMenu, Menubar or NavigationMenu APIs.", LEGACY_CHECKS),
    component!("Dialog", Component, "rustify_components::Dialog", "crates/rustify-components/src/dialog.rs", Yes, PROJECTED, ["open", "on_open_change", "title", "description", "modal", "close_on_backdrop", "close_label", "class", "test_id", "children"], [], "Controlled scoped dialog and modal focus behavior; no AlertDialog or public Root/Trigger/Content parts.", LEGACY_CHECKS),
    component!("Tabs", Component, "rustify_components::Tabs", "crates/rustify-components/src/tabs.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Partial }, ["active", "tabs", "on_activate", "orientation", "aria_label", "class", "test_id", "children"], [], "Controlled tabs data, roving focus and retained panels; no SegmentedControl or TabNav API.", LEGACY_CHECKS),
    component!("TabPanel", Subcomponent, "rustify_components::TabPanel", "crates/rustify-components/src/tabs.rs", Yes, DOM_ONLY, ["value", "class", "children"], [], "Existing native controlled component contract; no complete upstream visual or primitive part API is implied.", LEGACY_CHECKS),
    component!("ScrollArea", Component, "rustify_components::ScrollArea", "crates/rustify-components/src/scroll_area.rs", Yes, Presentation { dom: Support::Yes, gpu: Support::Partial, across_regions: Support::Yes }, ["boundary", "aria_label", "class", "test_id", "children"], [], "Native browser scrolling and Boundary overscroll behavior; no custom scrollbar part API.", LEGACY_CHECKS),
    component!("DataTable", Component, "rustify_components::DataTable", "crates/rustify-components/src/data_table/view.rs", Yes, PROJECTED, ["rows", "columns", "cell", "row_id", "selected", "goto", "version", "focus", "on_select", "on_activate", "sorted", "on_sort", "row_height", "column_width", "initial_viewport", "aria_label", "class", "test_id"], [], "Virtualized interactive DOM grid; distinct from an ordinary semantic Table component. No GPU-drawn data table.", LEGACY_CHECKS),
    component!("Tree", Component, "rustify_components::Tree", "crates/rustify-components/src/tree.rs", Partial, PROJECTED, ["nodes", "expanded", "selected", "on_select", "aria_label", "class", "test_id"], [], "Two group levels without tree virtualization or arbitrary depth.", LEGACY_CHECKS),
    component!("Toast", Component, "rustify_components::Toast", "crates/rustify-components/src/toast.rs", Yes, DOM_ONLY, ["toast", "class", "test_id"], [], "One replaceable status message in the scope overlay plane; not a complete Toast primitive parts API.", LEGACY_CHECKS),
    component!("Toaster", Component, "rustify_components::Toaster", "crates/rustify-components/src/toast.rs", Yes, DOM_ONLY, ["class", "test_id"], [], "Renders the SDK toast owner; no second queue or application business-state owner.", LEGACY_CHECKS),
    component!("NumberField", Component, "rustify_components::NumberField", "crates/rustify-components/src/number_field.rs", Yes, PROJECTED, ["value", "on_change", "on_preview", "on_cancel", "min", "max", "step", "id", "name", "form", "aria_label", "labelled_by", "described_by", "placeholder", "autocomplete", "inputmode", "required", "disabled", "read_only", "invalid", "pattern", "minlength", "maxlength", "size", "variant", "color", "radius", "class", "input_class", "test_id", "children"], [], "Focused numeric Draft with preview/completion/cancel; named mirror holds accepted value. Native bounds and FormRoot flush; separate Root/Input support composition.", FORM_CHECKS),
    component!("ColorField", Component, "rustify_components::ColorField", "crates/rustify-components/src/color_field.rs", Yes, PROJECTED, ["value", "on_change", "on_preview", "on_cancel", "alpha", "id", "name", "form", "aria_label", "labelled_by", "described_by", "placeholder", "autocomplete", "inputmode", "required", "disabled", "read_only", "invalid", "pattern", "minlength", "maxlength", "size", "variant", "color", "radius", "class", "input_class", "test_id", "children"], [], "Focused hexadecimal Draft and accepted-value mirror; optional alpha and explicit decorative Swatch. No arbitrary CSS color parser or palette editor.", FORM_CHECKS),
    component!("ToggleGroup", Component, "rustify_components::ToggleGroup", "crates/rustify-components/src/toggle_group.rs", Yes, PROJECTED, ["value", "on_change", "items", "multiple", "toolbar", "orientation", "r#loop", "allow_empty", "name", "form", "disabled", "read_only", "invalid", "required", "described_by", "aria_label", "class", "test_id"], [], "Convenience group/toolbar view over toggle_group Root/Item; native pressed state, accepted values and disabled/readonly gates. Not the complete reference Toolbar.", FORM_CHECKS),
    component!("FilePicker", Component, "rustify_components::FilePicker", "crates/rustify-components/src/file_picker.rs", Yes, DOM_ONLY, ["on_files", "accept", "id", "aria_label", "disabled", "described_by", "class", "test_id"], [], "Rustify extension with its existing controlled contract; no reference counterpart is required.", &["examples/property-workbench/src/main.rs", "tests/browser/p2-workspace.spec.ts", "tests/browser/p2-form.spec.ts", "tests/browser/p2-clipboard-files.spec.ts"]),
    component!("DropZone", Component, "rustify_components::DropZone", "crates/rustify-components/src/drop_zone.rs", Yes, DOM_ONLY, ["on_files", "label", "disabled", "described_by", "class", "test_id"], [], "Rustify extension with its existing controlled contract; no reference counterpart is required.", &["examples/property-workbench/src/main.rs", "tests/browser/p2-workspace.spec.ts", "tests/browser/p2-form.spec.ts", "tests/browser/p2-clipboard-files.spec.ts"]),
    component!("Splitter", Component, "rustify_components::Splitter", "crates/rustify-components/src/workspace/splitter.rs", Yes, DOM_ONLY, ["sizes", "mins", "on_resize", "step", "divider_label", "test_id"], [], "Rustify extension with its existing controlled contract; no reference counterpart is required.", &["examples/property-workbench/src/main.rs", "tests/browser/p2-workspace.spec.ts", "tests/browser/p2-form.spec.ts", "tests/browser/p2-clipboard-files.spec.ts"]),
    component!("PanelTabs", Component, "rustify_components::PanelTabs", "crates/rustify-components/src/workspace/panel_tabs.rs", Yes, DOM_ONLY, ["tabs", "active", "on_activate", "on_close", "aria_label", "close_label", "class", "test_id"], [], "Rustify extension with its existing controlled contract; no reference counterpart is required.", &["examples/property-workbench/src/main.rs", "tests/browser/p2-workspace.spec.ts", "tests/browser/p2-form.spec.ts", "tests/browser/p2-clipboard-files.spec.ts"]),
    component!("CommandPalette", Component, "rustify_components::CommandPalette", "crates/rustify-components/src/workspace/command_palette.rs", Yes, DOM_ONLY, ["open", "on_open_change", "commands", "on_run", "placeholder", "aria_label", "test_id"], [], "Rustify extension with its existing controlled contract; no reference counterpart is required.", &["examples/property-workbench/src/main.rs", "tests/browser/p2-workspace.spec.ts", "tests/browser/p2-form.spec.ts", "tests/browser/p2-clipboard-files.spec.ts"]),
    component!("Field", Component, "rustify_components::Field", "crates/rustify-components/src/form/field.rs", Yes, DOM_ONLY, ["form", "field", "label", "description", "control", "class"], [], "Convenience label/description/control/message with FieldBinding. Composed Root/Label/Description/Message use the same native association and source-specific errors.", FORM_CHECKS),
    component!("SubmitButton", Component, "rustify_components::SubmitButton", "crates/rustify-components/src/form/submit.rs", Yes, DOM_ONLY, ["form", "class", "test_id", "children"], [], "Native submit inside FormRoot, legacy direct owner submission outside. Root owns rules; aria-disabled retains focus and allows an explanatory validation request.", FORM_CHECKS),
    component!("FormStatus", Component, "rustify_components::FormStatus", "crates/rustify-components/src/form/submit.rs", Yes, DOM_ONLY, ["form", "class", "test_id"], [], "Live save/wait/error status from the existing form owner. It does not store application values or provide a business backend.", FORM_CHECKS),
    component!("Theme", Component, "rustify_components::theme::Theme", "crates/rustify-components/src/theme.rs", Yes, DOM_ONLY, ["resolved", "preview_mode"], [], "Browser-only alias of the existing ThemeScope; claims one mount root and retains complete two-mode source. SSR uses provide_static_theme.", THEME_CHECKS),
    component!("ThemeBoundary", Component, "rustify_components::theme::ThemeBoundary", "crates/rustify-components/src/theme.rs", Yes, DOM_ONLY, ["patch", "class", "test_id", "children"], [], "Browser-only scoped boundary with configuration and light/dark patches; an empty patch resumes parent inheritance.", THEME_CHECKS),
    component!("ThemePanel", Component, "rustify_components::ThemePanel", "crates/rustify-components/src/theme_panel.rs", Yes, DOM_ONLY, ["config", "overrides", "on_change", "on_reset", "locale", "class", "test_id"], [], "Controlled configuration/override-count panel and reset requests; not a full theme editor or a second document/history owner.", &["crates/rustify-components/src/theme_panel.rs", "examples/component-catalog/src/button_fixture.rs", "tests/browser/dom-buttons.spec.ts"]),
    component!("Form", Helper, "rustify_components::Form", "crates/rustify-components/src/form/provider.rs", Yes, DOM_ONLY, ["fields", "save"], [], "Form/Field business validation handle, with explicit provide_form/use_form. Not the reference Form DOM-part API.", &["examples/property-workbench/src/main.rs", "tests/browser/p2-form.spec.ts"]),
    component!("ThemeContext", Helper, "rustify_components::theme::use_resolved_theme", "crates/rustify-components/src/theme.rs", Yes, DOM_ONLY, [], [], "Effective resolved snapshot signal; not the React ThemeContext object or a second provider.", THEME_CHECKS),
    component!("Responsive", Helper, "rustify_components::Responsive", "crates/rustify-components/src/responsive.rs", Yes, DOM_ONLY, ["initial", "xs", "sm", "md", "lg", "xl"], [], "Sparse initial/xs/sm/md/lg/xl at 0/520/768/1024/1280/1640 CSS px. Not Tailwind breakpoints.", LAYOUT_CHECKS),
    component!("GeometryStyles", Helper, "rustify_components::GeometryStyles", "crates/rustify-components/src/geometry.rs", Yes, DOM_ONLY, ["css", "diagnostics"], [], "External SSR rules and field diagnostics; shared CSSOM owner/fatal lifecycle. Does not set arbitrary string style attributes.", &["crates/rustify-components/src/geometry.rs", "tests/browser/dom-layout.spec.ts"]),
    component!("Class helpers", Helper, "rustify_components::cn", "crates/rustify-components/src/macros/mod.rs", Yes, DOM_ONLY, ["cn", "clx!", "variants!", "CSS", "CANDIDATES_JSON", "styles"], [], "Existing class merging and generation helpers; application class overrides remain separate from static theme author values.", &["crates/rustify-components/src/macros/mod.rs", "tests/browser/utility-baseline.spec.ts"]),
    component!("Colors", Data, "rustify_colors::ColorFamily", "crates/rustify-colors/src/lib.rs", Yes, Presentation { dom: Support::No, gpu: Support::No, across_regions: Support::No }, ["ALL", "scale", "css", "ColorMode", "ColorGamut", "black_alpha", "white_alpha", "black_alpha_css", "white_alpha_css"], [], "Separate script-free palette data/assets. Rust P3 lookup requires p3; static P3 assets include sRGB fallback. Source-only until separately published.", &["crates/rustify-colors/src/lib.rs", "xtask/src/colors.rs", "tests/colors-static/colors.spec.ts"]),
    component!("Radio", Component, "rustify_components::Radio", "crates/rustify-components/src/radio.rs", Yes, DOM_ONLY, ["checked", "on_change", "id", "aria_label", "labelled_by", "name", "value", "form", "required", "disabled", "read_only", "invalid", "described_by", "size", "variant", "color", "high_contrast", "class", "test_id"], [], "Standalone controlled native radio; group composition uses radio_group Root/Item.", FORM_CHECKS),
    component!("CheckboxGroup", Component, "rustify_components::CheckboxGroup", "crates/rustify-components/src/checkbox_group.rs", Yes, DOM_ONLY, ["value", "options", "on_change", "name", "form", "aria_label", "described_by", "disabled", "read_only", "invalid", "required", "orientation", "r#loop", "size", "variant", "color", "high_contrast", "class", "test_id"], [], "Array convenience over shared multi-selection parts; explicit native name and accepted values only.", FORM_CHECKS),
    component!("CheckboxCards", Component, "rustify_components::checkbox_cards::Root", "crates/rustify-components/src/checkbox_cards.rs", Yes, DOM_ONLY, ["value", "on_change", "name", "form", "aria_label", "described_by", "disabled", "read_only", "invalid", "required", "size", "variant", "color", "high_contrast", "columns", "gap", "class", "test_id", "children"], [], "Multi-selection card Root/Item with explicit label/description content. Size 1/2/3, classic/surface; nested interactive content is diagnosed.", FORM_CHECKS),
    component!("RadioCards", Component, "rustify_components::radio_cards::Root", "crates/rustify-components/src/radio_cards.rs", Yes, DOM_ONLY, ["value", "on_change", "name", "form", "aria_label", "described_by", "disabled", "read_only", "invalid", "required", "size", "variant", "color", "high_contrast", "columns", "gap", "class", "test_id", "children"], [], "Single-selection card Root/Item using native group semantics. Responsive columns/gap; no nested interactive descendants.", FORM_CHECKS),
    component!("Toggle", Component, "rustify_components::Toggle", "crates/rustify-components/src/toggle.rs", Yes, DOM_ONLY, ["pressed", "required", "on_change", "id", "name", "value", "form", "disabled", "read_only", "invalid", "described_by", "aria_label", "labelled_by", "class", "test_id", "children"], [], "Controlled native pressed button. Keeps Rustify visual treatment; no invented upstream Themes variant matrix.", FORM_CHECKS),
    component!("RangeSlider", Component, "rustify_components::RangeSlider", "crates/rustify-components/src/slider.rs", Yes, DOM_ONLY, ["values", "thumbs", "on_change", "on_commit", "on_cancel", "min", "max", "step", "min_steps_between", "preserve_order", "orientation", "inverted", "id", "name", "form", "disabled", "read_only", "invalid", "described_by", "aria_label", "size", "variant", "color", "high_contrast", "radius", "class", "test_id"], [], "Convenience assembly from stable ThumbValue/ThumbLabel declarations. Repeated name mirrors use normalized accepted values; commit reports the last requested candidate.", FORM_CHECKS),
    component!("FormRoot", Component, "rustify_components::FormRoot", "crates/rustify-components/src/form/mod.rs", Yes, DOM_ONLY, ["form", "rules", "on_reset", "on_asked", "id", "aria_label", "class", "test_id", "children"], [], "Native form boundary with composition/Draft completion, actual native constraints and source-specific field errors. Application owns reset values and save completion.", FORM_CHECKS),
    component!("checkbox.Root", Subcomponent, "rustify_components::checkbox::Root", "crates/rustify-components/src/checkbox.rs", Yes, DOM_ONLY, ["checked", "on_change", "id", "name", "value", "form", "aria_label", "labelled_by", "described_by", "disabled", "read_only", "invalid", "required", "size", "variant", "color", "high_contrast", "class", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("switch.Root", Subcomponent, "rustify_components::switch::Root", "crates/rustify-components/src/switch.rs", Yes, DOM_ONLY, ["checked", "on_change", "id", "name", "value", "form", "aria_label", "labelled_by", "described_by", "disabled", "read_only", "invalid", "required", "size", "variant", "color", "high_contrast", "class", "radius", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("radio.Root", Subcomponent, "rustify_components::radio::Root", "crates/rustify-components/src/radio.rs", Yes, DOM_ONLY, ["checked", "on_change", "id", "name", "value", "form", "aria_label", "labelled_by", "described_by", "disabled", "read_only", "invalid", "required", "size", "variant", "color", "high_contrast", "class", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("checkbox_group.Root", Subcomponent, "rustify_components::checkbox_group::Root", "crates/rustify-components/src/checkbox_group.rs", Yes, DOM_ONLY, ["value", "on_change", "name", "form", "aria_label", "described_by", "disabled", "read_only", "invalid", "required", "orientation", "r#loop", "size", "variant", "color", "high_contrast", "class", "test_id", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("radio_group.Root", Subcomponent, "rustify_components::radio_group::Root", "crates/rustify-components/src/radio/group.rs", Yes, DOM_ONLY, ["value", "on_change", "name", "form", "aria_label", "described_by", "disabled", "read_only", "invalid", "required", "orientation", "r#loop", "size", "variant", "color", "high_contrast", "class", "test_id", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("checkbox_cards.Root", Subcomponent, "rustify_components::checkbox_cards::Root", "crates/rustify-components/src/checkbox_cards.rs", Yes, DOM_ONLY, ["value", "on_change", "name", "form", "aria_label", "described_by", "disabled", "read_only", "invalid", "required", "size", "variant", "color", "high_contrast", "columns", "gap", "class", "test_id", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("radio_cards.Root", Subcomponent, "rustify_components::radio_cards::Root", "crates/rustify-components/src/radio_cards.rs", Yes, DOM_ONLY, ["value", "on_change", "name", "form", "aria_label", "described_by", "disabled", "read_only", "invalid", "required", "size", "variant", "color", "high_contrast", "columns", "gap", "class", "test_id", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("toggle_group.Root", Subcomponent, "rustify_components::toggle_group::Root", "crates/rustify-components/src/toggle_group.rs", Yes, DOM_ONLY, ["value", "on_change", "mode", "allow_empty", "name", "form", "disabled", "read_only", "invalid", "required", "described_by", "aria_label", "orientation", "r#loop", "toolbar", "class", "test_id", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("text_field.Root", Subcomponent, "rustify_components::text_field::Root", "crates/rustify-components/src/input.rs", Yes, DOM_ONLY, ["value", "on_change", "kind", "id", "name", "form", "aria_label", "labelled_by", "described_by", "placeholder", "autocomplete", "inputmode", "required", "disabled", "read_only", "invalid", "pattern", "minlength", "maxlength", "size", "variant", "color", "radius", "class", "input_class", "test_id", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("text_area.Root", Subcomponent, "rustify_components::text_area::Root", "crates/rustify-components/src/textarea.rs", Yes, DOM_ONLY, ["value", "on_change", "rows", "id", "name", "form", "aria_label", "labelled_by", "described_by", "placeholder", "autocomplete", "inputmode", "required", "disabled", "read_only", "invalid", "pattern", "minlength", "maxlength", "size", "variant", "color", "radius", "resize", "class", "input_class", "test_id"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("number_field.Root", Subcomponent, "rustify_components::number_field::Root", "crates/rustify-components/src/number_field.rs", Yes, DOM_ONLY, ["value", "on_change", "on_preview", "on_cancel", "min", "max", "step", "id", "name", "form", "aria_label", "labelled_by", "described_by", "placeholder", "autocomplete", "inputmode", "required", "disabled", "read_only", "invalid", "pattern", "minlength", "maxlength", "size", "variant", "color", "radius", "class", "input_class", "test_id", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("color_field.Root", Subcomponent, "rustify_components::color_field::Root", "crates/rustify-components/src/color_field.rs", Yes, DOM_ONLY, ["value", "on_change", "on_preview", "on_cancel", "alpha", "id", "name", "form", "aria_label", "labelled_by", "described_by", "placeholder", "autocomplete", "inputmode", "required", "disabled", "read_only", "invalid", "pattern", "minlength", "maxlength", "size", "variant", "color", "radius", "class", "input_class", "test_id", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("slider.Root", Subcomponent, "rustify_components::slider::Root", "crates/rustify-components/src/slider.rs", Yes, DOM_ONLY, ["values", "on_change", "on_commit", "on_cancel", "min", "max", "step", "min_steps_between", "preserve_order", "orientation", "inverted", "id", "name", "form", "disabled", "read_only", "invalid", "described_by", "aria_label", "size", "variant", "color", "high_contrast", "radius", "class", "test_id", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("select.Root", Subcomponent, "rustify_components::select::Root", "crates/rustify-components/src/select.rs", Yes, DOM_ONLY, ["value", "on_change", "open", "on_open_change", "entries", "on_select", "on_escape_key_down", "on_pointer_down_outside", "on_open_auto_focus", "on_close_auto_focus", "id", "name", "form", "placeholder", "aria_label", "labelled_by", "described_by", "disabled", "read_only", "invalid", "required", "size", "class", "children"], [], "Controlled themed Root; native subparts share the unstyled implementation. Visual fields apply only to this declared DOM family.", FORM_CHECKS),
    component!("Select.Trigger", Subcomponent, "rustify_components::select::Trigger", "crates/rustify-components/src/select.rs", Yes, DOM_ONLY, ["variant", "color", "radius", "placeholder", "class", "test_id", "children"], [], "Independent themed Select part. Trigger has five variants and optional radius; Content has solid/soft and high_contrast. Other parts inherit Root size.", FORM_CHECKS),
    component!("Select.Content", Subcomponent, "rustify_components::select::Content", "crates/rustify-components/src/select.rs", Yes, DOM_ONLY, ["variant", "color", "high_contrast", "class", "test_id", "children"], [], "Independent themed Select part. Trigger has five variants and optional radius; Content has solid/soft and high_contrast. Other parts inherit Root size.", FORM_CHECKS),
    component!("Primitives", Helper, "rustify_components::primitives", "crates/rustify-components/src/primitives/mod.rs", Yes, DOM_ONLY, [], [], "Unstyled form/selection namespaces and structural PRIMITIVES_CSS. No automatic theme/style loading or complete reference-wide primitive inventory.", FORM_CHECKS),
    component!("Slot", Helper, "rustify_components::primitives::Slot", "crates/rustify-components/src/primitives/slot.rs", Yes, DOM_ONLY, ["target", "attrs"], [], "One typed native HtmlElement; preserves caller content/listeners. Fragments, erased component views and generic React asChild/ref forwarding are not accepted.", FORM_CHECKS),
    component!("DirectionProvider", Helper, "rustify_components::primitives::DirectionProvider", "crates/rustify-components/src/primitives/direction.rs", Yes, DOM_ONLY, ["direction", "children"], [], "Local controlled direction with its own Owner; sibling direction and the host page are not overwritten.", FORM_CHECKS),
    component!("Portal", Helper, "rustify_components::primitives::Portal", "crates/rustify-dom/src/overlay.rs", Yes, DOM_ONLY, ["client_only", "children"], [], "Scope-owned portal retains theme/direction and releases its wrapper. SSR requires client_only placeholder; not a global body portal.", FORM_CHECKS),
    component!("AccessibleIcon", Helper, "rustify_components::primitives::AccessibleIcon", "crates/rustify-components/src/primitives/accessible.rs", Yes, DOM_ONLY, ["label", "children"], [], "Named image or decoration around caller content; icon-only operations must name the native button separately.", FORM_CHECKS),
    component!("VisuallyHidden", Helper, "rustify_components::primitives::VisuallyHidden", "crates/rustify-components/src/primitives/accessible.rs", Yes, DOM_ONLY, ["children"], [], "Structural visual clipping retains assistive-technology content; does not load a theme.", FORM_CHECKS),
    component!("Primitives.Checkbox", Helper, "rustify_components::primitives::checkbox", "crates/rustify-components/src/primitives/choice.rs", Yes, DOM_ONLY, [], [], "Root/Control/Indicator; controlled CheckedState and native checked/required/name behavior.", FORM_CHECKS),
    component!("Primitives.Switch", Helper, "rustify_components::primitives::switch", "crates/rustify-components/src/primitives/choice.rs", Yes, DOM_ONLY, [], [], "Root/Control/Thumb; native controlled boolean and form participation.", FORM_CHECKS),
    component!("Primitives.Toggle", Helper, "rustify_components::primitives::toggle", "crates/rustify-components/src/primitives/choice.rs", Yes, DOM_ONLY, [], [], "Native Root button with controlled pressed state and accepted-value form mirror.", FORM_CHECKS),
    component!("Primitives.RadioGroup", Helper, "rustify_components::primitives::radio_group", "crates/rustify-components/src/primitives/choice.rs", Yes, DOM_ONLY, [], [], "Root/Item/Indicator with stable item identity, roving focus and actual native form ownership.", FORM_CHECKS),
    component!("Primitives.CheckboxGroup", Helper, "rustify_components::primitives::checkbox_group", "crates/rustify-components/src/primitives/choice.rs", Yes, DOM_ONLY, [], [], "Root/Item/Indicator with controlled declaration-order multi-selection.", FORM_CHECKS),
    component!("Primitives.ToggleGroup", Helper, "rustify_components::primitives::toggle_group", "crates/rustify-components/src/primitives/choice.rs", Yes, DOM_ONLY, [], [], "Root/Item with single/multiple and toolbar behavior; not a full Toolbar product.", FORM_CHECKS),
    component!("Primitives.Slider", Helper, "rustify_components::primitives::slider", "crates/rustify-components/src/primitives/slider.rs", Yes, DOM_ONLY, [], [], "Root/Track/Range/Thumb with stable ids, pointer/keyboard completion and normalized accepted-value mirrors; application CSS supplies visual dimensions.", FORM_CHECKS),
    component!("Primitives.TextField", Helper, "rustify_components::primitives::text_field", "crates/rustify-components/src/primitives/field.rs", Yes, DOM_ONLY, [], [], "Root/Input/Slot with native text completion, composition and form metadata.", FORM_CHECKS),
    component!("Primitives.TextArea", Helper, "rustify_components::primitives::text_area", "crates/rustify-components/src/primitives/field.rs", Yes, DOM_ONLY, [], [], "Native textarea Root with controlled accepted text and composition completion.", FORM_CHECKS),
    component!("Primitives.NumberField", Helper, "rustify_components::primitives::number_field", "crates/rustify-components/src/primitives/field.rs", Yes, DOM_ONLY, [], [], "Root/Input with numeric Draft and accepted-value mirror.", FORM_CHECKS),
    component!("Primitives.ColorField", Helper, "rustify_components::primitives::color_field", "crates/rustify-components/src/primitives/field.rs", Yes, DOM_ONLY, [], [], "Root/Input/Swatch with hexadecimal Draft and accepted-value mirror.", FORM_CHECKS),
    component!("Primitives.Select", Helper, "rustify_components::primitives::select", "crates/rustify-components/src/primitives/select.rs", Yes, DOM_ONLY, [], [], "Root, Trigger/Value/Icon, Content/Viewport, Group/Label, Item/ItemText/ItemIndicator, Separator and scroll buttons. Explicit metadata, typeahead and cancellable lifecycle hooks.", FORM_CHECKS),
    component!("Primitives.Form", Helper, "rustify_components::primitives::form", "crates/rustify-components/src/primitives/form.rs", Yes, DOM_ONLY, [], [], "Root/Field/Label/Description/Message with native constraints and shared Form owner; no duplicate business values.", FORM_CHECKS),
    component!("CheckedState", Data, "rustify_components::primitives::CheckedState", "crates/rustify-components/src/primitives/choice.rs", Yes, DOM_ONLY, [], [], "Unchecked, Checked, Indeterminate; only accepted checked state participates as a checked native value.", FORM_CHECKS),
    component!("SelectEntry", Data, "rustify_components::SelectEntry", "crates/rustify-components/src/primitives/select.rs", Yes, DOM_ONLY, ["value", "text_value", "disabled"], [], "Stable nonempty identity, metadata search/display label and native option state; independent of Item layout.", FORM_CHECKS),
    component!("ThumbValue", Data, "rustify_components::slider::ThumbValue", "crates/rustify-components/src/primitives/slider/model.rs", Yes, DOM_ONLY, ["id", "value"], [], "Stable business thumb identity and accepted numeric value.", FORM_CHECKS),
    component!("ThumbLabel", Data, "rustify_components::slider::ThumbLabel", "crates/rustify-components/src/primitives/slider/model.rs", Yes, DOM_ONLY, ["id", "label"], [], "Accessible name for one stable thumb in RangeSlider convenience assembly.", FORM_CHECKS),
];

/// Complete stable runtime names from the fixed Themes and Primitives public barrels.
pub const STABLE_REFERENCES: &[ReferenceEntry] = &[
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "AccessibleIcon", component: Some("AccessibleIcon"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "AlertDialog", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "AspectRatio", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Avatar", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Badge", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Blockquote", component: Some("Blockquote"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Box", component: Some("Box"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Button", component: Some("Button"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Callout", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Card", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "CheckboxCards", component: Some("CheckboxCards"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "CheckboxGroup", component: Some("CheckboxGroup"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Checkbox", component: Some("Checkbox"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Code", component: Some("Code"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Container", component: Some("Container"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "ContextMenu", component: Some("Menu"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "DataList", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Dialog", component: Some("Dialog"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "DropdownMenu", component: Some("Menu"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Em", component: Some("Em"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Flex", component: Some("Flex"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Grid", component: Some("Grid"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Heading", component: Some("Heading"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "HoverCard", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "IconButton", component: Some("IconButton"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "ChevronDownIcon", component: Some("Icon"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "ThickCheckIcon", component: Some("Icon"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "ThickChevronRightIcon", component: Some("Icon"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "ThickDividerHorizontalIcon", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Inset", component: Some("Inset"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Kbd", component: Some("Kbd"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Link", component: Some("Link"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Popover", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Portal", component: Some("Portal"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Progress", component: Some("Progress"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Quote", component: Some("Quote"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "RadioCards", component: Some("RadioCards"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "RadioGroup", component: Some("RadioGroup"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Radio", component: Some("Radio"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Reset", component: Some("Reset"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "ScrollArea", component: Some("ScrollArea"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "SegmentedControl", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Section", component: Some("Section"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Select", component: Some("Select"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Separator", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Skeleton", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Slider", component: Some("Slider"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Slot", component: Some("Slot"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Slottable", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Spinner", component: Some("Spinner"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Strong", component: Some("Strong"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Switch", component: Some("Switch"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "TabNav", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Table", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Tabs", component: Some("Tabs"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "TextArea", component: Some("TextArea"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "TextField", component: Some("TextField"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Text", component: Some("Text"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "ThemePanel", component: Some("ThemePanel"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Theme", component: Some("Theme"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "ThemeContext", component: Some("ThemeContext"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "useThemeContext", component: Some("ThemeContext"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "Tooltip", component: Some("Tooltip"), support: Support::Partial, limitation: "Mapped Rust entry supports its listed fields; generic asChild/Slot and exact React export/Props compatibility are not provided." },
    ReferenceEntry { library: ReferenceLibrary::Themes, name: "VisuallyHidden", component: Some("VisuallyHidden"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "AccessibleIcon", component: Some("AccessibleIcon"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Accordion", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "AlertDialog", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "AspectRatio", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Avatar", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Checkbox", component: Some("Primitives.Checkbox"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Collapsible", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "ContextMenu", component: Some("Menu"), support: Support::Partial, limitation: "Existing styled Rustify behavior is only a partial correspondence; no complete unstyled primitive or public parts contract." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Dialog", component: Some("Dialog"), support: Support::Partial, limitation: "Existing styled Rustify behavior is only a partial correspondence; no complete unstyled primitive or public parts contract." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Direction", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "DropdownMenu", component: Some("Menu"), support: Support::Partial, limitation: "Existing styled Rustify behavior is only a partial correspondence; no complete unstyled primitive or public parts contract." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Form", component: Some("Primitives.Form"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "HoverCard", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Label", component: Some("Label"), support: Support::Partial, limitation: "Existing styled Rustify behavior is only a partial correspondence; no complete unstyled primitive or public parts contract." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Menubar", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "NavigationMenu", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Popover", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Portal", component: Some("Portal"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Progress", component: Some("Progress"), support: Support::Partial, limitation: "Existing styled Rustify behavior is only a partial correspondence; no complete unstyled primitive or public parts contract." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "RadioGroup", component: Some("Primitives.RadioGroup"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "ScrollArea", component: Some("ScrollArea"), support: Support::Partial, limitation: "Existing styled Rustify behavior is only a partial correspondence; no complete unstyled primitive or public parts contract." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Select", component: Some("Primitives.Select"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Separator", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Slider", component: Some("Primitives.Slider"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Slot", component: Some("Slot"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Switch", component: Some("Primitives.Switch"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Tabs", component: Some("Tabs"), support: Support::Partial, limitation: "Existing styled Rustify behavior is only a partial correspondence; no complete unstyled primitive or public parts contract." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Toast", component: Some("Toast"), support: Support::Partial, limitation: "Existing styled Rustify behavior is only a partial correspondence; no complete unstyled primitive or public parts contract." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Toggle", component: Some("Primitives.Toggle"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "ToggleGroup", component: Some("Primitives.ToggleGroup"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Toolbar", component: None, support: Support::No, limitation: "No standalone public Rust counterpart; scope internals, examples and similarly named extensions do not implement this reference API." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "Tooltip", component: Some("Tooltip"), support: Support::Partial, limitation: "Existing styled Rustify behavior is only a partial correspondence; no complete unstyled primitive or public parts contract." },
    ReferenceEntry { library: ReferenceLibrary::Primitives, name: "VisuallyHidden", component: Some("VisuallyHidden"), support: Support::Partial, limitation: "Controlled native Rust parts and the listed visual subset are available; generic React props/ref/asChild compatibility is not supplied." },
];

/// Explicit stable Props/type exports at the Themes barrel, excluding namespace-owned types.
/// Rust equivalents use component-specific Leptos props and public field/enumeration types.
pub const STABLE_THEME_PROP_TYPES: &[&str] = &[
    "AccessibleIconProps",
    "AspectRatioProps",
    "AvatarProps",
    "BadgeProps",
    "BlockquoteProps",
    "BoxProps",
    "ButtonProps",
    "CardProps",
    "CheckboxProps",
    "CodeProps",
    "ContainerProps",
    "EmProps",
    "FlexProps",
    "GridProps",
    "HeadingProps",
    "IconButtonProps",
    "IconProps",
    "InsetProps",
    "KbdProps",
    "LinkProps",
    "PortalProps",
    "ProgressProps",
    "QuoteProps",
    "RadioProps",
    "ResetProps",
    "ScrollAreaProps",
    "SectionProps",
    "SeparatorProps",
    "SkeletonProps",
    "SliderProps",
    "SpinnerProps",
    "StrongProps",
    "SwitchProps",
    "TextAreaProps",
    "TextProps",
    "ThemePanelProps",
    "ThemeProps",
    "TooltipProps",
    "VisuallyHiddenProps",
];

/// Non-stable public and internal candidates; no item is an implementation claim.
pub const CANDIDATES: &[Candidate] = &[
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "unstable_OneTimePasswordField",
        source: "ref/primitives-main/packages/react/radix-ui/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "unstable_PasswordToggleField",
        source: "ref/primitives-main/packages/react/radix-ui/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Themes,
        name: "unstable_ThemePanelContent",
        source: "ref/themes-main/packages/radix-ui-themes/src/components/index.tsx",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Themes,
        name: "unstable_ThemePanelContentProps",
        source: "ref/themes-main/packages/radix-ui-themes/src/components/index.tsx",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "checkbox.unstable_CheckboxProvider",
        source: "ref/primitives-main/packages/react/checkbox/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "checkbox.unstable_CheckboxTrigger",
        source: "ref/primitives-main/packages/react/checkbox/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "checkbox.unstable_CheckboxBubbleInput",
        source: "ref/primitives-main/packages/react/checkbox/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "checkbox.unstable_Provider",
        source: "ref/primitives-main/packages/react/checkbox/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "checkbox.unstable_Trigger",
        source: "ref/primitives-main/packages/react/checkbox/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "checkbox.unstable_BubbleInput",
        source: "ref/primitives-main/packages/react/checkbox/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "checkbox.unstable_CheckboxProviderProps",
        source: "ref/primitives-main/packages/react/checkbox/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "checkbox.unstable_CheckboxTriggerProps",
        source: "ref/primitives-main/packages/react/checkbox/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "checkbox.unstable_CheckboxBubbleInputProps",
        source: "ref/primitives-main/packages/react/checkbox/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "radio-group.unstable_RadioGroupItemProvider",
        source: "ref/primitives-main/packages/react/radio-group/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "radio-group.unstable_RadioGroupItemTrigger",
        source: "ref/primitives-main/packages/react/radio-group/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "radio-group.unstable_RadioGroupItemBubbleInput",
        source: "ref/primitives-main/packages/react/radio-group/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "radio-group.unstable_ItemProvider",
        source: "ref/primitives-main/packages/react/radio-group/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "radio-group.unstable_ItemTrigger",
        source: "ref/primitives-main/packages/react/radio-group/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "radio-group.unstable_ItemBubbleInput",
        source: "ref/primitives-main/packages/react/radio-group/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "radio-group.unstable_RadioGroupItemProviderProps",
        source: "ref/primitives-main/packages/react/radio-group/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "radio-group.unstable_RadioGroupItemTriggerProps",
        source: "ref/primitives-main/packages/react/radio-group/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "radio-group.unstable_RadioGroupItemBubbleInputProps",
        source: "ref/primitives-main/packages/react/radio-group/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "switch.unstable_SwitchProvider",
        source: "ref/primitives-main/packages/react/switch/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "switch.unstable_SwitchTrigger",
        source: "ref/primitives-main/packages/react/switch/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "switch.unstable_SwitchBubbleInput",
        source: "ref/primitives-main/packages/react/switch/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "switch.unstable_Provider",
        source: "ref/primitives-main/packages/react/switch/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "switch.unstable_Trigger",
        source: "ref/primitives-main/packages/react/switch/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "switch.unstable_BubbleInput",
        source: "ref/primitives-main/packages/react/switch/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "switch.unstable_SwitchProviderProps",
        source: "ref/primitives-main/packages/react/switch/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "switch.unstable_SwitchTriggerProps",
        source: "ref/primitives-main/packages/react/switch/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "switch.unstable_SwitchBubbleInputProps",
        source: "ref/primitives-main/packages/react/switch/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "select.unstable_SelectProvider",
        source: "ref/primitives-main/packages/react/select/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "select.unstable_SelectBubbleInput",
        source: "ref/primitives-main/packages/react/select/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "select.unstable_Provider",
        source: "ref/primitives-main/packages/react/select/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "select.unstable_BubbleInput",
        source: "ref/primitives-main/packages/react/select/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "select.unstable_SelectProviderProps",
        source: "ref/primitives-main/packages/react/select/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "select.unstable_SelectBubbleInputProps",
        source: "ref/primitives-main/packages/react/select/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "slider.unstable_SliderThumbProvider",
        source: "ref/primitives-main/packages/react/slider/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "slider.unstable_SliderThumbTrigger",
        source: "ref/primitives-main/packages/react/slider/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "slider.unstable_SliderBubbleInput",
        source: "ref/primitives-main/packages/react/slider/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "slider.unstable_ThumbProvider",
        source: "ref/primitives-main/packages/react/slider/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "slider.unstable_ThumbTrigger",
        source: "ref/primitives-main/packages/react/slider/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "slider.unstable_BubbleInput",
        source: "ref/primitives-main/packages/react/slider/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "slider.unstable_SliderThumbProviderProps",
        source: "ref/primitives-main/packages/react/slider/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "slider.unstable_SliderThumbTriggerProps",
        source: "ref/primitives-main/packages/react/slider/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "slider.unstable_SliderBubbleInputProps",
        source: "ref/primitives-main/packages/react/slider/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "collection.unstable_createCollection",
        source: "ref/primitives-main/packages/react/collection/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "collection.unstable_CollectionSlotProps",
        source: "ref/primitives-main/packages/react/collection/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
    Candidate {
        library: ReferenceLibrary::Primitives,
        name: "collection.unstable_CollectionDict",
        source: "ref/primitives-main/packages/react/collection/src/index.ts",
        limitation: "Candidate only; no stable Rust API or implementation commitment.",
    },
];

/// Behavioral and composition boundaries, separate from component name matching.
pub const SUB_CAPABILITIES: &[SubCapability] = &[
    SubCapability { name: "Responsive visual fields", component: "Responsive", support: Support::Yes, public_path: Some("rustify_components::Responsive"), limitation: "Only fields explicitly declared responsive by their component. Fixed Themes breakpoints; an omitted initial value retains component defaults.", verification: LAYOUT_CHECKS },
    SubCapability { name: "Geometry validation and diagnostics", component: "GeometryStyles", support: Support::Yes, public_path: Some("rustify_components::GeometryStyles"), limitation: "SSR/CSR share validation; invalid candidates fall back to defaults and report a field. Declarations, rules, URL and import injection are rejected.", verification: &["crates/rustify-components/src/geometry.rs", "tests/browser/dom-layout.spec.ts"] },
    SubCapability { name: "Single native-element reset", component: "Reset", support: Support::Yes, public_path: Some("rustify_components::Reset"), limitation: "One typed HtmlElement; fragments, tuples and arbitrary component views are rejected. This is not generic Slot/asChild composition.", verification: LAYOUT_CHECKS },
    SubCapability { name: "Component-local visual palette", component: "Button / IconButton / Link / Text / Heading / Blockquote / Code", support: Support::Yes, public_path: Some("rustify_components::theme::AccentColor"), limitation: "Optional named palette roles stay local; unset color consumes semantic roles, including explicit author overrides. No automatic arbitrary-HEX palette generator.", verification: BUTTON_CHECKS },
    SubCapability { name: "Root and nested source-backed theme", component: "Theme / ThemeBoundary", support: Support::Yes, public_path: Some("rustify_components::theme"), limitation: "Browser root owns one mount scope; boundaries retain two-mode source and controlled local patches. Static consumers use provide_static_theme and external CSS.", verification: THEME_CHECKS },
    SubCapability { name: "Controlled configuration panel", component: "ThemePanel", support: Support::Yes, public_path: Some("rustify_components::ThemePanel"), limitation: "Six configuration controls and two-mode group-reset requests; application owns the document and accepts or refuses requests.", verification: &["crates/rustify-components/src/theme_panel.rs", "tests/browser/dom-buttons.spec.ts"] },
    SubCapability { name: "Generic React asChild / Slottable", component: "All components", support: Support::No, public_path: None, limitation: "Generic React child cloning/ref composition and Slottable are absent. Typed single-native-element Slot is separately supported.", verification: &[] },
    SubCapability { name: "Complete reference-wide Primitives", component: "All reference primitives", support: Support::No, public_path: None, limitation: "The supplied form/selection namespaces are unstyled. Other content, navigation and arbitrary overlay families are not implied.", verification: &[] },
    SubCapability { name: "Three-state checkbox", component: "Checkbox", support: Support::Yes, public_path: Some("rustify_components::primitives::checkbox"), limitation: "Root/Control/Indicator and bool convenience indeterminate projection; accepted checked values alone enter native FormData.", verification: FORM_CHECKS },
    SubCapability { name: "Selection groups and cards", component: "CheckboxGroup / CheckboxCards / RadioCards / Radio", support: Support::Yes, public_path: Some("rustify_components::checkbox_cards"), limitation: "Controlled native groups and themed cards with declaration-order identity, readonly gates and independent anonymous names.", verification: FORM_CHECKS },
    SubCapability { name: "Multiple slider thumbs", component: "Slider / RangeSlider", support: Support::Yes, public_path: Some("rustify_components::slider"), limitation: "Stable named thumbs, normalized values, pointer/keyboard preview and one commit or cancel; repeated accepted-value native name mirrors.", verification: FORM_CHECKS },
    SubCapability { name: "Dialog / menu parts", component: "Dialog / Menu", support: Support::No, public_path: None, limitation: "Existing controlled scoped APIs remain; no new arbitrary dialog/menu Root/Content reference part families. Select parts are separately supported.", verification: &[] },
    SubCapability { name: "Form validation and field binding", component: "Form / FormRoot / Field / SubmitButton", support: Support::Yes, public_path: Some("rustify_components::form"), limitation: "Native constraints, Draft/composition completion, source-specific errors and reset generations; application owns values and actual save completion.", verification: FORM_CHECKS },
    SubCapability { name: "Command menus and toolbar groups", component: "Menu / ToggleGroup", support: Support::Partial, public_path: Some("rustify_components::Menu"), limitation: "Controlled command/menu data and toggle-toolbar behavior; no ContextMenu, Menubar, NavigationMenu or complete Toolbar API.", verification: LEGACY_CHECKS },
    SubCapability { name: "Accessible icon wrapper", component: "Icon / AccessibleIcon", support: Support::Yes, public_path: Some("rustify_components::primitives::AccessibleIcon"), limitation: "Caller content receives a name or decorative semantics. Native icon-only buttons still need their own accessible name.", verification: FORM_CHECKS },
    SubCapability { name: "GPU visual parity", component: "DOM visual additions", support: Support::No, public_path: None, limitation: "New layouts, typography, component-local palettes and visual option scales have no matching region component catalogue; shared theme snapshots do not require parity.", verification: &[] },
    SubCapability { name: "Typed native-element composition", component: "Slot / control_as / trigger_as / item_as", support: Support::Yes, public_path: Some("rustify_components::primitives::Slot"), limitation: "One supported native target; caller classes/listeners retained and caller cancellation precedes owning Root defaults. Reserved semantics belong to the part.", verification: FORM_CHECKS },
    SubCapability { name: "Unstyled form and selection parts", component: "Primitives", support: Support::Yes, public_path: Some("rustify_components::primitives"), limitation: "Separate structural CSS only; native controls, keyboard/focus and controlled acceptance share behavior with Themes. No default palette/font/elevation.", verification: FORM_CHECKS },
    SubCapability { name: "Select composition and lifecycle", component: "Select", support: Support::Yes, public_path: Some("rustify_components::select"), limitation: "Explicit entries, independent rich/grouped content, 1000ms typeahead, accepted-value mirror, cancellable selection/dismissal/autofocus and actual RTL placement. Trigger keeps focus; SSR starts closed.", verification: FORM_CHECKS },
    SubCapability { name: "Field slots and native input completion", component: "TextField / TextArea / NumberField / ColorField", support: Support::Yes, public_path: Some("rustify_components::text_field"), limitation: "Explicit Root/Input/Slot, independent slot colour and responsive gap/px/pl/pr. Native text completion and focused Draft follow one form registration.", verification: FORM_CHECKS },
];

/// Looks up a current Rust entry without assuming that a category names one.
pub fn component(name: &str) -> Option<&'static ComponentEntry> {
    COMPONENTS.iter().find(|entry| entry.name == name)
}

fn cell(value: &str) -> String {
    value.replace('|', "\\|").replace(['\n', '\r'], " ")
}

fn codes(values: &[&str]) -> String {
    if values.is_empty() {
        return "—".into();
    }
    values
        .iter()
        .map(|value| format!("`{}`", cell(value)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Complete generated documentation; the legacy [`markdown`] table remains available.
pub fn catalogue_markdown() -> String {
    let mut out = String::new();
    let components = COMPONENTS
        .iter()
        .filter(|entry| entry.kind == ComponentKind::Component)
        .count();
    let subcomponents = COMPONENTS
        .iter()
        .filter(|entry| entry.kind == ComponentKind::Subcomponent)
        .count();
    let helpers = COMPONENTS
        .iter()
        .filter(|entry| entry.kind == ComponentKind::Helper)
        .count();
    let data = COMPONENTS
        .iter()
        .filter(|entry| entry.kind == ComponentKind::Data)
        .count();
    let _ = writeln!(out, "## Public source entries\n\n{} entries: {components} components, {subcomponents} subcomponent, {helpers} helper groups and {data} data surface. These are source APIs, not a count of all exported Rust symbols. The legacy application navigation has {} categories.\n", COMPONENTS.len(), CATALOG.len());
    out.push_str("`yes` describes the listed Rust fields and limits; `partial` describes an available subset. Verification paths name checks and consumers to rerun, not a permanent browser/accessibility certification. GPU columns describe existing presentation subsets, not DOM/GPU API parity.\n\n");
    out.push_str("| entry | kind / support | public path | DOM / GPU / across | fields | groups | limits | source / verification |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for entry in COMPONENTS {
        let _ = writeln!(
            out,
            "| {} | {} / {} | `{}` | {} / {} / {} | {} | {} | {} | `{}`; {} |",
            cell(entry.name),
            entry.kind.name(),
            entry.support.name(),
            entry.public_path,
            entry.presentation.dom.name(),
            entry.presentation.gpu.name(),
            entry.presentation.across_regions.name(),
            codes(entry.fields),
            codes(entry.property_groups),
            cell(entry.limitation),
            entry.source,
            codes(entry.verification)
        );
    }
    out.push_str("\n## Shared property groups\n\nOnly entries that name a group expose it. `Layout.padding` uses the separately enumerated Padding fields. Component-specific enums and defaults are documented in the [parameter matrix](validation/dom-components-p1/field-matrix.md).\n\n| group | fields | contract | verification |\n| --- | --- | --- | --- |\n");
    for group in PROPERTY_GROUPS {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} |",
            group.name,
            codes(group.fields),
            cell(group.contract),
            codes(group.verification)
        );
    }
    out.push_str("\n## Stable reference inventory\n\nThese are the fixed local snapshot exports. A partial correspondence is not full upstream compatibility; composition and behavior gaps are listed separately below. Reference files identify provenance and are not build inputs.\n\n");
    for library in ReferenceLibrary::ALL {
        let count = STABLE_REFERENCES
            .iter()
            .filter(|entry| entry.library == library)
            .count();
        let _ = writeln!(
            out,
            "- {}: {count} stable runtime export names from `{}`.",
            library.name(),
            library.source()
        );
    }
    out.push_str("\n| reference | runtime name | mapped Rust entry | correspondence | limits |\n| --- | --- | --- | --- | --- |\n");
    for entry in STABLE_REFERENCES {
        let mapped = entry
            .component
            .and_then(component)
            .map(|entry| entry.public_path)
            .unwrap_or("—");
        let _ = writeln!(
            out,
            "| {} | `{}` | `{mapped}` | {} | {} |",
            entry.library.name(),
            entry.name,
            entry.support.name(),
            cell(entry.limitation)
        );
    }
    let _ = writeln!(out, "\n### Direct Themes Props/type exports\n\nThe public barrel also directly exports {} stable type names (namespace-owned part types are outside this direct-export count):\n\n{}\n\nRust counterparts use component-specific Leptos props and Rust enums/structures. This list records reference names; it does not promise identical React types, refs or generated `Props` names. In particular Reset is a typed element decorator, and root Theme aliases the existing browser ThemeScope.\n", STABLE_THEME_PROP_TYPES.len(), codes(STABLE_THEME_PROP_TYPES));
    out.push_str("## Sub-capability limits\n\n| capability | applicable entries | support | public path | limits | verification |\n| --- | --- | --- | --- | --- | --- |\n");
    for entry in SUB_CAPABILITIES {
        let _ = writeln!(
            out,
            "| {} | {} | {} | `{}` | {} | {} |",
            cell(entry.name),
            cell(entry.component),
            entry.support.name(),
            entry.public_path.unwrap_or("—"),
            cell(entry.limitation),
            codes(entry.verification)
        );
    }
    let _ = writeln!(out, "\n## Unstable candidates\n\n{} qualified public/internal names below are candidates only. They are excluded from stable-reference and implementation counts; no candidate has a public Rust implementation commitment. Internal aliases are qualified by their package.\n\n| reference | candidate | source | limit |\n| --- | --- | --- | --- |", CANDIDATES.len());
    for entry in CANDIDATES {
        let _ = writeln!(
            out,
            "| {} | `{}` | `{}` | {} |",
            entry.library.name(),
            entry.name,
            entry.source,
            entry.limitation
        );
    }
    out.push_str("\n## Legacy application categories\n\n`CATALOG` retains its existing 24-category navigation and six capability dimensions. It does not enumerate all current components, public symbols or reference APIs.\n\n");
    out.push_str(&markdown());
    out
}

#[cfg(test)]
mod inventory_tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn frozen_reference_inventories_are_complete_unique_and_separate_from_candidates() {
        let keys = STABLE_REFERENCES
            .iter()
            .map(|entry| (entry.library.name(), entry.name))
            .collect::<BTreeSet<_>>();
        assert_eq!(keys.len(), STABLE_REFERENCES.len());
        assert_eq!(
            STABLE_REFERENCES
                .iter()
                .filter(|entry| entry.library == ReferenceLibrary::Themes)
                .count(),
            64
        );
        assert_eq!(
            STABLE_REFERENCES
                .iter()
                .filter(|entry| entry.library == ReferenceLibrary::Primitives)
                .count(),
            33
        );
        assert_eq!(STABLE_THEME_PROP_TYPES.len(), 39);
        assert_eq!(
            STABLE_THEME_PROP_TYPES
                .iter()
                .collect::<BTreeSet<_>>()
                .len(),
            STABLE_THEME_PROP_TYPES.len()
        );
        for entry in STABLE_REFERENCES {
            assert!(!entry.name.starts_with("unstable_"));
            if let Some(name) = entry.component {
                assert!(
                    component(name).is_some(),
                    "{} maps to missing {name}",
                    entry.name
                );
            } else {
                assert_eq!(entry.support, Support::No);
            }
            assert!(!entry.limitation.is_empty());
        }
        let candidates = CANDIDATES
            .iter()
            .map(|entry| (entry.library.name(), entry.name))
            .collect::<BTreeSet<_>>();
        assert_eq!(candidates.len(), CANDIDATES.len());
        assert!(keys.is_disjoint(&candidates));
        assert!(CANDIDATES
            .iter()
            .all(|entry| entry.name.contains("unstable_") && !entry.source.is_empty()));
    }

    #[test]
    fn public_entries_and_property_groups_have_no_dangling_or_unsupported_claims() {
        assert_eq!(
            COMPONENTS
                .iter()
                .map(|entry| entry.name)
                .collect::<BTreeSet<_>>()
                .len(),
            COMPONENTS.len()
        );
        for entry in COMPONENTS {
            assert!(
                !entry.public_path.is_empty()
                    && !entry.source.is_empty()
                    && !entry.limitation.is_empty(),
                "{}",
                entry.name
            );
            assert_ne!(
                entry.support,
                Support::No,
                "{} should be a reference gap, not a public entry",
                entry.name
            );
            assert!(
                !entry.verification.is_empty(),
                "{} lacks a consumer/check",
                entry.name
            );
            for group in entry.property_groups {
                assert!(
                    PROPERTY_GROUPS
                        .iter()
                        .any(|candidate| candidate.name == *group),
                    "{} names unknown group {group}",
                    entry.name
                );
            }
        }
        for entry in SUB_CAPABILITIES {
            assert!(!entry.limitation.is_empty());
            if entry.support == Support::No {
                assert!(
                    entry.public_path.is_none(),
                    "{} claims an unimplemented path",
                    entry.name
                );
            } else {
                assert!(
                    entry.public_path.is_some() && !entry.verification.is_empty(),
                    "{} lacks a public path/check",
                    entry.name
                );
            }
        }
    }

    #[test]
    fn layout_field_inventory_compiles_against_the_complete_public_structures() {
        use crate::layout::*;
        let Layout {
            padding: _,
            width: _,
            min_width: _,
            max_width: _,
            height: _,
            min_height: _,
            max_height: _,
            position: _,
            inset: _,
            top: _,
            right: _,
            bottom: _,
            left: _,
            overflow: _,
            overflow_x: _,
            overflow_y: _,
            flex_basis: _,
            flex_grow: _,
            flex_shrink: _,
            grid_area: _,
            grid_column: _,
            grid_column_start: _,
            grid_column_end: _,
            grid_row: _,
            grid_row_start: _,
            grid_row_end: _,
            align_self: _,
            justify_self: _,
        } = Layout::default();
        let Margin {
            m: _,
            mx: _,
            my: _,
            mt: _,
            mr: _,
            mb: _,
            ml: _,
        } = Margin::default();
        let Padding {
            p: _,
            px: _,
            py: _,
            pt: _,
            pr: _,
            pb: _,
            pl: _,
        } = Padding::default();
        let Gap {
            gap: _,
            gap_x: _,
            gap_y: _,
        } = Gap::default();
        let InsetPadding {
            p: _,
            px: _,
            py: _,
            pt: _,
            pr: _,
            pb: _,
            pl: _,
        } = InsetPadding::default();
        assert_eq!(
            PROPERTY_GROUPS
                .iter()
                .find(|group| group.name == "Layout")
                .unwrap()
                .fields
                .len(),
            28
        );
        for name in ["Margin", "Padding", "InsetPadding"] {
            assert_eq!(
                PROPERTY_GROUPS
                    .iter()
                    .find(|group| group.name == name)
                    .unwrap()
                    .fields
                    .len(),
                7
            );
        }
        assert_eq!(
            PROPERTY_GROUPS
                .iter()
                .find(|group| group.name == "Gap")
                .unwrap()
                .fields,
            ["gap", "gap_x", "gap_y"]
        );
        assert_eq!(
            component("Em").unwrap().property_groups,
            ["InlineText", "Element"]
        );
        assert!(!component("Link")
            .unwrap()
            .property_groups
            .contains(&"Layout"));
    }

    #[test]
    fn generator_keeps_reference_and_runtime_counts_distinct_and_includes_legacy_table() {
        let document = catalogue_markdown();
        assert!(document.contains("64 stable runtime export names"));
        assert!(document.contains("33 stable runtime export names"));
        assert!(document.contains("39 stable type names"));
        assert!(document.contains(&markdown()));
        for name in [
            "asChild",
            "Multiple slider thumbs",
            "Skeleton",
            "DataTable",
            "ThemePanel",
            "unstable_OneTimePasswordField",
        ] {
            assert!(document.contains(name), "missing {name}");
        }
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn committed_source_and_verification_paths_exist_without_reading_reference_trees() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for entry in COMPONENTS {
            for path in std::iter::once(entry.source).chain(entry.verification.iter().copied()) {
                assert!(!path.starts_with("ref/"));
                assert!(
                    root.join(path).is_file(),
                    "{} has a missing path {path}",
                    entry.name
                );
            }
        }
        let document = std::fs::read_to_string(root.join("docs/components.md")).unwrap();
        assert!(
            document.contains(&catalogue_markdown()),
            "regenerate the capability document"
        );
    }

    #[test]
    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    fn newly_listed_component_paths_are_consumable_with_their_public_props() {
        use crate::*;
        use leptos::prelude::*;
        let owner = Owner::new();
        let config = rustify_ui::resolve(
            &crate::theme::ThemeDocument::standard("catalog-consumer", "Catalog consumer"),
            crate::theme::ThemeMode::Light,
            &rustify_ui::ResolveContext::default(),
        )
        .unwrap()
        .config;
        let html = owner.with(|| {
            view! {
                <Button on_click=|| {}>"action"</Button>
                <IconButton on_click=|| {} aria_label="Close"><Icon glyph=Glyph::Close/></IconButton>
                <Link href="#target">"native link"</Link>
                <Box><Flex><Grid><Container><Section><Inset>"layout"</Inset></Section></Container></Grid></Flex></Box>
                <Text>"text"</Text><Heading>"heading"</Heading><Blockquote>"quote"</Blockquote><Code>"code"</Code>
                <Kbd>"key"</Kbd><Em>"em"</Em><Quote>"q"</Quote><Strong>"strong"</Strong>
                {Reset(leptos::html::button().child("reset"))}
                <ThemePanel config=config overrides=ThemeOverrideCounts::default() on_change=|_| {} on_reset=|_| {}/>
            }.to_html()
        });
        for tag in [
            "button",
            "a",
            "div",
            "section",
            "span",
            "h1",
            "blockquote",
            "code",
            "kbd",
            "em",
            "q",
            "strong",
        ] {
            assert!(html.contains(&format!("<{tag}")), "missing native {tag}");
        }
    }
}
