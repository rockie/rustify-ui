//! Scheduling facts, independent of browser time and controlled callbacks.

use super::{ToastConfig, ToastIssue, ToastQueuePolicy, MAX_DURATION_MS};

pub(super) type Id = u64;

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Pause {
    Hover = 1,
    Focus = 2,
    WindowBlur = 4,
    Hidden = 8,
    Swipe = 16,
}

#[derive(Clone, Debug)]
struct Entry {
    id: Id,
    order: u64,
    open: bool,
    visible: bool,
    duration: u32,
    remaining: f64,
    started: Option<f64>,
    pauses: u8,
    requested: bool,
    announced: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct Events {
    pub close: Vec<Id>,
    pub announce: Vec<Id>,
}

#[derive(Clone)]
pub(super) struct Scheduler {
    pub config: ToastConfig,
    entries: Vec<Entry>,
    next_id: Id,
    next_order: u64,
    pauses: u8,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Scheduler {
    pub fn new(config: ToastConfig) -> (Self, Vec<ToastIssue>) {
        let (config, issues) = validate_config(config, &ToastConfig::default());
        (
            Self {
                config,
                entries: Vec::new(),
                next_id: 0,
                next_order: 0,
                pauses: 0,
            },
            issues,
        )
    }

    pub fn register(&mut self) -> Id {
        self.next_id += 1;
        self.entries.push(Entry {
            id: self.next_id,
            order: 0,
            open: false,
            visible: false,
            duration: super::DEFAULT_DURATION_MS,
            remaining: 0.,
            started: None,
            pauses: 0,
            requested: false,
            announced: false,
        });
        self.next_id
    }

    pub fn sync(
        &mut self,
        id: Id,
        open: bool,
        duration: u32,
        now: f64,
    ) -> (Events, Option<ToastIssue>) {
        self.advance(now);
        let Some(entry) = self.entries.iter_mut().find(|entry| entry.id == id) else {
            return (Events::default(), None);
        };
        let issue = (duration > MAX_DURATION_MS).then(|| ToastIssue::duration(duration));
        let duration = if issue.is_some() {
            entry.duration
        } else {
            duration
        };
        let reopening = open && !entry.open;
        if reopening {
            self.next_order += 1;
            entry.order = self.next_order;
            entry.announced = false;
        }
        if reopening || duration != entry.duration {
            entry.remaining = f64::from(duration);
            entry.requested = false;
        }
        entry.duration = duration;
        entry.open = open;
        if !open {
            entry.visible = false;
            entry.started = None;
            entry.pauses = 0;
        }
        (self.settle(now), issue)
    }

    pub fn remove(&mut self, id: Id, now: f64) -> Events {
        self.advance(now);
        self.entries.retain(|entry| entry.id != id);
        self.settle(now)
    }

    pub fn visible(&self, id: Id) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.id == id && entry.open && entry.visible)
    }

    pub fn visible_ids(&self) -> Vec<Id> {
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .filter(|entry| entry.open && entry.visible)
            .collect();
        entries.sort_by_key(|entry| entry.order);
        entries.into_iter().map(|entry| entry.id).collect()
    }

    pub fn paused(&self, id: Id) -> bool {
        self.entries
            .iter()
            .find(|entry| entry.id == id)
            .is_some_and(|entry| entry.pauses | self.pauses != 0)
    }

    pub fn paused_by(&self, id: Option<Id>, reason: Pause) -> bool {
        let mask = match id {
            Some(id) => self
                .entries
                .iter()
                .find(|entry| entry.id == id)
                .map_or(0, |entry| entry.pauses),
            None => self.pauses,
        };
        mask & reason as u8 != 0
    }

    pub fn pause(&mut self, id: Option<Id>, reason: Pause, active: bool, now: f64) -> Events {
        self.advance(now);
        let set = |mask: &mut u8| {
            if active {
                *mask |= reason as u8;
            } else {
                *mask &= !(reason as u8);
            }
        };
        match id {
            Some(id) => {
                if let Some(entry) = self.entries.iter_mut().find(|entry| entry.id == id) {
                    set(&mut entry.pauses);
                }
            }
            None => set(&mut self.pauses),
        }
        self.settle(now)
    }

