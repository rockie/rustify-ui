#![recursion_limit = "256"]

use rustify_docs::SiteRegistry;

pub mod demos;
mod pages;
pub mod showcase;

pub fn site() -> SiteRegistry {
    rustify_docs::site! {
        demos {
            ButtonDemo => crate::demos::basics::button_demo,
            IconButtonDemo => crate::demos::basics::icon_button_demo,
            LabelDemo => crate::demos::basics::label_demo,
            LinkDemo => crate::demos::basics::link_demo,
            IconDemo => crate::demos::basics::icon_demo,
            LoadingDemo => crate::demos::basics::loading_demo,
            TextFieldDemo => crate::demos::fields::text_field_demo,
            TextAreaDemo => crate::demos::fields::text_area_demo,
            CheckboxDemo => crate::demos::choices::checkbox_demo,
            RadioDemo => crate::demos::choices::radio_demo,
            SwitchDemo => crate::demos::choices::switch_demo,
            SelectDemo => crate::demos::choices::select_demo,
            SliderDemo => crate::demos::choices::slider_demo,
            ProgressDemo => crate::demos::feedback::progress_demo,
            TooltipDemo => crate::demos::feedback::tooltip_demo,
            MenuDemo => crate::demos::feedback::menu_demo,
            DialogDemo => crate::demos::feedback::dialog_demo,
            TabsDemo => crate::demos::collections::tabs_demo,
            ScrollAreaDemo => crate::demos::collections::scroll_area_demo,
            DataTableDemo => crate::demos::collections::data_table_demo,
            TreeDemo => crate::demos::collections::tree_demo,
            ToastDemo => crate::demos::feedback::toast_demo,
            NumberFieldDemo => crate::demos::fields::number_field_demo,
            ColorFieldDemo => crate::demos::fields::color_field_demo,
            ToggleGroupDemo => crate::demos::choices::toggle_group_demo,
            CheckboxGroupDemo => crate::demos::choices::checkbox_group_demo,
            RadioCardsDemo => crate::demos::choices::radio_cards_demo,
            ToggleDemo => crate::demos::choices::toggle_demo,
            SegmentedControlDemo => crate::demos::choices::segmented_control_demo,
            CheckboxCardsDemo => crate::demos::choices::checkbox_cards_demo,
            RangeSliderDemo => crate::demos::choices::range_slider_demo,
            FormDemo => crate::demos::composition::form_demo,
            FieldPartsDemo => crate::demos::composition::field_parts_demo,
            SelectPartsDemo => crate::demos::composition::select_parts_demo,
            SelectGroupedDemo => crate::demos::composition::select_grouped_demo,
            AlertDialogDemo => crate::demos::feedback::alert_dialog_demo,
            PopoverDemo => crate::demos::feedback::popover_demo,
            HoverCardDemo => crate::demos::feedback::hover_card_demo,
            SkeletonDemo => crate::demos::feedback::skeleton_demo,
            SpinnerDemo => crate::demos::feedback::spinner_demo,
            DropdownMenuDemo => crate::demos::menus::dropdown_menu_demo,
            ContextMenuDemo => crate::demos::menus::context_menu_demo,
            MenubarDemo => crate::demos::menus::menubar_demo,
            TabNavDemo => crate::demos::collections::tab_nav_demo,
            AccordionDemo => crate::demos::collections::accordion_demo,
            CollapsibleDemo => crate::demos::collections::collapsible_demo,
            NavigationMenuDemo => crate::demos::collections::navigation_menu_demo,
            ToolbarDemo => crate::demos::collections::toolbar_demo,
            AvatarDemo => crate::demos::content::avatar_demo,
            BadgeDemo => crate::demos::content::badge_demo,
            CardDemo => crate::demos::content::card_demo,
            CalloutDemo => crate::demos::content::callout_demo,
            DataListDemo => crate::demos::content::data_list_demo,
            TableDemo => crate::demos::content::table_demo,
            ScrollAreaPartsDemo => crate::demos::content::scroll_area_parts_demo,
            AspectRatioDemo => crate::demos::content::aspect_ratio_demo,
            SeparatorDemo => crate::demos::content::separator_demo,
            LayoutDemo => crate::demos::layout::layout_demo,
            ContainerDemo => crate::demos::layout::container_demo,
            InsetDemo => crate::demos::layout::inset_demo,
            ResetDemo => crate::demos::layout::reset_demo,
            PrimitivesDemo => crate::demos::layout::primitives_demo,
            DirectionDemo => crate::demos::layout::direction_demo,
            TypographyDemo => crate::demos::typography::typography_demo,
            TextControlsDemo => crate::demos::typography::text_controls_demo,
            ColorsDemo => crate::demos::colors::colors_demo,
            ThemeBoundaryDemo => crate::demos::colors::theme_boundary_demo,
            FilePickerDemo => crate::demos::collections::file_picker_demo,
            DropZoneDemo => crate::demos::collections::drop_zone_demo,
            SplitterDemo => crate::demos::collections::splitter_demo,
            PanelTabsDemo => crate::demos::collections::panel_tabs_demo,
            CommandPaletteDemo => crate::demos::collections::command_palette_demo,
        }
        pages {
            Home { route: "/", title: "Rustify UI", description: "An experimental Rust SDK with DOM components and optional GPU regions." } => crate::pages::home,
            HomeZh { route: "/", lang: "zh", title: "Rustify UI", description: "面向浏览器应用的实验性 Rust SDK，提供 DOM 组件和按需启用的 GPU region。" } => crate::pages::home_zh,
            Button { route: "/components/button/", title: "Button", description: "A DOM button example with its exact Rust source." } => crate::pages::category,
            Label { route: "/components/label/", title: "Label", description: "A DOM label example with its exact Rust source." } => crate::pages::category,
            Link { route: "/components/link/", title: "Link", description: "A DOM link example with its exact Rust source." } => crate::pages::category,
            Icon { route: "/components/icon/", title: "Icon", description: "A DOM icon example with its exact Rust source." } => crate::pages::category,
            TextField { route: "/components/text-field/", title: "Text field", description: "A DOM text field example with its exact Rust source." } => crate::pages::category,
            TextArea { route: "/components/text-area/", title: "Text area", description: "A DOM text area example with its exact Rust source." } => crate::pages::category,
            Checkbox { route: "/components/checkbox/", title: "Checkbox", description: "A DOM checkbox example with its exact Rust source." } => crate::pages::category,
            Radio { route: "/components/radio/", title: "Radio", description: "A DOM radio example with its exact Rust source." } => crate::pages::category,
            Switch { route: "/components/switch/", title: "Switch", description: "A DOM switch example with its exact Rust source." } => crate::pages::category,
            Select { route: "/components/select/", title: "Select", description: "A DOM select example with its exact Rust source." } => crate::pages::category,
            Slider { route: "/components/slider/", title: "Slider", description: "A DOM slider example with its exact Rust source." } => crate::pages::category,
            Progress { route: "/components/progress/", title: "Progress", description: "A DOM progress example with its exact Rust source." } => crate::pages::category,
            Loading { route: "/components/loading/", title: "Loading", description: "A DOM loading example with its exact Rust source." } => crate::pages::category,
            Tooltip { route: "/components/tooltip/", title: "Tooltip", description: "A DOM tooltip example with its exact Rust source." } => crate::pages::category,
            Menu { route: "/components/menu/", title: "Menu", description: "A DOM menu example with its exact Rust source." } => crate::pages::category,
            Dialog { route: "/components/dialog/", title: "Dialog", description: "A DOM dialog example with its exact Rust source." } => crate::pages::category,
            Tabs { route: "/components/tabs/", title: "Tabs", description: "A DOM tabs example with its exact Rust source." } => crate::pages::category,
            ScrollArea { route: "/components/scroll-area/", title: "Scroll area", description: "A DOM scroll area example with its exact Rust source." } => crate::pages::category,
            DataTable { route: "/components/data-table/", title: "Data table", description: "A DOM data table example with its exact Rust source." } => crate::pages::category,
            Tree { route: "/components/tree/", title: "Tree", description: "A DOM tree example with its exact Rust source." } => crate::pages::category,
            Toast { route: "/components/toast/", title: "Toast", description: "A DOM toast example with its exact Rust source." } => crate::pages::category,
            NumberField { route: "/components/number-field/", title: "Number field", description: "A DOM number field example with its exact Rust source." } => crate::pages::category,
            ColorField { route: "/components/color-field/", title: "Color field", description: "A DOM color field example with its exact Rust source." } => crate::pages::category,
            ToggleGroup { route: "/components/toggle-group/", title: "Toggle group", description: "A DOM toggle group example with its exact Rust source." } => crate::pages::category,
            Interaction { route: "/components/interaction/", title: "Interaction", description: "Buttons, choices, ranges, fields and pickers with working examples." } => crate::pages::showcase_category,
            Overlays { route: "/components/overlays/", title: "Overlays", description: "Dialogs, popovers, hover cards and tooltips with working examples." } => crate::pages::showcase_category,
            MenusCategory { route: "/components/menus/", title: "Menus", description: "Menu, dropdown, context menu and menubar with working examples." } => crate::pages::showcase_category,
            Navigation { route: "/components/navigation/", title: "Navigation", description: "Tabs, tab nav, accordion, collapsible, navigation menu and toolbar." } => crate::pages::showcase_category,
            Content { route: "/components/content/", title: "Content", description: "Avatar, badge, card, callout, lists, tables and sizing surfaces." } => crate::pages::showcase_category,
            Feedback { route: "/components/feedback/", title: "Feedback", description: "Progress, spinner, skeleton and toast surfaces." } => crate::pages::showcase_category,
            Layout { route: "/components/layout/", title: "Layout", description: "Box, flex, grid, container, section, inset and reset." } => crate::pages::showcase_category,
            Typography { route: "/components/typography/", title: "Typography", description: "Semantic text, headings, quotes, code and keyboard text." } => crate::pages::showcase_category,
            Extensions { route: "/components/extensions/", title: "Extensions", description: "Data table, tree, files and workspace surfaces from the Rustify SDK." } => crate::pages::showcase_category,
            ColorsPage { route: "/colors/", title: "Colors", description: "Every released color family, its scales and packaged CSS." } => crate::pages::colors_page,
            PrimitivesPage { route: "/primitives/", title: "Primitives", description: "Raw parts and typed native decorators next to their themed pairs." } => crate::pages::primitives_page,
            StudioPage { route: "/studio/", title: "Theme Studio", description: "The complete public Theme Studio workbench for Rustify themes." } => crate::pages::studio_page,
        }
    }
}

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn docs_hydrate() -> Result<(), wasm_bindgen::JsValue> {
    rustify_docs::hydrate_site(site())
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use rustify_docs::site::{RenderRequest, RenderTarget, SiteEnvironment};

    #[test]
    fn official_chinese_home_renders_real_copy_and_local_destinations() {
        let site = super::site();
        assert_eq!(
            site.describe().registry["demos"].as_array().unwrap().len(),
            72
        );
        let rendered = site.render(
            &RenderRequest {
                key: "chinese-home".into(),
                seed: "chinese-home".into(),
                target: RenderTarget::Page {
                    name: "HomeZh".into(),
                    route: "/".into(),
                },
            },
            &SiteEnvironment {
                lang: "zh".into(),
                base: "/rustify-ui/".into(),
                ..Default::default()
            },
        );
        assert!(
            rendered.diagnostics.is_empty(),
            "{:?}",
            rendered.diagnostics
        );
        let html = rendered.html.unwrap();
        assert!(
            html.contains("快速开始") && html.contains("按需启用的 GPU"),
            "{html}"
        );
        assert!(html.contains("href=\"/rustify-ui/zh/quickstart/\""));
        assert!(html.contains("href=\"/rustify-ui/zh/components/\""));
        assert!(!html.contains("An experimental Rust SDK"));
    }

    #[test]
    fn showcase_category_pages_render_their_entries() {
        let site = super::site();
        let names = [
            ("Interaction", "interaction"),
            ("Overlays", "overlays"),
            ("MenusCategory", "menus"),
            ("Navigation", "navigation"),
            ("Content", "content"),
            ("Feedback", "feedback"),
            ("Layout", "layout"),
            ("Typography", "typography"),
            ("Extensions", "extensions"),
        ];
        for (name, group) in names {
            let rendered = site.render(
                &RenderRequest {
                    key: format!("category-{group}"),
                    seed: format!("category-{group}"),
                    target: RenderTarget::Page {
                        name: name.into(),
                        route: format!("/components/{group}/"),
                    },
                },
                &SiteEnvironment {
                    lang: "en".into(),
                    base: "/rustify-ui/".into(),
                    ..Default::default()
                },
            );
            assert!(
                rendered.diagnostics.is_empty(),
                "{group}: {:?}",
                rendered.diagnostics
            );
            let html = rendered.html.unwrap_or_default();
            assert!(
                html.contains(&format!("data-showcase-category=\"{group}\"")),
                "{group}"
            );
        }
    }

    #[test]
    fn showcase_entries_reference_registered_demos() {
        let site = super::site();
        let registered = site.describe().registry["demos"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| {
                value
                    .get("name")
                    .and_then(|name| name.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| value.to_string())
            })
            .collect::<Vec<_>>();
        for entry in crate::showcase::SHOWCASE {
            assert!(
                registered.iter().any(|name| name == entry.demo),
                "demo {} is not registered",
                entry.demo
            );
            assert!(
                crate::showcase::GROUPS
                    .iter()
                    .any(|(slug, _)| *slug == entry.group),
                "group {} is not a showcase group",
                entry.group
            );
        }
    }
}
