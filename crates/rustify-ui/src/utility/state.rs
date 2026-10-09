#[cfg(any(target_arch = "wasm32", test))]
use futures_channel::oneshot;
#[cfg(any(target_arch = "wasm32", test))]
use rustify_style::program::{GpuCondition, GpuValue};
use rustify_style::protocol::{CompileError, CompileReport};
#[cfg(any(target_arch = "wasm32", test))]
use rustify_style::protocol::{Limits, SourceContent, SourceSnapshot, VirtualSourceEntry};
#[cfg(any(target_arch = "wasm32", test))]
use rustify_style::text::Text;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[cfg(any(target_arch = "wasm32", test))]
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UtilityError {
    Protocol { message: String },
    Compile { error: CompileError },
    Scan { message: String },
    Worker { message: String },
    Install { message: String },
    Timeout,
    Disposed,
    Superseded,
}
impl<'de> Deserialize<'de> for UtilityError {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum Kind {
            Protocol,
            Compile,
            Scan,
            Worker,
            Install,
            Timeout,
            Disposed,
            Superseded,
        }
        // Internally tagged Content buffers nested strings as scalar text before
        // CompileError's Text fields can read their original UTF-16 units.
        #[derive(Deserialize)]
        struct Fields {
            kind: Kind,
            message: Option<String>,
            error: Option<CompileError>,
        }
        fn required<T, E: serde::de::Error>(field: Option<T>, name: &'static str) -> Result<T, E> {
            field.ok_or_else(|| E::missing_field(name))
        }
        let fields = Fields::deserialize(deserializer)?;
        Ok(match fields.kind {
            Kind::Protocol => Self::Protocol {
                message: required(fields.message, "message")?,
            },
            Kind::Compile => Self::Compile {
                error: required(fields.error, "error")?,
            },
            Kind::Scan => Self::Scan {
                message: required(fields.message, "message")?,
            },
            Kind::Worker => Self::Worker {
                message: required(fields.message, "message")?,
            },
            Kind::Install => Self::Install {
                message: required(fields.message, "message")?,
            },
            Kind::Timeout => Self::Timeout,
            Kind::Disposed => Self::Disposed,
            Kind::Superseded => Self::Superseded,
        })
    }
}
impl std::fmt::Display for UtilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Protocol { message }
            | Self::Worker { message }
            | Self::Install { message }
            | Self::Scan { message } => f.write_str(message),
            Self::Compile { error } => error.fmt(f),
            Self::Timeout => f.write_str("utility Worker request timed out"),
            Self::Disposed => f.write_str("utility scope is disposed"),
            Self::Superseded => f.write_str("utility configuration was superseded"),
        }
    }
}
impl std::error::Error for UtilityError {}
impl From<CompileError> for UtilityError {
    fn from(error: CompileError) -> Self {
        Self::Compile { error }
    }
}

/// Compiler inputs and resource budgets for one utility scope.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UtilityConfig {
    pub input: rustify_style::protocol::CompilerInput,
    pub assets: Vec<rustify_style::protocol::SourceAsset>,
    pub limits: rustify_style::protocol::Limits,
    pub request_timeout_ms: u32,
}

#[derive(Clone, Debug)]
pub struct CommittedSnapshot {
    pub epoch: u32,
    pub report: Arc<CompileReport>,
    /// Installed compiler inputs, including extracted CSS variable dependencies.
    pub classes: BTreeSet<String>,
}

