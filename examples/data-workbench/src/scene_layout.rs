//! Where the scene's ten thousand objects are.
//!
//! The layout is arithmetic on an index, not a stored list: a region that has
//! to decide what is on screen sixty times a second answers that from a range
//! of rows and columns rather than by testing ten thousand rectangles, and a
//! second implementation elsewhere can work out the same answer to check a
//! marquee selection against.

/// Objects in the scene. PRD load B3.
pub const COUNT: usize = 10_000;
/// Objects per row of the grid.
pub const COLUMNS: usize = 100;

pub const WIDTH: f64 = 120.0;
pub const HEIGHT: f64 = 24.0;
pub const STEP_X: f64 = 130.0;
pub const STEP_Y: f64 = 40.0;

/// Every tenth object is drawn shifted, which is what puts a second layer over
/// the first. Two layers at most: the shift is smaller than one step across
/// and smaller than the gap down, so an object can be covered by one neighbour
/// and never by two.
pub const OVERLAY_EVERY: usize = 10;
pub const OVERLAY_X: f64 = 60.0;
pub const OVERLAY_Y: f64 = 12.0;

/// The scene's own extent, which is what a camera is clamped to. It follows
/// from the formulas above rather than being chosen: the shifted objects of
/// the last column and the last row are what reach furthest.
pub const EXTENT_X: f64 = (COLUMNS as f64 - 1.0) * STEP_X + OVERLAY_X + WIDTH;
pub const EXTENT_Y: f64 = (COUNT / COLUMNS) as f64 * STEP_Y - STEP_Y + OVERLAY_Y + HEIGHT;

/// Where one object sits, in scene coordinates. The index is the identity:
/// nothing about an object moves, so there is nothing else to hold.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SceneRect {
    pub x: f64,
    pub y: f64,
}

/// Whether this object is on the upper layer. The upper layer is drawn last
/// and hit first, so what a pointer picks is what a person sees.
pub fn is_overlay(index: usize) -> bool {
    index % OVERLAY_EVERY == OVERLAY_EVERY - 1
}

pub fn rect(index: usize) -> SceneRect {
    let column = (index % COLUMNS) as f64;
    let row = (index / COLUMNS) as f64;
    let (shift_x, shift_y) = if is_overlay(index) {
        (OVERLAY_X, OVERLAY_Y)
    } else {
        (0.0, 0.0)
    };
    SceneRect {
        x: column * STEP_X + shift_x,
        y: row * STEP_Y + shift_y,
    }
}

/// The object's label: eight characters, so every one of them is a string the
/// shaper has to lay out and none of them is the same string twice.
pub fn label(index: usize) -> String {
    format!("OBJ{index:05}")
}

/// Which objects a camera at `(x, y)` can see in a pane of `width` by
/// `height`, as inclusive column and row bounds.
///
/// The bounds are widened by one either way so that an object whose body
/// starts before the pane, or whose shifted copy reaches past it, is still
/// drawn - otherwise panning would make objects appear at the edge rather than
/// arrive at it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Visible {
    pub first_column: usize,
    pub last_column: usize,
    pub first_row: usize,
    pub last_row: usize,
}

impl Visible {
    pub fn count(&self) -> usize {
        self.indices().count()
    }

    pub fn contains(&self, index: usize) -> bool {
        let column = index % COLUMNS;
        let row = index / COLUMNS;
        column >= self.first_column
            && column <= self.last_column
            && row >= self.first_row
            && row <= self.last_row
    }

    /// The indices inside the window, row by row. Drawing takes two passes
    /// over this, because the upper layer has to go down last.
    pub fn indices(&self) -> impl Iterator<Item = usize> + '_ {
        (self.first_row..=self.last_row)
            .flat_map(move |row| {
                (self.first_column..=self.last_column).map(move |column| row * COLUMNS + column)
            })
            .filter(|index| *index < COUNT)
    }
}

