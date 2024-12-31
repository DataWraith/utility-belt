use crate::prelude::{Coordinate, Grid2D};

impl<T: Clone> Grid2D<T> {
    /// 'Zooms' the grid by a factor of `factor` and returns it as a new grid.
    pub fn zoom(&self, factor: u8) -> Self {
        assert!(factor > 0, "Zoom factor must be greater than 0");

        let new_w = self.width as usize * factor as usize;
        let new_h = self.height as usize * factor as usize;

        let mut new_grid = Grid2D::new(new_w, new_h, self[Coordinate::new(0, 0)].clone());

        for (coord, value) in self.iter() {
            for x in 0..factor {
                for y in 0..factor {
                    new_grid.set(
                        Coordinate::new(
                            coord.x * factor as i32 + x as i32,
                            coord.y * factor as i32 + y as i32,
                        ),
                        value.clone(),
                    );
                }
            }
        }

        new_grid
    }

    /// Replaces every tile of the grid with a template that is provided by the
    /// `templater` closure.
    pub fn template_zoom<const X: usize, const Y: usize, TMPL, T2>(
        &self,
        mut templater: TMPL,
    ) -> Grid2D<T2>
    where
        TMPL: FnMut(&T) -> [[T2; X]; Y],
        T2: Clone,
    {
        if X == 0 || Y == 0 {
            panic!("Template size must be greater than 0");
        }

        let new_w = self.width as usize * X;
        let new_h = self.height as usize * Y;
        let default = templater(&self[Coordinate::new(0, 0)])[0][0].clone();

        let mut new_grid = Grid2D::new(new_w, new_h, default);

        for (coord, value) in self.iter() {
            let template = templater(value);

            for (y, row) in template.iter().enumerate() {
                for (x, col) in row.iter().enumerate() {
                    new_grid.set(
                        Coordinate::new(
                            coord.x * X as i32 + x as i32,
                            coord.y * Y as i32 + y as i32,
                        ),
                        col.clone(),
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

    #[test]
    fn test_template_zoom_2x2() {
        let grid = Grid2D::from_shape_vec(
            2,
            2,
            vec![
                1, 2, //
                3, 4, //
            ],
        );

        let expectd = Grid2D::from_shape_vec(
            4,
            4,
            vec![
                1, 2, 2, 3, //
                3, 4, 4, 0, //
                3, 4, 4, 0, //
                0, 1, 1, 2, //
            ],
        );

        let templater = |v: &i32| [[*v, (v + 1) % 5], [(v + 2) % 5, (v + 3) % 5]];
        let zoomed = grid.template_zoom(templater);
        assert_eq!(zoomed, expectd);
    }

    #[test]
    fn test_template_zoom_1x3() {
        let grid = Grid2D::from_shape_vec(
            2,
            2,
            vec![
                1, 2, //
                3, 4, //
            ],
        );

        let expected = Grid2D::from_shape_vec(
            2,
            6,
            vec![
                1, 2, //
                1, 2, //
                1, 2, //
                3, 4, //
                3, 4, //
                3, 4, //
            ],
        );

        let zoomed = grid.template_zoom(|&v| [[v; 1]; 3]);
        assert_eq!(zoomed, expected);
    }
}
