use ndarray::array;

use crate::prelude::{Coordinate, CoordinateNum};

use super::gauss_jordan;

/// This computes the intersection point (if one exists) of two lines given as pairs of points.
///
/// NOTE: This returns `None` if the lines are coincidental, so you may need to check for
///       that case separately.
///
/// # Arguments
///
/// * `a` - The first line, given as a pair of points.
/// * `b` - The second line, given as a pair of points.
/// * `eps` - A small value (e.g. 1e-9) to help with limited floating point precision.
pub fn line_intersection_point<T: CoordinateNum>(
    a: (Coordinate<T>, Coordinate<T>),
    b: (Coordinate<T>, Coordinate<T>),
    eps: T,
) -> Option<Coordinate<T>> {
    let (x1, y1) = (a.0.x, a.0.y);
    let (x2, y2) = (a.1.x, a.1.y);
    let (x3, y3) = (b.0.x, b.0.y);
    let (x4, y4) = (b.1.x, b.1.y);

    let line1_is_vertical = x1 == x2;
    let line2_is_vertical = x3 == x4;

    if line1_is_vertical && line2_is_vertical {
        // Both lines are vertical -> Parallel or coincidental
        return None;
    }

    let slope1 = if line1_is_vertical {
        T::zero()
    } else {
        (y2 - y1) / (x2 - x1)
    };

    let slope2 = if line2_is_vertical {
        T::zero()
    } else {
        (y4 - y3) / (x4 - x3)
    };

    let intercept1 = y1 - slope1 * x1;
    let intercept2 = y3 - slope2 * x3;

    if line1_is_vertical {
        return Some((T::zero(), intercept2).into());
    }

    if line2_is_vertical {
        return Some((T::zero(), intercept1).into());
    }

    if slope1 == slope2 && intercept1 == intercept2 {
        // Lines are coincidental
        return None;
    }

    if slope1 == slope2 {
        // Lines are parallel
        return None;
    }

    // This simple system of equations follows directly from the line equation:
    //
    //             y = m * x + b
    // =>          0 = m * x + b - y
    // =>         -b = m * x - y
    // =>          b = -m * x + y
    // => -m * x + y = b
    //
    let matrix = array![
        // -m*x  +  1*y    =    b
        [-slope1, T::one(), intercept1],
        [-slope2, T::one(), intercept2],
    ];

    let mut ans = array![T::zero(), T::zero()];
    let soln = gauss_jordan(matrix, &mut ans, eps);

    if soln != super::Solution::Unique {
        return None;
    }

    let result = Coordinate::new(ans[0], ans[1]);

    Some(result)
}

