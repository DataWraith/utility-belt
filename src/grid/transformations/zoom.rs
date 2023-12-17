use crate::{grid::Grid2D, prelude::Coordinate};

impl<T: Clone> Grid2D<T> {
    pub fn zoom(&self, factor: u8) -> Self {
        assert!(factor > 0, "Zoom factor must be greater than 0");

        let new_w = self.width as usize * factor as usize;
        let new_h = self.height as usize * factor as usize;

        let mut new_grid = Grid2D::new(new_w, new_h, self[Coordinate::new(0, 0)].clone());

        for (coord, value) in self.indexed_iter() {
            for x in 0..factor {
                for y in 0..factor {
                    new_grid.set(
                        Coordinate::new(
                            coord.x() * factor as i32 + x as i32,
                            coord.y() * factor as i32 + y as i32,
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
    fn test_zoom_2x() {
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
            4,
            vec![
                1, 1, 2, 2, //
                1, 1, 2, 2, //
                3, 3, 4, 4, //
                3, 3, 4, 4, //
            ],
        );

        let mirrored = grid.zoom(2);
        assert_eq!(mirrored, expected);
    }

    #[test]
    fn test_zoom_3x() {
        let grid = Grid2D::from_shape_vec(
            2,
            2,
            vec![
                1, 2, //
                3, 4, //
            ],
        );

        let expected = Grid2D::from_shape_vec(
            6,
            6,
            vec![
                1, 1, 1, 2, 2, 2, //
                1, 1, 1, 2, 2, 2, //
                1, 1, 1, 2, 2, 2, //
                3, 3, 3, 4, 4, 4, //
                3, 3, 3, 4, 4, 4, //
                3, 3, 3, 4, 4, 4, //
            ],
        );

        let mirrored = grid.zoom(3);
        assert_eq!(mirrored, expected);
    }
}
