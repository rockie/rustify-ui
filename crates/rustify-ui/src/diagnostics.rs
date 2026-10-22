//! The runtime's diagnostics log, plus the region resource watcher.
//!
//! The log itself is the core's: the DOM half and a region write the same
//! entries through the same `note` and `record`. Only the watcher is added
//! here, because a resource failure inside a region is Makepad's to report.

pub use rustify_dom::diagnostics::*;

/// Reports an asset the region asked for and did not get.
///
/// Without this the failure is a log line inside the fork and nothing else:
/// the region carries on drawing what it can, which is the right behaviour,
/// but nobody is told that a file they deployed did not arrive.
#[cfg(all(target_arch = "wasm32", feature = "gpu"))]
pub fn watch_assets() {
    use rustify_makepad::makepad_widgets::makepad_platform::script::res::on_resource_failure;
    on_resource_failure(std::rc::Rc::new(|path: &str, error: &str| {
        // The reason is one of a fixed set the fork produces; the varying part
        // of it is a status code, which is not worth a dynamic string here.
        let detail = if error.contains("status") {
            "a resource the region asked for came back as an error"
        } else if error.contains("empty response body") {
            "a resource the region asked for came back empty"
        } else {
            "a resource the region asked for did not arrive"
        };
        record(note(ErrorKind::AssetLoadFailed, detail).about_asset(path));
    }));
}

/// Nothing to watch without GPU regions: a DOM asset that fails is the
/// browser's to report.
#[cfg(not(all(target_arch = "wasm32", feature = "gpu")))]
pub fn watch_assets() {}
