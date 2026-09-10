//! Where an arrow key lands.
//!
//! A tab strip, a menu and a listbox all answer this the same way, and all
//! three get it wrong the same way if each writes its own: stopping on an item
//! nobody can choose, or refusing to wrap and leaving the last item with no
//! way back to the first without a mouse.

/// The index `step` places along from `from`, wrapping at the ends and
/// stepping over anything `reachable` says no to.
///
/// `from` is `None` when nothing is current yet, or when what was current is
/// no longer in the list: a forward step then lands on the first reachable
/// item and a backward one on the last, so the arrows are a way in rather than
/// a dead key.
pub fn step(
    count: usize,
    reachable: impl Fn(usize) -> bool,
    from: Option<usize>,
    step: i32,
) -> Option<usize> {
    let open: Vec<usize> = (0..count).filter(|index| reachable(*index)).collect();
    if open.is_empty() {
        return None;
    }
    let at = from.and_then(|from| open.iter().position(|index| *index == from));
    let Some(at) = at else {
        return if step >= 0 {
            open.first().copied()
        } else {
            open.last().copied()
        };
    };
    let len = open.len() as i32;
    let next = ((at as i32 + step) % len + len) % len;
    open.get(next as usize).copied()
}

/// The first item an arrow can reach, or the last.
pub fn edge(count: usize, reachable: impl Fn(usize) -> bool, last: bool) -> Option<usize> {
    let mut open = (0..count).filter(|index| reachable(*index));
    if last {
        open.last()
    } else {
        open.next()
    }
}

#[cfg(test)]
mod tests {
    use super::{edge, step};

    /// Three items with the middle one out of reach.
    fn reachable(index: usize) -> bool {
        index != 1
    }

    #[test]
    fn an_arrow_steps_over_what_nobody_can_choose() {
        assert_eq!(step(3, reachable, Some(0), 1), Some(2));
        assert_eq!(step(3, reachable, Some(2), -1), Some(0));
    }

    #[test]
    fn the_ends_of_the_list_are_joined() {
        assert_eq!(step(3, reachable, Some(2), 1), Some(0));
        assert_eq!(step(3, reachable, Some(0), -1), Some(2));
    }

    #[test]
    fn with_nothing_current_the_arrows_are_a_way_in() {
        assert_eq!(step(3, reachable, None, 1), Some(0));
        assert_eq!(step(3, reachable, None, -1), Some(2));
    }

    #[test]
    fn a_list_with_nothing_reachable_goes_nowhere() {
        assert_eq!(step(3, |_| false, Some(0), 1), None);
        assert_eq!(step(0, |_| true, None, 1), None);
        assert_eq!(edge(3, |_| false, false), None);
    }

    #[test]
    fn the_edges_skip_the_unreachable_too() {
        assert_eq!(edge(3, |index| index != 0, false), Some(1));
        assert_eq!(edge(3, |index| index != 2, true), Some(1));
    }
}
