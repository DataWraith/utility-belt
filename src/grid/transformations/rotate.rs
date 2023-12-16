use crate::grid::{Coordinate, Grid2D};

impl<T: Clone> Grid2D<T> {
    /// Rotates the grid 90 degrees clockwise and returns it as a new grid.
    pub fn rotate_right(&self) -> Self {
        let mut new = self.clone();
        for (coord, value) in self.indexed_iter() {
            new.set(
                Coordinate::new(self.height() - 1 - coord.y(), coord.x()),
                value.clone(),
            )
        }

        new
    }

    /// Rotates the grid 90 degrees counter-clockwise and returns it as a new grid.
    pub fn rotate_left(&self) -> Self {
        let mut new = self.clone();

        for (coord, value) in self.indexed_iter() {
            new.set(
                Coordinate::new(coord.y(), self.width() - 1 - coord.x()),
                value.clone(),
            );
        }

        new
    }

    /// Rotates the grid 180 degrees and returns it as a new grid.
    pub fn rotate_180(&self) -> Self {
        let mut new = self.clone();

        for (coord, value) in self.indexed_iter() {
            new.set(
                Coordinate::new(self.width() - 1 - coord.x(), self.height() - 1 - coord.y()),
                value.clone(),
            );
        }

        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_right() {
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
                7, 4, 1, //
                8, 5, 2, //
                9, 6, 3, //
            ],
        );

        let rotated = grid.rotate_right();
        assert_eq!(rotated, expected);
    }

    #[test]
    fn test_rotate_left() {
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
                3, 6, 9, //
                2, 5, 8, //
                1, 4, 7, //
            ],
        );

        let rotated = grid.rotate_left();
        assert_eq!(rotated, expected);
    }

    #[test]
    fn test_rotate_180() {
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
                9, 8, 7, //
                6, 5, 4, //
                3, 2, 1, //
            ],
        );

        let rotated = grid.rotate_180();
        assert_eq!(rotated, expected);
    }
}
