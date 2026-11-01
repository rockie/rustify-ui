use super::ProgressValue;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct State {
    pub value: ProgressValue,
    pub issue: Option<&'static str>,
}
impl State {
    pub fn new(value: Option<f64>, max: f64) -> Self {
        let invalid_max = !max.is_finite() || max <= 0.;
        let max = if invalid_max { 100. } else { max };
        let invalid_value =
            value.is_some_and(|value| !value.is_finite() || value < 0. || value > max);
        Self {
            value: ProgressValue {
                value: if invalid_value { None } else { value },
                max,
            },
            issue: if invalid_max {
                Some("max")
            } else if invalid_value {
                Some("value")
            } else {
                None
            },
        }
    }
    pub fn state(self) -> &'static str {
        match self.value.value {
            None => "indeterminate",
            Some(value) if value == self.value.max => "complete",
            Some(_) => "loading",
        }
    }
    pub fn percent(self) -> Option<f64> {
        self.value.value.map(|value| value / self.value.max * 100.)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_maximums_use_the_documented_default_and_keep_a_field_diagnostic() {
        for max in [0., -1., f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let state = State::new(Some(25.), max);
            assert_eq!(state.value.max, 100.);
            assert_eq!(state.issue, Some("max"));
            assert_eq!(state.value.value, Some(25.));
        }
    }
    #[test]
    fn invalid_values_do_not_claim_a_clamped_percentage() {
        for value in [-1., 76., f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let state = State::new(Some(value), 75.);
            assert_eq!(state.value.value, None);
            assert_eq!(state.issue, Some("value"));
            assert_eq!(state.state(), "indeterminate");
            assert_eq!(state.percent(), None);
        }
    }
    #[test]
    fn known_progress_keeps_original_units_and_uses_a_ratio_only_for_visuals() {
        let state = State::new(Some(37.5), 75.);
        assert_eq!(
            state.value,
            ProgressValue {
                value: Some(37.5),
                max: 75.
            }
        );
        assert_eq!(state.state(), "loading");
        assert_eq!(state.percent(), Some(50.));
    }
    #[test]
    fn zero_complete_and_unknown_have_distinct_observable_states() {
        assert_eq!(State::new(Some(0.), 75.).state(), "loading");
        assert_eq!(State::new(Some(75.), 75.).state(), "complete");
        assert_eq!(State::new(None, 75.).state(), "indeterminate");
        assert_eq!(State::new(None, 75.).percent(), None);
    }
}
