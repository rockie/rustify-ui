use leptos::prelude::*;
use rustify_colors::{black_alpha, white_alpha, ColorFamily, ColorGamut, ColorMode};

fn family_name(family: ColorFamily) -> String {
    format!("{family:?}").to_lowercase()
}

/// Every released family, its real scales and the exact strings behind them.
pub fn colors_demo() -> impl IntoView {
    let families: &'static [ColorFamily] = ColorFamily::ALL;
    let family = RwSignal::new(ColorFamily::Indigo);
    let dark = RwSignal::new(false);
    let alpha = RwSignal::new(false);
    let p3 = RwSignal::new(false);
    let scale = move || {
        let gamut = if p3.get() {
            ColorGamut::P3
        } else {
            ColorGamut::Srgb
        };
        family.get().scale(
            if dark.get() {
                ColorMode::Dark
            } else {
                ColorMode::Light
            },
            gamut,
            alpha.get(),
        )
    };
    let steps = move || scale().steps().to_vec();
    // The packaged file for exactly this selection; the page's download list
    // carries its deploy-qualified link under the same name.
    let file = move || {
        format!(
            "{}-{}{}{}",
            family_name(family.get()),
            if dark.get() { "dark" } else { "light" },
            if alpha.get() { "-alpha" } else { "" },
            if p3.get() { "-p3" } else { "" },
        )
    };
    view! {
        <div class="space-y-4">
            <div class="flex flex-wrap items-center gap-3 text-sm">
                <label class="flex items-center gap-2">"Family "
                    <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| {
                        let chosen = event_target_value(&event);
                        if let Some(found) = ColorFamily::ALL.iter().find(|candidate| family_name(**candidate) == chosen) {
                            family.set(*found);
                        }
                    }>
                        {families.iter().map(|candidate| view! {
                            <option value=family_name(*candidate) selected=move || family_name(*candidate) == family_name(family.get())>
                                {family_name(*candidate)}
                            </option>
                        }).collect_view()}
                    </select>
                </label>
                <label class="flex items-center gap-2">"Mode "
                    <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| dark.set(event_target_value(&event) == "dark")>
                        <option value="light" selected=move || !dark.get()>"light"</option>
                        <option value="dark" selected=move || dark.get()>"dark"</option>
                    </select>
                </label>
                <label class="flex items-center gap-2">"Style "
                    <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| alpha.set(event_target_value(&event) == "alpha")>
                        <option value="solid" selected=move || !alpha.get()>"solid"</option>
                        <option value="alpha" selected=move || alpha.get()>"alpha"</option>
                    </select>
                </label>
                <label class="flex items-center gap-2">
                    <input type="checkbox" checked=p3 on:change=move |event| p3.set(event_target_checked(&event)) />
                    "Display P3"
                </label>
            </div>
            <ol class="flex flex-wrap gap-2">
                {move || steps().into_iter().enumerate().map(|(index, value)| view! {
                    <li class="w-36 space-y-1">
                        <svg viewBox="0 0 144 36" class="h-9 w-36" aria-hidden="true">
                            <rect x="0" y="0" width="144" height="36" rx="6" fill=value />
                        </svg>
                        <p class="text-xs">{format!("Step {}", index + 1)}</p>
                        <code class="block break-all text-xs">{value}</code>
                    </li>
                }).collect_view()}
            </ol>
            <div class="space-y-2 rounded-md border border-border p-3">
                <p class="text-sm font-medium">"Black and white alpha scales"</p>
                <div class="flex flex-wrap gap-4">
                    {move || {
                        let gamut = if p3.get() { ColorGamut::P3 } else { ColorGamut::Srgb };
                        [(black_alpha(gamut), "black"), (white_alpha(gamut), "white")].iter().map(|(steps, name)| view! {
                            <div class="space-y-1">
                                <svg viewBox="0 0 144 24" class="h-6 w-36" aria-hidden="true">
                                    {(0..12).map(|index| view! {
                                        <rect x=index * 12 y="0" width="12" height="24" fill=steps.step((index + 1) as u8).unwrap_or_default().to_string() />
                                    }).collect_view()}
                                </svg>
                                <p class="text-xs font-medium">{format!("{name}-alpha")}</p>
                                <ol class="space-y-0.5 text-xs">
                                    {(1..=12_u8).map(|step| view! {
                                        <li>{format!("{step}: ")}<code class="break-all">{steps.step(step).unwrap_or_default()}</code></li>
                                    }).collect_view()}
                                </ol>
                            </div>
                        }).collect_view()
                    }}
                </div>
            </div>
            <p class="text-sm">"Matching CSS: "<a data-rustify-native-navigation="" href=move || format!("#css-{}", file())>{move || format!("{}.css", file())}</a></p>
            <p class="text-sm">"Selection: "
                <output data-demo-state="">{move || format!("{} · {} · {} · {}", family_name(family.get()), if dark.get() { "dark" } else { "light" }, if alpha.get() { "alpha" } else { "solid" }, if p3.get() { "p3" } else { "srgb" })}</output>
            </p>
            <p class="text-sm text-muted-foreground">"Display P3 strings need a P3-capable browser and display; the adjacent sRGB strings are the fallback. Copy the raw string you need, or download the packaged CSS below."</p>
        </div>
    }
}

/// A local boundary changes tokens for one surface while the page keeps its
/// own theme. The interactive boundary lives in the public Theme Studio,
/// whose Inspector reports root, local and inherited sources; docs islands
/// run outside a ThemeScope, so this page shows the concept without mounting
/// an editor.
pub fn theme_boundary_demo() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="grid max-w-xl gap-4 md:grid-cols-2">
                <div class="space-y-2 rounded-md border border-border p-3">
                    <p class="text-sm font-medium">"Page theme"</p>
                    <p class="text-sm">"Every control on this site renders from the site theme document."</p>
                </div>
                <div class="space-y-2 rounded-md border border-border p-3">
                    <p class="text-sm font-medium">"Local boundary"</p>
                    <p class="text-sm">"A boundary applies a token patch to one surface only; overlays inherit the theme that created them. Open the Theme Studio to edit boundaries live and read each value's source."</p>
                </div>
            </div>
            <p class="text-sm">"Boundary state: "
                <output data-demo-state="">"explained here; interactive in the public Theme Studio"</output>
            </p>
        </div>
    }
}
