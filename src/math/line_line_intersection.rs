use num::{Num, Signed};

/// This computes the intersection point of two lines (if any), given as pairs of points.
///
/// NOTE: This returns `None` if the lines are coincidental, so you may need to check for
///       that case separately.
///
/// Reference:
///   https://www.topcoder.com/thrive/articles/Geometry%20Concepts%20part%202:%20%20Line%20Intersection%20and%20its%20Applications#LineLineIntersection
///
pub fn line_intersection_point<T: Num + Clone + Signed + std::ops::Sub>(
    a: ((T, T), (T, T)),
    b: ((T, T), (T, T)),
) -> Option<(T, T)> {
    let x1 = (a.0).0.clone();
    let y1 = (a.0).1.clone();

    let a1 = (a.1).1.clone() - y1.clone();
    let b1 = x1.clone() - (a.1).0.clone();
    let c1 = (a1.clone() * x1.clone()) + (b1.clone() * y1.clone());

    let x2 = (b.0).0.clone();
    let y2 = (b.0).1.clone();

    let a2 = (b.1).1.clone() - y2.clone();
    let b2 = x2.clone() - (b.1).0.clone();
    let c2 = (a2.clone() * x2.clone()) + (b2.clone() * y2.clone());

    let det = a1.clone() * b2.clone() - a2.clone() * b1.clone();

    if det == T::zero() {
        // Lines are parallel or coincidental
        return None;
    }

    let x = (b2.clone() * c1.clone() - b1.clone() * c2.clone()) / det.clone();
    let y = (a1.clone() * c2.clone() - a2.clone() * c1.clone()) / det.clone();

    Some((x, y))
}

#[cfg(test)]
mod tests {
    use num::rational::Ratio;

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

    #[test]
    fn test_coincidental_lines() {
        assert_eq!(
            line_intersection_point(((0, 0), (1, 1)), ((0, 0), (1, 1))),
            None,
        );

        assert_eq!(
            line_intersection_point(((0, 0), (1, 1)), ((1, 1), (15, 15))),
            None,
        );
    }

    #[test]
    fn test_parallel_lines() {
        assert_eq!(
            line_intersection_point(((0, 0), (1, 1)), ((0, 1), (1, 2))),
            None,
        );

        assert_eq!(
            line_intersection_point(((0, 0), (1, 1)), ((1, 0), (2, 1))),
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
            ((x2, y2), (x2 + dx2, y2 + dy2))
        )
        .is_some());
    }
}
