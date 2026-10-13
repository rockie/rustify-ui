#![recursion_limit = "256"]

use rustify_docs::SiteRegistry;

pub mod demos;
mod pages;

pub fn site() -> SiteRegistry {
    rustify_docs::site! {
        demos {
            ButtonDemo => crate::demos::basics::button_demo,
            LabelDemo => crate::demos::basics::label_demo,
            LinkDemo => crate::demos::basics::link_demo,
            IconDemo => crate::demos::basics::icon_demo,
            TextFieldDemo => crate::demos::fields::text_field_demo,
            TextAreaDemo => crate::demos::fields::text_area_demo,
            CheckboxDemo => crate::demos::choices::checkbox_demo,
            RadioDemo => crate::demos::choices::radio_demo,
            SwitchDemo => crate::demos::choices::switch_demo,
            SelectDemo => crate::demos::choices::select_demo,
            SliderDemo => crate::demos::choices::slider_demo,
            ProgressDemo => crate::demos::feedback::progress_demo,
            LoadingDemo => crate::demos::basics::loading_demo,
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
        }
        pages {
            Home { route: "/", title: "Rustify UI", description: "An experimental Rust SDK with DOM components and optional GPU regions." } => crate::pages::home,
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
        }
    }
}

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn docs_hydrate() -> Result<(), wasm_bindgen::JsValue> {
    rustify_docs::hydrate_site(site())
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
