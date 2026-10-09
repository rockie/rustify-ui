/// Shared source-change selection for native and asynchronous JS builders.
pub enum WatchChange {
    Rebuild,
    Keep,
    Candidates(Vec<String>),
}
pub struct ProjectWatch {
    cwd: String,
    options: ProjectOptions,
    matches: ContentMatcher,
    matches_css: ContentMatcher,
}
impl ProjectWatch {
    pub fn new(tree: &BuildSourceTree, options: &ProjectOptions) -> Result<Self, CnError> {
        let cwd = tree.cwd(options.cwd.as_deref().unwrap_or("."))?;
        Self::new_at(cwd, &content_patterns(options), options.clone())
    }
    /// Construct from cwd/content already read by the host at creation time.
    pub fn new_at(
        cwd: String,
        patterns: &[String],
        options: ProjectOptions,
    ) -> Result<Self, CnError> {
        Ok(Self {
            matches: create_content_matcher(patterns, &cwd)?,
            matches_css: create_content_matcher(&["**/*.css".into()], &cwd)?,
            cwd,
            options,
        })
    }
    pub fn resolve_change(&self, tree: &BuildSourceTree, file: &str) -> Result<String, CnError> {
        tree.resolve_path(&self.cwd, file)
    }
    pub fn discovered_theme_source(
        &self,
        tree: &BuildSourceTree,
        abs: &str,
    ) -> Result<bool, CnError> {
        Ok(self
            .matches_css
            .matches_relative(&tree.relative_path(&self.cwd, abs)?)
            && tree.read_text(abs).is_ok_and(|css| is_tailwind_css(&css)))
    }
    pub fn changed(
        &self,
        tree: &BuildSourceTree,
        file: &str,
        out_path: &str,
        css_files: &[String],
        tokens: &[String],
    ) -> Result<WatchChange, CnError> {
        self.changed_with_options(tree, &self.options, file, out_path, css_files, tokens)
    }
    pub fn changed_with_options(
        &self,
        tree: &BuildSourceTree,
        options: &ProjectOptions,
        file: &str,
        out_path: &str,
        css_files: &[String],
        tokens: &[String],
    ) -> Result<WatchChange, CnError> {
        let abs = self.resolve_change(tree, file)?;
        if css_files.contains(&abs)
            || matches!(options.css, CssInput::Discover)
                && self.discovered_theme_source(tree, &abs)?
        {
            return Ok(WatchChange::Rebuild);
        }
        if options.full || options.tokens.is_some() {
            return Ok(WatchChange::Keep);
        }
        self.source_candidates_at(tree, &abs, out_path, tokens)
    }
    /// Source-only checks after lazy css/full/tokens option reads.
    pub fn source_candidates_at(
        &self,
        tree: &BuildSourceTree,
        abs: &str,
        out_path: &str,
        tokens: &[String],
    ) -> Result<WatchChange, CnError> {
        if abs == out_path
            || !self
                .matches
                .matches_relative(&tree.relative_path(&self.cwd, abs)?)
        {
            return Ok(WatchChange::Keep);
        }
        let Ok(text) = tree.read_text(abs) else {
            return Ok(WatchChange::Keep);
        };
        let mut fresh = Vec::new();
        extract_tokens(&text, &mut fresh);
        let existing: std::collections::HashSet<_> = tokens.iter().map(String::as_str).collect();
        fresh.retain(|s| !existing.contains(s.as_str()));
        if fresh.is_empty() {
            return Ok(WatchChange::Keep);
        }
        Ok(WatchChange::Candidates(fresh))
    }
}
use super::project::content_patterns;
use super::*;
use std::cell::{Cell, RefCell};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

