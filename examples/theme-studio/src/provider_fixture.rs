use leptos::prelude::*;
use rustify_components::{Button, ButtonVariant, Dialog, Menu, MenuItem};
use rustify_ui::{Anchor, ThemeBoundary, ThemeValuePatch};

/// A small real component surface used to verify theme projection and layers.
#[component]
pub fn ProviderFixture(#[prop(into)] patch: Signal<ThemeValuePatch>) -> impl IntoView {
    let count = RwSignal::new(0usize);
    let dialog = RwSignal::new(false);
    let menu = RwSignal::new(false);
    let action = RwSignal::new(String::new());
    let menu_anchor = NodeRef::<leptos::html::Div>::new();
    let anchor = Signal::derive(move || {
        menu_anchor
            .get()
            .map(|node| Anchor::element(&node.into()))
            .unwrap_or(Anchor::Centred)
    });
    view! {
        <section class="preview-surface bg-background text-foreground p-6" data-testid="preview-surface">
            <header class="flex items-center justify-between gap-4 mb-6">
                <div><h2 class="text-xl font-semibold">"Build something that feels yours"</h2>
                    <p class="text-sm text-muted-foreground">"Real controls, one shared theme."</p></div>
                <Button on_click=move || count.update(|value| *value+=1) test_id="preview-primary">
                    {move || format!("Create · {}",count.get())}
                </Button>
            </header>
            <div class="grid gap-4" data-testid="metrics-row">
                <div class="rounded-sm border border-border bg-card text-card-foreground p-4" data-testid="radius-sm">"Small"</div>
                <div class="rounded-md border border-border bg-card text-card-foreground p-4" data-testid="radius-md">"Medium"</div>
                <div class="rounded-lg border border-border bg-card text-card-foreground p-4 shadow-lg" data-testid="radius-lg">"Large · shadow-lg"</div>
                <div class="rounded-xl border border-border bg-card text-card-foreground p-4" data-testid="radius-xl">"Extra large"</div>
            </div>
            <ThemeBoundary patch=patch class="local-preview mt-6" test_id="local-boundary">
                <div class="rounded-lg bg-card text-card-foreground border border-border p-4 shadow-lg" data-testid="local-card">
                    <p class="text-sm mb-4">"A local theme boundary"</p>
                    <div class="flex gap-2" node_ref=menu_anchor>
                        <Button on_click=move || dialog.set(true) variant=ButtonVariant::Secondary test_id="open-preview-dialog">"Open dialog"</Button>
                        <Button on_click=move || menu.set(true) variant=ButtonVariant::Outline test_id="open-preview-menu">"Actions"</Button>
                    </div>
                    <p class="text-sm text-muted-foreground" role="status">{move || action.get()}</p>
                    <Dialog open=dialog on_open_change=move |open| dialog.set(open) title=Signal::derive(|| "Theme preview".to_owned()) modal=false test_id="preview-dialog">
                        <p>"This layer follows the theme where it was opened."</p>
                        <Button on_click=move || action.set("Confirmed".into()) test_id="dialog-confirm">"Confirm"</Button>
                    </Dialog>
                    <Menu open=menu on_open_change=move |open| menu.set(open) anchor=anchor
                        items=Signal::derive(|| vec![MenuItem::new("duplicate","Duplicate"),MenuItem::new("archive","Archive")])
                        on_activate=move |id| action.set(id) test_id="preview-menu" />
                </div>
            </ThemeBoundary>
            <div class="mt-6 flex gap-2">
                <Button class="bg-warning text-foreground rounded-none" on_click=|| () test_id="class-override">"Caller class override"</Button>
                <Button on_click=|| () disabled=true>"Disabled"</Button>
            </div>
            <p class="mt-6 font-serif" data-testid="font-serif">"Serif · Shape a thoughtful interface 中文"</p>
            <p class="font-mono text-sm" data-testid="font-mono">"let theme = resolve(document);"</p>
        </section>
    }
}
