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
    /// Another scope on this page already owns the address bar. One page has
    /// one of those, and a second owner would be two answers to where the user
    /// is.
    UrlOwnerConflict,
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
            Self::UrlOwnerConflict => f.write_str("another scope owns this page's URL"),
        }
    }
}

impl std::error::Error for UiError {}

/// The failure classes this release registers.
///
/// Every diagnostic entry names one of these. There is no `Other`: a class
/// nobody named is a gap in this list, and the point of the list is that a
/// reader can act on what they find in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidContainer,
    OccupiedContainer,
    /// The application asked the region for something the embedded contract
    /// refuses: the document title, the URL, fullscreen, opening a link.
    UnsupportedCapability,
    /// A font, an image or a data file did not arrive, or arrived broken.
    AssetLoadFailed,
    /// The JS, the wasm and the generated bridge are not one build.
    BuildContractMismatch,
    /// The canvas could not give the region a WebGL2 context to start with.
    GpuInitFailed,
    /// A running region's context went away. Unlike the one above, this one
    /// is recoverable: the region is rebuilt and the application's state is
    /// projected into it again.
    GpuContextLost,
    /// A scope refused an action because its queue was full. The action did
    /// not run and changed nothing.
    Backpressure,
    /// Something arrived for a scope, a region or a request that has gone.
    Disposed,
    /// The shared wasm trapped. Every mount in this runtime is dead; only a
    /// reload brings them back.
    RuntimeFatal,
    /// A form was told about a field it does not have. Nothing happened, which
    /// is the problem: a misspelled field is a control that never validates
    /// and a form that never becomes dirty, with nothing on screen to say so.
    UnknownField,
    /// A second scope asked to own the page's URL. One address bar cannot have
    /// two owners; the scope that asked is not mounted and the one that has it
    /// is untouched.
    UrlOwnerConflict,
    /// A guard refused a move through history and the router could not put the
    /// user back where they were. They are somewhere else, and the address bar
    /// and the view agree about where.
    NavigationRestoreFailed,
    /// A guard refused a navigation. Informational: the application asked, the
    /// answer was no, and nothing is broken - but a developer wondering why a
    /// link did nothing should be able to find out.
    NavigationBlocked,
    /// A navigation was asked for while the router was putting the user back
    /// after a refused one. Informational, and for the same reason.
    NavigationBusy,
}

/// Whether an entry is something that went wrong or something that happened.
///
/// The distinction earns its place at exactly one moment: a report says how
/// many errors a run had, and a guard doing its job twenty times is not twenty
/// errors. Everything the runtime could not do stays an error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Info,
}

impl Severity {
    pub fn name(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Info => "info",
        }
    }
}

impl ErrorKind {
    pub const ALL: [ErrorKind; 15] = [
        Self::InvalidContainer,
        Self::OccupiedContainer,
        Self::UnsupportedCapability,
        Self::AssetLoadFailed,
        Self::BuildContractMismatch,
        Self::GpuInitFailed,
        Self::GpuContextLost,
        Self::Backpressure,
        Self::Disposed,
        Self::RuntimeFatal,
        Self::UnknownField,
        Self::UrlOwnerConflict,
        Self::NavigationRestoreFailed,
        Self::NavigationBlocked,
        Self::NavigationBusy,
    ];

