use std::ops::Neg;

use num::Num;

use crate::prelude::Coordinate;

/// Point in polygon algorithm
///
/// This algorithm determines whether a point is inside a polygon. It works by
/// drawing a horizontal line from the point to the right, and counting the
/// number of times the line intersects with the polygon. If the number of
/// intersections is odd, the point is inside the polygon. If the number of
/// intersections is even, the point is outside the polygon.
///
/// NOTE: This is a bit wonky with integer math. You may want to either increase
///       your number's resolution or resort to using floating point numbers.
///
pub fn point_in_polygon<T: Num + Ord + Copy + Neg<Output = T>>(
    vertices: &[Coordinate<T>],
    query: Coordinate<T>,
) -> bool {
    let mut result = false;

    let n = vertices.len();

    for i in 0..n {
        let j = (i + 1) % n;

        let a = vertices[i];
        let b = vertices[j];

        // Is a above the query point?
        let first_is_above = a.y > query.y;

        // Is b below the query point?
        let second_is_below = b.y <= query.y;

        // If both are above or both are below, there can be no intersection.
        // So we need to check whether they are on opposite sides of the query
        let opposite_sides = first_is_above != second_is_below;

        if !opposite_sides {
            continue;
        }

        // This calculates the x-coordinate of the intersection of the line
        // between a and b with the horizontal line through the query point.
        let horizontal_intersection_point = {
            let horizontal_distance_ab = b.x - a.x;
            let vertical_distance_ab = b.y - a.y;

            if vertical_distance_ab == T::zero() {
                a.x
            } else {
                horizontal_distance_ab * (query.y - a.y) / (vertical_distance_ab) + a.x
            }
        };

        // If the intersection point is to the right of the query point, we
        // count it as an intersection.
        if query.x < horizontal_intersection_point {
            result = !result;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((0, 0), true)]
    #[case((1, 5), true)]
    #[case((-1, 5), true)]
    #[case((1, 15), false)]
    #[case((-1, 15), false)]
    #[case((15, 2), false)]
    #[case((15, -7), false)]
    #[case((0, 5), true)]
    #[case((0, 10), false)]
    #[case((3, -9), true)]
    #[case((-9, -9), true)]
    #[case((9, -9), true)]
    #[case((-9, 9), true)]
    #[case((-9, -9), true)]
    #[case((-11, -11), false)]
    #[case((11, -11), false)]
    #[case((-11, 11), false)]
    #[case((-11, -11), false)]
    #[case((10, 10), false)]
    #[case((4, 10), false)]
    #[case((5, 10), false)]
    #[case((5, -10), true)]
    #[case((10, 5), false)]
    #[case((-10, 5), true)]
    fn test_point_in_polygon(#[case] query: (isize, isize), #[case] expected: bool) {
        let vertices = vec![
            (-10, -10).into(),
            (-10, 10).into(),
            (10, 10).into(),
            (10, -10).into(),
        ];

        assert_eq!(point_in_polygon(&vertices, query.into()), expected);
    }
}
