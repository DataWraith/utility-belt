use ndarray::array;
use num::{Num, Signed};

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
pub fn line_intersection_point<T: Num + Signed + PartialOrd + Clone + std::fmt::Debug>(
    a: ((T, T), (T, T)),
    b: ((T, T), (T, T)),
    eps: T,
) -> Option<(T, T)> {
    let ((x1, y1), (x2, y2)) = a;
    let ((x3, y3), (x4, y4)) = b;

    let line1_is_vertical = x1 == x2;
    let line2_is_vertical = x3 == x4;

    if line1_is_vertical && line2_is_vertical {
        // Both lines are vertical -> Parallel or coincidental
        return None;
    }

    let slope1 = if line1_is_vertical {
        T::zero()
    } else {
        (y2.clone() - y1.clone()) / (x2.clone() - x1.clone())
    };

    let slope2 = if line2_is_vertical {
        T::zero()
    } else {
        (y4.clone() - y3.clone()) / (x4.clone() - x3.clone())
    };

    let intercept1 = y1.clone() - slope1.clone() * x1.clone();
    let intercept2 = y3.clone() - slope2.clone() * x3.clone();

    if line1_is_vertical {
        return Some((T::zero(), intercept2.clone()));
    }

    if line2_is_vertical {
        return Some((T::zero(), intercept1.clone()));
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
        //   -m * x     +   1*y    =    b
        [-slope1.clone(), T::one(), intercept1.clone()],
        [-slope2.clone(), T::one(), intercept2.clone()],
    ];

    let mut ans = array![T::zero(), T::zero()];
    let soln = gauss_jordan(matrix, &mut ans, eps);

    if soln != super::Solution::Unique {
        return None;
    }

    Some((ans[0].clone(), ans[1].clone()))
}

#[cfg(test)]
mod tests {
    use num::rational::Ratio;

    use super::*;

    #[test]
    fn test_line_intersection_point() {
        assert_eq!(
            line_intersection_point(((0., 0.), (1., 1.)), ((0., 1.), (1., 0.)), 1e-9),
            Some((0.5, 0.5))
        );

        assert_eq!(
            line_intersection_point(((0.0, 0.0), (1.0, 1.0)), ((0.0, 2.0), (2.0, 0.0)), 1e-9),
            Some((1.0, 1.0))
        );

        assert_eq!(
            line_intersection_point(((0.0, 0.0), (1.0, 1.0)), ((2.0, 2.0), (3.0, 3.0)), 1e-9),
            None
        )
    }

    #[test]
    fn test_coincidental_lines() {
        assert_eq!(
            line_intersection_point(((0.0, 0.0), (1.0, 1.0)), ((0.0, 0.0), (1.0, 1.0)), 1e-9),
            None,
        );

        assert_eq!(
            line_intersection_point(((0.0, 0.0), (1.0, 1.0)), ((1.0, 1.0), (1.05, 1.05)), 1e-9),
            None,
        );
    }

    #[test]
    fn test_parallel_lines() {
        assert_eq!(
            line_intersection_point(((0.0, 0.0), (1.0, 1.0)), ((0.0, 1.0), (1.0, 2.0)), 1e-9),
            None,
        );

        assert_eq!(
            line_intersection_point(((0.0, 0.0), (1.0, 1.0)), ((1.0, 0.0), (2.0, 1.0)), 1e-9),
            None,
        );
    }

    #[test]
    fn test_vertical_lines() {
        assert_eq!(
            line_intersection_point(((0.0, 0.0), (0.0, 1.0)), ((0.0, 1.0), (1.0, 1.0)), 1e-9),
            Some((0.0, 1.0)),
        );

        assert_eq!(
            line_intersection_point(((0.0, 0.0), (0.0, 1.0)), ((5.0, 4.0), (-5.0, -2.0)), 1e-9),
            Some((0.0, 1.0)),
        );

        assert_eq!(
            line_intersection_point(((0.0, 0.0), (0.0, 1.0)), ((1.0, 1.0), (1.0, 2.0)), 1e-9),
            None,
        );
    }

    #[test]
    fn test_with_rationals() {
        let x1 = Ratio::new(0xfbbb8c2bc3ed57i128, 1);
        let y1 = Ratio::new(0xb120ca41205a7ci128, 1);

        let x2 = Ratio::new(0x649fefb18d3ed0i128, 1);
        let y2 = Ratio::new(0xfd63e6bbb2f6a9i128, 1);

        let dx1 = Ratio::new(1i128, 1i128);
        let dy1 = Ratio::new(1i128, 1i128);

        let dx2 = Ratio::new(1i128, 1i128);
        let dy2 = Ratio::new(-1i128, 1i128);

        assert!(line_intersection_point(
            ((x1, y1), (x1 + dx1, y1 + dy1)),
            ((x2, y2), (x2 + dx2, y2 + dy2)),
            Ratio::new(0i128, 1i128)
        )
        .is_some());
    }
}
