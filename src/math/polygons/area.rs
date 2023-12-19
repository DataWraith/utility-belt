/// Calculates the area of a simple polygon given its vertices.
///
/// See https://en.wikipedia.org/wiki/Shoelace_formula for more information.
pub fn polygon_area(vertices: &[(isize, isize)]) -> usize {
    let mut area = 0isize;

    let num_vertices = vertices.len();

    let last_index = if vertices[0] == vertices[vertices.len() - 1] {
        vertices.len() - 1
    } else {
        vertices.len()
    };

    for i in 0..(last_index) {
        area += vertices[i].0 * vertices[(i + 1) % num_vertices].1;
        area -= vertices[(i + 1) % num_vertices].0 * vertices[i].1;
    }

    (area.abs() / 2) as usize
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
    fn polygon_area_works(#[case] vertices: [(isize, isize); 4], #[case] expected: usize) {
        assert_eq!(polygon_area(&vertices), expected);
    }

    #[test]
    fn polygon_area_works_for_triangle() {
        let vertices = [(0, 0), (0, 100), (100, 0), (0, 0)];
        assert_eq!(polygon_area(&vertices), 100 * 100 / 2);
    }

    #[test]
    fn polygon_area_wikipedia_example() {
        let vertices = [(1, 6), (3, 1), (7, 2), (4, 4), (8, 5)];
        assert_eq!(polygon_area(&vertices), 16); // Technically 16.5
    }
}
