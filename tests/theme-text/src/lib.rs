//! Execute the fork's actual CPU text modules without the removed native backend.

pub mod makepad_platform {
    pub use makepad_shared_bytes::SharedBytes;
    pub use std::println as log;
}

pub mod text;
