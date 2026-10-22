//! The probe that says the split did not fork the types.
//!
//! `rustify-ui` re-exports the core rather than redeclaring it, so a name
//! resolved through `rustify_ui::` and the same name resolved through
//! `rustify_dom::` are one type. Nearly every line below is an assignment
//! between the two spellings: if the facade ever grew its own copy, this stops
//! compiling, which is the failure worth hearing about early.

use rustify_dom::{ActionSink, Admission, Pace, Theme};
use rustify_ui as sdk;

#[test]
fn the_facade_names_the_core_theme() {
    let core: Theme = sdk::Theme::light();
    let facade: sdk::Theme = Theme::dark();
    // Constructed through each spelling and both are the same type, so the
    // two values differ only because they were built from different defaults.
    assert_ne!(core.name, facade.name);
}

#[test]
fn the_facade_names_the_core_pace() {
    let from_core: Pace = sdk::Pace::Discrete;
    let from_facade: sdk::Pace = Pace::Continuous("scroll");
    assert_eq!(from_core, Pace::Discrete);
    assert_eq!(from_facade, Pace::Continuous("scroll"));
}

#[test]
fn an_action_sink_the_facade_hands_out_is_the_core_sink() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let delivered = Arc::new(AtomicUsize::new(0));
    let sink: ActionSink = sdk::ActionSink::new();
    assert_eq!(sink.pending(), 0);

    let counter = delivered.clone();
    let admission = sink.submit(sdk::Pace::Discrete, move || {
        counter.fetch_add(1, Ordering::SeqCst);
    });
    assert!(
        matches!(admission, Admission::Accepted(_)),
        "a sink with room in it refuses nothing"
    );
    assert_eq!(sink.pending(), 1);

    // Queued is not run: the host owns the turn, and this is the turn.
    assert_eq!(delivered.load(Ordering::SeqCst), 0);
    assert!(!sink.drain_once(), "one action in, none left waiting");
    assert_eq!(delivered.load(Ordering::SeqCst), 1);
    assert_eq!(sink.pending(), 0);
}

#[cfg(feature = "gpu")]
#[test]
fn a_region_bridge_names_the_same_pace_the_core_defines() {
    let from_bridge: rustify_makepad::Pace = Pace::Discrete;
    let from_core: Pace = rustify_makepad::Pace::Continuous("scroll");
    assert_eq!(from_bridge, Pace::Discrete);
    assert_eq!(from_core, Pace::Continuous("scroll"));
}