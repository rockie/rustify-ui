//! Sparse responsive values using the Themes breakpoints, in CSS pixels.

/// The six minimum viewport widths used by [`Responsive`].
pub const BREAKPOINTS: [u16; 6] = [0, 520, 768, 1024, 1280, 1640];

/// A single value or sparse values that inherit the closest smaller breakpoint.
///
/// `Responsive::from(value)` sets only `initial`. Leaving a field unset emits
/// no override at that breakpoint; an entirely empty value leaves CSS defaults
/// and inherited values in effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Responsive<T> {
    pub initial: Option<T>,
    pub xs: Option<T>,
    pub sm: Option<T>,
    pub md: Option<T>,
    pub lg: Option<T>,
    pub xl: Option<T>,
}

impl<T> Default for Responsive<T> {
    fn default() -> Self {
        Self {
            initial: None,
            xs: None,
            sm: None,
            md: None,
            lg: None,
            xl: None,
        }
    }
}

impl<T> From<T> for Responsive<T> {
    fn from(value: T) -> Self {
        Self {
            initial: Some(value),
            ..Self::default()
        }
    }
}

impl<T> Responsive<T> {
    /// Explicit values in increasing breakpoint order, without filling gaps.
    pub fn iter(&self) -> impl Iterator<Item = (u16, &T)> {
        BREAKPOINTS
            .into_iter()
            .zip([
                self.initial.as_ref(),
                self.xs.as_ref(),
                self.sm.as_ref(),
                self.md.as_ref(),
                self.lg.as_ref(),
                self.xl.as_ref(),
            ])
            .filter_map(|(width, value)| value.map(|value| (width, value)))
    }

    /// The effective explicit value at a viewport width, if one exists.
    pub fn at(&self, width: u16) -> Option<&T> {
        self.iter()
            .take_while(|(minimum, _)| *minimum <= width)
            .last()
            .map(|(_, value)| value)
    }

    /// Transforms only explicitly set fields, preserving the sparse shape.
    pub fn map<U>(&self, map: impl Fn(&T) -> U) -> Responsive<U> {
        Responsive {
            initial: self.initial.as_ref().map(&map),
            xs: self.xs.as_ref().map(&map),
            sm: self.sm.as_ref().map(&map),
            md: self.md.as_ref().map(&map),
            lg: self.lg.as_ref().map(&map),
            xl: self.xl.as_ref().map(&map),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparse_values_inherit_only_the_nearest_smaller_breakpoint() {
        let value = Responsive {
            xs: Some("small"),
            md: Some("large"),
            ..Responsive::default()
        };
        assert_eq!(value.at(519), None);
        assert_eq!(value.at(520), Some(&"small"));
        assert_eq!(value.at(1023), Some(&"small"));
        assert_eq!(value.at(1024), Some(&"large"));
        assert_eq!(value.at(1640), Some(&"large"));
        assert_eq!(
            value.iter().collect::<Vec<_>>(),
            [(520, &"small"), (1024, &"large")]
        );
    }

    #[test]
    fn each_breakpoint_changes_at_its_exact_css_pixel_boundary() {
        let values = Responsive {
            initial: Some(0),
            xs: Some(1),
            sm: Some(2),
            md: Some(3),
            lg: Some(4),
            xl: Some(5),
        };
        for (index, width) in BREAKPOINTS.into_iter().enumerate() {
            assert_eq!(values.at(width), Some(&index));
            if width > 0 {
                assert_eq!(values.at(width - 1), Some(&(index - 1)));
            }
        }
    }

    #[test]
    fn a_single_value_and_mapping_keep_the_sparse_shape() {
        assert_eq!(
            Responsive::from(3).map(|value| value * 2),
            Responsive::from(6)
        );
        assert_eq!(Responsive::<u8>::default().iter().count(), 0);
    }
}
