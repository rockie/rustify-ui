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
    /// Nothing with that identity is in the application's current state. The
    /// answer is final: waiting longer would not change it.
    NotFound,
    /// It existed and is gone. Distinguished from `NotFound` because a caller
    /// that held it can tell the difference between a typo and a deletion.
    Disposed,
    /// It was neither found nor ruled out before the caller's deadline.
    Timeout,
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidContainer => f.write_str("container is missing or not connected"),
            Self::OccupiedContainer => f.write_str("container is already mounted"),
            Self::GpuUnavailable => f.write_str("no WebGL2 context for the region canvas"),
            Self::NotFound => f.write_str("no object with that identity"),
            Self::Disposed => f.write_str("that object has been deleted"),
            Self::Timeout => f.write_str("no answer before the deadline"),
        }
    }
}

impl std::error::Error for UiError {}
