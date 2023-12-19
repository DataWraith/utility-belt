use num::{Num, Signed};

/// Calculates the area of a simple polygon given its vertices.
///
/// See https://en.wikipedia.org/wiki/Shoelace_formula for more information.
///
/// NOTE: If you use this on a discrete grid, you may need to add one plus half
/// the perimeter of the polygon to the result. Imagine your grid as being made
/// up of small squares, with the vertices (marked '@') being anchored to the
/// center of the squares.
///
/// +-----+-----+-----+
/// |ooooo|ooooo|ooooo|
/// |oo@--+-----+--@oo|
/// |oo|**|*****|**|oo|
/// +--+--+-----+--+--+
/// |oo|**|*****|**|oo|
/// |oo|**|*****|**|oo|
/// |oo|**|*****|**|oo|
/// +--+--+-----+--+--+
/// |oo|**|*****|**|oo|
/// |oo@--+-----+--@oo|
/// |ooooo|ooooo|ooooo|
/// +-----+-----+-----+
///
/// You'd think a 3x3 rectangle (marked '*') would have an area of 9, but
/// because the vertices are anchored to the center of the squares, the
/// actual area is (1 + 4 * 0.5 + 4 * 0.25) = 4.
///
/// To get an area of 9, you add  one plus half the perimeter of the polygon:
/// (8 / 2) = 4. Then 4 + 4 + 1 = 9. Note that perimeter refers to the perimeter
/// of the smaller box inside, not the perimeter of the larger box outside.
///
/// The other solution to this problem involves moving the vertices to the
/// corners of the squares, but that's a lot more work.
///
// TODO: Make this generic using Num
pub fn polygon_area<T: Num + Copy + Signed>(vertices: &[(T, T)]) -> T {
    let mut area = T::zero();

    let num_vertices = vertices.len();

    // Allow the polygon to be open or closed
    let last_index = if vertices[0] == vertices[vertices.len() - 1] {
        vertices.len() - 1
    } else {
        vertices.len()
    };

    for i in 0..(last_index) {
        area = area + vertices[i].0 * vertices[(i + 1) % num_vertices].1;
        area = area - vertices[(i + 1) % num_vertices].0 * vertices[i].1;
    }

    area.abs() / (T::one() + T::one())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case([(0, 0), (0, 1), (1, 1), (1, 0)], 1)]
    #[case([(0, 0), (0, 2), (2, 2), (2, 0)], 4)]
    #[case([(0, 0), (0, 6), (6, 6), (6, 0)], 36)]
    #[case([(0, 0), (5, 0), (5, 5), (0, 0)], 5 * 5 / 2)]
    fn polygon_area_works(#[case] vertices: [(isize, isize); 4], #[case] expected: isize) {
        assert_eq!(polygon_area(&vertices), expected);
    }

    #[test]
    fn polygon_area_works_for_triangle() {
        let vertices = [(0, 0), (0, 100), (100, 0), (0, 0)];
        assert_eq!(polygon_area(&vertices), 100 * 100 / 2);
    }

    #[test]
    fn polygon_area_wikipedia_example() {
        let vertices = [(1.0, 6.0), (3.0, 1.0), (7.0, 2.0), (4.0, 4.0), (8.0, 5.0)];
        assert_eq!(polygon_area(&vertices), 16.5);
    }
}