pub fn visible(camera_x: f64, camera_y: f64, width: f64, height: f64) -> Visible {
    let rows = COUNT / COLUMNS;
    let first_column = ((camera_x / STEP_X).floor() as isize - 1).clamp(0, COLUMNS as isize - 1);
    let last_column =
        (((camera_x + width) / STEP_X).ceil() as isize).clamp(first_column, COLUMNS as isize - 1);
    let first_row = ((camera_y / STEP_Y).floor() as isize - 1).clamp(0, rows as isize - 1);
    let last_row =
        (((camera_y + height) / STEP_Y).ceil() as isize).clamp(first_row, rows as isize - 1);
    Visible {
        first_column: first_column as usize,
        last_column: last_column as usize,
        first_row: first_row as usize,
        last_row: last_row as usize,
    }
}

/// Whether an object's body meets the rectangle `(x, y, width, height)`.
///
/// Touching counts as meeting: a marquee drawn along an object's edge took it.
fn meets(index: usize, x: f64, y: f64, width: f64, height: f64) -> bool {
    let object = rect(index);
    object.x <= x + width
        && x <= object.x + WIDTH
        && object.y <= y + height
        && y <= object.y + HEIGHT
}

/// The columns and rows an area could reach, widened by the shift that puts
/// the upper layer off its own slot.
fn candidates(x: f64, y: f64, width: f64, height: f64) -> Visible {
    let rows = COUNT / COLUMNS;
    let first_column = (((x - WIDTH) / STEP_X).floor() as isize).clamp(0, COLUMNS as isize - 1);
    let last_column =
        (((x + width) / STEP_X).ceil() as isize).clamp(first_column, COLUMNS as isize - 1);
    let first_row = (((y - HEIGHT) / STEP_Y).floor() as isize).clamp(0, rows as isize - 1);
    let last_row = (((y + height) / STEP_Y).ceil() as isize).clamp(first_row, rows as isize - 1);
    Visible {
        first_column: first_column as usize,
        last_column: last_column as usize,
        first_row: first_row as usize,
        last_row: last_row as usize,
    }
}

/// The objects a marquee over `(x, y, width, height)` took, in index order.
///
/// The rectangle is in scene coordinates, so it means the same thing whatever
/// the camera was doing when it was drawn. Only the rows and columns the area
/// could possibly reach are tested: a marquee over one object must not cost
/// ten thousand comparisons.
pub fn within(x: f64, y: f64, width: f64, height: f64) -> Vec<usize> {
    candidates(x, y, width, height)
        .indices()
        .filter(|index| meets(*index, x, y, width, height))
        .collect()
}

/// The object under a point, or nothing. The upper layer is drawn last, so it
/// is tested first: what a pointer picks is what a person sees.
pub fn pick(x: f64, y: f64) -> Option<usize> {
    if x < 0.0 || y < 0.0 {
        return None;
    }
    let candidates = candidates(x, y, 0.0, 0.0);
    let under = |index: usize| {
        let object = rect(index);
        x >= object.x && x < object.x + WIDTH && y >= object.y && y < object.y + HEIGHT
    };
    for upper in [true, false] {
        for index in candidates.indices() {
            if is_overlay(index) == upper && under(index) {
                return Some(index);
            }
        }
    }
    None
}

