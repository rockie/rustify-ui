use leptos::prelude::*;
use rustify_components::{
    Button, ButtonVariant, Checkbox, Dialog, Menu, MenuItem, Select, SelectOption, Switch, Tab,
    TabPanel, Tabs, TextArea, TextField, TextKind,
};
use rustify_ui::{
    theme::{format_color, ColorFormat, COLOR_TOKENS},
    use_resolved_theme, Anchor, Locale,
};

pub const SCENE_NAMES: [&str; 7] = [
    "Cards",
    "Dashboard",
    "Application",
    "Marketing",
    "Mail",
    "Typography",
    "Color Palette",
];

fn tr(locale: Locale, en: &'static str, zh: &'static str) -> &'static str {
    match locale {
        Locale::English => en,
        Locale::Chinese => zh,
    }
}

#[component]
pub fn ScenePreview(
    #[prop(into)] scene: Signal<String>,
    #[prop(into)] locale: Signal<Locale>,
) -> impl IntoView {
    let scene = Memo::new(move |_| scene.get());
    view! {
        <div class="scene-preview min-w-0 bg-background text-foreground font-sans" data-theme-tokens="background,foreground,font-sans,font-size,letter-spacing">
            <p class="text-sm text-muted-foreground mb-4" data-testid="scene-local-note">
                {move || tr(locale.get(), "Local demonstration · changes stay in this preview.", "本地演示 · 操作仅保留在此预览中。")}
            </p>
            {move || match scene.get().as_str() {
                "Dashboard" => view! { <DashboardScene locale=locale /> }.into_any(),
                "Application" => view! { <ApplicationScene locale=locale /> }.into_any(),
                "Marketing" => view! { <MarketingScene locale=locale /> }.into_any(),
                "Mail" => view! { <MailScene locale=locale /> }.into_any(),
                "Typography" => view! { <TypographyScene locale=locale /> }.into_any(),
                "Color Palette" => view! { <PaletteScene locale=locale /> }.into_any(),
                _ => view! { <CardsScene locale=locale /> }.into_any(),
            }}
        </div>
    }
}