    /// The name the diagnostics ring and the host notice both print.
    pub fn name(self) -> &'static str {
        match self {
            Self::InvalidContainer => "InvalidContainer",
            Self::OccupiedContainer => "OccupiedContainer",
            Self::UnsupportedCapability => "UnsupportedCapability",
            Self::AssetLoadFailed => "AssetLoadFailed",
            Self::BuildContractMismatch => "BuildContractMismatch",
            Self::GpuInitFailed => "GpuInitFailed",
            Self::GpuContextLost => "GpuContextLost",
            Self::Backpressure => "Backpressure",
            Self::Disposed => "Disposed",
            Self::RuntimeFatal => "RuntimeFatal",
            Self::UnknownField => "UnknownField",
            Self::UrlOwnerConflict => "UrlOwnerConflict",
            Self::NavigationRestoreFailed => "NavigationRestoreFailed",
            Self::NavigationBlocked => "NavigationBlocked",
            Self::NavigationBusy => "NavigationBusy",
        }
    }

    /// Whether this is a failure or a thing that happened. Only the four
    /// navigation entries are informational; everything else is something the
    /// runtime could not do.
    pub fn severity(self) -> Severity {
        match self {
            Self::NavigationBlocked | Self::NavigationBusy => Severity::Info,
            _ => Severity::Error,
        }
    }

    /// What to do next. Every kind has one, so no entry can be a dead end.
    pub fn suggestion(self) -> &'static str {
        match self {
            Self::InvalidContainer => "give mount an element that is in the document",
            Self::OccupiedContainer => "dispose the scope that owns the container, or use another",
            Self::UnsupportedCapability => "do it from the host page; a region cannot",
            Self::AssetLoadFailed => "check the asset is deployed beside the build, then retry",
            Self::BuildContractMismatch => "deploy the JS, the wasm and the bridge from one build",
            Self::GpuInitFailed => "use the DOM path; this browser or machine gave no WebGL2",
            Self::GpuContextLost => "nothing: the region is rebuilt with the current state",
            Self::Backpressure => "run the action again; it did not happen",
            Self::Disposed => "drop the handle; what it named is gone",
            Self::RuntimeFatal => "reload the page; unsaved in-memory state is lost",
            Self::UnknownField => "name the field in FormState::new, or correct the spelling",
            Self::UrlOwnerConflict => {
                "mount this scope with url_owner: false; only one scope owns the address bar"
            }
            Self::NavigationRestoreFailed => {
                "nothing: the user is where the browser left them, and the view agrees"
            }
            Self::NavigationBlocked => "nothing: a guard refused, and the application was told",
            Self::NavigationBusy => "ask again once the router has finished putting the user back",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The diagnostic class of an error the application was handed.
///
/// `NotFound` and `Timeout` have no entry here on purpose: they are answers to
/// a question, not failures of the runtime, and recording them would bury the
/// failures under lookups that worked exactly as intended.
impl UiError {
    pub fn kind(self) -> Option<ErrorKind> {
        match self {
            Self::InvalidContainer => Some(ErrorKind::InvalidContainer),
            Self::OccupiedContainer => Some(ErrorKind::OccupiedContainer),
            Self::GpuUnavailable => Some(ErrorKind::GpuInitFailed),
            Self::Disposed => Some(ErrorKind::Disposed),
            Self::UrlOwnerConflict => Some(ErrorKind::UrlOwnerConflict),
            Self::NotFound | Self::Timeout => None,
        }
    }
}

/// One thing that went wrong.
///
/// `detail` is `&'static str` by design: everything that varies is a typed
/// field, so no path exists for a user's text - a name being edited, a
/// response body - to reach a log. NFR-4 asks for that; a type is a better
/// guarantee of it than a review.
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub kind: ErrorKind,
    /// Milliseconds since the runtime started.
    pub at_ms: f64,
    /// The mount scope's name, when one was involved.
    pub scope: Option<String>,
    /// The region's id, when one was involved.
    pub region: Option<u32>,
    /// The asset the entry is about, when one is. This is the only field that
    /// carries a string from outside, and it is build data - a path the
    /// application's own resource table asked for - never anything a user
    /// typed.
    pub asset: Option<String>,
    /// The form field the entry is about, when one is. `&'static str` for the
    /// same reason `detail` is: a form's fields are named in its own source,
    /// so nothing a user typed can arrive here.
    pub field: Option<&'static str>,
    pub detail: &'static str,
}

impl Diagnostic {
    pub fn new(kind: ErrorKind, at_ms: f64, detail: &'static str) -> Self {
        Self {
            kind,
            at_ms,
            scope: None,
            region: None,
            asset: None,
            field: None,
            detail,
        }
    }

    /// Names the asset this entry is about. Only for paths that came from the
    /// build's own resource table.
    pub fn about_asset(mut self, asset: impl Into<String>) -> Self {
        self.asset = Some(asset.into());
        self
    }

    pub fn in_scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    pub fn in_region(mut self, region: u32) -> Self {
        self.region = Some(region);
        self
    }

