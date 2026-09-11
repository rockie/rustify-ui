//! Rustify UI: experimental SDK for composing Leptos CSR DOM with Makepad GPU
//! regions inside one browser page and one wasm module.

pub mod binding;
pub mod clipboard;
pub mod components;
pub mod diagnostics;
pub mod drag;
pub mod files;
pub mod form;
#[cfg(target_arch = "wasm32")]
pub mod gpu;
pub mod i18n;
pub mod job;
pub mod mount;
pub mod overlay;
pub mod region;
pub mod router;
pub mod scheduler;
pub mod selection;
pub mod task;
pub mod text;
pub mod theme;

pub use binding::{duplicate_key, ActionSink};
pub use clipboard::ClipboardError;
pub use components::snap;
pub use diagnostics::{
    identify_runtime, note, record, report_json, set_recording, watch_assets, with_log, Diagnostic,
    Diagnostics, ErrorKind, Severity, UiError, REFUSED_CAPABILITIES,
};
pub use drag::{Drags, HitAnswer, HitQuery, Outcome, Query};
pub use files::{Import, Limits, Refusal};
pub use form::{FormState, Generation, Submit};
pub use i18n::{Direction, Locale, Message};
pub use job::{Budget, Ended, Job, Step};
pub use mount::{mount, AppHandle, MountConfig, ScopeRoots};
pub use overlay::{LayerId, LocalRect};
pub use region::{GpuRegion, RegionState};
pub use router::{Arrival, History, Location, Navigation, Params, Route, Routes};
pub use rustify_makepad::RegionId;
pub use scheduler::{Admission, Pace, Scheduler, Seq};
pub use selection::{Counts, Selection};
pub use task::{Load, Requests, Ticket};

#[cfg(target_arch = "wasm32")]
pub use components::{Button, Checkbox, Label, LoadView, Slider, TextArea, TextField};
#[cfg(target_arch = "wasm32")]
pub use diagnostics::asset_failures;
#[cfg(target_arch = "wasm32")]
pub use drag::{provide_drags, use_drags, DragHandle};
#[cfg(target_arch = "wasm32")]
pub use gpu::{
    Glyph as RegionGlyph, RustifyButton, RustifyCheckBox, RustifyDropDown, RustifyIcon,
    RustifyProgress, RustifyRadio, RustifySlider, RustifySpinner, RustifyTabBar, RustifyToggle,
};
#[cfg(target_arch = "wasm32")]
pub use i18n::{browser_languages, format_date, format_number, provide_locale, use_locale};
#[cfg(target_arch = "wasm32")]
pub use overlay::{use_overlay, Anchor, Layer, OverlayStack};
#[cfg(target_arch = "wasm32")]
pub use router::{
    navigate, provide_routes, use_location, use_params, use_route, Link, NavigationGuard,
};
#[cfg(target_arch = "wasm32")]
pub use rustify_makepad::{makepad_widgets, RegionApp};
#[cfg(target_arch = "wasm32")]
pub use text::TextEdit;
#[cfg(target_arch = "wasm32")]
pub use theme::{use_theme, use_theme_values, ThemeOverride, ThemedScope};
pub use theme::{Theme, ThemePatch};
