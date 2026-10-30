//! Rustify DOM: the shared core for Leptos DOM applications.
//!
//! Everything that is not a GPU region lives here: the runtime and the host it
//! boots under, the bindings a scope answers with, one theme table, the
//! scheduler both halves admit actions to, and the diagnostics log they both
//! write to. This crate links no Makepad, so a DOM application that never
//! draws a canvas never pulls one in.
//!
//! A host that *does* draw canvases composes this with its own region module
//! rather than replacing it: `rustify-ui` is that facade, and it re-exports
//! everything below under the same paths, so a type has one identity whichever
//! side of the boundary it was named from.

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
pub mod i18n;
pub mod id;
pub mod job;
mod listeners;
pub mod mount;
pub mod overlay;
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
pub use form::{FieldErrorSource, FormState, Generation, Submit};
pub use i18n::{provide_direction, use_direction, Direction, Locale, Message};
pub use job::{Budget, Ended, Job, Step};
#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
pub use mount::hydrate;
#[cfg(feature = "ssr")]
pub use mount::ssr_scope;
pub use mount::{mount, AppHandle, MountConfig, ScopeRoots};
pub use overlay::{
    Align, Cancelable, LayerDismiss, LayerId, LayerPlacement, LocalRect, ScopePortal, Side,
};
pub use router::{Arrival, History, Location, Navigation, Params, Route, Routes};
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
#[cfg(target_arch = "wasm32")]
pub use shortcut::is_text_entry;
#[cfg(target_arch = "wasm32")]
pub use text::TextEdit;
pub use theme::{
    provide_static_theme, resolve, use_resolved_theme, AccentColor, Appearance, ColorFamily,
    ConfigChange, EffectiveThemeConfig, FontFace, FontResolution, GrayColor, PanelBackground,
    ResolveContext, ResolvedPalette, ResolvedTheme, Theme, ThemeBoundaryPatch, ThemeConfig,
    ThemeConfigGroup, ThemeConfigPatch, ThemeDocument, ThemeError, ThemeGeometry, ThemeMode,
    ThemeOverrides, ThemePatch, ThemeRadius, ThemeScaling, ThemeSource, ThemeStyles,
    ThemeValueOrigin, ThemeValuePatch, ThemeValues,
};
#[cfg(target_arch = "wasm32")]
pub use theme::{
    use_theme, use_theme_values, ThemeBoundary, ThemeOverride, ThemeScope, ThemedScope,
};
pub use toast::{provide_toasts, use_toasts, ToastHandle, ToastRegion};

#[cfg(all(test, rustify_repo_tests))]
mod repo_tests;