    /// Names the form field this entry is about.
    pub fn about_field(mut self, field: &'static str) -> Self {
        self.field = Some(field);
        self
    }

    pub fn suggestion(&self) -> &'static str {
        self.kind.suggestion()
    }

    /// Roughly what this entry costs to keep, for the ring's byte bound.
    fn weight(&self) -> usize {
        std::mem::size_of::<Self>()
            + self.detail.len()
            + self.scope.as_ref().map(String::len).unwrap_or(0)
            + self.asset.as_ref().map(String::len).unwrap_or(0)
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0}ms {}", self.at_ms, self.kind.name())?;
        if let Some(scope) = &self.scope {
            write!(f, " scope={scope}")?;
        }
        if let Some(region) = self.region {
            write!(f, " region={region}")?;
        }
        if let Some(asset) = &self.asset {
            write!(f, " asset={asset}")?;
        }
        write!(f, ": {}; {}", self.detail, self.suggestion())
    }
}

/// A bounded record of what went wrong in one runtime.
///
/// NFR-5 puts two ceilings on it, a count and a size, and asks that reaching
/// either be visible. Both are enforced by dropping the oldest entries and
/// counting them: a reader who sees `dropped` above zero knows the record is
/// a tail rather than the whole story, which is the one thing a truncated log
/// must never hide.
#[derive(Debug)]
pub struct Diagnostics {
    /// Which runtime these belong to, for a page that boots more than one.
    runtime: u32,
    /// The build the runtime came from, so an entry can be matched to source.
    build: String,
    entries: std::collections::VecDeque<Diagnostic>,
    bytes: usize,
    dropped: u64,
    /// Whether writes are kept. Off, the record costs a counter and nothing
    /// else, which is what makes its own cost measurable.
    recording: bool,
    suppressed: u64,
}

impl Diagnostics {
    pub const MAX_ENTRIES: usize = 1_000;
    pub const MAX_BYTES: usize = 4 * 1024 * 1024;

    pub fn new(runtime: u32, build: impl Into<String>) -> Self {
        Self {
            runtime,
            build: build.into(),
            entries: std::collections::VecDeque::new(),
            bytes: 0,
            dropped: 0,
            recording: true,
            suppressed: 0,
        }
    }

    pub fn runtime(&self) -> u32 {
        self.runtime
    }

    pub fn build(&self) -> &str {
        &self.build
    }

