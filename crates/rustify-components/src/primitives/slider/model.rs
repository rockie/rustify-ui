//! Target-independent step, identity and gesture calculations.

use crate::geometry::PropError;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// An accepted value whose identity remains stable when thumbs cross.
#[derive(Clone, Debug, PartialEq)]
pub struct ThumbValue {
    pub id: String,
    pub value: f64,
}
/// The accessible name for a thumb assembled by RangeSlider.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThumbLabel {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Config {
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub min_steps_between: u32,
    pub preserve_order: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            step: 1.0,
            min_steps_between: 0,
            preserve_order: false,
        }
    }
}
fn error(field: &'static str, message: &str) -> PropError {
    PropError {
        field,
        message: message.into(),
    }
}
impl Config {
    fn validate(self, count: usize) -> Result<(), PropError> {
        if !self.min.is_finite() {
            return Err(error("min", "must be finite"));
        }
        if !self.max.is_finite() || self.max <= self.min || !(self.max - self.min).is_finite() {
            return Err(error("max", "must define a finite positive range"));
        }
        if !self.step.is_finite() || self.step <= 0.0 {
            return Err(error("step", "must be finite and positive"));
        }
        if (self.max - self.min) / self.step > (1_u64 << 53) as f64
            || self.min + self.step == self.min
        {
            return Err(error("step", "must define representable step indices"));
        }
        let needed = f64::from(self.min_steps_between) * count.saturating_sub(1) as f64;
        if !needed.is_finite() || needed > self.index(self.max) {
            return Err(error(
                "min_steps_between",
                "not enough space for the declared thumbs",
            ));
        }
        Ok(())
    }
    fn index(self, value: f64) -> f64 {
        let index = (value - self.min) / self.step;
        let rounded = index.round();
        // Ordinary snapped values use integer indices. A clamped terminal step
        // has only its actual fractional capacity, unless it differs from the
        // full grid point by floating-point projection error alone.
        if value == self.max {
            let grid = self.min + rounded * self.step;
            let tolerance = 4.0 * f64::EPSILON * index.abs().max(1.0);
            if grid != value && (index - rounded).abs() > tolerance {
                return index;
            }
        }
        rounded
    }
    fn separated(self, high: f64, low: f64) -> bool {
        self.index(high) - self.index(low) >= f64::from(self.min_steps_between)
    }
    pub fn snap(self, value: f64) -> f64 {
        rustify_ui::snap(value, self.min, self.max, self.step)
    }
    fn at(self, index: f64) -> f64 {
        (self.min + index * self.step).clamp(self.min, self.max)
    }
    fn bounded(self, value: f64, low: f64, high: f64) -> Option<f64> {
        let last = self.index(self.snap(self.max));
        let low_index = self.index(low).ceil().min(last);
        let high_index = if high == self.max {
            last
        } else {
            self.index(high).floor()
        };
        if low_index > high_index {
            return None;
        }
        let index = self
            .index(self.snap(value.clamp(low, high)))
            .clamp(low_index, high_index);
        Some(self.at(index))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Neighbours {
    before: Option<String>,
    after: Option<String>,
}

/// Cached once per accepted-value/configuration version, never per pointer move.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Model {
    pub config: Config,
    pub values: Vec<ThumbValue>,
    pub order: Arc<Vec<String>>,
    index: Arc<HashMap<String, usize>>,
    rank: Arc<HashMap<String, usize>>,
}
impl Model {
    pub fn new(
        input: &[ThumbValue],
        config: Config,
        previous: Option<&Self>,
    ) -> Result<Self, PropError> {
        config.validate(input.len())?;
        let mut ids = HashSet::with_capacity(input.len());
        for item in input {
            if item.id.is_empty() || !ids.insert(item.id.as_str()) {
                return Err(error("values.id", "must be nonempty and unique"));
            }
            if !item.value.is_finite() {
                return Err(error("values", "must be finite"));
            }
        }
        let same_ids = previous.filter(|previous| {
            previous.order.len() == ids.len()
                && previous.order.iter().all(|id| ids.contains(id.as_str()))
        });
        let (order, rank) = if let Some(previous) = same_ids {
            (previous.order.clone(), previous.rank.clone())
        } else {
            let mut order: Vec<_> = previous
                .into_iter()
                .flat_map(|previous| previous.order.iter())
                .filter(|id| ids.contains(id.as_str()))
                .cloned()
                .collect();
            let prior: HashSet<_> = order.iter().cloned().collect();
            order.extend(
                input
                    .iter()
                    .filter(|item| !prior.contains(&item.id))
                    .map(|item| item.id.clone()),
            );
            let rank = order
                .iter()
                .enumerate()
                .map(|(at, id)| (id.clone(), at))
                .collect();
            (Arc::new(order), Arc::new(rank))
        };
        let mut values: Vec<_> = input
            .iter()
            .map(|item| ThumbValue {
                id: item.id.clone(),
                value: config.snap(item.value),
            })
            .collect();
        let compare = |a: &ThumbValue, b: &ThumbValue| {
            a.value
                .total_cmp(&b.value)
                .then_with(|| rank[&a.id].cmp(&rank[&b.id]))
        };
        // Requests are already sorted. Only an unordered external replacement sorts again.
        if values
            .windows(2)
            .any(|pair| compare(&pair[0], &pair[1]).is_gt())
        {
            values.sort_by(compare);
        }
        if !values
            .windows(2)
            .all(|pair| config.separated(pair[1].value, pair[0].value))
        {
            return Err(error(
                "values",
                "must satisfy the declared minimum interval after normalization",
            ));
        }
        let index = previous
            .filter(|previous| {
                previous.values.len() == values.len()
                    && previous
                        .values
                        .iter()
                        .zip(&values)
                        .all(|(a, b)| a.id == b.id)
            })
            .map(|previous| previous.index.clone())
            .unwrap_or_else(|| {
                Arc::new(
                    values
                        .iter()
                        .enumerate()
                        .map(|(at, item)| (item.id.clone(), at))
                        .collect(),
                )
            });
        Ok(Self {
            config,
            values,
            order,
            index,
            rank,
        })
    }
    pub fn get(&self, id: &str) -> Option<f64> {
        self.index.get(id).map(|&at| self.values[at].value)
    }
    pub fn neighbours(&self, id: &str) -> Option<Neighbours> {
        let &at = self.index.get(id)?;
        Some(Neighbours {
            before: at.checked_sub(1).map(|at| self.values[at].id.clone()),
            after: self.values.get(at + 1).map(|item| item.id.clone()),
        })
    }
    pub fn bounds(&self, id: &str, neighbours: Option<&Neighbours>) -> (f64, f64) {
        if !self.config.preserve_order {
            return (self.config.min, self.config.max);
        }
        let current = self.neighbours(id);
        let neighbours = neighbours.or(current.as_ref());
        let low = neighbours
            .and_then(|n| n.before.as_ref())
            .and_then(|id| self.get(id))
            .map_or(self.config.min, |value| {
                if self.config.min_steps_between == 0 {
                    return value;
                }
                self.config.at((self.config.index(value)
                    + f64::from(self.config.min_steps_between))
                .ceil())
            });
        let high = neighbours
            .and_then(|n| n.after.as_ref())
            .and_then(|id| self.get(id))
            .map_or(self.config.max, |value| {
                if self.config.min_steps_between == 0 {
                    return value;
                }
                self.config.at((self.config.index(value)
                    - f64::from(self.config.min_steps_between))
                .floor())
            });
        (low, high)
    }
    pub fn candidate(
        &self,
        id: &str,
        asked: f64,
        neighbours: Option<&Neighbours>,
    ) -> Option<Vec<ThumbValue>> {
        if !asked.is_finite() {
            return None;
        }
        let &old = self.index.get(id)?;
        let (low, high) = self.bounds(id, neighbours);
        if low > high {
            return None;
        }
        let value = self.config.bounded(asked, low, high)?;
        let mut result = self.values.clone();
        let mut active = result.remove(old);
        active.value = value;
        let at = result.partition_point(|item| {
            item.value < value || (item.value == value && self.rank[&item.id] < self.rank[id])
        });
        if at > 0 && !self.config.separated(value, result[at - 1].value) {
            return None;
        }
        if at < result.len() && !self.config.separated(result[at].value, value) {
            return None;
        }
        result.insert(at, active);
        Some(result)
    }
    pub fn nearest<'a>(
        &'a self,
        value: f64,
        enabled: &[&str],
        focused: Option<&str>,
    ) -> Option<&'a str> {
        let enabled: HashSet<_> = enabled.iter().copied().collect();
        self.values
            .iter()
            .filter(|item| enabled.contains(item.id.as_str()))
            .min_by(|a, b| {
                (a.value - value)
                    .abs()
                    .total_cmp(&(b.value - value).abs())
                    .then_with(|| {
                        (focused != Some(a.id.as_str())).cmp(&(focused != Some(b.id.as_str())))
                    })
                    .then_with(|| self.rank[&a.id].cmp(&self.rank[&b.id]))
            })
            .map(|item| item.id.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Key {
    Home,
    End,
    Step(i32),
}
pub(super) fn key(
    key: &str,
    orientation: crate::Orientation,
    reverse: bool,
    inverted: bool,
) -> Option<Key> {
    use crate::Orientation::{Horizontal, Vertical};
    let step = match key {
        "Home" => return Some(Key::Home),
        "End" => return Some(Key::End),
        "PageUp" => 10,
        "PageDown" => -10,
        "ArrowRight" => {
            if orientation == Horizontal && reverse {
                -1
            } else {
                1
            }
        }
        "ArrowLeft" => {
            if orientation == Horizontal && reverse {
                1
            } else {
                -1
            }
        }
        "ArrowUp" => {
            if orientation == Vertical && inverted {
                -1
            } else {
                1
            }
        }
        "ArrowDown" => {
            if orientation == Vertical && inverted {
                1
            } else {
                -1
            }
        }
        _ => return None,
    };
    Some(Key::Step(step))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Input {
    Pointer(i32),
    Keyboard,
}
#[derive(Clone, Debug)]
pub(super) struct Session {
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub input: Input,
    pub active: String,
    pub neighbours: Option<Neighbours>,
    start: Vec<ThumbValue>,
    pub last: Option<Vec<ThumbValue>>,
}
impl Session {
    pub fn new(input: Input, active: String, model: &Model) -> Self {
        Self {
            input,
            neighbours: model.neighbours(&active),
            active,
            start: model.values.clone(),
            last: None,
        }
    }
    pub fn request(&mut self, values: Vec<ThumbValue>) {
        self.last = Some(values);
    }
    pub fn finish(self) -> Option<Vec<ThumbValue>> {
        self.last.filter(|values| *values != self.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(items: &[(&str, f64)]) -> Vec<ThumbValue> {
        items
            .iter()
            .map(|(id, value)| ThumbValue {
                id: (*id).into(),
                value: *value,
            })
            .collect()
    }
    fn model(items: &[(&str, f64)], config: Config) -> Model {
        Model::new(&values(items), config, None).unwrap()
    }

    #[test]
    fn normalization_is_min_based_and_never_rewrites_the_input() {
        let input = values(&[("b", 0.64), ("a", -2.0)]);
        let state = Model::new(
            &input,
            Config {
                min: 0.1,
                max: 1.0,
                step: 0.1,
                ..Config::default()
            },
            None,
        )
        .unwrap();
        assert_eq!(input, values(&[("b", 0.64), ("a", -2.0)]));
        assert_eq!(state.values, values(&[("a", 0.1), ("b", 0.6)]));
    }

    #[test]
    fn crossing_sorts_values_but_keeps_identity_and_declaration_order() {
        let state = model(&[("left", 20.0), ("right", 80.0)], Config::default());
        assert_eq!(
            state.candidate("left", 90.0, None).unwrap(),
            values(&[("right", 80.0), ("left", 90.0)])
        );
        let next = Model::new(
            &values(&[("right", 80.0), ("left", 90.0), ("new", 10.0)]),
            Config::default(),
            Some(&state),
        )
        .unwrap();
        assert_eq!(next.order.as_slice(), ["left", "right", "new"]);
    }

    #[test]
    fn equal_values_use_first_declaration_order_even_after_external_reordering() {
        let state = model(&[("b", 40.0), ("a", 40.0)], Config::default());
        let next = Model::new(
            &values(&[("a", 40.0), ("b", 40.0)]),
            Config::default(),
            Some(&state),
        )
        .unwrap();
        assert_eq!(next.values, values(&[("b", 40.0), ("a", 40.0)]));
    }

    #[test]
    fn gap_checks_both_candidate_neighbours_and_rejects_impossible_configuration() {
        let config = Config {
            step: 5.0,
            min_steps_between: 2,
            ..Config::default()
        };
        let state = model(&[("a", 20.0), ("b", 50.0), ("c", 90.0)], config);
        assert!(state.candidate("a", 45.0, None).is_none());
        assert!(state.candidate("a", 55.0, None).is_none());
        assert_eq!(
            state.candidate("a", 65.0, None).unwrap(),
            values(&[("b", 50.0), ("a", 65.0), ("c", 90.0)])
        );
        let error = Model::new(
            &values(&[("a", 0.0), ("b", 100.0)]),
            Config {
                min_steps_between: 101,
                ..Config::default()
            },
            None,
        )
        .unwrap_err();
        assert_eq!(error.field, "min_steps_between");
    }

    #[test]
    fn decimal_gap_accepts_adjacent_steps_at_a_nonzero_minimum() {
        let config = Config {
            min: 100.0,
            max: 200.0,
            step: 0.1,
            min_steps_between: 1,
            ..Config::default()
        };
        let state = model(&[("a", 100.2), ("b", 100.3)], config);
        assert_eq!(state.values, values(&[("a", 100.2), ("b", 100.3)]));
    }

    #[test]
    fn decimal_candidate_accepts_exact_gap_and_rejects_a_smaller_gap() {
        let config = Config {
            min: 100.0,
            max: 200.0,
            step: 0.1,
            min_steps_between: 1,
            ..Config::default()
        };
        let state = model(&[("a", 100.0), ("b", 100.3)], config);
        assert_eq!(
            state.candidate("a", 100.2, None).unwrap(),
            values(&[("a", 100.2), ("b", 100.3)])
        );
        assert!(state.candidate("a", 100.3, None).is_none());
        assert!(Model::new(
            &values(&[("a", 100.2), ("b", 100.3)]),
            Config {
                min_steps_between: 2,
                ..config
            },
            None,
        )
        .is_err());
    }

    #[test]
    fn decimal_preserve_order_uses_the_same_step_limits_as_gap_validation() {
        let config = Config {
            min: 100.0,
            max: 200.0,
            step: 0.1,
            min_steps_between: 1,
            preserve_order: true,
        };
        let state = model(&[("a", 100.1), ("b", 100.5), ("c", 100.7)], config);
        let neighbours = state.neighbours("b");
        assert_eq!(state.bounds("b", neighbours.as_ref()), (100.2, 100.6));
        assert_eq!(
            state.candidate("b", 100.0, neighbours.as_ref()).unwrap(),
            values(&[("a", 100.1), ("b", 100.2), ("c", 100.7)])
        );
        assert_eq!(
            state.candidate("b", 200.0, neighbours.as_ref()).unwrap(),
            values(&[("a", 100.1), ("b", 100.6), ("c", 100.7)])
        );
    }

    #[test]
    fn decimal_configuration_can_fill_the_range_with_exact_step_intervals() {
        let state = model(
            &[("a", 100.0), ("b", 100.1), ("c", 100.2), ("d", 100.3)],
            Config {
                min: 100.0,
                max: 100.3,
                step: 0.1,
                min_steps_between: 1,
                ..Config::default()
            },
        );
        assert_eq!(state.values.len(), 4);
    }

    #[test]
    fn clamped_maximum_does_not_expand_available_space() {
        let config = Config {
            min: 100.0,
            max: 100.26,
            step: 0.1,
            min_steps_between: 3,
            preserve_order: true,
        };
        assert!(Model::new(&values(&[("a", 100.0), ("b", 100.26)]), config, None).is_err());
    }

    #[test]
    fn clamped_maximum_requires_the_actual_minimum_distance() {
        let config = Config {
            min: 100.0,
            max: 100.26,
            step: 0.1,
            min_steps_between: 1,
            preserve_order: false,
        };
        assert!(Model::new(&values(&[("a", 100.2), ("b", 100.26)]), config, None).is_err());
        let state = model(&[("a", 100.1), ("b", 100.26)], config);
        assert!(state.candidate("a", 100.2, None).is_none());
        let state = model(&[("a", 100.1), ("b", 100.2)], config);
        assert!(state.candidate("b", 100.26, None).is_some());
        assert!(state.candidate("a", 100.26, None).is_none());
    }

    #[test]
    fn clamped_maximum_preserve_order_bounds_stay_on_valid_grid_points() {
        let config = Config {
            min: 100.0,
            max: 100.26,
            step: 0.1,
            min_steps_between: 1,
            preserve_order: true,
        };
        let state = model(&[("a", 100.1), ("b", 100.26)], config);
        assert_eq!(state.bounds("a", None), (100.0, 100.1));
        assert_eq!(state.bounds("b", None), (100.2, 100.26));
        assert_eq!(state.candidate("a", 200.0, None).unwrap(), state.values);
        assert_eq!(
            state.candidate("b", 100.0, None).unwrap(),
            values(&[("a", 100.1), ("b", 100.2)])
        );
    }

    #[test]
    fn large_minimum_does_not_hide_a_truncated_interval_in_value_tolerance() {
        let config = Config {
            min: 1_000_000_000.0,
            max: 1_000_000_000.000_002_6,
            step: 0.000_001,
            min_steps_between: 3,
            ..Config::default()
        };
        assert!(Model::new(
            &values(&[("a", config.min), ("b", config.max)]),
            config,
            None
        )
        .is_err());
    }

    #[test]
    fn terminal_candidates_keep_snap_semantics_and_allow_zero_gap() {
        for (max, expected) in [(100.24, 100.2), (100.26, 100.26), (100.3, 100.3)] {
            let config = Config {
                min: 100.0,
                max,
                step: 0.1,
                preserve_order: true,
                ..Config::default()
            };
            let state = model(&[("a", 100.1), ("b", max)], config);
            assert_eq!(state.bounds("a", None).1, expected);
            assert_eq!(
                state.candidate("a", max, None).unwrap(),
                values(&[("a", expected), ("b", expected)])
            );
        }
    }

    #[test]
    fn preserve_order_uses_gesture_start_neighbours_and_snaps_inside_limits() {
        let config = Config {
            step: 3.0,
            min_steps_between: 2,
            preserve_order: true,
            ..Config::default()
        };
        let state = model(&[("a", 21.0), ("b", 51.0), ("c", 90.0)], config);
        let neighbours = state.neighbours("b");
        assert_eq!(state.bounds("b", neighbours.as_ref()), (27.0, 84.0));
        assert_eq!(
            state.candidate("b", 100.0, neighbours.as_ref()).unwrap(),
            values(&[("a", 21.0), ("b", 84.0), ("c", 90.0)])
        );
    }

    #[test]
    fn invalid_numbers_and_duplicate_ids_report_only_the_prop() {
        for config in [
            Config {
                min: f64::NAN,
                ..Config::default()
            },
            Config {
                max: f64::INFINITY,
                ..Config::default()
            },
            Config {
                step: 0.0,
                ..Config::default()
            },
        ] {
            assert!(Model::new(&values(&[("a", 5.0)]), config, None).is_err());
        }
        assert_eq!(
            Model::new(
                &values(&[("private", 0.0), ("private", 50.0)]),
                Config::default(),
                None
            )
            .unwrap_err()
            .field,
            "values.id"
        );
        assert_eq!(
            Model::new(&values(&[("private", f64::NAN)]), Config::default(), None)
                .unwrap_err()
                .field,
            "values"
        );
    }

    #[test]
    fn track_ties_prefer_focused_thumb_then_declaration_order() {
        let state = model(&[("a", 20.0), ("b", 80.0)], Config::default());
        assert_eq!(state.nearest(50.0, &["a", "b"], None), Some("a"));
        assert_eq!(state.nearest(50.0, &["a", "b"], Some("b")), Some("b"));
    }

    #[test]
    fn keyboard_direction_uses_the_visual_axis_and_pages_are_ten_steps() {
        use crate::Orientation::{Horizontal, Vertical};
        assert_eq!(
            key("ArrowRight", Horizontal, true, false),
            Some(Key::Step(-1))
        );
        assert_eq!(key("ArrowUp", Horizontal, true, true), Some(Key::Step(1)));
        assert_eq!(key("ArrowUp", Vertical, true, true), Some(Key::Step(-1)));
        assert_eq!(key("ArrowLeft", Vertical, true, true), Some(Key::Step(-1)));
        assert_eq!(key("PageUp", Vertical, true, true), Some(Key::Step(10)));
        assert_eq!(
            key("PageDown", Horizontal, true, true),
            Some(Key::Step(-10))
        );
    }

    #[test]
    fn refused_preview_can_return_to_the_accepted_start_without_a_stale_commit() {
        let state = model(&[("a", 20.0)], Config::default());
        let mut session = Session::new(Input::Pointer(1), "a".into(), &state);
        session.request(state.candidate("a", 60.0, None).unwrap());
        session.request(state.candidate("a", 20.0, None).unwrap());
        assert_eq!(session.finish(), None);
    }

    #[test]
    fn session_commits_last_request_even_when_application_refuses_and_only_once() {
        let state = model(&[("a", 20.0)], Config::default());
        let mut session = Session::new(Input::Pointer(7), "a".into(), &state);
        session.request(values(&[("a", 60.0)]));
        assert_eq!(session.finish(), Some(values(&[("a", 60.0)])));
        assert_eq!(state.values, values(&[("a", 20.0)]));
        let unchanged = Session::new(Input::Keyboard, "a".into(), &state);
        assert_eq!(unchanged.finish(), None);
    }
}
