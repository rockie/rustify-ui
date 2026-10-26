use leptos::prelude::*;
use rustify_components::{
    Button, ButtonVariant, Dialog, Heading, HeadingLevel, Responsive, Text, TextSize,
};
use rustify_docs::SiteRegistry;
use rustify_ui::use_resolved_theme;

pub fn theme_v2_demo() -> impl IntoView {
    let theme = use_resolved_theme().expect("the Docs source consumer supplies its full theme");
    let clicks = RwSignal::new(0_u32);
    let open = RwSignal::new(false);
    view! {
        <div data-testid="docs-v2-probe" class="space-y-4"
            data-mode=move || theme.get().mode.as_str()
            data-appearance=move || theme.get().config.selected.appearance.as_str()
            data-brand=move || theme.get().brand.family.name()
            data-gray=move || theme.get().gray.family.name()
            data-scaling=move || theme.get().config.selected.scaling.percent()
            data-radius=move || theme.get().radius_px
            data-full=move || theme.get().geometry.radius_full_px
            data-unit=move || theme.get().spacing_px
            data-control-height=move || theme.get().geometry.control_heights_px[1]>
            <Heading as_=HeadingLevel::H2 test_id="docs-v2-heading">"A source-backed theme"</Heading>
            <Text size=Responsive::from(TextSize::Three) test_id="docs-v2-text">"Native text, geometry and color follow the same snapshot."</Text>
            <div class="flex flex-wrap items-center gap-3">
                <Button on_click=move || clicks.update(|value| *value += 1) test_id="docs-v2-button">"Increment"</Button>
                <Button variant=ButtonVariant::Outline on_click=move || open.set(true) test_id="docs-v2-dialog-open">"Open themed dialog"</Button>
                <output data-testid="docs-v2-count">{move || clicks.get()}</output>
            </div>
            <div class="h-12 w-32 rounded-md bg-card shadow-lg" data-testid="docs-v2-shadow">"Layered shadow"</div>
            <Dialog open=open on_open_change=move |value| open.set(value) title=Signal::derive(|| "Source v2 dialog".to_owned())
                modal=false test_id="docs-v2-dialog">
                <p data-testid="docs-v2-dialog-content" class="rounded-md bg-card p-4 text-card-foreground">"The overlay reads the current full source snapshot."</p>
            </Dialog>
        </div>
    }
}

pub fn site() -> SiteRegistry {
    rustify_docs::site! {
        demos {
            ThemeV2Demo => crate::theme_v2_demo,
        }
    }
}

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn docs_hydrate() -> Result<(), wasm_bindgen::JsValue> {
    rustify_docs::hydrate_site(site())
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
