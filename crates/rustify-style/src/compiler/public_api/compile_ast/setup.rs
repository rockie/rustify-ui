//! Admitted setup replay with host acknowledgement and owned detached jobs.
use super::*;
use compiler::imports::schedule::{
    Boundary, Fault, Host, Job, Machine, Program, Reaction, Scope, Settlement,
};

/// A native semantic failure or the host's opaque original-exception token.
#[derive(Debug)]
pub enum AstSetupFault {
    Semantic(CompileError),
    Host(usize),
}
/// One owned reaction. Hosts post every returned job individually when created.
pub struct AstSetupJob(Job);
/// Prepared native setup. No caller writes occur until `start` is invoked.
pub struct PreparedAstSetup {
    outcome: Option<AstSetupOutcome>,
    program: Option<Program>,
    machine: Option<Machine>,
    reserve_mutations: Vec<InputMutation>,
    cache_bytes: usize,
    prefix_length: usize,
    compatibility: Option<usize>,
    publication: Option<Result<(), AstSetupFault>>,
    started: bool,
    has_loader: bool,
}
struct Replay<'a> {
    apply: &'a mut dyn FnMut(&InputMutation) -> Result<(), usize>,
    post: &'a mut dyn FnMut(AstSetupJob),
}
impl Host for Replay<'_> {
    fn begin(
        &mut self,
        program: &mut Program,
        machine: &mut Machine,
        task: usize,
    ) -> Result<Settlement, CompileError> {
        // The loader await precedes the setter, including any external markers
        // queued by that setter. Post at creation, rather than after the walk.
        for job in machine.take_ready() {
            (self.post)(AstSetupJob(job));
        }
        Ok(program.tasks[task]
            .begin
            .as_ref()
            .map_or(Ok(()), |effect| (self.apply)(effect).map_err(Fault::Host)))
    }
    fn loaded(
        &mut self,
        program: &mut Program,
        _machine: &mut Machine,
        task: usize,
    ) -> Result<Result<usize, Fault>, CompileError> {
        Ok(match &program.tasks[task].loaded {
            Some(Ok(scope)) => Ok(*scope),
            Some(Err(error)) => Err(error.clone().into()),
            None => Err(CompileError::PendingFeature {
                feature: "unprepared import occurrence".into(),
            }
            .into()),
        })
    }
    fn complete(&mut self, program: &mut Program, task: usize) -> Result<Settlement, CompileError> {
        Ok(program.tasks[task]
            .complete
            .as_ref()
            .map_or(Ok(()), |effect| (self.apply)(effect).map_err(Fault::Host)))
    }
}
fn mutations(outcome: &AstSetupOutcome) -> &[InputMutation] {
    match outcome {
        AstSetupOutcome::Complete(compiler) => compiler.input_mutations(),
        AstSetupOutcome::Failed(failure) => &failure.input_mutations,
    }
}
/// Prepare actual native fragments and the complete finite-bundle reserve.
pub fn prepare_ast_setup(
    input: PublicAst,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
) -> Result<PreparedAstSetup, CompileError> {
    let has_loader = !assets.is_empty();
    let mut outcome = compile_ast_setup(input, options, assets, limits.clone())?;
    let (program, mut cache_bytes) = match &mut outcome {
        AstSetupOutcome::Complete(compiler) => (
            compiler.import_program.take(),
            compiler.retained_cache_bytes(),
        ),
        AstSetupOutcome::Failed(failure) => (failure.import_program.take(), failure.cache_bytes),
    };
    let prefix_length = program.as_ref().map_or(0, |program| program.prefix_length);
    let compatibility = match &outcome {
        AstSetupOutcome::Complete(compiler) => compiler.setup_compatibility,
        AstSetupOutcome::Failed(failure) => failure.setup_compatibility,
    };
    let reserve_mutations = program.as_ref().map_or_else(Vec::new, |program| {
        program
            .effects()
            .cloned()
            .chain(mutations(&outcome)[prefix_length..].iter().cloned())
            .collect::<Vec<_>>()
    });
    cache_bytes = cache_bytes.saturating_add(
        reserve_mutations
            .capacity()
            .saturating_mul(std::mem::size_of::<InputMutation>()),
    );
    compiler::check_limit(cache_bytes, limits.cache_bytes, "cache_bytes")?;
    let machine = program
        .as_ref()
        .map(|program| Machine::prepared(program, &limits))
        .transpose()?;
    Ok(PreparedAstSetup {
        outcome: Some(outcome),
        program,
        machine,
        reserve_mutations,
        cache_bytes,
        prefix_length,
        compatibility,
        publication: None,
        started: false,
        has_loader,
    })
}
/// Prepare a native Promise boundary, including the awaits when no imports exist.
pub fn prepare_ast_setup_async(
    input: PublicAst,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
) -> Result<PreparedAstSetup, CompileError> {
    let mut setup = prepare_ast_setup(input, options, assets, limits.clone())?;
    if setup.program.is_none() {
        let mut machine = Machine::new(&limits);
        let mut program = Program::new(&mut machine)?;
        program.scopes.push(Scope {
            returned: machine.promise()?,
            all: machine.promise()?,
            members: vec![],
            error: None,
        });
        program.promise_count = machine.promise_count();
        setup.cache_bytes = setup.cache_bytes.saturating_add(program.owned_bytes());
        compiler::check_limit(setup.cache_bytes, limits.cache_bytes, "cache_bytes")?;
        setup.machine = Some(Machine::prepared(&program, &limits)?);
        setup.program = Some(program);
    }
    Ok(setup)
}
impl PreparedAstSetup {
    /// Native reserve, including the complete failed-session continuation.
    pub fn cache_bytes(&self) -> usize {
        self.cache_bytes
    }
    /// All potential host values, including writes after root rejection.
    pub fn reserve_mutations(&self) -> &[InputMutation] {
        if !self.reserve_mutations.is_empty() {
            &self.reserve_mutations
        } else {
            self.outcome.as_ref().map_or(&[], mutations)
        }
    }
    fn post_imports(
        &mut self,
        range: std::ops::Range<usize>,
        completed: bool,
        apply: &mut dyn FnMut(&InputMutation) -> Result<(), usize>,
    ) -> Settlement {
        let Some(outcome) = &self.outcome else {
            return Ok(());
        };
        for effect in &mutations(outcome)[range] {
            if let Err(token) = apply(effect) {
                return Err(Fault::Host(token));
            }
        }
        match outcome {
            _ if !completed => Ok(()),
            AstSetupOutcome::Complete(_) => Ok(()),
            AstSetupOutcome::Failed(failure) => Err(failure.error.clone().into()),
        }
    }
    fn fault(&self, fault: Fault) -> AstSetupFault {
        match fault {
            Fault::Host(token) => AstSetupFault::Host(token),
            Fault::Native(error)
                if !self.has_loader && matches!(&*error, CompileError::MissingAsset { .. }) =>
            {
                AstSetupFault::Semantic(syntax(
                    "No `loadStylesheet` function provided to `compile`",
                ))
            }
            Fault::Native(error) => AstSetupFault::Semantic((*error).clone()),
        }
    }
    fn publish(&mut self, outcome: Settlement) {
        if self.publication.is_some() {
            return;
        }
        self.publication = Some(outcome.map_err(|fault| self.fault(fault)));
    }
    /// Begin the synchronous walk; host faults stop that walk, retaining tasks
    /// launched before the failing replacement. Call exactly once.
    pub fn start(
        &mut self,
        mut apply: impl FnMut(&InputMutation) -> Result<(), usize>,
    ) -> Result<Vec<AstSetupJob>, CompileError> {
        let mut jobs = vec![];
        self.start_posted(Boundary::Observed, &mut apply, &mut |job| jobs.push(job))?;
        Ok(jobs)
    }
    /// Start the actual Promise boundary. Post each callback immediately; no
    /// session borrow may be held across a user setter or a posting callback.
    pub fn start_async(
        &mut self,
        mut apply: impl FnMut(&InputMutation) -> Result<(), usize>,
        mut post: impl FnMut(AstSetupJob),
    ) -> Result<(), CompileError> {
        self.start_posted(Boundary::Promise, &mut apply, &mut post)
    }
    fn start_posted(
        &mut self,
        boundary: Boundary,
        apply: &mut dyn FnMut(&InputMutation) -> Result<(), usize>,
        post: &mut dyn FnMut(AstSetupJob),
    ) -> Result<(), CompileError> {
        if self.started {
            return Ok(());
        }
        self.started = true;
        if let (Some(machine), Some(program)) = (&mut self.machine, &mut self.program) {
            machine.start(program, &mut Replay { apply, post }, boundary)?;
            for job in machine.take_ready() {
                post(AstSetupJob(job));
            }
        } else {
            let length = self
                .outcome
                .as_ref()
                .map_or(0, |outcome| mutations(outcome).len());
            let outcome = self.post_imports(self.prefix_length..length, true, apply);
            self.publish(outcome);
        }
        Ok(())
    }
    /// Execute one previously returned reaction, preserving the original host
    /// exception token. Newly activated jobs must each be posted separately.
    pub fn drive(
        &mut self,
        job: AstSetupJob,
        mut apply: impl FnMut(&InputMutation) -> Result<(), usize>,
    ) -> Result<Vec<AstSetupJob>, CompileError> {
        let mut jobs = vec![];
        self.drive_async(job, &mut apply, &mut |job| jobs.push(job))?;
        Ok(jobs)
    }
    /// Run one posted callback; new jobs are posted at creation before setters.
    pub fn drive_async(
        &mut self,
        job: AstSetupJob,
        mut apply: impl FnMut(&InputMutation) -> Result<(), usize>,
        mut post: impl FnMut(AstSetupJob),
    ) -> Result<(), CompileError> {
        let Job { reaction, outcome } = job.0;
        let length = self
            .outcome
            .as_ref()
            .map_or(0, |outcome| mutations(outcome).len());
        let outcome = match (&reaction, outcome) {
            (Reaction::ParseCss, Ok(())) => self.post_imports(
                self.prefix_length..self.compatibility.unwrap_or(length),
                self.compatibility.is_none(),
                &mut apply,
            ),
            (Reaction::Compatibility, Ok(())) => self.post_imports(
                self.compatibility.unwrap_or(length)..length,
                true,
                &mut apply,
            ),
            (_, outcome) => outcome,
        };
        let observed =
            if let (Some(machine), Some(program)) = (&mut self.machine, &mut self.program) {
                machine.drive(
                    program,
                    &mut Replay {
                        apply: &mut apply,
                        post: &mut post,
                    },
                    Job { reaction, outcome },
                )?
            } else {
                None
            };
        if let Some(outcome) = observed {
            self.publish(outcome);
        }
        if let Some(machine) = &mut self.machine {
            for job in machine.take_ready() {
                post(AstSetupJob(job));
            }
        }
        Ok(())
    }
    /// Whether already launched reactions still own work after publication.
    pub fn has_pending_jobs(&self) -> bool {
        self.machine.as_ref().is_some_and(Machine::has_pending_jobs)
    }
    /// Rejected import tasks left without an await/aggregate when their
    /// synchronous walk failed. The host reports each original fault once.
    pub fn take_unhandled(&mut self) -> Vec<AstSetupFault> {
        let faults = self
            .machine
            .as_mut()
            .map_or_else(Vec::new, Machine::take_unhandled);
        faults.into_iter().map(|fault| self.fault(fault)).collect()
    }
    /// Takes the single publication. Remaining owned jobs may still run after
    /// an error; success has already joined all import tasks.
    pub fn take_result(&mut self) -> Option<Result<AstCompiler, AstSetupFault>> {
        let publication = self.publication.take()?;
        match publication {
            Err(error) => {
                self.outcome.take();
                Some(Err(error))
            }
            Ok(()) => match self.outcome.take()? {
                AstSetupOutcome::Complete(compiler) => Some(Ok(*compiler)),
                AstSetupOutcome::Failed(failure) => {
                    Some(Err(AstSetupFault::Semantic(failure.error)))
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    fn limits() -> Limits {
        Limits {
            input_bytes: 262144,
            asset_bytes: 262144,
            candidate_count: 1024,
            ast_depth: 32,
            css_bytes: 262144,
            queued_bytes: 262144,
            cache_bytes: 16000000,
        }
    }
    fn input(ids: &[&str]) -> PublicAst {
        Arc::new(ids.iter().map(|id| public_json!({"kind":"at-rule", "name":"@import", "params":format!("\"{id}\""), "nodes":public_json!([])})).collect())
    }
    fn asset(id: &str, content: &str) -> Stylesheet {
        Stylesheet {
            id: id.into(),
            path: Some(format!("/mapped/{id}")),
            base: Some("".into()),
            content: content.into(),
        }
    }
    #[test]
    fn parallel_import_failure_selects_first_settlement_instead_of_first_ast_visit() {
        let error = compile_ast(
            input(&["outer.css", "direct-missing.css"]),
            CompileOptions::default(),
            vec![asset("outer.css", "@import \"indirect-missing.css\";")],
            limits(),
        )
        .err();
        assert!(
            matches!(error, Some(CompileError::MissingAsset { id, .. }) if id == "direct-missing.css")
        );
    }
    #[test]
    fn failed_parser_source_owner_is_reserved_before_any_host_write() {
        let css = format!(".broken {{{}", " ".repeat(65536));
        let mut limits = limits();
        limits.cache_bytes = 32768;
        let options = CompileOptions {
            from: Some("entry.css".into()),
            ..CompileOptions::default()
        };
        assert!(
            matches!(prepare_ast_setup(input(&["bad.css"]), options, vec![asset("bad.css", &css)], limits),
            Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes")
        );
    }
    #[test]
    fn task_launched_before_failed_begin_keeps_its_context_and_source_until_quiescence() {
        let root = input(&["leaf.css", "chain.css"]);
        let options = CompileOptions {
            from: Some("entry.css".into()),
            ..CompileOptions::default()
        };
        let assets = vec![
            asset("leaf.css", ".leaf { color: red; }"),
            asset("chain.css", "@import \"middle.css\";"),
            asset("middle.css", "@import \"leaf.css\";"),
        ];
        let mut setup = prepare_ast_setup(root.clone(), options, assets, limits()).unwrap();
        let source = Arc::downgrade(&setup.program.as_ref().unwrap().sources[0]);
        let mut captured = None;
        let mut tail_completed = false;
        let mut apply = |effect: &InputMutation| {
            if let InputMutation::Set {
                target: Value::Array(parent),
                field,
                value,
            } = effect
            {
                if Arc::ptr_eq(parent, &root) && field == "1" {
                    captured = Some(value.clone());
                    return Err(7);
                }
            }
            if let (
                Some(captured),
                InputMutation::Set {
                    target: Value::Object(target),
                    field,
                    ..
                },
            ) = (&captured, effect)
            {
                if field == "nodes"
                    && Arc::ptr_eq(target.identity(), captured.as_object().unwrap().identity())
                {
                    tail_completed = true;
                }
            }
            Ok(())
        };
        let mut ready = VecDeque::from(setup.start(&mut apply).unwrap());
        loop {
            if let Some(result) = setup.take_result() {
                assert!(matches!(result, Err(AstSetupFault::Host(7))));
                break;
            }
            let job = ready.pop_front().unwrap();
            ready.extend(setup.drive(job, &mut apply).unwrap());
        }
        while let Some(job) = ready.pop_front() {
            ready.extend(setup.drive(job, &mut apply).unwrap());
        }
        assert!(tail_completed);
        assert!(!setup.has_pending_jobs());
        assert!(source.upgrade().is_some());
        drop(setup);
        assert!(source.upgrade().is_none());
    }
}
