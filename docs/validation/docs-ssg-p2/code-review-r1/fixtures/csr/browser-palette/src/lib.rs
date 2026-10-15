#[cfg(not(feature = "alternate"))]
pub const ACTIVE_CLASS: &str = "bg-[#123abc]";
#[cfg(feature = "alternate")]
pub const ACTIVE_CLASS: &str = "bg-[#fedcba]";