    pub fn record(&mut self, entry: Diagnostic) {
        if !self.recording {
            self.suppressed += 1;
            return;
        }
        self.bytes += entry.weight();
        self.entries.push_back(entry);
        while self.entries.len() > Self::MAX_ENTRIES || self.bytes > Self::MAX_BYTES {
            let Some(oldest) = self.entries.pop_front() else {
                break;
            };
            self.bytes -= oldest.weight();
            self.dropped += 1;
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// How many entries the ceilings cost. Above zero, what is kept is the
    /// most recent tail and the reader is told so.
    pub fn dropped(&self) -> u64 {
        self.dropped
    }

    pub fn bytes(&self) -> usize {
        self.bytes
    }

    /// Turns writing on or off and answers what it was.
    ///
    /// A record is on unless someone turns it off, and turning it off is a
    /// measurement instrument rather than a way to make a problem quiet: what
    /// it refuses is counted and printed beside the entries it kept.
    pub fn set_recording(&mut self, on: bool) -> bool {
        std::mem::replace(&mut self.recording, on)
    }

    pub fn is_recording(&self) -> bool {
        self.recording
    }

    /// How many entries the switch turned away. Above zero, the record is
    /// missing entries nobody bounded it into missing.
    pub fn suppressed(&self) -> u64 {
        self.suppressed
    }

    pub fn entries(&self) -> impl Iterator<Item = &Diagnostic> {
        self.entries.iter()
    }

    /// The record as a developer tool reads it.
    ///
    /// The header comes first and always includes `dropped`, so a reader
    /// cannot take a truncated tail for the whole story. No field here can
    /// carry user content: `detail` is `&'static str` and the rest are ids.
    pub fn report_json(&self) -> String {
        let entries: Vec<String> = self
            .entries
            .iter()
            .map(|entry| {
                format!(
                    "{{\"kind\":\"{}\",\"severity\":\"{}\",\"at_ms\":{:.0},\"scope\":{},\"region\":{},\"asset\":{},\"field\":{},\"detail\":\"{}\",\"suggestion\":\"{}\"}}",
                    entry.kind.name(),
                    entry.kind.severity().name(),
                    entry.at_ms,
                    entry
                        .scope
                        .as_deref()
                        .map(|scope| format!("\"{scope}\""))
                        .unwrap_or_else(|| "null".to_string()),
                    entry
                        .region
                        .map(|region| region.to_string())
                        .unwrap_or_else(|| "null".to_string()),
                    entry
                        .asset
                        .as_deref()
                        .map(|asset| format!("\"{asset}\""))
                        .unwrap_or_else(|| "null".to_string()),
                    entry
                        .field
                        .map(|field| format!("\"{field}\""))
                        .unwrap_or_else(|| "null".to_string()),
                    entry.detail,
                    entry.suggestion(),
                )
            })
            .collect();
        format!(
            "{{\"runtime\":{},\"build\":\"{}\",\"count\":{},\"errors\":{},\"bytes\":{},\"dropped\":{},\"recording\":{},\"suppressed\":{},\"max_entries\":{},\"max_bytes\":{},\"entries\":[{}]}}",
            self.runtime,
            self.build,
            self.entries.len(),
            self.entries
                .iter()
                .filter(|entry| entry.kind.severity() == Severity::Error)
                .count(),
            self.bytes,
            self.dropped,
            self.recording,
            self.suppressed,
            Self::MAX_ENTRIES,
            Self::MAX_BYTES,
            entries.join(",")
        )
    }

    /// The header a reader needs before the entries mean anything, including
    /// whether they are all of them.
    pub fn summary(&self) -> String {
        let switch = if self.recording {
            String::new()
        } else {
            format!(", recording off, {} suppressed", self.suppressed)
        };
        format!(
            "runtime {} build {}: {} entries ({} errors), {} bytes, {} dropped{switch}",
            self.runtime,
            self.build,
            self.entries.len(),
            self.entries
                .iter()
                .filter(|entry| entry.kind.severity() == Severity::Error)
                .count(),
            self.bytes,
            self.dropped
        )
    }
}

/// The capabilities an embedded region is refused, in the order the host's
/// codes use.
///
/// The host passes a code rather than a name because a diagnostic's `detail`
/// is `&'static str`: the set of things that can be refused is fixed and known
/// here, so nothing has to be built from a string that crossed the boundary.
pub const REFUSED_CAPABILITIES: [&str; 5] = [
    "a region asked to open a URL; only the host page can",
    "a region asked to change the page's URL; only the host page can",
    "a region asked to move through history; only the host page can",
    "a region asked for fullscreen; only the host page can",
    "a region asked to set the document title; only the host page can",
];

/// Called by the host when a region asked for something the embedded contract
/// refuses. Once per capability per region: the host dedupes, so a region in a
/// loop cannot fill the record with one mistake.
///
/// # Safety
/// Called from the JS host with a region id and a capability code. An unknown
/// code is ignored rather than trusted.
#[cfg(target_arch = "wasm32")]
#[export_name = "rustify_note_unsupported"]
pub unsafe extern "C" fn note_unsupported(region: u32, capability: u32) {
    let Some(detail) = REFUSED_CAPABILITIES.get(capability as usize) else {
        return;
    };
    record(note(ErrorKind::UnsupportedCapability, detail).in_region(region));
}

thread_local! {
    /// One record per runtime, which on this platform means per wasm module.
    /// It is a thread-local rather than a value the application threads
    /// through: a diagnostic is written from places that have no application
    /// in reach - a host callback, a failed mount, a region that never
    /// started - and a record nothing can reach is a record nobody keeps.
    static LOG: std::cell::RefCell<Diagnostics> =
        std::cell::RefCell::new(Diagnostics::new(0, "unknown"));
}

/// Names the runtime and the build every later entry belongs to, and starts
/// watching for assets that do not arrive. Called once by the page as it
/// boots; before that, entries carry the unknown build, which is itself worth
/// seeing in a report.
pub fn identify_runtime(runtime: u32, build: &str) {
    LOG.with(|log| {
        let mut log = log.borrow_mut();
        *log = Diagnostics::new(runtime, build);
    });
    watch_assets();
}

/// Reports an asset the region asked for and did not get.
///
/// Without this the failure is a log line inside the fork and nothing else:
/// the region carries on drawing what it can, which is the right behaviour,
/// but nobody is told that a file they deployed did not arrive.
#[cfg(target_arch = "wasm32")]
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

#[cfg(not(target_arch = "wasm32"))]
pub fn watch_assets() {}

/// Records one entry in this runtime's bounded record.
pub fn record(entry: Diagnostic) {
    #[cfg(target_arch = "wasm32")]
    if entry.kind == ErrorKind::AssetLoadFailed {
        browser::note_asset_failure();
    }
    LOG.with(|log| log.borrow_mut().record(entry));
}

/// A count of the assets that have failed to arrive in this runtime, tracked.
///
/// The record itself is a plain log, and a view that read it would never hear
/// about the next entry. This is the one thing in it a view has to react to: a
/// font that does not arrive changes what a page can draw, and the page is the
/// only thing that can say so. Counting rather than listing, because what a
/// view does about it is the same whichever asset it was.
#[cfg(target_arch = "wasm32")]
pub fn asset_failures() -> u32 {
    browser::asset_failures()
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use leptos::prelude::*;

    thread_local! {
        /// `ArcRwSignal` rather than `RwSignal`: this is created the first
        /// time anything asks, which is not inside anybody's reactive owner,
        /// and an owned signal there would belong to whoever happened to be
        /// running.
        static FAILURES: ArcRwSignal<u32> = ArcRwSignal::new(0);
    }

    pub fn note_asset_failure() {
        FAILURES.with(|failures| failures.update(|count| *count += 1));
    }

    pub fn asset_failures() -> u32 {
        FAILURES.with(|failures| failures.get())
    }
}

/// Turns this runtime's record on or off and answers what it was.
///
/// The reason it exists is measurement: with it off, the cost of keeping the
/// record can be subtracted from the cost of the path that writes it. What was
/// turned away is counted, so a report taken while it was off says so.
pub fn set_recording(on: bool) -> bool {
    LOG.with(|log| log.borrow_mut().set_recording(on))
}

/// Milliseconds since the page started. Off the browser there is no such
/// clock and nothing reads the field, so it is zero rather than a fiction.
pub fn now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        leptos::prelude::window()
            .performance()
            .map(|performance| performance.now())
            .unwrap_or(0.0)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0.0
    }
}

