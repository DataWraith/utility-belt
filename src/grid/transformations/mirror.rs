use ndarray::Axis;

use crate::grid::Grid2D;

impl<T: Clone> Grid2D<T> {
    /// Mirrors the grid along the x-axis and returns it as a new grid.
    pub fn mirror_x(&mut self) {
        self.data.invert_axis(Axis(1));
    }

    /// Mirrors the grid along the y-axis and returns it as a new grid.
    pub fn mirror_y(&mut self) {
        self.data.invert_axis(Axis(0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_x() {
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
                3, 2, 1, //
                6, 5, 4, //
                9, 8, 7, //
            ],
        );

        grid.mirror_x();

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_mirror_y() {
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
                7, 8, 9, //
                4, 5, 6, //
                1, 2, 3, //
            ],
        );

        grid.mirror_y();
        assert_eq!(grid, expected);
    }
}