pub type Outcome = Result<Arc<CommittedSnapshot>, UtilityError>;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn merge_candidates<'a>(
    retained: &mut BTreeSet<String>,
    candidates: impl IntoIterator<Item = &'a String>,
    limits: &Limits,
) -> Result<(), UtilityError> {
    let mut bytes = retained.iter().fold(0usize, |sum, value| {
        sum.saturating_add(value.len())
            .saturating_add(std::mem::size_of::<String>() + 4 * std::mem::size_of::<usize>())
    });
    if retained.len() > limits.candidate_count {
        return Err(CompileError::Limit {
            resource: "candidate_count".into(),
            limit: limits.candidate_count,
        }
        .into());
    }
    if bytes > limits.cache_bytes {
        return Err(CompileError::Limit {
            resource: "cache_bytes".into(),
            limit: limits.cache_bytes,
        }
        .into());
    }
    for candidate in candidates {
        if retained.contains(candidate) {
            continue;
        }
        if retained.len() >= limits.candidate_count {
            return Err(CompileError::Limit {
                resource: "candidate_count".into(),
                limit: limits.candidate_count,
            }
            .into());
        }
        bytes = bytes
            .saturating_add(candidate.len())
            .saturating_add(std::mem::size_of::<String>() + 4 * std::mem::size_of::<usize>());
        if bytes > limits.cache_bytes {
            return Err(CompileError::Limit {
                resource: "cache_bytes".into(),
                limit: limits.cache_bytes,
            }
            .into());
        }
        retained.insert(candidate.clone());
    }
    // Check JSON expansion (quotes, control characters, escapes) before making the bridge copy.
    serialize_input(retained, limits.queued_bytes)?;
    Ok(())
}
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) type Completion = oneshot::Receiver<Outcome>;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn serialize_input(
    input: &impl Serialize,
    limit: usize,
) -> Result<String, UtilityError> {
    struct Bounded {
        bytes: Vec<u8>,
        limit: usize,
        exceeded: bool,
    }
    impl std::io::Write for Bounded {
        fn write(&mut self, input: &[u8]) -> std::io::Result<usize> {
            let size = self.bytes.len().saturating_add(input.len());
            if size > self.limit {
                self.exceeded = true;
                return Err(std::io::Error::other("queued_bytes limit"));
            }
            if size > self.bytes.capacity() {
                let capacity = size
                    .checked_next_power_of_two()
                    .unwrap_or(self.limit)
                    .min(self.limit);
                self.bytes.reserve_exact(capacity - self.bytes.len());
            }
            self.bytes.extend_from_slice(input);
            Ok(input.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut output = Bounded {
        bytes: Vec::new(),
        limit,
        exceeded: false,
    };
    if let Err(error) = serde_json::to_writer(&mut output, input) {
        return Err(if output.exceeded {
            CompileError::Limit {
                resource: "queued_bytes".into(),
                limit,
            }
            .into()
        } else {
            UtilityError::Protocol {
                message: error.to_string(),
            }
        });
    }
    String::from_utf8(output.bytes).map_err(|error| UtilityError::Protocol {
        message: error.to_string(),
    })
}
#[cfg(any(target_arch = "wasm32", test))]
fn report_bytes(report: &CompileReport) -> usize {
    let mut bytes = std::mem::size_of::<CompileReport>().saturating_add(report.css.owned_bytes());
    bytes = bytes.saturating_add(report.admitted.capacity() * std::mem::size_of::<Text>());
    for class in &report.admitted {
        bytes = bytes.saturating_add(class.owned_bytes());
    }
    for diagnostics in [&report.rejected, &report.unsupported] {
        bytes = bytes.saturating_add(
            diagnostics.capacity()
                * std::mem::size_of::<rustify_style::protocol::CandidateDiagnostic>(),
        );
        for diagnostic in diagnostics {
            bytes = bytes
                .saturating_add(diagnostic.candidate.owned_bytes())
                .saturating_add(diagnostic.family.as_ref().map_or(0, Text::owned_bytes));
        }
    }
    for (name, value) in &report.gpu_variables {
        bytes = bytes
            .saturating_add(name.owned_bytes())
            .saturating_add(value.owned_bytes())
            .saturating_add(
                2 * std::mem::size_of::<rustify_style::text::Text>()
                    + 4 * std::mem::size_of::<usize>(),
            );
    }
    bytes = bytes.saturating_add(
        report.gpu_programs.capacity() * std::mem::size_of::<rustify_style::program::GpuProgram>(),
    );
    for program in &report.gpu_programs {
        bytes = bytes
            .saturating_add(program.candidate.owned_bytes())
            .saturating_add(program.conditions.capacity() * std::mem::size_of::<GpuCondition>())
            .saturating_add(
                program.declarations.capacity()
                    * std::mem::size_of::<rustify_style::program::GpuDeclaration>(),
            );
        for condition in &program.conditions {
            bytes = bytes.saturating_add(match condition {
                GpuCondition::Selector { selector } => selector.owned_bytes(),
                GpuCondition::Media { query } => query.owned_bytes(),
                GpuCondition::Unsupported { feature } => feature.capacity(),
                _ => 0,
            });
        }
        for declaration in &program.declarations {
            bytes = bytes
                .saturating_add(declaration.property.capacity())
                .saturating_add(match &declaration.value {
                    GpuValue::Pixels { value }
                    | GpuValue::CssLength { value }
                    | GpuValue::Color { value }
                    | GpuValue::Number { value }
                    | GpuValue::Keyword { value }
                    | GpuValue::CssExpression { value } => value.capacity(),
                    GpuValue::ThemeVariable { name, fallback } => name
                        .capacity()
                        .saturating_add(fallback.as_ref().map_or(0, String::capacity)),
                });
        }
    }
    bytes
}
#[cfg(any(target_arch = "wasm32", test))]
struct Pending {
    input: Input,
    bytes: usize,
    sender: oneshot::Sender<Outcome>,
}
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) struct Batch {
    pub epoch: u32,
    pub input: Input,
    senders: Vec<oneshot::Sender<Outcome>>,
}
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) enum Input {
    Classes(Vec<String>),
    Flush,
    Content(Vec<SourceContent>),
    Sources(Box<SourceSnapshot>),
    Replace {
        config: Box<UtilityConfig>,
        cn: std::rc::Rc<super::cn::PreparedCn>,
        /// Present for a rebuild: the complete set the caller still uses, replacing retained history.
        active: Option<Vec<String>>,
    },
}
#[cfg(any(target_arch = "wasm32", test))]
impl Batch {
    /// Whether the installed candidates carry over into this batch's snapshot.
    pub fn retains_installed(&self) -> bool {
        !matches!(
            self.input,
            Input::Replace {
                active: Some(_),
                ..
            }
        )
    }
}
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) struct State {
    pub epoch: u32,
    pub disposed: bool,
    pub failed: Option<UtilityError>,
    pub snapshot: Option<Arc<CommittedSnapshot>>,
    limits: Limits,
    queue: VecDeque<Pending>,
    queued_bytes: usize,
    inflight_bytes: usize,
    snapshot_bytes: usize,
    seed_bytes: usize,
    running: bool,
    changing: bool,
}
#[cfg(any(target_arch = "wasm32", test))]
impl State {
    pub fn new(limits: Limits) -> Self {
        Self {
            epoch: 1,
            disposed: false,
            failed: None,
            snapshot: None,
            limits,
            queue: VecDeque::new(),
            queued_bytes: 0,
            inflight_bytes: 0,
            snapshot_bytes: 0,
            seed_bytes: 0,
            running: false,
            changing: false,
        }
    }
    pub fn set_seed_bytes(&mut self, bytes: usize, limits: &Limits) -> Result<(), UtilityError> {
        if bytes
            .saturating_add(self.snapshot_bytes)
            .saturating_add(self.queued_bytes)
            .saturating_add(self.inflight_bytes)
            > limits.cache_bytes
        {
            return Err(CompileError::Limit {
                resource: "cache_bytes".into(),
                limit: limits.cache_bytes,
            }
            .into());
        }
        self.seed_bytes = bytes;
        Ok(())
    }
    pub fn enqueue(&mut self, classes: &[String]) -> Result<Completion, UtilityError> {
        if self.disposed {
            return Err(UtilityError::Disposed);
        }
        let new_classes = classes.iter().filter(|class| {
            self.changing
                || self
                    .snapshot
                    .as_ref()
                    .is_none_or(|snapshot| !snapshot.classes.contains(*class))
        });
        if new_classes.clone().next().is_none() {
            if let Some(snapshot) = &self.snapshot {
                let (sender, receiver) = oneshot::channel();
                let _ = sender.send(Ok(snapshot.clone()));
                return Ok(receiver);
            }
        }
        if let Some(error) = &self.failed {
            return Err(error.clone());
        }
        // Include a conservative allowance for the channel's heap allocation.
        let bytes =
            new_classes
                .clone()
                .try_fold(std::mem::size_of::<Pending>() + 128, |sum, class| {
                    sum.checked_add(class.len())?.checked_add(
                        std::mem::size_of::<String>() + 4 * std::mem::size_of::<usize>(),
                    )
                });
        let bytes = self.check_budget(bytes)?;
        let classes = new_classes.cloned().collect();
        Ok(self.push(Input::Classes(classes), bytes))
    }
    pub fn enqueue_flush(&mut self) -> Result<Completion, UtilityError> {
        let bytes = self.check_budget(Some(std::mem::size_of::<Pending>() + 128))?;
        Ok(self.push(Input::Flush, bytes))
    }
    pub fn enqueue_replace(
        &mut self,
        config: UtilityConfig,
        cn: std::rc::Rc<super::cn::PreparedCn>,
        active: Option<Vec<String>>,
        bytes: usize,
    ) -> Result<Completion, UtilityError> {
        if self.disposed {
            return Err(UtilityError::Disposed);
        }
        let epoch = self
            .epoch
            .checked_add(1)
            .ok_or(CompileError::RevisionExhausted)?;
        if bytes.saturating_add(self.inflight_bytes) > config.limits.queued_bytes {
            return Err(CompileError::Limit {
                resource: "queued_bytes".into(),
                limit: config.limits.queued_bytes,
            }
            .into());
        }
        if bytes
            .saturating_add(self.inflight_bytes)
            .saturating_add(self.snapshot_bytes)
            .saturating_add(self.seed_bytes)
            > config.limits.cache_bytes
        {
            return Err(CompileError::Limit {
                resource: "cache_bytes".into(),
                limit: config.limits.cache_bytes,
            }
            .into());
        }
        for pending in self.queue.drain(..) {
            let _ = pending.sender.send(Err(UtilityError::Superseded));
        }
        self.queued_bytes = 0;
        self.epoch = epoch;
        self.changing = true;
        self.failed = None;
        Ok(self.push(
            Input::Replace {
                config: Box::new(config),
                cn,
                active,
            },
            bytes,
        ))
    }

