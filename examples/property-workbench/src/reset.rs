//! What a reset puts back.
//!
//! The page's checks share one page, because what a check costs here is not
//! loading the page but starting its region: every new WebGL context has a
//! software rasteriser compile the region's shaders again, and mounting the
//! scope again makes a new one. So a reset keeps the scope and its region and
//! gives every piece of the application's state the value it had when the
//! page loaded; the region redraws from the props that follow.
//!
//! A piece of state and its first value are written once, where the state is
//! made: `resets.signal(|| value)` starts at `value` and goes back to it. What
//! the region reports about itself - its state, where it drew, how far its
//! list is scrolled - is not registered. The region is not reset, and a copy
//! of its report that disagreed with it would be a second answer to one
//! question.

use leptos::prelude::*;
use std::sync::Arc;

/// Every reset one scope has registered, in the order it registered them.
#[derive(Clone, Copy)]
pub struct Resets(StoredValue<Vec<Arc<dyn Fn() + Send + Sync>>>);

impl Resets {
    /// A new set, in context so the components below can add their own.
    pub fn provide() -> Self {
        let resets = Self(StoredValue::new(Vec::new()));
        provide_context(resets);
        resets
    }

    /// The set of the scope this component is in, if it keeps one.
    pub fn used() -> Option<Self> {
        use_context::<Self>()
    }

    /// A signal that starts at `initial()` and goes back to it.
    pub fn signal<T: Send + Sync + 'static>(
        self,
        initial: impl Fn() -> T + Send + Sync + 'static,
    ) -> RwSignal<T> {
        let signal = RwSignal::new(initial());
        self.on_reset(move || signal.set(initial()));
        signal
    }

    /// State that is not one signal, or whose first value depends on other
    /// state: it is put back after everything registered before it.
    pub fn on_reset(self, reset: impl Fn() + Send + Sync + 'static) {
        self.0.update_value(|resets| resets.push(Arc::new(reset)));
    }

    pub fn run(self) {
        // Taken out first: a reset that ended up registering another must
        // not find the list still borrowed.
        let resets = self.0.with_value(Clone::clone);
        for reset in resets {
            reset();
        }
    }
}