#[component]
fn CardsScene(locale: Signal<Locale>) -> impl IntoView {
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let terms = RwSignal::new(false);
    let notifications = RwSignal::new(true);
    let feedback = RwSignal::new(0u8);
    let confirmation = RwSignal::new(false);
    let submitted = RwSignal::new(String::new());
    let menu = RwSignal::new(false);
    let menu_action = RwSignal::new(0u8);
    let anchor_node = NodeRef::<leptos::html::Div>::new();
    let anchor = Signal::derive(move || {
        anchor_node
            .get()
            .map(|node| Anchor::element(&node.into()))
            .unwrap_or(Anchor::Centred)
    });
    let submit = move || {
        let name_value = name.get_untracked();
        let email_value = email.get_untracked();
        let valid_email = email_value.split_once('@').is_some_and(|(local, host)| {
            !local.is_empty() && host.contains('.') && !email_value.contains(char::is_whitespace)
        });
        if name_value.trim().is_empty() {
            feedback.set(1);
        } else if !valid_email {
            feedback.set(2);
        } else if !terms.get_untracked() {
            feedback.set(3);
        } else {
            feedback.set(0);
            confirmation.set(true);
        }
    };
    view! {
        <section data-testid="scene-cards" class="grid gap-6 md:grid-cols-2">
            <article class="rounded-lg bg-card text-card-foreground border border-border p-6 space-y-4" data-theme-tokens="card,card-foreground,border,radius" data-testid="cards-form">
                <h2 class="text-xl font-semibold">{move || tr(locale.get(), "Create a local profile", "创建本地演示资料")}</h2>
                <form class="space-y-4" on:submit=move |event| { event.prevent_default(); submit(); }>
                    <label class="block space-y-2"><span>{move || tr(locale.get(), "Name", "姓名")}</span>
                        <TextField value=name on_change=move |value| name.set(value) invalid=Signal::derive(move || feedback.get()==1) test_id="cards-name" />
                    </label>
                    <label class="block space-y-2"><span>{move || tr(locale.get(), "Email", "邮箱")}</span>
                        <TextField value=email on_change=move |value| email.set(value) kind=TextKind::Email invalid=Signal::derive(move || feedback.get()==2) test_id="cards-email" />
                    </label>
                    <label class="flex items-center gap-2" data-theme-tokens="primary,primary-foreground,input,border,ring">
                        <Checkbox checked=terms on_change=move |value| terms.set(value) invalid=Signal::derive(move || feedback.get()==3) test_id="cards-terms" />
                        <span>{move || tr(locale.get(), "I understand this is a local demo", "我知道这是本地演示")}</span>
                    </label>
                    <p role="status" class="text-sm text-destructive" data-testid="cards-error" data-theme-tokens="destructive">
                        {move || match feedback.get() {1=>tr(locale.get(), "Enter a name.", "请输入姓名。"),2=>tr(locale.get(), "Enter a valid email address.", "请输入有效邮箱。"),3=>tr(locale.get(), "Confirm the local-demo notice.", "请确认本地演示提示。"),_=>""}}
                    </p>
                    <div data-theme-tokens="primary,primary-foreground,ring">
                        <Button on_click=submit test_id="cards-submit">{move || tr(locale.get(), "Create profile", "创建资料")}</Button>
                    </div>
                </form>
                <p role="status" class="text-sm" data-testid="cards-success">{move || if submitted.get().is_empty() { String::new() } else { format!("{} {}",tr(locale.get(), "Local profile created for", "已创建本地资料："),submitted.get()) }}</p>
            </article>
            <article class="rounded-lg bg-card text-card-foreground border border-border p-6 space-y-4" data-theme-tokens="card,card-foreground,border,shadow-color,shadow-opacity" data-testid="cards-preferences">
                <h2 class="text-xl font-semibold">{move || tr(locale.get(), "Preview preferences", "预览偏好")}</h2>
                <label class="flex items-center justify-between gap-4"><span>{move || tr(locale.get(), "Local notifications", "本地通知")}</span>
                    <Switch checked=notifications on_change=move |value| notifications.set(value) test_id="cards-notifications" />
                </label>
                <p class="text-sm text-muted-foreground" data-testid="cards-notification-status">{move || if notifications.get() {tr(locale.get(), "Notifications enabled in this preview.", "此预览已开启通知。")} else {tr(locale.get(), "Notifications paused in this preview.", "此预览已暂停通知。")}}</p>
                <div node_ref=anchor_node data-theme-tokens="secondary,secondary-foreground,popover,popover-foreground">
                    <Button on_click=move || menu.set(true) variant=ButtonVariant::Secondary test_id="cards-actions">{move || tr(locale.get(), "Profile actions", "资料操作")}</Button>
                </div>
                <Menu open=menu on_open_change=move |value| menu.set(value) anchor=anchor
                    items=Signal::derive(move || {
                        let duplicate = MenuItem::new("duplicate",tr(locale.get(), "Duplicate local profile", "复制本地资料"));
                        vec![if name.get().trim().is_empty() {duplicate.disabled(tr(locale.get(), "Enter a name first", "请先输入姓名"))} else {duplicate},MenuItem::new("clear",tr(locale.get(), "Clear form", "清空表单"))]
                    })
                    on_activate=move |action| {if action=="clear" {name.set(String::new());email.set(String::new());terms.set(false);feedback.set(0);submitted.set(String::new());menu_action.set(2);} else if !name.get_untracked().trim().is_empty() {name.set(format!("{} ({})",name.get_untracked().trim(),tr(locale.get_untracked(),"copy","副本")));submitted.set(String::new());feedback.set(0);menu_action.set(1);} menu.set(false);}
                    test_id="cards-menu" />
                <p role="status" class="text-sm" data-testid="cards-action-status">{move || match menu_action.get() {1=>tr(locale.get(), "A local copy is ready to edit.", "本地副本已准备好，可编辑。"),2=>tr(locale.get(), "The form was cleared.", "表单已清空。"),_=>""}}</p>
                <Button on_click=move || {name.set(String::new());email.set(String::new());terms.set(false);feedback.set(0);submitted.set(String::new());menu_action.set(2);}
                    disabled=Signal::derive(move || name.get().is_empty() && email.get().is_empty() && !terms.get()) variant=ButtonVariant::Outline test_id="cards-clear-draft">{move || tr(locale.get(), "Clear local draft", "清空本地草稿")}</Button>
            </article>
            <Dialog open=confirmation on_open_change=move |value| confirmation.set(value)
                title=Signal::derive(move || tr(locale.get(), "Confirm local profile", "确认本地资料").to_owned())
                description=Signal::derive(move || tr(locale.get(), "This creates demonstration data in memory.", "此操作仅创建内存中的演示资料。").to_owned()) test_id="cards-dialog">
                <p>{move || name.get()}</p>
                <div class="flex flex-wrap gap-2">
                    <Button on_click=move || {submitted.set(name.get_untracked().trim().to_owned());confirmation.set(false);} test_id="cards-confirm">{move || tr(locale.get(), "Confirm", "确认")}</Button>
                    <Button on_click=move || confirmation.set(false) variant=ButtonVariant::Outline test_id="cards-cancel">{move || tr(locale.get(), "Cancel", "取消")}</Button>
                </div>
            </Dialog>
        </section>
    }
}

