use leptos::prelude::*;
use rustify_components::table;
use rustify_components::typography as t;
use rustify_components::{Avatar, Badge, Button, Card, Glyph, Icon};

/// Avatars fall back to initials when the image cannot load; both states are
/// real and visible.
pub fn avatar_demo() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="flex items-center gap-4">
                <Avatar src="data:image/png;base64,AAAA" alt="Unreachable portrait" fallback="RU" />
                <Avatar src="data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><rect width='24' height='24' fill='%23175cd3'/></svg>" alt="Solid square" fallback="SQ" />
            </div>
            <p data-demo-state-line="" class="text-sm">"Image states: "
                <output data-demo-state="">{"Missing image shows the initials fallback; the loaded square shows the image."}</output>
            </p>
        </div>
    }
}

/// A badge marks state next to real content.
pub fn badge_demo() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <p class="flex items-center gap-3">
                "Release channel " <Badge>"stable"</Badge> " · " <Badge>"0.3.0"</Badge>
            </p>
            <p data-demo-state-line="" class="text-sm">"Badges: "<output data-demo-state="">"2 rendered next to release text"</output></p>
        </div>
    }
}

/// Cards hold real actions; the button reports through the page.
pub fn card_demo() -> impl IntoView {
    let count = RwSignal::new(0_u32);
    view! {
        <div class="space-y-4">
            <Card class="max-w-sm space-y-3 p-4">
                <t::Heading as_=t::HeadingLevel::H2 size=Signal::stored(t::TextSize::Five.into())>"Deployment card"</t::Heading>
                <p class="text-sm">"Every card action is a real application action."</p>
                <Button on_click=move || count.update(|count| *count += 1)>"Deploy once"</Button>
            </Card>
            <p data-demo-state-line="" class="text-sm">"Deployments: "<output data-demo-state="">{move || count.get()}</output></p>
        </div>
    }
}

/// A callout carries a decorative icon and its own text part.
pub fn callout_demo() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <rustify_components::callout::Root class="max-w-md">
                <rustify_components::callout::Icon><Icon glyph=Glyph::Dot label="" /></rustify_components::callout::Icon>
                <rustify_components::callout::Text>"This environment keeps drafts local; exports stay on your machine."</rustify_components::callout::Text>
            </rustify_components::callout::Root>
            <p data-demo-state-line="" class="text-sm">"Callout: "<output data-demo-state="">"visible with icon and text"</output></p>
        </div>
    }
}

/// A data list pairs labels and values for quick scanning.
pub fn data_list_demo() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <rustify_components::data_list::Root class="max-w-md">
                <rustify_components::data_list::Item>
                    <rustify_components::data_list::Label>"Registry"</rustify_components::data_list::Label>
                    <rustify_components::data_list::Value>"crates.io 0.3.0"</rustify_components::data_list::Value>
                </rustify_components::data_list::Item>
                <rustify_components::data_list::Item>
                    <rustify_components::data_list::Label>"Families"</rustify_components::data_list::Label>
                    <rustify_components::data_list::Value>"31 named scales"</rustify_components::data_list::Value>
                </rustify_components::data_list::Item>
                <rustify_components::data_list::Item>
                    <rustify_components::data_list::Label>"Modes"</rustify_components::data_list::Label>
                    <rustify_components::data_list::Value>"light and dark"</rustify_components::data_list::Value>
                </rustify_components::data_list::Item>
            </rustify_components::data_list::Root>
            <p data-demo-state-line="" class="text-sm">"Rows: "<output data-demo-state="">"3"</output></p>
        </div>
    }
}

