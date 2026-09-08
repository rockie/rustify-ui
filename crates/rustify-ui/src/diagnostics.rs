use std::fmt;

/// Errors the SDK reports to the application. Variants map to the failure
/// classes a host can act on; none carries user content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiError {
    /// The container element is missing or not attached to a document.
    InvalidContainer,
    /// Another mount scope already owns the container.
    OccupiedContainer,
    /// The region's canvas could not provide a WebGL2 context. The DOM around
    /// the region keeps working.
    GpuUnavailable,
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidContainer => f.write_str("container is missing or not connected"),
            Self::OccupiedContainer => f.write_str("container is already mounted"),
            Self::GpuUnavailable => f.write_str("no WebGL2 context for the region canvas"),
        }
    }
}

impl std::error::Error for UiError {}
