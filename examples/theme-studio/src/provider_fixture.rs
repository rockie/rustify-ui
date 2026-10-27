use leptos::prelude::*;
use rustify_components::{Button, ButtonVariant, Dialog, Menu, MenuItem, Select, SelectOption};
use rustify_ui::{
    Anchor, Appearance, ConfigChange, Locale, ThemeBoundary, ThemeBoundaryPatch, ThemeMode,
    ThemeValuePatch,
};

/// A small real component surface used to verify theme projection and layers.
#[component]
pub fn ProviderFixture(
    #[prop(into)] patch: Signal<ThemeValuePatch>,
    #[prop(into)] locale: Signal<Locale>,
    #[prop(into)] preview_mode: Signal<ThemeMode>,
    #[prop(optional, into)] reset_epoch: Signal<u64>,
) -> impl IntoView {
    let tr = move |en: &'static str, zh: &'static str| {
        if locale.get() == Locale::Chinese {
            zh
        } else {
            en
        }
    };
    let count = RwSignal::new(0usize);
    let dialog = RwSignal::new(false);
    let menu = RwSignal::new(false);
    let action = RwSignal::new(String::new());
    let appearance = RwSignal::new("inherit".to_owned());
    let appearance_open = RwSignal::new(false);
    Effect::new(move || {
        reset_epoch.get();
        appearance.set("inherit".into());
        appearance_open.set(false);
    });
    let boundary = Signal::derive(move || {
        let mut boundary = ThemeBoundaryPatch::from(patch.get());
        boundary.config.appearance = match appearance.get().as_str() {
            "light" => ConfigChange::Set(Appearance::Light),
            "dark" => ConfigChange::Set(Appearance::Dark),
            "opposite" if preview_mode.get() == ThemeMode::Light => {
                ConfigChange::Set(Appearance::Dark)
            }
            "opposite" => ConfigChange::Set(Appearance::Light),
            _ => ConfigChange::Keep,
        };
        boundary
    });
    let menu_anchor = NodeRef::<leptos::html::Span>::new();
    let anchor = Signal::derive(move || {
        menu_anchor
            .get()
            .map(|node| Anchor::element(&node.into()))
            .unwrap_or(Anchor::Centred)
    });
    view! {
        <section class="preview-surface bg-background text-foreground p-6" data-testid="preview-surface">
            <header class="flex items-center justify-between gap-4 mb-6">
                <div><h2 class="text-xl font-semibold">{move || tr("Build something that feels yours", "创造属于你的界面")}</h2>
                    <p class="text-sm text-muted-foreground">{move || tr("Real controls, one shared theme.", "真实控件，共享同一主题。")}</p></div>
                <Button on_click=move || count.update(|value| *value+=1) test_id="preview-primary">
                    {move || format!("{} · {}",tr("Create", "创建"),count.get())}
                </Button>
            </header>
            <div class="grid gap-4" data-testid="metrics-row">
                <div class="rounded-sm border border-border bg-card text-card-foreground p-4" data-testid="radius-sm">"Small"</div>
                <div class="rounded-md border border-border bg-card text-card-foreground p-4" data-testid="radius-md">"Medium"</div>
                <div class="rounded-lg border border-border bg-card text-card-foreground p-4 shadow-lg" data-testid="radius-lg">"Large · shadow-lg"</div>
                <div class="rounded-xl border border-border bg-card text-card-foreground p-4" data-testid="radius-xl">"Extra large"</div>
            </div>
            <ThemeBoundary patch=boundary class="local-preview mt-6" test_id="local-boundary">
                <div class="rounded-lg bg-card text-card-foreground border border-border p-4 shadow-lg" data-testid="local-card">
                    <p class="text-sm mb-4">{move || tr("A local theme boundary", "局部主题边界")}</p>
                    <label class="block space-y-2 mb-4"><span>{move || tr("Boundary appearance", "边界外观")}</span>
                        <Select value=appearance open=appearance_open on_open_change=move |value| appearance_open.set(value)
                            options=Signal::derive(move || vec![
                                SelectOption::new("inherit",tr("Inherit root", "继承根主题")),
                                SelectOption::new("opposite",tr("Opposite to root", "与根主题相反")),
                                SelectOption::new("light",tr("Light", "浅色")),
                                SelectOption::new("dark",tr("Dark", "深色")),
                            ]) on_change=move |value| appearance.set(value) test_id="local-appearance" />
                    </label>
                    <div class="flex flex-wrap gap-2">
                        <Button on_click=move || dialog.set(true) variant=ButtonVariant::Soft color=Some(rustify_ui::AccentColor::Gray) test_id="open-preview-dialog">{move || tr("Open dialog", "打开对话框")}</Button>
                        <span class="inline-flex" node_ref=menu_anchor><Button on_click=move || menu.set(true) variant=ButtonVariant::Outline test_id="open-preview-menu">{move || tr("Actions", "操作")}</Button></span>
                    </div>
                    <p class="text-sm text-muted-foreground" role="status">{move || action.get()}</p>
                    <Dialog open=dialog on_open_change=move |open| dialog.set(open) title=Signal::derive(move || tr("Theme preview", "主题预览").to_owned()) class="tools-dialog" test_id="preview-dialog">
                        <p>{move || tr("This layer follows the theme where it was opened.", "此浮层跟随创建它的局部主题。")}</p>
                        <Button on_click=move || {action.set(tr("Confirmed", "已确认").into());dialog.set(false);} test_id="dialog-confirm">{move || tr("Confirm", "确认")}</Button>
                    </Dialog>
                    <Menu open=menu on_open_change=move |open| menu.set(open) anchor=anchor
                        items=Signal::derive(move || vec![MenuItem::new("duplicate",tr("Duplicate", "复制")),MenuItem::new("archive",tr("Archive", "归档"))])
                        on_activate=move |id| action.set(id) test_id="preview-menu" />
                </div>
            </ThemeBoundary>
            <div class="mt-6 flex gap-2">
                <Button class="bg-warning text-black rounded-none" on_click=|| () test_id="class-override">{move || tr("Caller class override", "调用方样式覆盖")}</Button>
                <Button on_click=|| () disabled=true>{move || tr("Disabled", "已禁用")}</Button>
            </div>
            <p class="mt-6 font-serif" data-testid="font-serif">"Serif · Shape a thoughtful interface 中文"</p>
            <p class="font-mono text-sm" data-testid="font-mono">"let theme = resolve(document);"</p>
        </section>
    }
}