    pub fn requested(&mut self, id: Id, now: f64) {
        self.advance(now);
        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.id == id) {
            entry.requested = true;
            entry.started = None;
        }
    }

    pub fn next_due(&self) -> Option<f64> {
        self.entries
            .iter()
            .filter_map(|entry| entry.started.map(|started| started + entry.remaining))
            .min_by(f64::total_cmp)
    }

    pub fn expire(&mut self, now: f64) -> Events {
        self.advance(now);
        let mut events = Events::default();
        for entry in &mut self.entries {
            if entry.started.is_some() && entry.remaining == 0. && !entry.requested {
                entry.requested = true;
                entry.started = None;
                events.close.push(entry.id);
            }
        }
        events
    }

    fn advance(&mut self, now: f64) {
        for entry in &mut self.entries {
            if let Some(started) = entry.started {
                entry.remaining = (entry.remaining - (now - started)).max(0.);
                entry.started = Some(now);
            }
        }
    }

    fn settle(&mut self, now: f64) -> Events {
        let mut events = Events::default();
        self.entries.sort_by_key(|entry| entry.order);
        let cutoff = self
            .entries
            .iter()
            .rev()
            .filter(|entry| entry.open)
            .nth(self.config.visible_limit - 1)
            .map_or(0, |entry| entry.order);
        if self.config.policy == ToastQueuePolicy::Replace {
            for entry in &mut self.entries {
                if entry.open && entry.order < cutoff && !entry.requested {
                    entry.requested = true;
                    events.close.push(entry.id);
                }
            }
        }
        let mut slots = self
            .entries
            .iter()
            .filter(|entry| entry.open && entry.visible)
            .count();
        for entry in &mut self.entries {
            if slots >= self.config.visible_limit {
                break;
            }
            let wanted = self.config.policy == ToastQueuePolicy::Fifo || entry.order >= cutoff;
            if entry.open && !entry.visible && wanted {
                entry.visible = true;
                slots += 1;
            }
        }
        for entry in &mut self.entries {
            if entry.visible && entry.open && !entry.announced {
                entry.announced = true;
                events.announce.push(entry.id);
            }
            let running = entry.visible
                && entry.open
                && entry.duration != 0
                && !entry.requested
                && entry.pauses | self.pauses == 0;
            entry.started = running.then_some(now);
        }
        events
    }
}

