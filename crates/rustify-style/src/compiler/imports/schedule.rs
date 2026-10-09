//! The Promise subset used by imports: settled await, Promise.all, and forwarding.
//! Ready jobs remain owned after the root observer has run.
use super::super::{public_api::InputMutation, CompileError, Limits};
use std::{collections::VecDeque, mem::size_of, sync::Arc};

#[derive(Clone, Copy, Debug)]
pub(crate) struct PromiseId(usize);
#[derive(Clone, Debug)]
pub(crate) enum Fault {
    Native(Arc<CompileError>),
    Host(usize),
}
impl From<CompileError> for Fault {
    fn from(error: CompileError) -> Self {
        Self::Native(Arc::new(error))
    }
}
pub(crate) type Settlement = Result<(), Fault>;
#[derive(Clone, Debug)]
pub(crate) enum Reaction {
    Loaded(usize),
    Nested(usize),
    Member(usize),
    Forward(PromiseId),
    ParseCss,
    Compatibility,
    CompileAst,
    Observe,
}
#[derive(Clone, Debug)]
pub(crate) struct Job {
    pub reaction: Reaction,
    pub outcome: Settlement,
}
#[derive(Default)]
struct Cell {
    outcome: Option<Settlement>,
    reactions: Vec<Reaction>,
    handled: bool,
}
pub(crate) struct Scope {
    pub returned: PromiseId,
    pub all: PromiseId,
    pub members: Vec<usize>,
    pub error: Option<CompileError>,
}
pub(crate) struct Task {
    pub returned: PromiseId,
    pub launch_error: Option<CompileError>,
    pub loader_error: Option<CompileError>,
    pub loaded: Option<Result<usize, CompileError>>,
    pub begin: Option<InputMutation>,
    pub complete: Option<InputMutation>,
}
pub(crate) struct Program {
    pub scopes: Vec<Scope>,
    pub tasks: Vec<Task>,
    pub parse_css: PromiseId,
    pub compatibility: PromiseId,
    pub compile_ast: PromiseId,
    pub sources: Vec<Arc<super::super::public_api::Source>>,
    pub promise_count: usize,
    pub prefix_length: usize,
}
impl Program {
    pub fn new(machine: &mut Machine) -> Result<Self, CompileError> {
        Ok(Self {
            scopes: vec![],
            tasks: vec![],
            sources: vec![],
            parse_css: machine.promise()?,
            compatibility: machine.promise()?,
            compile_ast: machine.promise()?,
            promise_count: 0,
            prefix_length: 0,
        })
    }
    pub fn effects(&self) -> impl Iterator<Item = &InputMutation> {
        self.tasks
            .iter()
            .flat_map(|task| task.begin.iter().chain(task.complete.iter()))
    }
    pub fn graph_bytes(&self, seen: &mut std::collections::BTreeSet<(u8, usize)>) -> usize {
        let values = self
            .effects()
            .map(|effect| {
                super::super::public_api::design_system::json_bytes_seen(effect.target(), seen)
                    .saturating_add(super::super::public_api::design_system::json_bytes_seen(
                        effect.value(),
                        seen,
                    ))
            })
            .sum::<usize>();
        values.saturating_add(
            self.sources
                .iter()
                .filter(|source| seen.insert((5, Arc::as_ptr(source) as usize)))
                .map(|source| {
                    size_of::<super::super::public_api::Source>()
                        + 2 * size_of::<usize>()
                        + source.code.owned_bytes()
                        + source.file.as_ref().map_or(0, String::capacity)
                })
                .sum::<usize>(),
        )
    }
    pub fn owned_bytes(&self) -> usize {
        fn error_bytes(error: &CompileError) -> usize {
            match error {
                CompileError::Syntax { message, .. } | CompileError::RangeError { message } => {
                    message.owned_bytes()
                }
                CompileError::Limit { resource, .. } | CompileError::InvalidLimit { resource } => {
                    resource.capacity()
                }
                CompileError::UnsupportedFeature { feature }
                | CompileError::PendingFeature { feature } => feature.capacity(),
                CompileError::DuplicateAsset { id } => id.capacity(),
                CompileError::MissingAsset { id, base } => {
                    id.capacity().saturating_add(base.capacity())
                }
                CompileError::PendingPrepared
                | CompileError::StalePrepared
                | CompileError::RevisionExhausted => 0,
            }
        }
        let errors = self
            .scopes
            .iter()
            .filter_map(|scope| scope.error.as_ref())
            .chain(self.tasks.iter().flat_map(|task| {
                task.launch_error
                    .iter()
                    .chain(task.loader_error.iter())
                    .chain(
                        task.loaded
                            .as_ref()
                            .and_then(|loaded| loaded.as_ref().err()),
                    )
            }))
            .map(error_bytes)
            .sum::<usize>();
        size_of::<Self>()
            .saturating_add(self.scopes.capacity().saturating_mul(size_of::<Scope>()))
            .saturating_add(self.tasks.capacity().saturating_mul(size_of::<Task>()))
            .saturating_add(
                self.sources
                    .capacity()
                    .saturating_mul(size_of::<Arc<super::super::public_api::Source>>()),
            )
            .saturating_add(
                self.scopes
                    .iter()
                    .map(|scope| scope.members.capacity().saturating_mul(size_of::<usize>()))
                    .sum::<usize>(),
            )
            // Stored faults and active settlement owners can overlap.
            .saturating_add(errors.saturating_mul(2))
            // The crossing retains opaque host exception handles until quiescence.
            .saturating_add(
                self.tasks
                    .len()
                    .saturating_mul(2 * (2 * size_of::<usize>() + 64) + 64),
            )
            // Execution cells/reactions and queued callback captures overlap the program.
            .saturating_add(self.promise_count.saturating_mul(
                size_of::<Cell>() + 4 * size_of::<Job>() + 2 * size_of::<PromiseId>(),
            ))
    }
}
pub(crate) struct Machine {
    cells: Vec<Cell>,
    ready: VecDeque<Job>,
    remaining: Vec<usize>,
    limits: Limits,
    reaction_count: usize,
    outstanding_jobs: usize,
    boundary: Boundary,
    unhandled: Vec<PromiseId>,
}
#[derive(Clone, Copy)]
pub(crate) enum Boundary {
    Observed,
    Promise,
}
impl Machine {
    pub fn new(limits: &Limits) -> Self {
        Self {
            cells: vec![],
            ready: VecDeque::new(),
            remaining: vec![],
            limits: limits.clone(),
            reaction_count: 0,
            outstanding_jobs: 0,
            boundary: Boundary::Observed,
            unhandled: vec![],
        }
    }
    pub fn prepared(program: &Program, limits: &Limits) -> Result<Self, CompileError> {
        super::super::check_limit(
            program.promise_count.saturating_mul(2 * size_of::<Job>()),
            limits.queued_bytes,
            "queued_bytes",
        )?;
        let mut machine = Self::new(limits);
        for _ in 0..program.promise_count {
            machine.promise()?;
        }
        machine.remaining.resize(program.scopes.len(), 0);
        Ok(machine)
    }
    pub fn promise(&mut self) -> Result<PromiseId, CompileError> {
        super::super::check_limit(
            (self.cells.len() + 1).saturating_mul(2 * size_of::<Cell>()),
            self.limits.cache_bytes,
            "cache_bytes",
        )?;
        let promise = PromiseId(self.cells.len());
        self.cells.push(Cell::default());
        Ok(promise)
    }
    fn queue(&mut self, reaction: Reaction, outcome: Settlement) -> Result<(), CompileError> {
        super::super::check_limit(
            (self.outstanding_jobs + 1).saturating_mul(2 * size_of::<Job>()),
            self.limits.queued_bytes,
            "queued_bytes",
        )?;
        self.outstanding_jobs += 1;
        self.ready.push_back(Job { reaction, outcome });
        Ok(())
    }
    pub fn react(&mut self, promise: PromiseId, reaction: Reaction) -> Result<(), CompileError> {
        self.cells[promise.0].handled = true;
        if let Some(outcome) = &self.cells[promise.0].outcome {
            self.queue(reaction, outcome.clone())
        } else {
            super::super::check_limit(
                (self.reaction_count + 1).saturating_mul(2 * size_of::<Reaction>()),
                self.limits.cache_bytes,
                "cache_bytes",
            )?;
            self.cells[promise.0].reactions.push(reaction);
            self.reaction_count += 1;
            Ok(())
        }
    }
    pub fn settle(&mut self, promise: PromiseId, outcome: Settlement) -> Result<(), CompileError> {
        let cell = &mut self.cells[promise.0];
        if cell.outcome.is_some() {
            return Ok(());
        }
        cell.outcome = Some(outcome.clone());
        if outcome.is_err() && !cell.handled {
            self.unhandled.push(promise);
        }
        let reactions = std::mem::take(&mut cell.reactions);
        self.reaction_count -= reactions.len();
        for reaction in reactions {
            self.queue(reaction, outcome.clone())?;
        }
        Ok(())
    }
    pub fn pop(&mut self) -> Option<Job> {
        self.ready.pop_front()
    }
    pub fn take_ready(&mut self) -> Vec<Job> {
        self.ready.drain(..).collect()
    }
    pub fn has_pending_jobs(&self) -> bool {
        self.outstanding_jobs != 0
    }
    pub fn promise_count(&self) -> usize {
        self.cells.len()
    }
    pub fn take_unhandled(&mut self) -> Vec<Fault> {
        std::mem::take(&mut self.unhandled)
            .into_iter()
            .filter_map(|promise| {
                let cell = &self.cells[promise.0];
                if cell.handled {
                    return None;
                }
                cell.outcome
                    .as_ref()
                    .and_then(|outcome| outcome.as_ref().err())
                    .cloned()
            })
            .collect()
    }
    pub fn start<H: Host>(
        &mut self,
        program: &mut Program,
        host: &mut H,
        boundary: Boundary,
    ) -> Result<(), CompileError> {
        self.boundary = boundary;
        // Actual Promise rejection reporting belongs to the exported JS
        // Promise; this private settlement must not report it a second time.
        if matches!(boundary, Boundary::Promise) {
            self.cells[program.compile_ast.0].handled = true;
        }
        self.walk(program, host, 0)?;
        self.react(program.scopes[0].returned, Reaction::ParseCss)?;
        self.react(program.parse_css, Reaction::CompileAst)?;
        if matches!(boundary, Boundary::Observed) {
            self.react(program.compile_ast, Reaction::Observe)?;
        }
        Ok(())
    }
    fn walk<H: Host>(
        &mut self,
        program: &mut Program,
        host: &mut H,
        scope: usize,
    ) -> Result<(), CompileError> {
        if self.remaining.len() <= scope {
            self.remaining.resize(scope + 1, 0);
        }
        for ordinal in 0..program.scopes[scope].members.len() {
            let task = program.scopes[scope].members[ordinal];
            if let Some(error) = program.tasks[task].launch_error.clone() {
                self.settle(program.tasks[task].returned, Err(error.into()))?;
            } else {
                let loaded = program.tasks[task]
                    .loader_error
                    .clone()
                    .map_or(Ok(()), |error| Err(error.into()));
                self.queue(Reaction::Loaded(task), loaded)?;
            }
            if let Err(fault) = host.begin(program, self, task)? {
                self.settle(program.scopes[scope].returned, Err(fault))?;
                return Ok(());
            }
        }
        if let Some(error) = program.scopes[scope].error.clone() {
            return self.settle(program.scopes[scope].returned, Err(error.into()));
        }
        let count = program.scopes[scope].members.len();
        if count == 0 {
            return self.settle(program.scopes[scope].returned, Ok(()));
        }
        self.remaining[scope] = count;
        for ordinal in 0..count {
            let task = program.scopes[scope].members[ordinal];
            self.react(program.tasks[task].returned, Reaction::Member(scope))?;
        }
        self.react(
            program.scopes[scope].all,
            Reaction::Forward(program.scopes[scope].returned),
        )
    }
    pub fn drive<H: Host>(
        &mut self,
        program: &mut Program,
        host: &mut H,
        job: Job,
    ) -> Result<Option<Settlement>, CompileError> {
        self.outstanding_jobs -= 1;
        let outcome = job.outcome;
        match job.reaction {
            Reaction::Loaded(task) => {
                if let Err(fault) = outcome {
                    self.settle(program.tasks[task].returned, Err(fault))?;
                } else {
                    match host.loaded(program, self, task)? {
                        Ok(scope) => {
                            self.walk(program, host, scope)?;
                            self.react(program.scopes[scope].returned, Reaction::Nested(task))?;
                        }
                        Err(fault) => self.settle(program.tasks[task].returned, Err(fault))?,
                    }
                }
            }
            Reaction::Nested(task) => {
                let outcome = if outcome.is_ok() {
                    host.complete(program, task)?
                } else {
                    outcome
                };
                self.settle(program.tasks[task].returned, outcome)?;
            }
            Reaction::Member(scope) => match outcome {
                Err(fault) => self.settle(program.scopes[scope].all, Err(fault))?,
                Ok(()) => {
                    self.remaining[scope] -= 1;
                    if self.remaining[scope] == 0 {
                        self.settle(program.scopes[scope].all, Ok(()))?;
                    }
                }
            },
            Reaction::Forward(promise) => self.settle(promise, outcome)?,
            Reaction::ParseCss => {
                if outcome.is_ok() {
                    self.settle(program.compatibility, Ok(()))?;
                    self.react(program.compatibility, Reaction::Compatibility)?;
                } else {
                    self.settle(program.parse_css, outcome)?;
                }
            }
            Reaction::Compatibility => self.settle(program.parse_css, outcome)?,
            Reaction::CompileAst => {
                self.settle(program.compile_ast, outcome.clone())?;
                if matches!(self.boundary, Boundary::Promise) {
                    return Ok(Some(outcome));
                }
            }
            Reaction::Observe => return Ok(Some(outcome)),
        }
        Ok(None)
    }
}
/// A preparation host parses/assembles native fragments; an execution host
/// acknowledges the prepared effects. Both use these same reaction transitions.
pub(crate) trait Host {
    fn begin(
        &mut self,
        program: &mut Program,
        machine: &mut Machine,
        task: usize,
    ) -> Result<Settlement, CompileError>;
    fn loaded(
        &mut self,
        program: &mut Program,
        machine: &mut Machine,
        task: usize,
    ) -> Result<Result<usize, Fault>, CompileError>;
    fn complete(&mut self, program: &mut Program, task: usize) -> Result<Settlement, CompileError>;
}
