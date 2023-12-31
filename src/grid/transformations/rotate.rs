use crate::grid::Grid2D;

impl<T: Clone> Grid2D<T> {
    /// Rotates the grid 90 degrees clockwise and returns it as a new grid.
    pub fn rotate_right(&mut self) {
        self.transpose();
        self.mirror_x();
    }

    /// Rotates the grid 90 degrees counter-clockwise and returns it as a new grid.
    pub fn rotate_left(&mut self) {
        self.transpose();
        self.mirror_y();
    }

    /// Rotates the grid 180 degrees and returns it as a new grid.
    pub fn rotate_180(&mut self) {
        self.mirror_x();
        self.mirror_y();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_right() {
        let mut grid = Grid2D::from_shape_vec(
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
                7, 4, 1, //
                8, 5, 2, //
                9, 6, 3, //
            ],
        );

        grid.rotate_right();

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_rotate_left() {
        let mut grid = Grid2D::from_shape_vec(
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
                3, 6, 9, //
                2, 5, 8, //
                1, 4, 7, //
            ],
        );

        grid.rotate_left();

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_rotate_180() {
        let mut grid = Grid2D::from_shape_vec(
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
                9, 8, 7, //
                6, 5, 4, //
                3, 2, 1, //
            ],
        );

        grid.rotate_180();

        assert_eq!(grid, expected);
    }
}