    pub fn enqueue_content(
        &mut self,
        contents: &[SourceContent],
    ) -> Result<Completion, UtilityError> {
        let bytes =
            contents
                .iter()
                .try_fold(std::mem::size_of::<Pending>() + 128, |size, content| {
                    size.checked_add(std::mem::size_of::<SourceContent>())?
                        .checked_add(content.content.len())?
                        .checked_add(content.extension.len())
                });
        let bytes = self.check_budget(bytes)?;
        Ok(self.push(Input::Content(contents.to_vec()), bytes))
    }
    pub fn enqueue_sources(
        &mut self,
        sources: &SourceSnapshot,
    ) -> Result<Completion, UtilityError> {
        let bytes = sources.entries.iter().fold(
            (std::mem::size_of::<Pending>() + 128 + std::mem::size_of::<SourceSnapshot>())
                .checked_add(sources.root.len())
                .and_then(|size| size.checked_add(sources.base.len())),
            |size, entry| {
                let (path, extra) = match entry {
                    VirtualSourceEntry::File { path, content, .. } => (path, content.len()),
                    VirtualSourceEntry::Symlink { path, target } => (path, target.len()),
                    VirtualSourceEntry::Directory { path }
                    | VirtualSourceEntry::Unavailable { path } => (path, 0),
                };
                size?
                    .checked_add(std::mem::size_of::<VirtualSourceEntry>())?
                    .checked_add(path.len())?
                    .checked_add(extra)
            },
        );
        let bytes = self.check_budget(bytes)?;
        Ok(self.push(Input::Sources(Box::new(sources.clone())), bytes))
    }
    fn check_budget(&self, bytes: Option<usize>) -> Result<usize, UtilityError> {
        if self.disposed {
            return Err(UtilityError::Disposed);
        }
        if let Some(error) = &self.failed {
            return Err(error.clone());
        }
        let total = bytes
            .and_then(|bytes| self.queued_bytes.checked_add(bytes))
            .and_then(|bytes| self.inflight_bytes.checked_add(bytes));
        let bytes = match (bytes, total) {
            (Some(bytes), Some(total)) if total <= self.limits.queued_bytes => bytes,
            _ => {
                return Err(CompileError::Limit {
                    resource: "queued_bytes".into(),
                    limit: self.limits.queued_bytes,
                }
                .into())
            }
        };
        if total
            .and_then(|size| size.checked_add(self.snapshot_bytes))
            .and_then(|size| size.checked_add(self.seed_bytes))
            .is_none_or(|size| size > self.limits.cache_bytes)
        {
            return Err(CompileError::Limit {
                resource: "cache_bytes".into(),
                limit: self.limits.cache_bytes,
            }
            .into());
        }
        Ok(bytes)
    }
    fn push(&mut self, input: Input, bytes: usize) -> Completion {
        let (sender, receiver) = oneshot::channel();
        self.queued_bytes += bytes;
        self.queue.push_back(Pending {
            input,
            bytes,
            sender,
        });
        receiver
    }
    pub fn take_batch(&mut self) -> Option<Batch> {
        if self.disposed || self.running || self.queue.is_empty() {
            return None;
        }
        let first = self.queue.pop_front()?;
        let mut input = first.input;
        let mut senders = vec![first.sender];
        let mut bytes = first.bytes;
        // Only adjacent complete-class requests can be coalesced. Scans are FIFO barriers.
        if let Input::Classes(first) = input {
            let mut classes = BTreeSet::from_iter(first);
            while self
                .queue
                .front()
                .is_some_and(|pending| matches!(pending.input, Input::Classes(_)))
            {
                let Some(pending) = self.queue.pop_front() else {
                    break;
                };
                let Input::Classes(next) = pending.input else {
                    unreachable!()
                };
                classes.extend(next);
                bytes += pending.bytes;
                senders.push(pending.sender);
            }
            input = Input::Classes(classes.into_iter().collect());
        }
        self.running = true;
        self.queued_bytes -= bytes;
        self.inflight_bytes = bytes;
        Some(Batch {
            epoch: self.epoch,
            input,
            senders,
        })
    }
    pub fn installed(
        &mut self,
        batch: &Batch,
        report: CompileReport,
        candidates: &[String],
    ) -> Outcome {
        if self.disposed {
            return Err(UtilityError::Disposed);
        }
        if self.epoch != batch.epoch {
            return Err(UtilityError::Superseded);
        }
        let limits = match &batch.input {
            Input::Replace { config, .. } => &config.limits,
            _ => &self.limits,
        };
        let retain = batch.retains_installed();
        let bytes = self.validate_with_limits(&report, candidates, limits, retain)?;
        let mut classes = match &self.snapshot {
            Some(snapshot) if retain => snapshot.classes.clone(),
            _ => BTreeSet::new(),
        };
        classes.extend(candidates.iter().cloned());
        let snapshot = Arc::new(CommittedSnapshot {
            epoch: self.epoch,
            report: Arc::new(report),
            classes,
        });
        if let Input::Replace { config, .. } = &batch.input {
            self.limits = config.limits.clone();
        }
        self.snapshot = Some(snapshot.clone());
        self.snapshot_bytes = bytes;
        Ok(snapshot)
    }
    /// Validates the full replay set before the host replaces the installed sheet.
    #[cfg(test)]
    pub fn validate_candidates(
        &self,
        report: &CompileReport,
        candidates: &[String],
    ) -> Result<usize, UtilityError> {
        self.validate_with_limits(report, candidates, &self.limits, true)
    }
    pub fn validate_with_limits(
        &self,
        report: &CompileReport,
        candidates: &[String],
        limits: &Limits,
        retain: bool,
    ) -> Result<usize, UtilityError> {
        if candidates.len() > limits.candidate_count {
            return Err(CompileError::Limit {
                resource: "candidate_count".into(),
                limit: limits.candidate_count,
            }
            .into());
        }
        if report.css.byte_len() > limits.css_bytes {
            return Err(CompileError::Limit {
                resource: "css_bytes".into(),
                limit: limits.css_bytes,
            }
            .into());
        }
        let mut unique = BTreeSet::new();
        let mut bytes =
            std::mem::size_of::<CommittedSnapshot>().saturating_add(report_bytes(report));
        let previous = self
            .snapshot
            .iter()
            .filter(|_| retain)
            .flat_map(|snapshot| snapshot.classes.iter());
        for candidate in previous.chain(candidates) {
            if unique.insert(candidate.as_str()) {
                if unique.len() > limits.candidate_count {
                    return Err(CompileError::Limit {
                        resource: "candidate_count".into(),
                        limit: limits.candidate_count,
                    }
                    .into());
                }
                // Include the owned String and a conservative share of its BTree node.
                bytes = bytes.saturating_add(candidate.len()).saturating_add(
                    std::mem::size_of::<String>() + 4 * std::mem::size_of::<usize>(),
                );
                if bytes
                    .saturating_add(self.queued_bytes)
                    .saturating_add(self.inflight_bytes)
                    .saturating_add(self.seed_bytes)
                    > limits.cache_bytes
                {
                    return Err(CompileError::Limit {
                        resource: "cache_bytes".into(),
                        limit: limits.cache_bytes,
                    }
                    .into());
                }
            }
        }
        if bytes
            .saturating_add(self.queued_bytes)
            .saturating_add(self.inflight_bytes)
            .saturating_add(self.seed_bytes)
            > limits.cache_bytes
        {
            return Err(CompileError::Limit {
                resource: "cache_bytes".into(),
                limit: limits.cache_bytes,
            }
            .into());
        }
        Ok(bytes)
    }
    pub fn complete(&mut self, batch: Batch, result: Outcome) {
        self.running = false;
        self.inflight_bytes = 0;
        if self.epoch == batch.epoch && matches!(batch.input, Input::Replace { .. }) {
            self.changing = false;
        }
        let result = if self.disposed {
            Err(UtilityError::Disposed)
        } else if self.epoch != batch.epoch {
            Err(UtilityError::Superseded)
        } else {
            result
        };
        for sender in batch.senders {
            let _ = sender.send(result.clone());
        }
    }
    pub fn dispose(&mut self) {
        self.disposed = true;
        for pending in self.queue.drain(..) {
            let _ = pending.sender.send(Err(UtilityError::Disposed));
        }
        self.queued_bytes = 0;
        self.seed_bytes = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustify_style::protocol::CompileMetrics;
    #[test]
    fn utility_error_syntax_message_decodes_original_lone_units_in_both_field_orders() {
        for (escape, unit) in [("\\ud800", 0xd800), ("\\udc00", 0xdc00)] {
            let expected = UtilityError::Compile {
                error: CompileError::Syntax {
                    offset: 7,
                    message: Text::from_units(
                        "before"
                            .encode_utf16()
                            .chain([unit])
                            .chain("after".encode_utf16())
                            .collect(),
                    ),
                },
            };
            for input in [
                format!(
                    r#"{{"kind":"compile","error":{{"kind":"syntax","offset":7,"message":"before{escape}after"}}}}"#
                ),
                format!(
                    r#"{{"error":{{"message":"before{escape}after","offset":7,"kind":"syntax"}},"kind":"compile"}}"#
                ),
            ] {
                let actual: UtilityError = serde_json::from_str(&input).unwrap();
                assert_eq!(actual, expected);
                assert_eq!(
                    serde_json::from_str::<UtilityError>(&serde_json::to_string(&actual).unwrap())
                        .unwrap(),
                    expected
                );
            }
        }
    }

    #[test]
    fn utility_error_range_message_decodes_original_lone_units_in_both_field_orders() {
        for (escape, unit) in [("\\ud800", 0xd800), ("\\udc00", 0xdc00)] {
            let expected = UtilityError::Compile {
                error: CompileError::RangeError {
                    message: Text::from_units(vec![unit, 33]),
                },
            };
            for input in [
                format!(
                    r#"{{"kind":"compile","error":{{"kind":"range_error","message":"{escape}!"}}}}"#
                ),
                format!(
                    r#"{{"error":{{"message":"{escape}!","kind":"range_error"}},"kind":"compile"}}"#
                ),
            ] {
                let actual: UtilityError = serde_json::from_str(&input).unwrap();
                assert_eq!(actual, expected);
                assert_eq!(
                    serde_json::from_str::<UtilityError>(&serde_json::to_string(&actual).unwrap())
                        .unwrap(),
                    expected
                );
            }
        }
    }

    #[test]
    fn utility_error_scalar_variants_keep_existing_json_schema_and_required_fields() {
        let cases = [
            (
                UtilityError::Protocol {
                    message: "protocol".into(),
                },
                r#"{"kind":"protocol","message":"protocol"}"#,
            ),
            (
                UtilityError::Scan {
                    message: "scan".into(),
                },
                r#"{"kind":"scan","message":"scan"}"#,
            ),
            (
                UtilityError::Worker {
                    message: "worker".into(),
                },
                r#"{"kind":"worker","message":"worker"}"#,
            ),
            (
                UtilityError::Install {
                    message: "install".into(),
                },
                r#"{"kind":"install","message":"install"}"#,
            ),
            (UtilityError::Timeout, r#"{"kind":"timeout"}"#),
            (UtilityError::Disposed, r#"{"kind":"disposed"}"#),
            (UtilityError::Superseded, r#"{"kind":"superseded"}"#),
            (
                UtilityError::Compile {
                    error: CompileError::Syntax {
                        offset: 2,
                        message: "syntax".into(),
                    },
                },
                r#"{"kind":"compile","error":{"kind":"syntax","offset":2,"message":"syntax"}}"#,
            ),
            (
                UtilityError::Compile {
                    error: CompileError::RangeError {
                        message: "range".into(),
                    },
                },
                r#"{"kind":"compile","error":{"kind":"range_error","message":"range"}}"#,
            ),
            (
                UtilityError::Compile {
                    error: CompileError::Limit {
                        resource: "cache_bytes".into(),
                        limit: 123,
                    },
                },
                r#"{"kind":"compile","error":{"kind":"limit","resource":"cache_bytes","limit":123}}"#,
            ),
        ];
        for (error, json) in cases {
            assert_eq!(serde_json::to_string(&error).unwrap(), json);
            assert_eq!(serde_json::from_str::<UtilityError>(json).unwrap(), error);
        }
        for input in [
            r#"{"kind":"compile"}"#,
            r#"{"kind":"protocol"}"#,
            r#"{"kind":"worker","message":null}"#,
            r#"{"kind":"unknown"}"#,
        ] {
            assert!(serde_json::from_str::<UtilityError>(input).is_err());
        }
    }

    fn limits() -> Limits {
        Limits {
            input_bytes: 65536,
            asset_bytes: 65536,
            candidate_count: 1024,
            ast_depth: 32,
            css_bytes: 262144,
            queued_bytes: 65536,
            cache_bytes: 1048576,
        }
    }
    #[test]
    fn seed_merge_deduplicates_and_checks_union_before_copying_new_candidates() {
        let mut configured = limits();
        configured.candidate_count = 2;
        let mut candidates = BTreeSet::from(["static".into()]);
        merge_candidates(
            &mut candidates,
            &["static".into(), "dynamic".into()],
            &configured,
        )
        .unwrap();
        assert_eq!(candidates.len(), 2);
        assert!(
            matches!(merge_candidates(&mut candidates, &["overflow".into()], &configured), Err(UtilityError::Compile {error: CompileError::Limit {resource, ..}}) if resource == "candidate_count")
        );
        assert!(!candidates.contains("overflow"));
    }
    #[test]
    fn seed_merge_checks_escaped_bridge_bytes_and_retained_memory() {
        let mut configured = limits();
        configured.queued_bytes = 6;
        let mut candidates = BTreeSet::new();
        assert!(
            matches!(merge_candidates(&mut candidates, &["\u{0001}".into()], &configured), Err(UtilityError::Compile {error: CompileError::Limit {resource, ..}}) if resource == "queued_bytes")
        );
        configured.queued_bytes = 1024;
        configured.cache_bytes = 1;
        candidates.clear();
        assert!(
            matches!(merge_candidates(&mut candidates, &["x".into()], &configured), Err(UtilityError::Compile {error: CompileError::Limit {resource, ..}}) if resource == "cache_bytes")
        );
        assert!(candidates.is_empty());
    }
    #[test]
    fn retained_seed_and_snapshot_share_the_queue_cache_budget() {
        let mut configured = limits();
        configured.cache_bytes = 1024;
        let mut state = State::new(configured.clone());
        state.snapshot_bytes = 512;
        state.set_seed_bytes(512, &configured).unwrap();
        assert!(
            matches!(state.enqueue(&["dynamic".into()]), Err(UtilityError::Compile { error: CompileError::Limit {resource, ..}}) if resource == "cache_bytes")
        );
        assert!(state.set_seed_bytes(513, &configured).is_err());
        assert_eq!(state.seed_bytes, 512);
        state.dispose();
        assert_eq!(state.seed_bytes, 0);
    }
    fn report(revision: u64) -> CompileReport {
        CompileReport {
            revision,
            changed: true,
            css: ".width { width: 137px; }".into(),
            admitted: vec!["w-[137px]".into()],
            rejected: vec![],
            unsupported: vec![],
            gpu_programs: vec![],
            gpu_variables: Default::default(),
            metrics: CompileMetrics::default(),
        }
    }
    #[test]
    fn report_bytes_charges_text_headers_and_actual_spare_capacity() {
        use rustify_style::{
            program::GpuProgram,
            protocol::{CandidateDiagnostic, DiagnosticKind},
        };
        fn spare(unit: u16) -> Text {
            let mut units = Vec::with_capacity(128);
            units.push(unit);
            Text::from_units(units)
        }
        let mut value = report(1);
        value.admitted[0] = Text::default();
        value.rejected = vec![CandidateDiagnostic {
            candidate: Text::default(),
            kind: DiagnosticKind::InvalidSyntax,
            family: Some(Text::default()),
        }];
        value.gpu_programs = vec![GpuProgram {
            candidate: Text::default(),
            conditions: vec![
                GpuCondition::Selector {
                    selector: Text::default(),
                },
                GpuCondition::Media {
                    query: Text::default(),
                },
            ],
            declarations: Vec::new(),
            cascade_order: 0,
        }];
        let baseline = report_bytes(&value);
        value.admitted[0] = spare(0xd800);
        value.rejected[0].candidate = spare(0xdc00);
        value.rejected[0].family = Some(spare(0xe000));
        value.gpu_programs[0].candidate = spare(0xfffd);
        value.gpu_programs[0].conditions[0] = GpuCondition::Selector {
            selector: spare(0xd800),
        };
        value.gpu_programs[0].conditions[1] = GpuCondition::Media {
            query: spare(0xdc00),
        };
        let payload = value.admitted[0].owned_bytes()
            + value.rejected[0].candidate.owned_bytes()
            + value.rejected[0].family.as_ref().unwrap().owned_bytes()
            + value.gpu_programs[0].candidate.owned_bytes()
            + value.gpu_programs[0]
                .conditions
                .iter()
                .map(|condition| match condition {
                    GpuCondition::Selector { selector } => selector.owned_bytes(),
                    GpuCondition::Media { query } => query.owned_bytes(),
                    _ => 0,
                })
                .sum::<usize>();
        assert_eq!(report_bytes(&value), baseline + payload);
        assert!(payload >= 6 * 128 * std::mem::size_of::<u16>());
    }

    fn staged_cn() -> std::rc::Rc<super::super::cn::PreparedCn> {
        leptos::prelude::Owner::new().with(|| {
            let context = super::super::CnContext::new(rustify_style::cn::Cn::default(), 4096);
            std::rc::Rc::new(
                context
                    .prepare(std::rc::Rc::new(rustify_style::cn::Cn::default()), 4096)
                    .unwrap(),
            )
        })
    }
    fn config() -> UtilityConfig {
        UtilityConfig {
            input: rustify_style::protocol::CompilerInput {
                css: "@tailwind utilities;".into(),
                entry: "memory:test.css".into(),
            },
            assets: vec![],
            limits: limits(),
            request_timeout_ms: 1000,
        }
    }
    #[test]
    fn flush_is_a_fifo_barrier_and_keeps_its_completion() {
        let mut state = State::new(limits());
        let _first = state.enqueue(&["first".into()]).unwrap();
        let mut flush = state.enqueue_flush().unwrap();
        let _last = state.enqueue(&["last".into()]).unwrap();
        let first = state.take_batch().unwrap();
        assert!(matches!(&first.input, Input::Classes(classes) if classes == &["first"]));
        let installed = state
            .installed(&first, report(1), &["first".into()])
            .unwrap();
        state.complete(first, Ok(installed.clone()));
        let batch = state.take_batch().unwrap();
        assert!(matches!(batch.input, Input::Flush));
        state.complete(batch, Ok(installed.clone()));
        assert!(Arc::ptr_eq(
            &flush.try_recv().unwrap().unwrap().unwrap(),
            &installed
        ));
        let last = state.take_batch().unwrap();
        assert!(matches!(&last.input, Input::Classes(classes) if classes == &["last"]));
    }
    #[test]
    fn replacement_supersedes_prior_receipts_without_releasing_same_epoch_additions() {
        let mut state = State::new(limits());
        let mut old = state.enqueue(&["old".into()]).unwrap();
        let running = state.take_batch().unwrap();
        let mut queued = state.enqueue(&["queued".into()]).unwrap();
        let mut replace = state
            .enqueue_replace(config(), staged_cn(), None, 256)
            .unwrap();
        let mut next = state.enqueue(&["new".into()]).unwrap();
        assert_eq!(
            queued.try_recv().unwrap().unwrap().unwrap_err(),
            UtilityError::Superseded
        );
        state.complete(running, Err(UtilityError::Timeout));
        assert_eq!(
            old.try_recv().unwrap().unwrap().unwrap_err(),
            UtilityError::Superseded
        );
        let replacement = state.take_batch().unwrap();
        assert!(
            matches!(&replacement.input, Input::Replace { cn, .. } if cn.candidates.is_empty())
        );
        let installed = state
            .installed(&replacement, report(1), &["raw".into()])
            .unwrap();
        state.complete(replacement, Ok(installed));
        assert_eq!(replace.try_recv().unwrap().unwrap().unwrap().epoch, 2);
        let addition = state.take_batch().unwrap();
        assert!(matches!(&addition.input, Input::Classes(classes) if classes == &["new"]));
        let installed = state
            .installed(&addition, report(2), &["new".into()])
            .unwrap();
        state.complete(addition, Ok(installed));
        assert!(next
            .try_recv()
            .unwrap()
            .unwrap()
            .unwrap()
            .classes
            .contains("new"));
    }
    #[test]
    fn failed_replacement_preserves_committed_limits_and_snapshot() {
        let mut state = State::new(limits());
        let _receipt = state.enqueue(&["old".into()]).unwrap();
        let batch = state.take_batch().unwrap();
        let previous = state.installed(&batch, report(1), &["old".into()]).unwrap();
        state.complete(batch, Ok(previous.clone()));
        let mut next_config = config();
        next_config.limits.candidate_count = 1;
        let _replace = state
            .enqueue_replace(next_config, staged_cn(), None, 256)
            .unwrap();
        let batch = state.take_batch().unwrap();
        assert!(state.installed(&batch, report(2), &["new".into()]).is_err());
        state.complete(
            batch,
            Err(UtilityError::Install {
                message: "injected".into(),
            }),
        );
        assert!(Arc::ptr_eq(state.snapshot.as_ref().unwrap(), &previous));
        assert_eq!(state.limits.candidate_count, 1024);
    }
    #[test]
    fn a_batch_coalesces_candidates_and_keeps_every_completion() {
        let mut state = State::new(limits());
        let mut first = state.enqueue(&["w-[137px]".into()]).unwrap();
        let mut second = state
            .enqueue(&["w-[137px]".into(), "bg-[#123456]".into()])
            .unwrap();
        let batch = state.take_batch().unwrap();
        assert!(
            matches!(&batch.input, Input::Classes(classes) if classes == &["bg-[#123456]", "w-[137px]"])
        );
        assert!(state.snapshot.is_none());
        let installed = state
            .installed(
                &batch,
                report(1),
                &["bg-[#123456]".into(), "w-[137px]".into()],
            )
            .unwrap();
        state.complete(batch, Ok(installed));
        assert_eq!(
            first.try_recv().unwrap().unwrap().unwrap().report.revision,
            1
        );
        assert_eq!(
            second.try_recv().unwrap().unwrap().unwrap().report.revision,
            1
        );
    }
    #[test]
    fn queued_requests_cannot_replace_inflight_work_or_exceed_the_shared_budget() {
        let mut configured = limits();
        configured.queued_bytes = 400;
        let mut state = State::new(configured);
        let _receipt = state.enqueue(&["a".repeat(100)]).unwrap();
        let batch = state.take_batch().unwrap();
        assert!(state.enqueue(&["b".repeat(250)]).is_err());
        assert!(state.take_batch().is_none());
        state.complete(
            batch,
            Err(UtilityError::Install {
                message: "injected".into(),
            }),
        );
        assert!(state.snapshot.is_none());
        assert!(state.enqueue(&["b".into()]).is_ok());
    }
    #[test]
    fn ack_failure_preserves_the_installed_snapshot_and_dispose_cancels_receipts() {
        let mut state = State::new(limits());
        let mut first = state.enqueue(&["w-[137px]".into()]).unwrap();
        let batch = state.take_batch().unwrap();
        state
            .installed(&batch, report(1), &["w-[137px]".into()])
            .unwrap();
        state.complete(batch, Err(UtilityError::Timeout));
        assert_eq!(state.snapshot.as_ref().unwrap().report.revision, 1);
        assert_eq!(
            first.try_recv().unwrap().unwrap().unwrap_err(),
            UtilityError::Timeout
        );
        let mut queued = state.enqueue(&["new".into()]).unwrap();
        state.dispose();
        assert_eq!(
            queued.try_recv().unwrap().unwrap().unwrap_err(),
            UtilityError::Disposed
        );
    }
    #[test]
    fn scans_are_fifo_barriers_between_coalesced_class_requests() {
        let mut state = State::new(limits());
        let _first = state.enqueue(&["first".into()]).unwrap();
        let _second = state.enqueue(&["second".into()]).unwrap();
        let content = SourceContent {
            content: "w-[137px]".into(),
            extension: "html".into(),
        };
        let _content = state
            .enqueue_content(std::slice::from_ref(&content))
            .unwrap();
        let _third = state.enqueue(&["third".into()]).unwrap();
        let sources = SourceSnapshot {
            root: "/project".into(),
            base: "/project".into(),
            entries: vec![],
        };
        let _sources = state.enqueue_sources(&sources).unwrap();
        let _fourth = state.enqueue(&["fourth".into()]).unwrap();

        let batch = state.take_batch().unwrap();
        assert!(matches!(&batch.input, Input::Classes(classes) if classes == &["first", "second"]));
        assert_eq!(batch.senders.len(), 2);
        state.complete(batch, Err(UtilityError::Timeout));
        let batch = state.take_batch().unwrap();
        assert!(matches!(&batch.input, Input::Content(contents) if contents == &[content]));
        state.complete(batch, Err(UtilityError::Timeout));
        let batch = state.take_batch().unwrap();
        assert!(matches!(&batch.input, Input::Classes(classes) if classes == &["third"]));
        state.complete(batch, Err(UtilityError::Timeout));
        let batch = state.take_batch().unwrap();
        assert!(matches!(&batch.input, Input::Sources(snapshot) if **snapshot == sources));
        state.complete(batch, Err(UtilityError::Timeout));
        let batch = state.take_batch().unwrap();
        assert!(matches!(&batch.input, Input::Classes(classes) if classes == &["fourth"]));
    }
    #[test]
    fn scan_queue_budget_includes_inflight_content_and_virtual_source_storage() {
        let mut configured = limits();
        configured.queued_bytes = 768;
        let mut state = State::new(configured);
        let _receipt = state
            .enqueue_content(&[SourceContent {
                content: "a".repeat(400),
                extension: "html".into(),
            }])
            .unwrap();
        let batch = state.take_batch().unwrap();
        assert!(state.enqueue(&["b".repeat(100)]).is_err());
        assert!(state
            .enqueue_sources(&SourceSnapshot {
                root: "/".into(),
                base: "/".into(),
                entries: vec![VirtualSourceEntry::File {
                    path: "a.html".into(),
                    content: "b".repeat(100),
                    revision: 1
                }]
            })
            .is_err());
        state.complete(
            batch,
            Err(UtilityError::Scan {
                message: "injected".into(),
            }),
        );
        assert!(state
            .enqueue_sources(&SourceSnapshot {
                root: "/".into(),
                base: "/".into(),
                entries: vec![VirtualSourceEntry::Symlink {
                    path: "link".into(),
                    target: "a".repeat(800)
                }]
            })
            .is_err());
        assert!(state.enqueue(&["b".into()]).is_ok());
    }
    #[test]
    fn scan_failures_keep_installed_candidates_and_ack_loss_keeps_variables_for_replay() {
        let mut state = State::new(limits());
        let _receipt = state.enqueue(&["w-[137px]".into()]).unwrap();
        let batch = state.take_batch().unwrap();
        let original = state
            .installed(&batch, report(1), &["w-[137px]".into()])
            .unwrap();
        state.complete(batch, Ok(original.clone()));

        let mut failed = state
            .enqueue_content(&[SourceContent {
                content: "invalid source".into(),
                extension: "html".into(),
            }])
            .unwrap();
        let batch = state.take_batch().unwrap();
        state.complete(
            batch,
            Err(UtilityError::Scan {
                message: "injected".into(),
            }),
        );
        assert!(Arc::ptr_eq(state.snapshot.as_ref().unwrap(), &original));
        assert!(matches!(
            failed.try_recv().unwrap().unwrap(),
            Err(UtilityError::Scan { .. })
        ));

        let mut installed = state
            .enqueue_content(&[SourceContent {
                content: "<div class=\"bg-red-500\" style=\"color:var(--application-color)\">"
                    .into(),
                extension: "html".into(),
            }])
            .unwrap();
        let batch = state.take_batch().unwrap();
        state
            .installed(
                &batch,
                report(2),
                &["bg-red-500".into(), "--application-color".into()],
            )
            .unwrap();
        state.complete(batch, Err(UtilityError::Timeout));
        assert_eq!(
            installed.try_recv().unwrap().unwrap().unwrap_err(),
            UtilityError::Timeout
        );
        assert_eq!(
            state.snapshot.as_ref().unwrap().classes,
            BTreeSet::from([
                "w-[137px]".into(),
                "bg-red-500".into(),
                "--application-color".into()
            ])
        );
        assert_eq!(state.snapshot.as_ref().unwrap().report.revision, 2);
    }
    #[test]
    fn installation_rejects_union_candidate_limits_without_changing_the_snapshot() {
        let mut configured = limits();
        configured.candidate_count = 2;
        let mut state = State::new(configured);
        let _first = state.enqueue(&["first".into()]).unwrap();
        let batch = state.take_batch().unwrap();
        let original = state
            .installed(&batch, report(1), &["first".into()])
            .unwrap();
        state.complete(batch, Ok(original.clone()));
        let _second = state.enqueue(&["second".into()]).unwrap();
        let batch = state.take_batch().unwrap();
        assert!(
            matches!(state.validate_candidates(&report(2), &["second".into(), "--variable".into()]), Err(UtilityError::Compile { error: CompileError::Limit { resource, .. } }) if resource == "candidate_count")
        );
        assert!(state
            .installed(&batch, report(2), &["second".into(), "--variable".into()])
            .is_err());
        assert!(Arc::ptr_eq(state.snapshot.as_ref().unwrap(), &original));
    }
    #[test]
    fn rebuild_installs_only_the_active_set_and_frees_the_candidate_quota() {
        let mut configured = limits();
        configured.candidate_count = 2;
        let mut state = State::new(configured.clone());
        let _first = state.enqueue(&["seed".into(), "stale".into()]).unwrap();
        let batch = state.take_batch().unwrap();
        let original = state
            .installed(&batch, report(1), &["seed".into(), "stale".into()])
            .unwrap();
        state.complete(batch, Ok(original.clone()));
        let mut rebuild_config = config();
        rebuild_config.limits = configured;
        let _rebuild = state
            .enqueue_replace(
                rebuild_config.clone(),
                staged_cn(),
                Some(vec!["seed".into()]),
                256,
            )
            .unwrap();
        let batch = state.take_batch().unwrap();
        assert!(!batch.retains_installed());
        assert!(
            matches!(state.validate_with_limits(&report(2), &["seed".into(), "fresh".into()], &rebuild_config.limits, true), Err(UtilityError::Compile { error: CompileError::Limit { resource, .. } }) if resource == "candidate_count")
        );
        // An oversized active set fails without touching the installed snapshot.
        assert!(state
            .installed(
                &batch,
                report(2),
                &["seed".into(), "fresh".into(), "third".into()]
            )
            .is_err());
        assert!(Arc::ptr_eq(state.snapshot.as_ref().unwrap(), &original));
        let rebuilt = state
            .installed(&batch, report(2), &["seed".into()])
            .unwrap();
        state.complete(batch, Ok(rebuilt.clone()));
        assert_eq!(rebuilt.classes, BTreeSet::from(["seed".to_string()]));
        let _fresh = state.enqueue(&["fresh".into()]).unwrap();
        let batch = state.take_batch().unwrap();
        assert!(batch.retains_installed());
        let installed = state
            .installed(&batch, report(3), &["fresh".into()])
            .unwrap();
        assert_eq!(
            installed.classes,
            BTreeSet::from(["fresh".to_string(), "seed".to_string()])
        );
    }
    #[test]
    fn retained_snapshot_and_queued_inputs_share_the_cache_budget() {
        let mut configured = limits();
        configured.cache_bytes = 1400;
        let mut state = State::new(configured);
        let _receipt = state.enqueue(&["first".into()]).unwrap();
        let batch = state.take_batch().unwrap();
        let mut configured_report = report(1);
        configured_report.css = "a".repeat(100).into();
        let installed = state
            .installed(&batch, configured_report, &["first".into()])
            .unwrap();
        state.complete(batch, Ok(installed));
        assert!(
            matches!(state.enqueue_content(&[SourceContent { content: "b".repeat(1000), extension: "html".into() }]), Err(UtilityError::Compile { error: CompileError::Limit { resource, .. } }) if resource == "cache_bytes")
        );
        assert!(state
            .validate_candidates(&report(2), &["a".repeat(1800)])
            .is_err());
    }
    #[test]
    fn serialization_checks_escaped_source_size_before_growing_past_the_limit() {
        let content = vec![SourceContent {
            content: "\u{0000}".repeat(100),
            extension: "html".into(),
        }];
        assert!(
            matches!(serialize_input(&content, 200), Err(UtilityError::Compile { error: CompileError::Limit { resource, limit: 200 } }) if resource == "queued_bytes")
        );
        let serialized = serialize_input(&content, 1024).unwrap();
        assert_eq!(
            serde_json::from_str::<Vec<SourceContent>>(&serialized).unwrap(),
            content
        );
    }
    #[test]
    fn retained_gpu_bindings_count_original_utf16_allocation_bytes() {
        let mut configured = limits();
        configured.cache_bytes = 1600;
        let state = State::new(configured);
        let mut prepared = report(1);
        prepared.gpu_variables.insert(
            "--probe".into(),
            rustify_style::text::Text::from_units(vec![0xd800; 800]),
        );
        assert!(
            matches!(state.validate_candidates(&prepared, &[]), Err(UtilityError::Compile { error: CompileError::Limit { resource, .. } }) if resource == "cache_bytes")
        );
        assert!(state.snapshot.is_none());
    }
}
