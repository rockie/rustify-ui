//! Rustify UI: experimental SDK for Leptos DOM applications in the browser.
//!
//! The default build is DOM only and links no Makepad. Makepad GPU regions,
//! drawn into canvases the DOM laid out inside the same page and wasm module,
//! are the `gpu` feature.

#[cfg(all(feature = "csr", feature = "hydrate"))]
compile_error!(
    "csr and hydrate are separate rendering modes; disable default features for hydrate"
);

pub mod binding;
pub mod clipboard;
pub mod components;
pub mod diagnostics;
mod draft;
pub mod drag;
pub mod files;
pub mod form;
#[cfg(target_arch = "wasm32")]
mod frame;
#[cfg(all(target_arch = "wasm32", feature = "gpu"))]
pub mod gpu;
pub mod i18n;
pub mod id;
pub mod job;
mod listeners;
pub mod mount;
pub mod overlay;
#[cfg(feature = "gpu")]
pub mod region;
pub mod router;
pub mod runtime;
pub mod scheduler;
pub mod selection;
pub mod shortcut;
#[cfg(feature = "ssr")]
mod static_render;
pub mod task;
pub mod text;
pub mod theme;
pub mod toast;
#[cfg(feature = "utility")]
pub mod utility;

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
#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
pub use mount::hydrate;
#[cfg(feature = "ssr")]
pub use mount::ssr_scope;
pub use mount::{mount, AppHandle, MountConfig, ScopeRoots};
pub use overlay::{LayerId, LocalRect};
#[cfg(feature = "gpu")]
pub use region::{GpuRegion, RegionState};
pub use router::{Arrival, History, Location, Navigation, Params, Route, Routes};
#[cfg(feature = "gpu")]
pub use rustify_makepad::RegionId;
pub use scheduler::{Admission, Pace, Scheduler, Seq};
pub use selection::{Counts, Selection};
pub use shortcut::{Keystroke, Modifiers, Platform, Shortcut, ShortcutError};
#[cfg(feature = "ssr")]
pub use static_render::{provide_static_render_issues, StaticRenderIssues};
pub use task::{Load, Requests, Ticket};
pub use toast::{Toast, ToastOptions, ToastTone};

#[cfg(target_arch = "wasm32")]
pub use components::{Button, Checkbox, Label, LoadView, Slider, TextArea, TextField};
#[cfg(target_arch = "wasm32")]
pub use diagnostics::asset_failures;
pub use draft::Draft;
#[cfg(target_arch = "wasm32")]
pub use drag::{provide_drags, use_drags, DragHandle};
#[cfg(target_arch = "wasm32")]
pub use frame::{next_frame, FrameHandle};
#[cfg(all(target_arch = "wasm32", feature = "gpu"))]
pub use gpu::{
    Glyph as RegionGlyph, RustifyButton, RustifyCheckBox, RustifyDropDown, RustifyIcon,
    RustifyProgress, RustifyRadio, RustifySlider, RustifySpinner, RustifyTabBar, RustifyToggle,
};
#[cfg(target_arch = "wasm32")]
pub use i18n::{browser_languages, format_date, format_number};
pub use i18n::{provide_locale, use_locale};
#[cfg(target_arch = "wasm32")]
pub use listeners::{
    instance_failed, listen, release_on_abort, AbortRelease, ListenOptions, Listener,
};
#[cfg(target_arch = "wasm32")]
pub use overlay::{use_overlay, OverlayStack};
pub use overlay::{Anchor, Layer};
#[cfg(target_arch = "wasm32")]
pub use router::{
    navigate, provide_routes, use_location, use_params, use_route, Link, NavigationGuard,
};
#[cfg(target_arch = "wasm32")]
pub use runtime::{defer, defer_after, observe_resize, ResizeObservation};
#[cfg(all(target_arch = "wasm32", feature = "gpu"))]
pub use rustify_makepad::{makepad_widgets, RegionApp};
#[cfg(target_arch = "wasm32")]
pub use shortcut::is_text_entry;
#[cfg(target_arch = "wasm32")]
pub use text::TextEdit;
pub use theme::{
    provide_static_theme, resolve, use_resolved_theme, FontFace, ResolveContext, ResolvedTheme,
    Theme, ThemeDocument, ThemeError, ThemeMode, ThemePatch, ThemeStyles, ThemeValuePatch,
    ThemeValues,
};
#[cfg(target_arch = "wasm32")]
pub use theme::{
    use_theme, use_theme_values, ThemeBoundary, ThemeOverride, ThemeScope, ThemedScope,
};
pub use toast::{provide_toasts, use_toasts, ToastHandle, ToastRegion};