/// The plain table keeps native semantics for small, complete datasets.
pub fn table_demo() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <table::Root class="w-full max-w-md text-sm">
                <table::Caption>"Released theme packages"</table::Caption>
                <table::Header>
                    <table::Row>
                        <table::ColumnHeaderCell>"Package"</table::ColumnHeaderCell>
                        <table::ColumnHeaderCell>"Version"</table::ColumnHeaderCell>
                        <table::ColumnHeaderCell>"Scope"</table::ColumnHeaderCell>
                    </table::Row>
                </table::Header>
                <table::Body>
                    <table::Row>
                        <table::RowHeaderCell>"rustify-colors"</table::RowHeaderCell>
                        <table::Cell>"0.3.0"</table::Cell>
                        <table::Cell>"color scales"</table::Cell>
                    </table::Row>
                    <table::Row>
                        <table::RowHeaderCell>"rustify-dom"</table::RowHeaderCell>
                        <table::Cell>"0.3.0"</table::Cell>
                        <table::Cell>"DOM SDK"</table::Cell>
                    </table::Row>
                    <table::Row>
                        <table::RowHeaderCell>"rustify-components"</table::RowHeaderCell>
                        <table::Cell>"0.3.0"</table::Cell>
                        <table::Cell>"components"</table::Cell>
                    </table::Row>
                </table::Body>
            </table::Root>
            <p data-demo-state-line="" class="text-sm">"Table rows: "<output data-demo-state="">"3 rows, each named by its row header; large datasets use the Data table extension"</output></p>
        </div>
    }
}

/// A scroll area built from its public parts with content longer and wider
/// than its frame: a scrollbar on each axis and the corner where they meet.
pub fn scroll_area_parts_demo() -> impl IntoView {
    use rustify_components::scroll_area;
    use rustify_components::tabs::Orientation;
    view! {
        <div class="space-y-4">
            <scroll_area::Root aria_label="Long reference list" visibility=scroll_area::Visibility::Auto class="h-40 w-full max-w-md rounded-md border border-border">
                <scroll_area::Viewport class="h-full w-full">
                    <ol class="w-[40rem] space-y-2 p-3">
                        {(1..=24).map(|step| view! { <li class="whitespace-nowrap">{format!("Reference step {step}: a line wide enough to need the horizontal scrollbar as well")}</li> }).collect_view()}
                    </ol>
                </scroll_area::Viewport>
                <scroll_area::Scrollbar orientation=Orientation::Vertical><scroll_area::Thumb /></scroll_area::Scrollbar>
                <scroll_area::Scrollbar orientation=Orientation::Horizontal><scroll_area::Thumb /></scroll_area::Scrollbar>
                <scroll_area::Corner />
            </scroll_area::Root>
            <p data-demo-state-line="" class="text-sm">"Scroll area parts: "<output data-demo-state="">"Root, Viewport, both Scrollbars with Thumbs and the Corner over 24 wide rows"</output></p>
        </div>
    }
}

/// Aspect ratio follows the container's width with the ratio you declare.
pub fn aspect_ratio_demo() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="grid max-w-md grid-cols-2 gap-3">
                <rustify_components::AspectRatio ratio=Signal::stored(16.0 / 9.0) class="rounded-md border border-border bg-muted">
                    <p class="flex h-full items-center justify-center text-sm">"16:9"</p>
                </rustify_components::AspectRatio>
                <rustify_components::AspectRatio ratio=Signal::stored(4.0 / 3.0) class="rounded-md border border-border bg-muted">
                    <p class="flex h-full items-center justify-center text-sm">"4:3"</p>
                </rustify_components::AspectRatio>
            </div>
            <p data-demo-state-line="" class="text-sm">"Ratios: "<output data-demo-state="">"both panes keep their ratio as the container narrows"</output></p>
        </div>
    }
}

/// Separators divide real content horizontally or vertically.
pub fn separator_demo() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="max-w-md space-y-3">
                <p>"Above the rule."</p>
                <rustify_components::Separator />
                <p>"Below the rule."</p>
            </div>
            <p class="flex items-center gap-3 text-sm">
                "Left" <rustify_components::Separator orientation=Signal::stored(rustify_components::tabs::Orientation::Vertical.into()) class="h-4" /> "Right"
            </p>
            <p data-demo-state-line="" class="text-sm">"Separators: "<output data-demo-state="">"horizontal and vertical"</output></p>
        </div>
    }
}