#[component]
fn DashboardScene(locale: Signal<Locale>) -> impl IntoView {
    let period = RwSignal::new("7".to_owned());
    let open = RwSignal::new(false);
    let empty = RwSignal::new(false);
    let data = Memo::new(move |_| {
        if empty.get() {
            Vec::new()
        } else {
            match period.get().as_str() {
                "30" => vec![42, 58, 37, 76, 65, 90, 82],
                "90" => vec![72, 54, 89, 66, 96, 83, 112],
                _ => vec![12, 18, 14, 27, 22, 31, 26],
            }
        }
    });
    view! {
        <section data-testid="scene-dashboard" class="space-y-6">
            <header class="flex flex-wrap items-end justify-between gap-4">
                <div><h2 class="text-2xl font-semibold">{move || tr(locale.get(), "Local activity overview", "本地活动概览")}</h2><p class="text-sm text-muted-foreground">{move || tr(locale.get(), "Illustrative data for theme comparison", "用于主题比较的演示数据")}</p></div>
                <label class="block w-40 space-y-2"><span class="text-sm">{move || tr(locale.get(), "Activity period", "活动时间段")}</span><Select value=period options=Signal::derive(move || vec![SelectOption::new("7",tr(locale.get(), "Last 7 days", "最近 7 天")),SelectOption::new("30",tr(locale.get(), "Last 30 days", "最近 30 天")),SelectOption::new("90",tr(locale.get(), "Last 90 days", "最近 90 天"))])
                    open=open on_open_change=move |value| open.set(value) on_change=move |value| period.set(value) test_id="dashboard-period" /></label>
            </header>
            <div class="flex flex-wrap gap-6 text-sm"><p>{move || tr(locale.get(), "Events in selected period", "所选时间段事件数")}<strong class="block text-3xl tabular-nums" data-testid="dashboard-total">{move || data.get().iter().sum::<u32>()}</strong></p>
                <label class="flex items-center gap-2"><Checkbox checked=empty on_change=move |value| empty.set(value) test_id="dashboard-empty-toggle" /><span>{move || tr(locale.get(), "Show empty data", "显示空数据")}</span></label>
            </div>
            <article class="rounded-lg bg-card text-card-foreground border border-border p-6" data-theme-tokens="card,card-foreground,chart-1,chart-2,chart-3,chart-4,chart-5,border" data-testid="dashboard-chart">
                <h3 class="font-semibold mb-4">{move || tr(locale.get(), "Events by day", "每日事件")}</h3>
                <Show when=move || !data.get().is_empty() fallback=move || view! {<p data-testid="dashboard-empty">{move || tr(locale.get(), "No activity in this demonstration. Clear the empty-data filter to restore it.", "此演示没有活动。取消空数据筛选即可恢复。")}</p>}>
                    <svg viewBox="0 0 420 180" role="img" aria-label=move || tr(locale.get(), "Demonstration event chart", "演示事件图表") class="w-full" data-testid="dashboard-bars">
                        {move || data.get().into_iter().enumerate().map(|(index,value)| view! {
                            <g><rect x=(index*58+10).to_string() y=(145.-f64::from(value)).to_string() width="34" height=value.to_string() rx="3" fill=format!("var(--chart-{})",index%5+1) data-theme-tokens=format!("chart-{}",index%5+1) />
                                <text x=(index*58+27).to_string() y="167" text-anchor="middle" fill="var(--foreground)" font-size="12">{index+1}</text></g>
                        }).collect_view()}
                    </svg>
                    <table class="w-full text-sm" data-testid="dashboard-data"><caption class="text-left text-muted-foreground">{move || tr(locale.get(), "Chart values", "图表数值")}</caption>
                        <tbody>{move || data.get().into_iter().enumerate().map(|(index,value)| view! {<tr class="border-b border-border"><th scope="row" class="text-left py-2">{format!("{} {}",tr(locale.get(), "Day", "第"),index+1)}</th><td class="text-right tabular-nums">{value}</td></tr>}).collect_view()}</tbody>
                    </table>
                </Show>
            </article>
        </section>
    }
}