/// Computes the intersection point of two line segments, if one exists.
///
/// # Arguments
///
/// * `a` - The first line segment, given as a pair of points.
/// * `b` - The second line segment, given as a pair of points.
/// * `eps` - A small value (e.g. 1e-9) to help with limited floating point precision.
pub fn segment_intersection_point<T: CoordinateNum>(
    a: (Coordinate<T>, Coordinate<T>),
    b: (Coordinate<T>, Coordinate<T>),
    eps: T,
) -> Option<Coordinate<T>> {
    // First check if the infinite lines intersect
    let intersection = line_intersection_point(a, b, eps)?;

    // Check if intersection point lies within both segments
    let in_segment = |p: &Coordinate<T>, seg: &(Coordinate<T>, Coordinate<T>)| {
        let (min_x, max_x) = if seg.0.x <= seg.1.x {
            (seg.0.x, seg.1.x)
        } else {
            (seg.1.x, seg.0.x)
        };

        let (min_y, max_y) = if seg.0.y <= seg.1.y {
            (seg.0.y, seg.1.y)
        } else {
            (seg.1.y, seg.0.y)
        };

        p.x >= min_x - eps && p.x <= max_x + eps && p.y >= min_y - eps && p.y <= max_y + eps
    };

    if in_segment(&intersection, &a) && in_segment(&intersection, &b) {
        Some(intersection)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_intersection_point() {
        assert_eq!(
            line_intersection_point(
                ((0., 0.).into(), (1., 1.).into()),
                ((0., 1.).into(), (1., 0.).into()),
                1e-9
            ),
            Some((0.5, 0.5).into())
        );

        assert_eq!(
            line_intersection_point(
                ((0.0, 0.0).into(), (1.0, 1.0).into()),
                ((0.0, 2.0).into(), (2.0, 0.0).into()),
                1e-9
            ),
            Some((1.0, 1.0).into())
        );

        assert_eq!(
            line_intersection_point(
                ((0.0, 0.0).into(), (1.0, 1.0).into()),
                ((2.0, 2.0).into(), (3.0, 3.0).into()),
                1e-9
            ),
            None
        )
    }

    #[test]
    fn test_coincidental_lines() {
        assert_eq!(
            line_intersection_point(
                ((0.0, 0.0).into(), (1.0, 1.0).into()),
                ((0.0, 0.0).into(), (1.0, 1.0).into()),
                1e-9
            ),
            None,
        );

        assert_eq!(
            line_intersection_point(
                ((0.0, 0.0).into(), (1.0, 1.0).into()),
                ((1.0, 1.0).into(), (1.05, 1.05).into()),
                1e-9
            ),
            None,
        );
    }

    #[test]
    fn test_parallel_lines() {
        assert_eq!(
            line_intersection_point(
                ((0.0, 0.0).into(), (1.0, 1.0).into()),
                ((0.0, 1.0).into(), (1.0, 2.0).into()),
                1e-9,
            ),
            None,
        );

        assert_eq!(
            line_intersection_point(
                ((0.0, 0.0).into(), (1.0, 1.0).into()),
                ((1.0, 0.0).into(), (2.0, 1.0).into()),
                1e-9,
            ),
            None,
        );
    }

    #[test]
    fn test_vertical_lines() {
        assert_eq!(
            line_intersection_point(
                ((0.0, 0.0).into(), (0.0, 1.0).into()),
                ((0.0, 1.0).into(), (1.0, 1.0).into()),
                1e-9,
            ),
            Some((0.0, 1.0).into()),
        );

        assert_eq!(
            line_intersection_point(
                ((0.0, 0.0).into(), (0.0, 1.0).into()),
                ((5.0, 4.0).into(), (-5.0, -2.0).into()),
                1e-9,
            ),
            Some((0.0, 1.0).into()),
        );

        assert_eq!(
            line_intersection_point(
                ((0.0, 0.0).into(), (0.0, 1.0).into()),
                ((1.0, 1.0).into(), (1.0, 2.0).into()),
                1e-9,
            ),
            None,
        );
    }

    #[test]
    fn test_with_rationals() {
        use num::rational::Ratio;

        let x1 = Ratio::new(0xfbbb8c2bc3ed57i128, 1);
        let y1 = Ratio::new(0xb120ca41205a7ci128, 1);
        let a = (Coordinate::new(x1, y1), Coordinate::new(x1 + 1, y1 + 1));

        let x2 = Ratio::new(0x649fefb18d3ed0i128, 1);
        let y2 = Ratio::new(0xfd63e6bbb2f6a9i128, 1);
        let b = (Coordinate::new(x2, y2), Coordinate::new(x2 + 1, y2 - 1));

        assert!(line_intersection_point(a, b, Ratio::new(0i128, 1i128)).is_some());
    }

    #[test]
    fn test_segment_intersection() {
        // Segments that intersect
        assert_eq!(
            segment_intersection_point(
                ((0., 0.).into(), (1., 1.).into()),
                ((0., 1.).into(), (1., 0.).into()),
                1e-9
            ),
            Some((0.5, 0.5).into())
        );

        // Segments that don't intersect (but their infinite lines would)
        assert_eq!(
            segment_intersection_point(
                ((0., 0.).into(), (1., 1.).into()),
                ((2., -1.).into(), (3., -2.).into()),
                1e-9
            ),
            None
        );

        // Parallel segments
        assert_eq!(
            segment_intersection_point(
                ((0., 0.).into(), (1., 1.).into()),
                ((0., 1.).into(), (1., 2.).into()),
                1e-9
            ),
            None
        );
    }
}
