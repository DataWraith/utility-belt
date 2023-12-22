use crate::grid::Grid2D;

impl<T: Clone> Grid2D<T> {
    /// Mirrors the grid along the x-axis and returns it as a new grid.
    pub fn mirror_x(&self) -> Self {
        let mut new = self.clone();

        for (coord, value) in self.iter() {
            new.set(coord.mirror_x_wrap(self.height), value.clone());
        }

        new
    }

    /// Mirrors the grid along the y-axis and returns it as a new grid.
    pub fn mirror_y(&self) -> Self {
        let mut new = self.clone();

        for (coord, value) in self.iter() {
            new.set(coord.mirror_y_wrap(self.height), value.clone());
        }

        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_x() {
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
                3, 2, 1, //
                6, 5, 4, //
                9, 8, 7, //
            ],
        );

        let mirrored = grid.mirror_x();
        assert_eq!(mirrored, expected);
    }

    #[test]
    fn test_mirror_y() {
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
                7, 8, 9, //
                4, 5, 6, //
                1, 2, 3, //
            ],
        );

        let mirrored = grid.mirror_y();
        assert_eq!(mirrored, expected);
    }
}
