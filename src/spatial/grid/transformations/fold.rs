use crate::prelude::*;

impl<T: Clone> Grid2D<T> {
    /// Fold left along a given column.
    ///
    /// The given closure determines what happens to overlapping elements and
    /// elements that are not overlapping are deleted.
    pub fn fold_left_along_column(&self, column: usize, f: impl Fn(&T, &T) -> T) -> Self {
        assert!(column < self.width as usize, "Column index out of bounds");

        let w = (self.width as usize - column).min(column + 1);
        let mut result = Grid2D::new(w, self.height(), self.data[[0, 0]].clone());

        if column + 1 < self.width as usize {
            for x in 0..column {
                for y in 0..self.height() {
                    let c = Coordinate::new(x as i32, y as i32);
                    let mirror = Coordinate::new(column as i32 * 2 - x as i32, y as i32);
                    result.set(c, f(&self[c], &self[mirror]));
                }
            }
        }

        for y in 0..self.height() {
            let mirror = Coordinate::new(column as i32, y as i32);
            let c = Coordinate::new((column as i32).min(w as i32 - 1), y as i32);
            result.set(c, self[mirror].clone());
        }

        result
    }

    /// Fold upwards along a given row. The given closure determines what happens to overlapping elements
    /// and elements that are not overlapping are deleted.
    pub fn fold_up_along_row(&self, row: usize, f: impl Fn(&T, &T) -> T) -> Self {
        assert!(row < self.height as usize, "Row index out of bounds");

        let h = (self.height as usize - row).min(row + 1);
        let mut result = Grid2D::new(self.width(), h, self.data[[0, 0]].clone());

        if row + 1 < self.height as usize {
            for y in 0..row {
                for x in 0..self.width() {
                    let c = Coordinate::new(x as i32, y as i32);
                    let mirror = Coordinate::new(x as i32, row as i32 * 2 - y as i32);
                    result.set(c, f(&self[c], &self[mirror]));
                }
            }
        }

        for x in 0..self.width() {
            let mirror = Coordinate::new(x as i32, row as i32);
            let c = Coordinate::new(x as i32, (row as i32).min(h as i32 - 1));
            result.set(c, self[mirror].clone());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_along_column0() {
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
            1,
            3,
            vec![
                1, //
                4, //
                7, //
            ],
        );

        let folded = grid.fold_left_along_column(0, |a, b| a + b);

        assert_eq!(folded, expected);
    }

    // TODO: Test non-odd grids

    #[test]
    fn test_fold_along_column1() {
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
            2,
            3,
            vec![
                4, 2, //
                10, 5, //
                16, 8, //
            ],
        );

        let folded = grid.fold_left_along_column(1, |a, b| a + b);

        assert_eq!(folded, expected);
    }

    #[test]
    fn test_fold_along_column2() {
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
            1,
            3,
            vec![
                3, //
                6, //
                9, //
            ],
        );

        let folded = grid.fold_left_along_column(2, |a, b| a + b);

        assert_eq!(folded, expected);
    }

    #[test]
    fn test_fold_up_along_row0() {
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

        let folded = grid.fold_up_along_row(0, |a, b| a + b);

        assert_eq!(folded, expected);
    }

    #[test]
    fn test_fold_up_along_row1() {
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
            2,
            vec![
                8, 10, 12, //
                4, 5, 6, //
            ],
        );

        let folded = grid.fold_up_along_row(1, |a, b| a + b);

        assert_eq!(folded, expected);
    }

    #[test]
    fn test_fold_up_along_row2() {
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
                7, 8, 9, //
            ],
        );

        let folded = grid.fold_up_along_row(2, |a, b| a + b);

        assert_eq!(folded, expected);
    }
}
