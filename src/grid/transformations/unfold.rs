use crate::grid::{Coordinate, Grid2D};

impl<T: Clone> Grid2D<T> {
    /// "Unfolds" the grid by mirroring it along the x-axis and concatenating the two halves.
    pub fn unfold_x(&self) -> Self {
        let mut mirror = self.clone();
        mirror.mirror_x();

        self.concat_x(&mirror)
    }

    /// "Unfolds" the grid by mirroring it along a given column.
    ///
    /// The grid unfolds from left to right, and any columns that are covered by
    /// folded columns (including non-existant ones) are replaced (if
    /// overlapping) or removed (if source column is off the grid)
    pub fn unfold_at_x(&self, column: usize) -> Self {
        let mut result = Grid2D::new(column * 2 + 1, self.height(), self.data[[0, 0]].clone());

        for x in 0..=column {
            for y in 0..self.height() {
                let c = Coordinate::new(x as i32, y as i32);
                let mirror = Coordinate::new(column as i32 * 2 - x as i32, y as i32);
                result.set(c, self[c].clone());
                result.set(mirror, self[c].clone());
            }
        }

        result
    }

    /// "Unfolds" the grid by mirroring it along the y-axis and concatenating the two halves.
    pub fn unfold_y(&self) -> Self {
        let mut mirror = self.clone();
        mirror.mirror_y();
        self.concat_y(&mirror)
    }

    /// "Unfolds" the grid by mirroring it along a given row.
    ///
    /// The top half is folded down, and any rows that are covered by folded
    /// columns (including non-existant ones) are replaced (if overlapping) or
    /// removed (if the source row is off the grid )
    pub fn unfold_at_y(&self, row: usize) -> Self {
        let mut result = Grid2D::new(self.width(), row * 2 + 1, self.data[[0, 0]].clone());

        for x in 0..self.width() {
            for y in 0..=row {
                let x = x as i32;
                let y = y as i32;

                let c = Coordinate::new(x, y);
                let mirror = Coordinate::new(x, row as i32 * 2 - y);

                result.set(c, self[c].clone());
                result.set(mirror, self[c].clone());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unfold_x() {
        let grid = Grid2D::from_shape_vec(
            3,
            3,
            vec![
                1, 2, 3, //
                4, 5, 6, //
                7, 8, 9, //
            ],
        );

        let expected = Grid2D::from_shape_vec(
            6,
            3,
            vec![
                1, 2, 3, 3, 2, 1, //
                4, 5, 6, 6, 5, 4, //
                7, 8, 9, 9, 8, 7, //
            ],
        );

        let unfolded = grid.unfold_x();
        assert_eq!(unfolded, expected);
    }

    #[test]
    fn test_unfold_at_x() {
        let grid = Grid2D::from_shape_vec(
            3,
            3,
            vec![
                1, 2, 3, //
                4, 5, 6, //
                7, 8, 9, //
            ],
        );

        let expected = Grid2D::from_shape_vec(
            5,
            3,
            vec![
                1, 2, 3, 2, 1, //
                4, 5, 6, 5, 4, //
                7, 8, 9, 8, 7, //
            ],
        );

        let unfolded = grid.unfold_at_x(2);
        assert_eq!(unfolded, expected);

        let expected = Grid2D::from_shape_vec(
            3,
            3,
            vec![
                1, 2, 1, //
                4, 5, 4, //
                7, 8, 7, //
            ],
        );

        let unfolded = grid.unfold_at_x(1);
        assert_eq!(unfolded, expected);

        let grid = Grid2D::from_shape_vec(
            4,
            2,
            vec![
                1, 2, 3, 4, //
                5, 6, 7, 8, //
            ],
        );

        let expected = Grid2D::from_shape_vec(
            3,
            2,
            vec![
                1, 2, 1, //
                5, 6, 5, //
            ],
        );

        let expected2 = Grid2D::from_shape_vec(
            5,
            2,
            vec![
                1, 2, 3, 2, 1, //
                5, 6, 7, 6, 5, //
            ],
        );

        let unfolded = grid.unfold_at_x(1);
        assert_eq!(unfolded, expected);

        let unfolded = grid.unfold_at_x(2);
        assert_eq!(unfolded, expected2);
    }

    #[test]
    fn test_unfold_y() {
        let grid = Grid2D::from_shape_vec(
            3,
            3,
            vec![
                1, 2, 3, //
                4, 5, 6, //
                7, 8, 9, //
            ],
        );

        let expected = Grid2D::from_shape_vec(
            3,
            6,
            vec![
                1, 2, 3, //
                4, 5, 6, //
                7, 8, 9, //
                7, 8, 9, //
                4, 5, 6, //
                1, 2, 3, //
            ],
        );

        let unfolded = grid.unfold_y();
        assert_eq!(unfolded, expected);
    }

    #[test]
    fn test_unfold_y_at() {
        let grid = Grid2D::from_shape_vec(
            3,
            3,
            vec![
                1, 2, 3, //
                4, 5, 6, //
                7, 8, 9, //
            ],
        );

        let expected = Grid2D::from_shape_vec(
            3,
            1,
            vec![
                1, 2, 3, //
            ],
        );

        let unfolded = grid.unfold_at_y(0);
        assert_eq!(unfolded, expected);

        let grid = Grid2D::from_shape_vec(
            2,
            4,
            vec![
                1, 5, //
                2, 6, //
                3, 7, //
                4, 8, //
            ],
        );

        let expected = Grid2D::from_shape_vec(
            2,
            3,
            vec![
                1, 5, //
                2, 6, //
                1, 5,
            ],
        );

        let unfolded = grid.unfold_at_y(1);
        assert_eq!(unfolded, expected);

        let expected2 = Grid2D::from_shape_vec(
            2,
            5,
            vec![
                1, 5, //
                2, 6, //
                3, 7, //
                2, 6, //
                1, 5, //
            ],
        );

        let unfolded = grid.unfold_at_y(2);
        assert_eq!(unfolded, expected2);
    }
}