#[component]
fn ApplicationScene(locale: Signal<Locale>) -> impl IntoView {
    let page = RwSignal::new("projects".to_owned());
    let project = RwSignal::new("Theme exploration".to_owned());
    let digest = RwSignal::new(true);
    let saved = RwSignal::new(None::<(String, bool)>);
    let invalid = RwSignal::new(false);
    view! {
        <section data-testid="scene-application" class="grid gap-6 md:grid-cols-[180px_1fr]">
            <nav aria-label=move || tr(locale.get(), "Local application", "本地应用") class="rounded-lg bg-sidebar text-sidebar-foreground border border-sidebar-border p-4 space-y-2" data-theme-tokens="sidebar,sidebar-foreground,sidebar-border,sidebar-primary,sidebar-primary-foreground,sidebar-accent,sidebar-accent-foreground,sidebar-ring">
                <h2 class="font-semibold mb-4">"Fieldnotes"</h2>
                {[("projects","Projects","项目"),("settings","Settings","设置")].into_iter().map(move |(id,en,zh)| view! {
                    <Button variant=ButtonVariant::Ghost class={Signal::<String>::derive(move || if page.get()==id {"w-full justify-start bg-sidebar-primary text-sidebar-primary-foreground".into()} else {"w-full justify-start hover:bg-sidebar-accent hover:text-sidebar-accent-foreground".into()})}
                        on_click=move || page.set(id.into()) test_id=format!("application-{id}")>{move || tr(locale.get(),en,zh)}</Button>
                }).collect_view()}
            </nav>
            <div class="min-w-0 space-y-4">
                <Show when=move || page.get()=="projects" fallback=move || view! {
                    <section class="space-y-4" data-testid="application-settings-panel">
                        <h3 class="text-xl font-semibold">{move || tr(locale.get(), "Workspace settings", "工作区设置")}</h3>
                        <label class="block space-y-2"><span>{move || tr(locale.get(), "Project name", "项目名称")}</span><TextField value=project on_change=move |value| project.set(value) invalid=invalid test_id="application-project-name" /></label>
                        <label class="flex gap-2 items-center"><Switch checked=digest on_change=move |value| digest.set(value) test_id="application-digest" /><span>{move || tr(locale.get(), "Weekly local summary", "每周本地摘要")}</span></label>
                        <Button on_click=move || {if project.get_untracked().trim().is_empty() {invalid.set(true);} else {invalid.set(false);saved.set(Some((project.get_untracked().trim().to_owned(),digest.get_untracked())));}} test_id="application-save">{move || tr(locale.get(), "Save settings locally", "保存本地设置")}</Button>
                        <p role="status" data-testid="application-save-status">{move || if invalid.get() {tr(locale.get(), "Enter a project name before saving.", "请先输入项目名称。").to_owned()} else if let Some((name,_))=saved.get() {format!("{} {name}",tr(locale.get(), "Saved in this preview:", "已保存到此预览："))} else {String::new()}}</p>
                    </section>
                }>
                    <h3 class="text-xl font-semibold">{move || tr(locale.get(), "Projects", "项目")}</h3>
                    <article class="rounded-lg bg-card text-card-foreground border border-border p-6 space-y-3" data-theme-tokens="card,card-foreground,border,accent,accent-foreground" data-testid="application-project">
                        <h4 class="font-semibold">{move || saved.get().map(|(name,_)|name).unwrap_or_else(||"Theme exploration".into())}</h4>
                        <p>{move || tr(locale.get(), "Compare color, typography and spacing across your components.", "比较各组件的颜色、排版与间距。")}</p>
                        <span class="inline-flex rounded-md bg-accent text-accent-foreground px-2 py-1 text-sm">{move || tr(locale.get(), "Local draft", "本地草稿")}</span>
                        <Button on_click=move || page.set("settings".into()) variant=ButtonVariant::Outline test_id="application-edit-project">{move || tr(locale.get(), "Edit project settings", "编辑项目设置")}</Button>
                    </article>
                </Show>
            </div>
        </section>
    }
}

