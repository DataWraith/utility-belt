use num::{Num, Signed};

// This computes the intersection point of two lines (if any), given as pairs of points.
//
// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
//
// NOTE: This returns `None` for coincidental lines, you'll have to check for
//       that separately.
//
// NOTE: If you don't use sufficiently large numeric types, this may overflow
//       during multiplication and panic.
pub fn line_intersection_point<T: Num + Clone + Signed>(
    a: ((T, T), (T, T)),
    b: ((T, T), (T, T)),
) -> Option<(T, T)> {
    let x1 = &(a.0).0;
    let y1 = &(a.0).1;
    let x2 = &(a.1).0;
    let y2 = &(a.1).1;

    let x3 = &(b.0).0;
    let y3 = &(b.0).1;
    let x4 = &(b.1).0;
    let y4 = &(b.1).1;

    let denominator = (x1.clone() - x2.clone()) * (y3.clone() - y4.clone())
        - (y1.clone() - y2.clone()) * (x3.clone() - x4.clone());

    if denominator.is_zero() {
        return None;
    }

    let x = (x1.clone() * y2.clone() - y1.clone() * x2.clone()) * (x3.clone() - x4.clone())
        - (x1.clone() - x2.clone()) * (x3.clone() * y4.clone() - y3.clone() * x4.clone())
            / denominator.clone();

    let y = (x1.clone() * y2.clone() - y1.clone() * x2.clone()) * (y3.clone() - y4.clone())
        - (y1.clone() - y2.clone()) * (x3.clone() * y4.clone() - y3.clone() * x4.clone())
            / denominator.clone();

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
    fn test_with_rationals() {
        let x1 = Ratio::new(0xfbbb8c2bc3ed57i128, 1);
        let y1 = Ratio::new(0xb120ca41205a7ci128, 1);

        let x2 = Ratio::new(0x649fefb18d3ed0i128, 1);
        let y2 = Ratio::new(0xfd63e6bbb2f6a9i128, 1);

        let dx1 = Ratio::new(1i128, 1i128);
        let dy1 = Ratio::new(1i128, 1i128);

        let dx2 = Ratio::new(1i128, 1i128);
        let dy2 = Ratio::new(-1i128, 1i128);

        assert!(dbg!(line_intersection_point(
            ((x1, y1), (x1 + dx1, y1 + dy1)),
            ((x2, y2), (x2 + dx2, y2 + dy2))
        ))
        .is_some());
    }
}
