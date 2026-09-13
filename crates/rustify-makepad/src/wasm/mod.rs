mod app;
mod host;

pub use makepad_widgets;

pub use app::RegionApp;
pub use host::{
    apply, create_region, defer, defer_after, destroy_region, listener_options, live_region_count,
    observe_resize, ResizeObservation,
};
