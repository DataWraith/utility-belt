use crate::{grid::Grid2D, prelude::Coordinate};

impl<T: Clone> Grid2D<T> {
    /// Replicates the grid `n` by `m` times and returns it as a new grid.
    pub fn replicate(&self, x_factor: u8, y_factor: u8) -> Self {
        assert!(x_factor > 0, "Replication factor must be greater than 0");
        assert!(y_factor > 0, "Replication factor must be greater than 0");

        let new_w = self.width as usize * x_factor as usize;
        let new_h = self.height as usize * y_factor as usize;

        let mut new_grid = Grid2D::new(new_w, new_h, self[Coordinate::new(0, 0)].clone());

        for (coord, value) in self.iter() {
            for x in 0..x_factor {
                for y in 0..y_factor {
                    new_grid.set(
                        Coordinate::new(
                            coord.x + x as i32 * self.width() as i32,
                            coord.y + y as i32 * self.height() as i32,
                        ),
                        value.clone(),
                    );
                }
            }
        }

        new_grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replicate_2x3() {
        let grid = Grid2D::from_shape_vec(
            2,
            2,
            vec![
                1, 2, //
                3, 4, //
            ],
        );

        let expected = Grid2D::from_shape_vec(
            4,
            6,
            vec![
                1, 2, 1, 2, //
                3, 4, 3, 4, //
                1, 2, 1, 2, //
                3, 4, 3, 4, //
                1, 2, 1, 2, //
                3, 4, 3, 4, //
            ],
        );

        let replicated = grid.replicate(2, 3);
        assert_eq!(replicated, expected);
    }
}