fn validate_config(
    mut candidate: ToastConfig,
    previous: &ToastConfig,
) -> (ToastConfig, Vec<ToastIssue>) {
    let mut issues = Vec::new();
    if candidate.visible_limit == 0 {
        issues.push(ToastIssue {
            field: "visible_limit",
            message: "expected at least one visible slot".into(),
        });
        candidate.visible_limit = previous.visible_limit;
    }
    if !candidate.swipe_threshold.is_finite() || candidate.swipe_threshold < 0. {
        issues.push(ToastIssue {
            field: "swipe_threshold",
            message: "expected a finite nonnegative distance".into(),
        });
        candidate.swipe_threshold = previous.swipe_threshold;
    }
    (candidate, issues)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fifo(limit: usize) -> Scheduler {
        Scheduler::new(ToastConfig {
            policy: ToastQueuePolicy::Fifo,
            visible_limit: limit,
            ..Default::default()
        })
        .0
    }

    #[test]
    fn invalid_fields_preserve_the_last_valid_configuration_and_duration() {
        let mut model = fifo(2);
        let (config, issues) = validate_config(
            ToastConfig {
                visible_limit: 0,
                swipe_threshold: f64::NAN,
                ..Default::default()
            },
            &model.config,
        );
        model.config = config;
        assert_eq!(
            issues.iter().map(|issue| issue.field).collect::<Vec<_>>(),
            ["visible_limit", "swipe_threshold"]
        );
        assert_eq!(model.config.visible_limit, 2);
        assert_eq!(model.config.swipe_threshold, 50.);
        let id = model.register();
        model.sync(id, true, 100, 0.);
        let (_, issue) = model.sync(id, true, u32::MAX, 20.);
        assert_eq!(issue.unwrap().field, "duration_ms");
        assert_eq!(model.next_due(), Some(100.));
    }

    #[test]
    fn queued_items_do_not_spend_their_duration_before_a_slot_is_free() {
        let mut model = fifo(1);
        let first = model.register();
        let second = model.register();
        model.sync(first, true, 100, 0.);
        model.sync(second, true, 100, 10.);
        assert_eq!(model.visible_ids(), [first]);
        assert_eq!(model.expire(100.).close, [first]);
        assert_eq!(
            model.next_due(),
            None,
            "a rejected close does not advance the queue"
        );
        let events = model.sync(first, false, 100, 150.).0;
        assert_eq!(events.announce, [second]);
        assert_eq!(model.visible_ids(), [second]);
        assert_eq!(model.next_due(), Some(250.));
    }

    #[test]
    fn replacement_requests_once_and_preserves_the_old_slot_until_the_application_accepts() {
        let mut model = Scheduler::new(ToastConfig::default()).0;
        let first = model.register();
        let second = model.register();
        model.sync(first, true, 100, 0.);
        assert_eq!(model.sync(second, true, 100, 20.).0.close, [first]);
        assert_eq!(model.visible_ids(), [first]);
        assert!(model.sync(second, true, 100, 30.).0.close.is_empty());
        assert!(model.expire(200.).close.is_empty());
        model.sync(first, false, 100, 220.);
        assert_eq!(model.visible_ids(), [second]);
        assert_eq!(model.next_due(), Some(320.));
    }

    #[test]
    fn all_pause_reasons_must_clear_and_migration_reconciliation_keeps_remaining_time() {
        let mut model = fifo(1);
        let id = model.register();
        model.sync(id, true, 1000, 0.);
        model.pause(None, Pause::Hover, true, 100.);
        model.pause(None, Pause::Focus, true, 200.);
        model.pause(None, Pause::Hidden, true, 250.);
        model.pause(Some(id), Pause::Swipe, true, 275.);
        model.pause(None, Pause::Hover, false, 300.);
        model.pause(None, Pause::Focus, false, 400.);
        assert_eq!(model.next_due(), None);
        model.pause(None, Pause::Hidden, false, 800.);
        assert_eq!(
            model.next_due(),
            None,
            "the active swipe still pauses its own session"
        );
        model.pause(Some(id), Pause::Swipe, false, 800.);
        assert_eq!(model.next_due(), Some(1700.));
        model.pause(None, Pause::Focus, true, 900.);
        model.pause(None, Pause::Hover, false, 1200.);
        model.pause(None, Pause::Focus, false, 1200.);
        assert_eq!(model.next_due(), Some(2000.));
    }

    #[test]
    fn an_expiry_rejection_stays_quiet_until_reopening_or_a_changed_duration() {
        let mut model = fifo(1);
        let id = model.register();
        model.sync(id, true, 100, 0.);
        assert_eq!(model.expire(100.).close, [id]);
        assert!(model.expire(200.).close.is_empty());
        model.sync(id, true, 100, 250.);
        assert_eq!(model.next_due(), None);
        assert!(model.sync(id, true, 200, 300.).0.announce.is_empty());
        assert_eq!(model.next_due(), Some(500.));
        model.sync(id, false, 200, 350.);
        assert_eq!(model.sync(id, true, 200, 400.).0.announce, [id]);
        assert_eq!(model.next_due(), Some(600.));
    }

    #[test]
    fn persistent_messages_have_no_deadline_and_unmount_frees_the_next_slot() {
        let mut model = fifo(1);
        let first = model.register();
        let second = model.register();
        model.sync(first, true, 0, 0.);
        model.sync(second, true, 100, 0.);
        assert_eq!(model.next_due(), None);
        assert!(model.expire(10000.).close.is_empty());
        assert_eq!(model.remove(first, 20000.).announce, [second]);
        assert_eq!(model.next_due(), Some(20100.));
    }

    #[test]
    fn one_deadline_covers_multiple_slots_and_early_timer_delivery_is_safe() {
        let mut model = fifo(2);
        let a = model.register();
        let b = model.register();
        model.sync(a, true, 100, 0.);
        model.sync(b, true, 50, 10.);
        assert_eq!(model.next_due(), Some(60.));
        assert!(model.expire(59.5).close.is_empty());
        assert_eq!(model.next_due(), Some(60.));
        assert_eq!(model.expire(60.).close, [b]);
        assert_eq!(model.next_due(), Some(100.));
    }

    #[test]
    fn viewport_hover_and_focus_pause_all_visible_slots_without_resetting_either_duration() {
        let mut model = fifo(2);
        let first = model.register();
        let second = model.register();
        model.sync(first, true, 1000, 0.);
        model.sync(second, true, 2000, 0.);
        model.pause(None, Pause::Hover, true, 100.);
        model.pause(None, Pause::Focus, true, 200.);
        assert!(model.paused(first) && model.paused(second));
        assert_eq!(model.next_due(), None);
        model.pause(None, Pause::Hover, false, 300.);
        assert_eq!(
            model.next_due(),
            None,
            "focus in a notification branch still pauses both slots"
        );
        model.pause(None, Pause::WindowBlur, true, 350.);
        model.pause(None, Pause::Focus, false, 400.);
        assert_eq!(model.next_due(), None);
        model.pause(None, Pause::WindowBlur, false, 500.);
        assert_eq!(model.next_due(), Some(1400.));
        assert_eq!(model.expire(1400.).close, [first]);
        assert_eq!(model.next_due(), Some(2400.));
    }
}