#[component]
fn MarketingScene(locale: Signal<Locale>) -> impl IntoView {
    let billing = RwSignal::new("monthly".to_owned());
    let selected = RwSignal::new(None::<(&'static str, String)>);
    view! {
        <section data-testid="scene-marketing" class="space-y-6">
            <header class="space-y-3"><h2 class="text-3xl font-semibold">{move || tr(locale.get(), "Make room for thoughtful work", "为用心的工作留出空间")}</h2>
                <p class="text-muted-foreground max-w-prose">{move || tr(locale.get(), "A fictional product page for exploring your theme. All prices and plans below are illustrative.", "用于探索主题的虚构产品页。以下价格与方案均为演示。")}</p>
            </header>
            <div role="group" aria-label=move || tr(locale.get(), "Billing period", "计费周期")><Tabs active=billing tabs=Signal::derive(move || vec![Tab::new("monthly",tr(locale.get(), "Monthly", "按月")),Tab::new("yearly",tr(locale.get(), "Yearly", "按年"))]) on_activate=move |value| billing.set(value) test_id="marketing-billing">
                <TabPanel value="monthly"><p class="text-sm text-muted-foreground">{move || tr(locale.get(), "Illustrative monthly billing", "演示按月计费")}</p></TabPanel>
                <TabPanel value="yearly"><p class="text-sm text-muted-foreground">{move || tr(locale.get(), "Illustrative annual billing, shown per month", "演示按年计费，价格折算为每月")}</p></TabPanel>
            </Tabs></div>
            <div class="grid gap-4 md:grid-cols-3">
                {[("Starter",15,10),("Pro",32,24),("Studio",60,48)].into_iter().map(move |(name,monthly,yearly)| view! {
                    <article class="rounded-lg bg-card text-card-foreground border border-border p-5 space-y-4" data-theme-tokens="card,card-foreground,border,primary,primary-foreground" data-testid=format!("marketing-plan-{}",name.to_lowercase())>
                        <h3 class="text-xl font-semibold">{name}</h3>
                        <p class="text-3xl tabular-nums" data-testid=format!("marketing-price-{}",name.to_lowercase())>{move || format!("${}",if billing.get()=="yearly" {yearly} else {monthly})}<span class="text-sm text-muted-foreground">{move || tr(locale.get(), "/ month", "/ 月")}</span></p>
                        <ul class="text-sm space-y-2"><li>{move || tr(locale.get(), "Local workspace examples", "本地工作区示例")}</li><li>{move || tr(locale.get(), "Reusable theme samples", "可复用主题样本")}</li><li>{move || tr(locale.get(), "No purchase takes place", "不会产生实际购买")}</li></ul>
                        <Button on_click=move || selected.set(Some((name,billing.get_untracked()))) variant=if name=="Pro" {ButtonVariant::Default} else {ButtonVariant::Outline} test_id=format!("marketing-choose-{}",name.to_lowercase())>{move || tr(locale.get(), "Select demo plan", "选择演示方案")}</Button>
                    </article>
                }).collect_view()}
            </div>
            <p role="status" data-testid="marketing-status">{move || selected.get().map(|(name,period)|format!("{} {name} · {}",tr(locale.get(), "Selected locally:", "本地已选择："),if period=="yearly" {tr(locale.get(), "yearly", "按年")} else {tr(locale.get(), "monthly", "按月")})).unwrap_or_default()}</p>
            <Show when=move || selected.get().is_some()><Button on_click=move || selected.set(None) variant=ButtonVariant::Ghost test_id="marketing-clear">{move || tr(locale.get(), "Clear selection", "清除选择")}</Button></Show>
        </section>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct DemoMail {
    id: u8,
    sender: &'static str,
    subject: [&'static str; 2],
    body: [&'static str; 2],
}

const MAILS: [DemoMail;3] = [
    DemoMail {id:1,sender:"Alex Chen",subject:["A quieter palette","更安静的配色"],body:["The new neutral surfaces make the project notes easier to read. Could we compare the serif sample next? This message is local demonstration data.","新的中性表面让项目笔记更易阅读。接下来可以比较衬线字体样本吗？这封邮件是本地演示数据。"]},
    DemoMail {id:2,sender:"Sam Rivera",subject:["Review the dashboard","检查仪表盘"],body:["Please review chart colors with both light and dark backgrounds. We can use the time filter to compare the sample data. No message was sent online.","请在浅深色背景下检查图表颜色。可以使用时间筛选比较样本数据。这封邮件没有在线发送。"]},
    DemoMail {id:3,sender:"Mika Ito",subject:["Typography notes","排版笔记"],body:["Combining characters and Chinese text are included in the typography preview. Try changing the sample text before the next local review.","排版预览包含组合字符与中文。下次本地检查前，可以试着更改样本文本。"]},
];

fn localized(locale: Locale, value: [&'static str; 2]) -> &'static str {
    value[usize::from(locale == Locale::Chinese)]
}

fn palette_format(value: &str) -> ColorFormat {
    match value {
        "RGB" => ColorFormat::Rgb,
        "HSL" => ColorFormat::Hsl,
        "OKLCH" => ColorFormat::Oklch,
        _ => ColorFormat::Hex,
    }
}

#[component]
fn MailScene(locale: Signal<Locale>) -> impl IntoView {
    let search = RwSignal::new(String::new());
    let selected = RwSignal::new(None::<u8>);
    let archived = RwSignal::new(Vec::<u8>::new());
    let status = RwSignal::new(false);
    let visible = Memo::new(move |_| {
        let query = search.get().trim().to_lowercase();
        MAILS
            .into_iter()
            .filter(|mail| {
                !archived.get().contains(&mail.id)
                    && format!("{} {} {}", mail.sender, mail.subject[0], mail.subject[1])
                        .to_lowercase()
                        .contains(&query)
            })
            .collect::<Vec<_>>()
    });
    view! {
        <section data-testid="scene-mail" class="space-y-4">
            <h2 class="text-2xl font-semibold">{move || tr(locale.get(), "Local inbox", "本地收件箱")}</h2>
            <label class="block space-y-2"><span>{move || tr(locale.get(), "Search messages", "搜索邮件")}</span><TextField value=search on_change=move |value| search.set(value) kind=TextKind::Search test_id="mail-search" /></label>
            <div class="grid gap-4 md:grid-cols-2">
                <div class="space-y-2" data-testid="mail-list">
                    <Show when=move || !visible.get().is_empty() fallback=move || view! {<p class="text-muted-foreground" data-testid="mail-empty">{move || tr(locale.get(), "No matching messages. Clear search or restore archived messages.", "没有匹配邮件。请清空搜索或恢复已归档邮件。")}</p>}>
                        {move || visible.get().into_iter().map(|mail| view! {
                            <button type="button" class="w-full text-left rounded-md border border-border bg-card text-card-foreground p-4 space-y-1 outline-none focus-visible:ring-2 focus-visible:ring-ring hover:bg-muted" data-testid=format!("mail-open-{}",mail.id) data-theme-tokens="card,card-foreground,border,muted,ring" aria-pressed=move || selected.get()==Some(mail.id) on:click=move |_| {selected.set(Some(mail.id));status.set(false);}>
                                <strong class="block">{mail.sender}</strong><span class="block text-sm">{move || localized(locale.get(),mail.subject)}</span>
                            </button>
                        }).collect_view()}
                    </Show>
                    <Button on_click=move || {search.set(String::new());archived.set(Vec::new());status.set(false);} variant=ButtonVariant::Outline test_id="mail-restore">{move || tr(locale.get(), "Restore demo inbox", "恢复演示收件箱")}</Button>
                </div>
                <article class="rounded-lg bg-card text-card-foreground border border-border p-6 min-w-0" data-testid="mail-message" data-theme-tokens="card,card-foreground,border">
                    {move || if let Some(mail)=MAILS.into_iter().find(|mail|Some(mail.id)==selected.get() && !archived.get().contains(&mail.id)) {
                        view! {<div class="space-y-4"><h3 class="text-xl font-semibold">{move || localized(locale.get(),mail.subject)}</h3><p class="text-sm text-muted-foreground">{mail.sender}</p><p>{move || localized(locale.get(),mail.body)}</p>
                            <Button on_click=move || {archived.update(|ids|ids.push(mail.id));selected.set(None);status.set(true);} variant=ButtonVariant::Secondary test_id="mail-archive">{move || tr(locale.get(), "Archive locally", "本地归档")}</Button>
                        </div>}.into_any()
                    } else {view! {<p class="text-muted-foreground">{move || tr(locale.get(), "Select a message to read it.", "选择一封邮件阅读。")}</p>}.into_any()}}
                    <p role="status" class="mt-4 text-sm" data-testid="mail-status">{move || if status.get() {tr(locale.get(), "Message archived in this preview.", "邮件已在此预览中归档。")} else {""}}</p>
                </article>
            </div>
        </section>
    }
}

#[component]
fn TypographyScene(locale: Signal<Locale>) -> impl IntoView {
    let sample = RwSignal::new("Shape a thoughtful interface · 中文 · cafe\u{301} · fi".to_owned());
    let size = RwSignal::new("24".to_owned());
    let open = RwSignal::new(false);
    view! {
        <section data-testid="scene-typography" class="space-y-6">
            <h2 class="text-2xl font-semibold">{move || tr(locale.get(), "Type in context", "查看真实排版")}</h2>
            <label class="block space-y-2"><span>{move || tr(locale.get(), "Sample text", "样本文本")}</span><TextArea value=sample on_change=move |value| sample.set(value) test_id="typography-input" /></label>
            <label class="block w-40 space-y-2"><span class="text-sm">{move || tr(locale.get(), "Sample size", "样本字号")}</span><Select value=size options=Signal::derive(|| ["16","24","32","48"].into_iter().map(|size|SelectOption::new(size,format!("{size}px"))).collect::<Vec<_>>())
                open=open on_open_change=move |value| open.set(value) on_change=move |value| size.set(value) test_id="typography-size" /></label>
            <p class="text-sm text-muted-foreground" data-testid="typography-empty-note">{move || if sample.get().trim().is_empty() {tr(locale.get(), "Enter text to compare all three font slots.", "请输入文本，比较三种字体槽位。")} else {tr(locale.get(), "Samples follow the resolved fonts and letter spacing.", "样本使用解析后的字体与字距。")}}</p>
            {[("sans","font-sans","Sans"),("serif","font-serif","Serif"),("mono","font-mono","Mono")].into_iter().map(move |(slot,class,name)| view! {
                <article class="border-b border-border pb-6" data-theme-tokens=format!("font-{slot},font-size,letter-spacing,foreground,border")>
                    <h3 class="text-sm text-muted-foreground mb-3">{name}</h3>
                    <p class=format!("{class} break-words") style:font-size=move || format!("{}px",size.get()) style:line-height="1.5" data-testid=format!("typography-{slot}")>{move || sample.get()}</p>
                </article>
            }).collect_view()}
        </section>
    }
}

#[component]
fn PaletteScene(locale: Signal<Locale>) -> impl IntoView {
    let theme = use_resolved_theme().expect("ScenePreview requires ThemeScope");
    let format = RwSignal::new("HEX".to_owned());
    let open = RwSignal::new(false);
    let copied = RwSignal::new(0u8);
    let selected = RwSignal::new(String::new());
    let copy_request = RwSignal::new(0u64);
    let copy = move |text: String| {
        selected.set(text.clone());
        copied.set(0);
        copy_request.update(|request| *request += 1);
        let request = copy_request.get_untracked();
        rustify_ui::clipboard::copy(&text, move |result| {
            if copy_request.try_get_untracked() == Some(request) {
                copied.try_set(if result.is_ok() { 1 } else { 2 });
            }
        });
    };
    view! {
        <section data-testid="scene-palette" class="space-y-4">
            <header class="flex flex-wrap items-center justify-between gap-4"><h2 class="text-2xl font-semibold">{move || tr(locale.get(), "Semantic color palette", "语义色板")}</h2>
                <label class="block w-40 space-y-2"><span class="text-sm">{move || tr(locale.get(), "Color format", "颜色格式")}</span><Select value=format options=Signal::derive(|| ["HEX","RGB","HSL","OKLCH"].into_iter().map(|value|SelectOption::new(value,value)).collect::<Vec<_>>()) open=open on_open_change=move |value| open.set(value) on_change=move |value| format.set(value) test_id="palette-format" /></label>
            </header>
            <p class="text-sm text-muted-foreground">{move || tr(locale.get(), "All 35 colors show the resolved sRGB preview. Select a value or token to copy.", "35 个颜色均显示解析后的 sRGB 预览。可选择色值或 token 复制。")}</p>
            <div class="grid gap-x-6 gap-y-3 md:grid-cols-2">
                {COLOR_TOKENS.iter().copied().map(move |token| view! {
                    <article class="flex items-center gap-3 min-w-0 py-2" data-testid=format!("palette-{token}") data-theme-tokens=token>
                        <div class="size-12 shrink-0 rounded-md border border-border" style:background-color=format!("var(--{token})") aria-hidden="true"></div>
                        <div class="min-w-0 flex-1"><h3 class="font-mono text-sm break-all">{token}</h3>
                            <p class="font-mono text-xs break-all text-muted-foreground" data-testid=format!("palette-value-{token}")>{move || format_color(theme.get().colors[token],palette_format(&format.get()))}</p>
                            <div class="flex flex-wrap gap-2 mt-1">
                                <Button on_click=move || copy(format!("--{token}")) variant=ButtonVariant::Ghost class="h-7 px-1 text-xs" test_id=format!("palette-copy-token-{token}")>{move || tr(locale.get(), "Copy token", "复制 token")}</Button>
                                <Button on_click=move || copy(format_color(theme.get_untracked().colors[token],palette_format(&format.get_untracked()))) variant=ButtonVariant::Ghost class="h-7 px-1 text-xs" test_id=format!("palette-copy-value-{token}")>{move || tr(locale.get(), "Copy value", "复制色值")}</Button>
                            </div>
                        </div>
                    </article>
                }).collect_view()}
            </div>
            <p role="status" data-testid="palette-copy-status">{move || match copied.get() {1=>tr(locale.get(), "Copied to clipboard.", "已复制到剪贴板。"),2=>tr(locale.get(), "Clipboard unavailable. Select the text below and copy it manually.", "剪贴板不可用。请选中下方文本手动复制。"),_=>""}}</p>
            <Show when=move || !selected.get().is_empty()><label class="block space-y-2"><span>{move || tr(locale.get(), "Selected text", "所选文本")}</span><TextField value=selected on_change=|_| () read_only=true test_id="palette-copy-text" /></label></Show>
        </section>
    }
}