/// Keeps a camera inside the scene, given the size of the pane looking at it.
pub fn clamp(camera_x: f64, camera_y: f64, width: f64, height: f64) -> (f64, f64) {
    (
        camera_x.clamp(0.0, (EXTENT_X - width).max(0.0)),
        camera_y.clamp(0.0, (EXTENT_Y - height).max(0.0)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_the_middle_and_the_last_are_where_the_formula_puts_them() {
        assert_eq!(rect(0), SceneRect { x: 0.0, y: 0.0 });
        assert_eq!(rect(5_000), SceneRect { x: 0.0, y: 2_000.0 });
        // The last object is on the upper layer: 9,999 % 10 == 9.
        assert!(is_overlay(9_999));
        assert_eq!(
            rect(9_999),
            SceneRect {
                x: 99.0 * STEP_X + OVERLAY_X,
                y: 99.0 * STEP_Y + OVERLAY_Y
            }
        );
    }

    #[test]
    fn the_extent_holds_every_object() {
        for index in [0, 9, 99, 5_000, 9_990, 9_999] {
            let rect = rect(index);
            assert!(rect.x + WIDTH <= EXTENT_X, "{index} runs past the right");
            assert!(rect.y + HEIGHT <= EXTENT_Y, "{index} runs past the bottom");
        }
        assert_eq!(EXTENT_X, 13_050.0);
        assert_eq!(EXTENT_Y, 3_996.0);
    }

    #[test]
    fn no_point_is_under_more_than_two_objects() {
        // Only the first two rows, at the resolution the layout is defined
        // with: what is being checked is the rule, not every pixel.
        for step_y in 0..80 {
            for step_x in 0..400 {
                let (x, y) = (step_x as f64, step_y as f64);
                let covering = (0..200)
                    .filter(|index| {
                        let rect = rect(*index);
                        x >= rect.x && x < rect.x + WIDTH && y >= rect.y && y < rect.y + HEIGHT
                    })
                    .count();
                assert!(covering <= 2, "{covering} objects cover ({x}, {y})");
            }
        }
    }

    #[test]
    fn a_pane_sees_a_window_not_the_scene() {
        let window = visible(0.0, 0.0, 1_200.0, 700.0);
        assert_eq!(window.first_column, 0);
        assert_eq!(window.first_row, 0);
        // Ten columns and eighteen rows of a hundred by a hundred.
        assert!(window.count() < 400, "{} is not a window", window.count());
        assert!(window.contains(0));
        assert!(!window.contains(50));
    }

    #[test]
    fn a_camera_at_the_far_corner_still_sees_the_last_object() {
        let (x, y) = clamp(1e6, 1e6, 1_200.0, 700.0);
        let window = visible(x, y, 1_200.0, 700.0);
        assert!(window.contains(9_999));
        assert_eq!(window.last_column, COLUMNS - 1);
        assert_eq!(window.last_row, COUNT / COLUMNS - 1);
    }

    #[test]
    fn a_marquee_takes_what_it_touches_and_nothing_it_misses() {
        // The first two of the top row, and not the third.
        let hit = within(0.0, 0.0, STEP_X + 1.0, HEIGHT);
        assert_eq!(hit, vec![0, 1]);
    }

    #[test]
    fn an_object_is_taken_where_it_is_drawn_not_where_its_index_says() {
        // Object 9 is on the upper layer, so it sits sixty across and twelve
        // down from the slot its column and row name.
        let slot = SceneRect {
            x: 9.0 * STEP_X,
            y: 0.0,
        };
        assert!(!within(slot.x, slot.y, WIDTH, 1.0).contains(&9));
        let drawn = rect(9);
        assert!(within(drawn.x, drawn.y, 1.0, 1.0).contains(&9));
    }

    #[test]
    fn the_upper_layer_is_picked_where_the_two_overlap() {
        // Where object 9's shifted body lies over object 10's.
        let over = rect(9);
        let under = rect(10);
        assert!(over.x + WIDTH > under.x && over.y + HEIGHT > under.y);
        let x = under.x + 1.0;
        let y = over.y + 1.0;
        assert_eq!(pick(x, y), Some(9));
        // A hand's width to the right, past object 9, is object 10 alone.
        assert_eq!(pick(over.x + WIDTH + 1.0, y), Some(10));
    }

    #[test]
    fn the_gaps_between_objects_are_nothing() {
        assert_eq!(pick(-1.0, -1.0), None);
        // Between two rows: the step down is forty and an object is twenty
        // four tall.
        assert_eq!(pick(0.0, HEIGHT + 1.0), None);
        assert_eq!(pick(EXTENT_X + 1.0, 0.0), None);
    }

    #[test]
    fn every_object_is_picked_at_its_own_corner() {
        for index in [0, 1, 9, 10, 99, 100, 5_000, 9_990, 9_999] {
            let rect = rect(index);
            let picked = pick(rect.x + WIDTH / 2.0, rect.y + HEIGHT / 2.0);
            // The middle of an object is either itself or whatever covers it,
            // and what covers it is always on the upper layer.
            let picked = picked.expect("a point on an object picks something");
            assert!(
                picked == index || is_overlay(picked),
                "{index} was picked as {picked}"
            );
        }
    }

    #[test]
    fn a_marquee_over_the_whole_scene_takes_every_object() {
        assert_eq!(within(0.0, 0.0, EXTENT_X, EXTENT_Y).len(), COUNT);
    }
}
