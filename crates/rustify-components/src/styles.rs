//! The stylesheets the component package ships, as Rust values.
//!
//! One owner per fact. Docs and the CLI read these through the crate's API
//! rather than reaching into this package's directory with `include_str!`,
//! because a path that leaves the reading crate's own directory names a file
//! its archive does not carry, and the build only fails once the package has
//! been packaged and consumed from a registry.

/// The SDK layer: the component package's own Tailwind layer.
pub const SDK: &str = include_str!("../css/sdk.css");
/// Slider styling, shared by the DOM slider and its previews.
pub const SLIDER: &str = include_str!("../css/slider.css");
/// Control states: hover, focus, active and disabled.
pub const CONTROLS: &str = include_str!("../css/controls.css");
/// Native action controls and component-local color role defaults.
pub const BUTTON: &str = include_str!("../css/button.css");
/// Layout primitives and their theme spacing defaults.
pub const LAYOUT: &str = include_str!("../css/layout.css");
/// Native text semantics and theme typography defaults.
pub const TYPOGRAPHY: &str = include_str!("../css/typography.css");
/// The fourth-generation theme surface.
pub const THEME_V4: &str = include_str!("../css/theme-v4.css");
