use crate::grid::{Coordinate, Grid2D};

// TODO: Docstrings

impl<T: Clone> Grid2D<T> {
    // TODO: Fold along a row/column (closure determines what happens to overlapping elements)
    // TODO: Fold between two rows/columns (closure determines what happens to overlapping elements)

    /// "Unfolds" the grid by mirroring it along the x-axis and concatenating the two halves.
    pub fn unfold_x(&self) -> Self {
        self.concat_x(&self.mirror_x())
    }

    // TODO: Figure out a better name for this
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

    pub fn unfold_y(&self) -> Self {
        self.concat_y(&self.mirror_y())
    }

    // TODO: Figure out a better name for this
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
            3,
            vec![
                1, 2, 1, //
                4, 5, 4, //
                7, 8, 7, //
            ],
        );

        let unfolded = grid.unfold_at_x(1);
        assert_eq!(unfolded, expected);
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
    }
}
