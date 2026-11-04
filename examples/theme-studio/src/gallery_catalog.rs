#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Section {
    Interaction,
    Overlay,
    Menu,
    Navigation,
    Content,
    Feedback,
    Layout,
    Typography,
}
impl Section {
    pub const ALL: [Self; 8] = [
        Self::Interaction,
        Self::Overlay,
        Self::Menu,
        Self::Navigation,
        Self::Content,
        Self::Feedback,
        Self::Layout,
        Self::Typography,
    ];
    pub fn label(self) -> [&'static str; 2] {
        match self {
            Self::Interaction => ["Interaction", "交互"],
            Self::Overlay => ["Overlay", "浮层"],
            Self::Menu => ["Menu", "菜单"],
            Self::Navigation => ["Navigation", "导航"],
            Self::Content => ["Content", "内容"],
            Self::Feedback => ["Feedback", "反馈"],
            Self::Layout => ["Layout", "布局"],
            Self::Typography => ["Typography", "排版"],
        }
    }
}

pub(crate) struct GalleryEntry {
    pub references: &'static [&'static str],
    pub section: Section,
    pub parts: &'static [&'static str],
    pub primitives_path: Option<&'static str>,
}
macro_rules! entry {
    ([$($name:literal),+], $section:ident, [$($part:literal),*], $path:expr) => {
        GalleryEntry { references: &[$($name),+], section: Section::$section, parts: &[$($part),*], primitives_path: $path }
    };
}
pub const ENTRIES: &[GalleryEntry] = &[
    entry!(
        ["Button", "IconButton"],
        Interaction,
        ["Button", "IconButton → Icon"],
        None
    ),
    entry!(
        ["Checkbox"],
        Interaction,
        ["Root → Control + Indicator"],
        Some("primitives::checkbox")
    ),
    entry!(
        ["CheckboxGroup"],
        Interaction,
        ["Root → Item → Control + Indicator"],
        Some("primitives::checkbox_group")
    ),
    entry!(
        ["CheckboxCards"],
        Interaction,
        ["Root → Item → Control + Indicator + Text"],
        Some("primitives::checkbox_group")
    ),
    entry!(
        ["Radio", "RadioGroup"],
        Interaction,
        ["Root → Item → Control + Indicator"],
        Some("primitives::radio_group")
    ),
    entry!(
        ["RadioCards"],
        Interaction,
        ["Root → Item → Control + Indicator + Text"],
        Some("primitives::radio_group")
    ),
    entry!(
        ["Switch"],
        Interaction,
        ["Root → Control + Thumb"],
        Some("primitives::switch")
    ),
    entry!(
        ["Toggle"],
        Interaction,
        ["Root"],
        Some("primitives::toggle")
    ),
    entry!(
        ["ToggleGroup"],
        Interaction,
        ["Root → Item"],
        Some("primitives::toggle_group")
    ),
    entry!(
        ["SegmentedControl"],
        Interaction,
        ["Root → Item"],
        Some("primitives::radio_group")
    ),
    entry!(
        ["Slider"],
        Interaction,
        ["Root → Track → Range", "Root → Thumb"],
        Some("primitives::slider")
    ),
    entry!(
        ["Select"],
        Interaction,
        [
            "Root → Trigger → Value",
            "Portal → Content → Viewport → Item"
        ],
        Some("primitives::select")
    ),
    entry!(
        ["Form", "Label", "TextField", "TextArea"],
        Interaction,
        ["Root → Field → Label + Control + Description + Message"],
        Some("primitives::form")
    ),
    entry!(
        ["Dialog"],
        Overlay,
        [
            "Root → Trigger",
            "Portal → Overlay + Content → Title + Description + Close"
        ],
        Some("primitives::dialog")
    ),
    entry!(
        ["AlertDialog"],
        Overlay,
        [
            "Root → Trigger",
            "Portal → Overlay + Content → Title + Description + Cancel + Action"
        ],
        Some("primitives::alert_dialog")
    ),
    entry!(
        ["Popover"],
        Overlay,
        ["Root → Trigger", "Portal → Content → Arrow + Close"],
        Some("primitives::popover")
    ),
    entry!(
        ["HoverCard"],
        Overlay,
        ["Root → Trigger", "Portal → Content → Arrow"],
        Some("primitives::hover_card")
    ),
    entry!(
        ["Tooltip"],
        Overlay,
        ["Provider → Root → Trigger", "Portal → Content → Arrow"],
        Some("primitives::tooltip")
    ),
    entry!(
        ["DropdownMenu"],
        Menu,
        [
            "Root → Trigger",
            "Portal → Content → Item + CheckboxItem + Sub"
        ],
        Some("primitives::dropdown_menu")
    ),
    entry!(
        ["ContextMenu"],
        Menu,
        ["Root → Trigger", "Portal → Content → Item"],
        Some("primitives::context_menu")
    ),
    entry!(
        ["Menubar"],
        Menu,
        ["Root → Menu → Trigger", "Portal → Content → Item"],
        Some("primitives::menubar")
    ),
    entry!(
        ["NavigationMenu"],
        Navigation,
        [
            "Root → List → Item → Trigger + Content → Link",
            "Indicator + Viewport"
        ],
        Some("primitives::navigation_menu")
    ),
    entry!(
        ["Toolbar"],
        Navigation,
        ["Root → Button + Separator + ToggleGroup + Link"],
        Some("primitives::toolbar")
    ),
    entry!(
        ["Tabs"],
        Navigation,
        ["Root → List → Trigger", "Root → Content"],
        Some("primitives::tabs")
    ),
    entry!(["TabNav"], Navigation, ["Root → Link"], None),
    entry!(
        ["Accordion"],
        Content,
        ["Root → Item → Header → Trigger", "Item → Content"],
        Some("primitives::accordion")
    ),
    entry!(
        ["Collapsible"],
        Content,
        ["Root → Trigger + Content"],
        Some("primitives::collapsible")
    ),
    entry!(
        ["Avatar"],
        Content,
        ["Root → Image + Fallback"],
        Some("primitives::avatar")
    ),
    entry!(["Badge"], Content, ["Badge"], None),
    entry!(["Card"], Content, ["Card"], None),
    entry!(["Callout"], Content, ["Root → Icon + Text"], None),
    entry!(["DataList"], Content, ["Root → Item → Label + Value"], None),
    entry!(
        ["Table"],
        Content,
        [
            "Root → Caption + Header + Body",
            "Row → ColumnHeaderCell + Cell"
        ],
        None
    ),
    entry!(
        ["ScrollArea"],
        Content,
        ["Root → Viewport + Scrollbar → Thumb", "Corner"],
        Some("primitives::scroll_area")
    ),
    entry!(
        ["Progress"],
        Feedback,
        ["Root → Indicator"],
        Some("primitives::progress")
    ),
    entry!(["Skeleton"], Feedback, ["Skeleton"], None),
    entry!(["Spinner"], Feedback, ["Spinner"], None),
    entry!(
        ["Toast"],
        Feedback,
        [
            "Provider → Root → Title + Description + Action + Close",
            "Viewport"
        ],
        Some("primitives::toast")
    ),
    entry!(
        [
            "Box",
            "Flex",
            "Grid",
            "Container",
            "Section",
            "Inset",
            "Reset"
        ],
        Layout,
        [
            "Section → Container → Flex + Grid → Box",
            "Inset",
            "Reset(native element)"
        ],
        None
    ),
    entry!(
        ["AspectRatio"],
        Layout,
        ["AspectRatio"],
        Some("primitives::aspect_ratio")
    ),
    entry!(
        ["Separator"],
        Layout,
        ["Separator"],
        Some("primitives::separator")
    ),
    entry!(
        [
            "Text",
            "Heading",
            "Blockquote",
            "Code",
            "Kbd",
            "Em",
            "Quote",
            "Strong",
            "Link"
        ],
        Typography,
        [
            "Heading",
            "Text → Em + Strong + Quote + Code + Kbd + Link",
            "Blockquote"
        ],
        None
    ),
    entry!(
        ["AccessibleIcon", "VisuallyHidden"],
        Typography,
        ["AccessibleIcon → Icon", "VisuallyHidden"],
        Some("primitives::{accessible_icon, visually_hidden}")
    ),
];
#[cfg(test)]
pub const EXCLUDED: &[(&str, &str)] = &[
    ("Slot", "Native element composition helper"),
    ("Slottable", "Native element composition helper"),
    ("Direction", "Inherited direction context"),
    ("Portal", "Mounting helper used by overlay samples"),
    ("Theme", "The preview's theme provider"),
    ("ThemePanel", "Studio editor"),
    ("ThemeContext", "Theme context type"),
    ("useThemeContext", "Theme context accessor"),
    ("ChevronDownIcon", "Icon export"),
    ("ThickCheckIcon", "Icon export"),
    ("ThickChevronRightIcon", "Icon export"),
    ("ThickDividerHorizontalIcon", "Icon export"),
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_stable_reference_has_one_sample_or_an_explicit_nonvisual_exclusion() {
        for reference in rustify_components::catalog::STABLE_REFERENCES {
            let samples = ENTRIES
                .iter()
                .filter(|entry| entry.references.contains(&reference.name))
                .count();
            let exclusions = EXCLUDED
                .iter()
                .filter(|(name, reason)| *name == reference.name && !reason.is_empty())
                .count();
            assert_eq!(samples + exclusions, 1, "{}", reference.name);
        }
    }
}