type Outcome = Result<Rc<ProjectReport>, CnError>;
struct Flight {
    outcome: RefCell<Option<Outcome>>,
    executing: Cell<bool>,
    queued: Cell<bool>,
    waiters: RefCell<Vec<Waker>>,
}
struct Inner {
    tree: Rc<BuildSourceTree>,
    options: ProjectOptions,
    loader: Option<ConfigLoader>,
    watch: ProjectWatch,
    last: RefCell<Option<Rc<ProjectReport>>>,
    flight: RefCell<Option<Rc<Flight>>>,
}
/// Incremental builder. Reports are shared so unchanged edits return the last
/// report by identity. Async run requests share one flight and one follow-up.
#[derive(Clone)]
pub struct Builder {
    inner: Rc<Inner>,
}
/// A coalesced build future. Building occurs on the host that polls the future;
/// native callers should poll off their UI thread when scanning large projects.
pub struct BuildFuture {
    inner: Rc<Inner>,
    flight: Rc<Flight>,
}
impl Future for BuildFuture {
    type Output = Outcome;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Outcome> {
        if let Some(outcome) = self.flight.outcome.borrow().as_ref() {
            return Poll::Ready(outcome.clone());
        }
        if self.flight.executing.replace(true) {
            let mut waiters = self.flight.waiters.borrow_mut();
            if !waiters.iter().any(|waiter| waiter.will_wake(cx.waker())) {
                waiters.push(cx.waker().clone());
            }
            return Poll::Pending;
        }
        let outcome = loop {
            let result = build(
                &self.inner.tree,
                &self.inner.options,
                self.inner.loader.as_ref(),
            )
            .map(Rc::new);
            if let Ok(report) = &result {
                *self.inner.last.borrow_mut() = Some(report.clone());
            }
            if result.is_err() || !self.flight.queued.replace(false) {
                break result;
            }
        };
        self.flight.executing.set(false);
        *self.flight.outcome.borrow_mut() = Some(outcome.clone());
        let mut active = self.inner.flight.borrow_mut();
        if active
            .as_ref()
            .is_some_and(|flight| Rc::ptr_eq(flight, &self.flight))
        {
            *active = None;
        }
        drop(active);
        for waiter in self.flight.waiters.borrow_mut().drain(..) {
            waiter.wake();
        }
        Poll::Ready(outcome)
    }
}
impl Builder {
    pub fn last(&self) -> Option<Rc<ProjectReport>> {
        self.inner.last.borrow().clone()
    }
    /// Concurrent calls queue at most one follow-up; all await the final report.
    pub fn run_async(&self) -> BuildFuture {
        let mut active = self.inner.flight.borrow_mut();
        let flight = if let Some(flight) = active.as_ref() {
            flight.queued.set(true);
            flight.clone()
        } else {
            let flight = Rc::new(Flight {
                outcome: RefCell::new(None),
                executing: Cell::new(false),
                queued: Cell::new(false),
                waiters: RefCell::new(Vec::new()),
            });
            *active = Some(flight.clone());
            flight
        };
        BuildFuture {
            inner: self.inner.clone(),
            flight,
        }
    }
    /// Synchronous host convenience; reentrant hosts should use `run_async`.
    pub fn run(&self) -> Outcome {
        let mut future = std::pin::pin!(self.run_async());
        let mut cx = Context::from_waker(Waker::noop());
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(result) => result,
            Poll::Pending => Err(CnError::new(
                "cn: synchronous builder run reentered; use run_async",
            )),
        }
    }
    fn needs_build(&self, file: &str) -> Result<bool, CnError> {
        let Some(last) = self.last() else {
            return Ok(true);
        };
        let fresh = match self.inner.watch.changed(
            &self.inner.tree,
            file,
            &last.out_path,
            &last.css_files,
            &last.artifact.tokens,
        )? {
            WatchChange::Rebuild => return Ok(true),
            WatchChange::Keep => return Ok(false),
            WatchChange::Candidates(fresh) => fresh,
        };
        let reached = super::super::compiler::subset_config(
            &last.artifact.full_config,
            fresh.iter().map(String::as_str),
        )?;
        Ok(reached.config.class_groups.iter().any(|(group, _)| {
            !last
                .artifact
                .config
                .class_groups
                .iter()
                .any(|(kept, _)| kept == group)
        }))
    }
    /// Rebuild only for theme changes or candidates reaching a dropped group.
    pub fn changed(&self, file: &str) -> Outcome {
        if self.needs_build(file)? {
            self.run()
        } else {
            self.last()
                .ok_or_else(|| CnError::new("cn: builder has no completed result"))
        }
    }
    /// Async watcher entry. Unchanged edits resolve with the existing report.
    pub fn changed_async(&self, file: &str) -> Result<BuildFuture, CnError> {
        if self.needs_build(file)? {
            Ok(self.run_async())
        } else {
            let result = self
                .last()
                .ok_or_else(|| CnError::new("cn: builder has no completed result"))?;
            Ok(BuildFuture {
                inner: self.inner.clone(),
                flight: Rc::new(Flight {
                    outcome: RefCell::new(Some(Ok(result))),
                    executing: Cell::new(false),
                    queued: Cell::new(false),
                    waiters: RefCell::new(Vec::new()),
                }),
            })
        }
    }
}
/// Build an incremental host adapter with exactly the project's source selectors.
pub fn create_builder(
    tree: Rc<BuildSourceTree>,
    options: ProjectOptions,
    loader: Option<ConfigLoader>,
) -> Result<Builder, CnError> {
    let watch = ProjectWatch::new(&tree, &options)?;
    Ok(Builder {
        inner: Rc::new(Inner {
            tree,
            options,
            loader,
            watch,
            last: RefCell::new(None),
            flight: RefCell::new(None),
        }),
    })
}
