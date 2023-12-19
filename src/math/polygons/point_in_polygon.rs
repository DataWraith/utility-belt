use num::Num;

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
pub fn point_in_polygon<T: Num + Ord + Copy>(vertices: &[(T, T)], query: (T, T)) -> bool {
    let mut result = false;

    let n = vertices.len();

    // Loop through every pair of vertices
    for i in 0..n {
        let j = (i + 1) % n;

        let a = vertices[i];
        let b = vertices[j];

        // Is  a above the query point?
        let first_is_above = a.1 > query.1;

        // Is b below the query point?
        let second_is_below = b.1 <= query.1;

        // If both are above or both are below, there can be no intersection.
        // So we need to check whether they are on opposite sides of the query
        let opposite_sides = first_is_above != second_is_below;

        if !opposite_sides {
            continue;
        }

        // This calculates the x-coordinate of the intersection of the line
        // between a and b with the horizontal line through the query point.
        let horizontal_intersection_point = {
            let horizontal_distance_ab = b.0 - a.0;
            let vertical_distance_ab = b.1 - a.1;

            if vertical_distance_ab == T::zero() {
                a.0
            } else {
                horizontal_distance_ab * (query.1 - a.1) / (vertical_distance_ab) + a.0
            }
        };

        // If the intersection point is to the right of the query point, we
        // count it as an intersection.
        if query.0 < horizontal_intersection_point {
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
        let vertices = vec![(-10, -10), (-10, 10), (10, 10), (10, -10)];
        assert_eq!(point_in_polygon(&vertices, query), expected);
    }
}