/// Records one entry, timed now. The usual way to write one.
pub fn note(kind: ErrorKind, detail: &'static str) -> Diagnostic {
    Diagnostic::new(kind, now_ms(), detail)
}

/// This runtime's record, as the page's diagnostic seam returns it.
pub fn report_json() -> String {
    with_log(|log| log.report_json())
}

/// Reads this runtime's record. The closure keeps the borrow short: recording
/// from inside it would be a re-entrant borrow, and there is nothing a reader
/// needs to record.
pub fn with_log<R>(f: impl FnOnce(&Diagnostics) -> R) -> R {
    LOG.with(|log| f(&log.borrow()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(kind: ErrorKind, at_ms: f64) -> Diagnostic {
        Diagnostic::new(kind, at_ms, "a detail the SDK wrote")
    }

    #[test]
    fn a_guard_doing_its_job_is_not_an_error() {
        // The whole reason severity exists: a report says how many errors a
        // run had, and a guard refusing twenty navigations is not twenty
        // errors. Everything the runtime could not do stays one.
        let informational: Vec<&str> = ErrorKind::ALL
            .iter()
            .filter(|kind| kind.severity() == Severity::Info)
            .map(|kind| kind.name())
            .collect();
        assert_eq!(informational, ["NavigationBlocked", "NavigationBusy"]);

        let mut log = Diagnostics::new(1, "test");
        log.record(entry(ErrorKind::NavigationBlocked, 1.0));
        log.record(entry(ErrorKind::NavigationBusy, 2.0));
        log.record(entry(ErrorKind::Disposed, 3.0));
        let report = log.report_json();
        assert!(report.contains("\"count\":3"), "{report}");
        assert!(report.contains("\"errors\":1"), "{report}");
        assert!(report.contains("\"severity\":\"info\""), "{report}");
    }

    #[test]
    fn the_fifteen_registered_kinds_are_all_there() {
        assert_eq!(ErrorKind::ALL.len(), 15);
        let names: Vec<&str> = ErrorKind::ALL.iter().map(|kind| kind.name()).collect();
        assert_eq!(
            names,
            [
                "InvalidContainer",
                "OccupiedContainer",
                "UnsupportedCapability",
                "AssetLoadFailed",
                "BuildContractMismatch",
                "GpuInitFailed",
                "GpuContextLost",
                "Backpressure",
                "Disposed",
                "RuntimeFatal",
                "UnknownField",
                "UrlOwnerConflict",
                "NavigationRestoreFailed",
                "NavigationBlocked",
                "NavigationBusy",
            ]
        );
        // No duplicates, and every one of them says what to do next.
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), names.len());
        for kind in ErrorKind::ALL {
            assert!(!kind.suggestion().trim().is_empty(), "{kind}");
        }
    }

    #[test]
    fn an_answer_to_a_lookup_is_not_a_failure_of_the_runtime() {
        assert_eq!(UiError::NotFound.kind(), None);
        assert_eq!(UiError::Timeout.kind(), None);
        assert_eq!(
            UiError::GpuUnavailable.kind(),
            Some(ErrorKind::GpuInitFailed)
        );
        assert_eq!(UiError::Disposed.kind(), Some(ErrorKind::Disposed));
    }

    #[test]
    fn an_entry_reads_as_a_cause_and_a_next_step() {
        let entry = entry(ErrorKind::GpuContextLost, 1234.0)
            .in_scope("workbench")
            .in_region(7);
        assert_eq!(
            entry.to_string(),
            "1234ms GpuContextLost scope=workbench region=7: a detail the SDK wrote; \
             nothing: the region is rebuilt with the current state"
        );
        let asset = Diagnostic::new(ErrorKind::AssetLoadFailed, 5.0, "it did not arrive")
            .about_asset("resources/a.ttf");
        assert_eq!(
            asset.to_string(),
            "5ms AssetLoadFailed asset=resources/a.ttf: it did not arrive; \
             check the asset is deployed beside the build, then retry"
        );
    }

    #[test]
    fn the_count_ceiling_keeps_the_newest_and_says_how_many_it_dropped() {
        let mut log = Diagnostics::new(1, "build-abc");
        for round in 0..(Diagnostics::MAX_ENTRIES + 25) {
            log.record(entry(ErrorKind::Backpressure, round as f64));
        }
        assert_eq!(log.len(), Diagnostics::MAX_ENTRIES);
        assert_eq!(log.dropped(), 25);
        // What is kept is the tail, not the head.
        let first = log.entries().next().expect("an entry");
        assert_eq!(first.at_ms, 25.0);
        assert!(log.summary().contains("25 dropped"));
    }

    #[test]
    fn the_byte_ceiling_holds_even_when_the_count_would_not() {
        let mut log = Diagnostics::new(2, "build-abc");
        // A scope name long enough that a handful of entries pass the ceiling.
        let long = "s".repeat(Diagnostics::MAX_BYTES / 4);
        for round in 0..8 {
            log.record(entry(ErrorKind::AssetLoadFailed, round as f64).in_scope(long.clone()));
        }
        assert!(log.len() < 8);
        assert!(log.bytes() <= Diagnostics::MAX_BYTES);
        assert!(log.dropped() > 0);
        // Still the newest ones.
        let last = log.entries().last().expect("an entry");
        assert_eq!(last.at_ms, 7.0);
    }

    #[test]
    fn the_runtime_record_is_named_once_and_written_from_anywhere() {
        identify_runtime(9, "build-xyz");
        record(entry(ErrorKind::GpuInitFailed, 5.0).in_region(2));
        with_log(|log| {
            assert_eq!(log.runtime(), 9);
            assert_eq!(log.build(), "build-xyz");
            assert_eq!(log.len(), 1);
            let entry = log.entries().next().expect("an entry");
            assert_eq!(entry.kind, ErrorKind::GpuInitFailed);
            assert_eq!(entry.region, Some(2));
        });
        // Naming the runtime again starts a new record: a page that boots a
        // second runtime is not continuing the first one's story.
        identify_runtime(10, "build-xyz");
        with_log(|log| assert!(log.is_empty()));
    }

    #[test]
    fn a_report_says_what_was_dropped_before_it_says_anything_else() {
        let mut log = Diagnostics::new(4, "build-abc");
        log.record(
            entry(ErrorKind::Backpressure, 12.0)
                .in_scope("workbench")
                .in_region(3),
        );
        let json = log.report_json();
        assert!(
            json.starts_with("{\"runtime\":4,\"build\":\"build-abc\",\"count\":1,"),
            "{json}"
        );
        assert!(json.contains("\"dropped\":0"), "{json}");
        assert!(json.contains("\"max_entries\":1000"), "{json}");
        assert!(json.contains("\"kind\":\"Backpressure\""), "{json}");
        assert!(json.contains("\"scope\":\"workbench\""), "{json}");
        assert!(json.contains("\"region\":3"), "{json}");
        assert!(
            json.contains("\"suggestion\":\"run the action again; it did not happen\""),
            "{json}"
        );
    }

    #[test]
    fn every_refusal_the_host_can_report_has_words_for_it() {
        // The host's codes are indices into this list; a gap would be a
        // refusal that reaches the record with nothing to say.
        assert_eq!(REFUSED_CAPABILITIES.len(), 5);
        for detail in REFUSED_CAPABILITIES {
            assert!(detail.starts_with("a region asked"), "{detail}");
            assert!(detail.contains("host page"), "{detail}");
        }
    }

    #[test]
    fn a_record_that_is_off_keeps_nothing_and_says_how_much_it_refused() {
        let mut log = Diagnostics::new(5, "build-abc");
        log.record(entry(ErrorKind::Backpressure, 1.0));
        assert!(log.is_recording());
        assert!(log.set_recording(false));
        for round in 0..7 {
            log.record(entry(ErrorKind::Backpressure, round as f64));
        }
        // What was already kept stays; nothing new is written, and the count
        // of what was turned away is not a guess.
        assert_eq!(log.len(), 1);
        assert_eq!(log.suppressed(), 7);
        assert_eq!(log.dropped(), 0);
        let json = log.report_json();
        assert!(json.contains("\"recording\":false"), "{json}");
        assert!(json.contains("\"suppressed\":7"), "{json}");
        assert!(log.summary().contains("recording off, 7 suppressed"));

        // And back on, it writes again without forgetting what it turned away.
        assert!(!log.set_recording(true));
        log.record(entry(ErrorKind::Backpressure, 9.0));
        assert_eq!(log.len(), 2);
        assert_eq!(log.suppressed(), 7);
        assert!(log.report_json().contains("\"recording\":true"));
    }

    #[test]
    fn the_switch_reaches_the_runtime_record_the_same_way_writes_do() {
        identify_runtime(11, "build-xyz");
        assert!(set_recording(false));
        record(entry(ErrorKind::Backpressure, 1.0));
        with_log(|log| {
            assert!(log.is_empty());
            assert_eq!(log.suppressed(), 1);
        });
        assert!(!set_recording(true));
        record(entry(ErrorKind::Backpressure, 2.0));
        with_log(|log| assert_eq!(log.len(), 1));
        // Naming a runtime starts a record that is on: a new runtime does not
        // inherit a switch someone left off.
        assert!(set_recording(false));
        identify_runtime(12, "build-xyz");
        with_log(|log| {
            assert!(log.is_recording());
            assert_eq!(log.suppressed(), 0);
        });
    }

    #[test]
    fn a_ring_that_never_filled_says_nothing_was_dropped() {
        let mut log = Diagnostics::new(3, "build-abc");
        assert!(log.is_empty());
        log.record(entry(ErrorKind::RuntimeFatal, 1.0));
        assert_eq!(log.len(), 1);
        assert_eq!(log.dropped(), 0);
        assert_eq!(log.runtime(), 3);
        assert_eq!(log.build(), "build-abc");
        assert!(log.summary().contains("0 dropped"));
    }
}
