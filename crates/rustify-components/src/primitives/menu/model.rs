use rustify_ui::{Cancelable, LayerDismiss};

pub(super) fn activate(
    disabled: bool,
    select: impl FnOnce(Cancelable<()>),
    change: impl FnOnce(),
    close: impl FnOnce(),
) {
    if disabled {
        return;
    }
    let event = Cancelable::new(());
    select(event.clone());
    change();
    if !event.default_prevented() {
        close();
    }
}

pub(super) fn outside(
    pointer: impl FnOnce(Cancelable<LayerDismiss>),
    interact: impl FnOnce(Cancelable<LayerDismiss>),
    close: impl FnOnce(),
) {
    let event = Cancelable::new(LayerDismiss::PointerOutside);
    pointer(event.clone());
    interact(event.clone());
    if !event.default_prevented() {
        close();
    }
}

#[derive(Default)]
pub(super) struct ContextKey {
    pending: Option<&'static str>,
}
impl ContextKey {
    pub fn down(&mut self, key: &str, shift: bool) -> bool {
        let name = match key {
            "ContextMenu" => "ContextMenu",
            "F10" if shift => "F10",
            _ => return false,
        };
        if self.pending.is_some() {
            return false;
        }
        self.pending = Some(name);
        true
    }
    pub fn up(&mut self, key: &str) {
        if self.pending == Some(key) {
            self.pending = None;
        }
    }
    pub fn consumes_contextmenu(&self) -> bool {
        self.pending.is_some()
    }
}

pub(super) fn next_match(
    labels: &[String],
    enabled: &[bool],
    query: &str,
    current: Option<usize>,
) -> Option<usize> {
    let mut chars = query.chars();
    let first = chars.next()?;
    let query = if chars.all(|c| c == first) {
        first.to_string()
    } else {
        query.to_string()
    }
    .to_lowercase();
    let single = query.chars().count() == 1;
    let start = current.map_or(0, |i| if single { i + 1 } else { i });
    (0..labels.len())
        .map(|offset| (start + offset) % labels.len())
        .find(|&i| {
            enabled[i]
                && (!single || current != Some(i))
                && labels[i].trim().to_lowercase().starts_with(&query)
        })
        .filter(|&i| current != Some(i))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn typeahead_cycles_same_letter_and_preserves_spaces() {
        let labels = ["Alpha", "Archive", "Add item", "Disabled"].map(String::from);
        let enabled = [true, true, true, false];
        assert_eq!(next_match(&labels, &enabled, "aaa", Some(0)), Some(1));
        assert_eq!(next_match(&labels, &enabled, "A", Some(2)), Some(0));
        assert_eq!(next_match(&labels, &enabled, "add i", Some(0)), Some(2));
        assert_eq!(next_match(&labels, &enabled, "d", None), None);
        assert_eq!(next_match(&[], &[], "a", None), None);
        assert_eq!(
            next_match(
                &["Alpha".into(), "Alphabet".into()],
                &[true, true],
                "alph",
                Some(0)
            ),
            None
        );
    }

    #[test]
    fn selection_changes_the_control_before_closing_and_cancellation_only_keeps_it_open() {
        for cancel in [false, true] {
            let log = RefCell::new(Vec::new());
            activate(
                false,
                |event| {
                    log.borrow_mut().push("select");
                    if cancel {
                        event.prevent_default();
                    }
                },
                || log.borrow_mut().push("checked:true"),
                || log.borrow_mut().push("close"),
            );
            assert_eq!(
                *log.borrow(),
                if cancel {
                    vec!["select", "checked:true"]
                } else {
                    vec!["select", "checked:true", "close"]
                }
            );
        }
        activate(
            true,
            |_| panic!("disabled select"),
            || panic!("disabled change"),
            || panic!("disabled close"),
        );
    }

    #[test]
    fn tree_outside_runs_both_root_hooks_and_respects_their_shared_cancellation() {
        for cancel in [false, true] {
            let log = RefCell::new(Vec::new());
            outside(
                |event| {
                    assert_eq!(*event.value(), LayerDismiss::PointerOutside);
                    log.borrow_mut().push("pointer");
                    if cancel {
                        event.prevent_default();
                    }
                },
                |event| {
                    assert_eq!(event.default_prevented(), cancel);
                    log.borrow_mut().push("interact");
                },
                || log.borrow_mut().push("close-root"),
            );
            assert_eq!(
                *log.borrow(),
                if cancel {
                    vec!["pointer", "interact"]
                } else {
                    vec!["pointer", "interact", "close-root"]
                }
            );
        }
    }

    #[test]
    fn keyboard_contextmenu_is_consumed_until_its_matching_key_is_released() {
        let mut key = ContextKey::default();
        assert!(!key.down("F10", false));
        assert!(!key.consumes_contextmenu());
        for (name, shift) in [("F10", true), ("ContextMenu", false)] {
            assert!(key.down(name, shift));
            assert!(key.consumes_contextmenu());
            key.up("Shift");
            assert!(key.consumes_contextmenu());
            key.up(name);
            assert!(!key.consumes_contextmenu());
        }
    }
}
