//! Unstyled notifications use the core Provider and Root scheduler directly.

pub use rustify_ui::toast::{
    provide_toasts, provide_toasts_with, use_toasts, Action, Close, Description, Provider, Root,
    SwipeDirection, Title, Toast, ToastAction, ToastConfig, ToastHandle, ToastImportance,
    ToastIssue, ToastOptions, ToastQueuePolicy, ToastRegion, ToastSwipe, ToastTone, Viewport,
};
