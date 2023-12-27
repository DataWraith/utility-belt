use num::{Num, Signed};

// This computes the intersection point of two lines (if any), given as pairs of points.
//
// TODO: Handle the case where the lines are vertical or identical.
pub fn line_intersection_point<T: Num + Copy + Signed>(
    a: ((T, T), (T, T)),
    b: ((T, T), (T, T)),
) -> Option<(T, T)> {
    let x1 = (a.0).0;
    let y1 = (a.0).1;
    let dx1 = (a.1).0 - x1;
    let dy1 = (a.1).1 - y1;

    let x2 = (b.0).0;
    let y2 = (b.0).1;
    let dx2 = (b.1).0 - x2;
    let dy2 = (b.1).1 - y2;

    // If both lines are vertical then they are parallel and will never intersect.
    if dx1 == T::zero() && dx2 == T::zero() {
        return None;
    }

    // TODO: Handle this case
    if dx1 == T::zero() || dx2 == T::zero() {
        return None;
    }

    // Calculate slopes
    let m1 = dy1 / dx1;
    let m2 = dy2 / dx2;

    // Identical slopes -- The lines are either identical or parallel.
    if m1 == m2 {
        return None;
    }

    let x = (m1 * x1 - m2 * x2 + y2 - y1) / (m1 - m2);
    let y = m1 * (x - x1) + y1;

    Some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_intersection_point() {
        assert_eq!(
            line_intersection_point(((0, 0), (1, 1)), ((0, 1), (1, 0))),
            Some((0, 0))
        );

        assert_eq!(
            line_intersection_point(((0, 0), (1, 1)), ((0, 2), (2, 0))),
            Some((1, 1))
        );

        assert_eq!(
            line_intersection_point(((0, 0), (1, 1)), ((2, 2), (3, 3))),
            None
        )
    }
}
