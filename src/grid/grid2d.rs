use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

use ndarray::{concatenate, Array2, ArrayView1, Axis};

use super::Coordinate;

/// A 2D grid backed by ndarray.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid2D<T>
where
    T: Clone,
{
    pub width: i32,
    pub height: i32,
    pub data: Array2<T>,
}

impl<T: Clone + From<char>> Grid2D<T> {
    /// Parses a grid from a string slice.
    ///
    /// This assumes that the string containing the grid is a well-formed n-by-m
    /// grid, where each row is separated by a newline character. Fortunately
    /// this is the case for all of the Advent of Code puzzles.
    ///
    /// The grid is indexed from the top-left corner, with the x-axis increasing
    /// to the right and the y-axis increasing downwards.
    ///
    /// The type T must implement `From<char>`, which is used to convert each
    /// character in the string to a value in the grid.
    ///
    pub fn parse(input: &str) -> Self {
        let mut width = 0;
        let mut cur_width = 0;
        let mut height = 0;
        let mut elems = Vec::new();

        for c in input.chars() {
            if c == '\n' {
                height += 1;
                cur_width = 0;
            } else {
                cur_width += 1;
                width = width.max(cur_width);
                elems.push(c.into());
            }
        }

        assert!(width > 0, "Grid width must be greater than 0");
        assert!(height > 0, "Grid height must be greater than 0");

        let data = Array2::from_shape_vec((height, width), elems).unwrap();

        Self {
            width: width as i32,
            height: height as i32,
            data,
        }
    }
}

impl<T: Clone> Grid2D<T> {
    // Creates a new grid of the given size, with all elements initialized to
    // the given value.
    pub fn new(width: usize, height: usize, default: T) -> Self {
        assert!(width > 0, "Grid width must be greater than 0");
        assert!(height > 0, "Grid height must be greater than 0");

        Self {
            width: width as i32,
            height: height as i32,
            data: Array2::from_elem((height, width), default),
        }
    }

    // Reshapes the given Vec<T> into a grid of the given size.
    //
    // Panics if the given width or height is 0 or if the given data does not
    // have the same number of elements as a grid of the given width and height.
    pub fn from_shape_vec(width: usize, height: usize, data: Vec<T>) -> Self {
        assert!(width > 0, "Grid width must be greater than 0");
        assert!(height > 0, "Grid height must be greater than 0");

        Self {
            width: width as i32,
            height: height as i32,
            data: Array2::from_shape_vec((height, width), data).unwrap(),
        }
    }

    /// Returns the width of the grid.
    pub fn width(&self) -> usize {
        self.width as usize
    }

    /// Returns the height of the grid
    pub fn height(&self) -> usize {
        self.height as usize
    }

    /// Returns the value at the given coordinate. Out-of-bounds accesses return
    /// `None`.
    pub fn get(&self, coord: Coordinate) -> Option<&T> {
        if coord.x() < 0 || coord.y() < 0 {
            return None;
        }

        self.data.get((coord.y() as usize, coord.x() as usize))
    }

    /// Returns the value at the given coordinate. Out-of-bound accesses wrap
    /// around back into the grid.
    pub fn get_wrap(&self, coord: Coordinate) -> &T {
        let x = coord.x().rem_euclid(self.width);
        let y = coord.y().rem_euclid(self.height);

        self.data.get((y as usize, x as usize)).unwrap()
    }

    /// Returns a mutable reference to the value at the given coordinate,
    /// or `None` if the coordinate is out-of-bounds.
    pub fn get_mut(&mut self, coord: Coordinate) -> Option<&mut T> {
        if coord.x() < 0 || coord.y() < 0 {
            return None;
        }

        self.data.get_mut((coord.y() as usize, coord.x() as usize))
    }

    /// Wraps the coordinate around the grid and returns a mutable reference to
    /// the value at the given coordinate.
    pub fn get_wrap_mut(&mut self, coord: Coordinate) -> &mut T {
        let x = coord.x().rem_euclid(self.width);
        let y = coord.y().rem_euclid(self.height);

        self.data.get_mut((y as usize, x as usize)).unwrap()
    }

    /// Sets the value at the given coordinate. Out-of-bounds accesses are
    /// ignored.
    pub fn set(&mut self, coord: Coordinate, value: T) {
        if coord.x() < 0 || coord.y() < 0 {
            return;
        }

        if coord.x() >= self.width || coord.y() >= self.height {
            return;
        }

        self.data[(coord.y() as usize, coord.x() as usize)] = value;
    }

    pub fn map<T2: Clone>(&self, f: impl Fn(&T) -> T2) -> Grid2D<T2> {
        Grid2D {
            width: self.width,
            height: self.height,
            data: self.data.map(f),
        }
    }

    /// Returns an iterator over the grid's elements and their coordinates.
    pub fn iter(&self) -> impl Iterator<Item = (Coordinate, &T)> + '_ {
        self.data
            .indexed_iter()
            .map(|((y, x), value)| (Coordinate::new(x as i32, y as i32), value))
    }

    /// Returns an iterator over the grid's rows
    pub fn row_iter(&self) -> impl Iterator<Item = ArrayView1<T>> + '_ {
        self.data.axis_iter(ndarray::Axis(0))
    }

    /// Returns an iterator over the grid's columns
    pub fn col_iter(&self) -> impl Iterator<Item = ArrayView1<T>> + '_ {
        self.data.axis_iter(ndarray::Axis(1))
    }

    /// Returns a the result of concatening `other` to the right of `self`.
    pub fn concat_x(&self, other: &Self) -> Self {
        let combined = concatenate![Axis(1), self.data.view(), other.data.view()];

        Grid2D {
            width: self.width + other.width,
            height: self.height,
            data: combined,
        }
    }

    /// Returns a the result of concatening `other` below `self`.
    pub fn concat_y(&self, other: &Self) -> Self {
        let combined = concatenate![Axis(0), self.data.view(), other.data.view()];

        Grid2D {
            width: self.width,
            height: self.height + other.height,
            data: combined,
        }
    }

    /// Transpose the grid
    pub fn transpose(&mut self) {
        std::mem::swap(&mut self.width, &mut self.height);
        self.data.swap_axes(0, 1);
    }
}

impl<T: Clone> Index<Coordinate> for Grid2D<T> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T: Clone> IndexMut<Coordinate> for Grid2D<T> {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl From<&str> for Grid2D<char> {
    fn from(input: &str) -> Self {
        Self::parse(input)
    }
}

impl<T: Clone> From<Vec<Vec<T>>> for Grid2D<T> {
    fn from(input: Vec<Vec<T>>) -> Self {
        let height = input.len();
        let width = input[0].len();
        let data =
            Array2::from_shape_vec((height, width), input.into_iter().flatten().collect()).unwrap();

        Self {
            width: width as i32,
            height: height as i32,
            data,
        }
    }
}

impl<T: Display + Clone> Display for Grid2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(Coordinate::new(x, y)).unwrap())?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T: Debug + Clone> Debug for Grid2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "[{:?}]", self.get(Coordinate::new(x, y)).unwrap())?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn get_test() {
        let grid: Grid2D<i32> = Grid2D::from_shape_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(grid.get(Coordinate::new(-1, 0)), None);
        assert_eq!(grid.get(Coordinate::new(0, 0)), Some(&1));
        assert_eq!(grid.get(Coordinate::new(3, 0)), None);

        assert_eq!(grid.get(Coordinate::new(0, -1)), None);
        assert_eq!(grid.get(Coordinate::new(0, 0)), Some(&1));
        assert_eq!(grid.get(Coordinate::new(0, 3)), None);
    }

    #[test]
    fn get_wrap_test() {
        let grid: Grid2D<i32> = Grid2D::from_shape_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(grid.get_wrap(Coordinate::new(0, 0)), &1);
        assert_eq!(grid.get_wrap(Coordinate::new(1, 0)), &2);
        assert_eq!(grid.get_wrap(Coordinate::new(2, 0)), &3);
        assert_eq!(grid.get_wrap(Coordinate::new(3, 0)), &1);

        assert_eq!(grid.get_wrap(Coordinate::new(0, 0)), &1);
        assert_eq!(grid.get_wrap(Coordinate::new(0, 1)), &4);
        assert_eq!(grid.get_wrap(Coordinate::new(0, 2)), &7);
        assert_eq!(grid.get_wrap(Coordinate::new(0, 3)), &1);
    }

    #[test]
    fn get_wrap_mut_test() {
        let mut grid: Grid2D<i32> = Grid2D::from_shape_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(grid.get_wrap_mut(Coordinate::new(0, 0)), &mut 1);
        assert_eq!(grid.get_wrap_mut(Coordinate::new(1, 0)), &mut 2);
        assert_eq!(grid.get_wrap_mut(Coordinate::new(2, 0)), &mut 3);
        assert_eq!(grid.get_wrap_mut(Coordinate::new(3, 0)), &mut 1);

        assert_eq!(grid.get_wrap_mut(Coordinate::new(0, 0)), &mut 1);
        assert_eq!(grid.get_wrap_mut(Coordinate::new(0, 1)), &mut 4);
        assert_eq!(grid.get_wrap_mut(Coordinate::new(0, 2)), &mut 7);
        assert_eq!(grid.get_wrap_mut(Coordinate::new(0, 3)), &mut 1);
    }

    #[test]
    fn test_transpose() {
        let input = indoc! {"
            12
            34
            56
        "};

        let input_transposed = indoc! {"
            135
            246
        "};

        let mut grid: Grid2D<char> = Grid2D::parse(input);
        let grid_t: Grid2D<char> = Grid2D::parse(input_transposed);

        grid.transpose();

        assert_eq!(grid, grid_t);
    }

    #[test]
    fn test_transpose_inverts_itself() {
        let input = indoc! {"
            123.
            456.
            789.
        "};

        let mut grid = Grid2D::<char>::parse(input);
        let grid2 = grid.clone();

        grid.transpose();
        grid.transpose();

        assert_eq!(grid, grid2);
    }

    #[test]
    fn parse_test() {
        let input = indoc! {"
            123
            456
            789
        "};

        let grid: Grid2D<char> = Grid2D::parse(input);

        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);

        assert_eq!(grid[Coordinate::new(0, 0)], '1');
        assert_eq!(grid[Coordinate::new(1, 0)], '2');
        assert_eq!(grid[Coordinate::new(2, 0)], '3');
        assert_eq!(grid[Coordinate::new(0, 1)], '4');
        assert_eq!(grid[Coordinate::new(1, 1)], '5');
        assert_eq!(grid[Coordinate::new(2, 1)], '6');
        assert_eq!(grid[Coordinate::new(0, 2)], '7');
        assert_eq!(grid[Coordinate::new(1, 2)], '8');
        assert_eq!(grid[Coordinate::new(2, 2)], '9');
    }

    #[test]
    fn new_from_default() {
        let grid: Grid2D<i32> = Grid2D::new(3, 3, 0);

        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);

        assert_eq!(grid[Coordinate::new(0, 0)], 0);
        assert_eq!(grid[Coordinate::new(1, 0)], 0);
        assert_eq!(grid[Coordinate::new(2, 0)], 0);
        assert_eq!(grid[Coordinate::new(0, 1)], 0);
        assert_eq!(grid[Coordinate::new(1, 1)], 0);
        assert_eq!(grid[Coordinate::new(2, 1)], 0);
        assert_eq!(grid[Coordinate::new(0, 2)], 0);
        assert_eq!(grid[Coordinate::new(1, 2)], 0);
        assert_eq!(grid[Coordinate::new(2, 2)], 0);
    }

    #[test]
    fn from_shape_vec_test() {
        let grid: Grid2D<i32> = Grid2D::from_shape_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);

        assert_eq!(grid[Coordinate::new(0, 0)], 1);
        assert_eq!(grid[Coordinate::new(1, 0)], 2);
        assert_eq!(grid[Coordinate::new(2, 0)], 3);
        assert_eq!(grid[Coordinate::new(0, 1)], 4);
        assert_eq!(grid[Coordinate::new(1, 1)], 5);
        assert_eq!(grid[Coordinate::new(2, 1)], 6);
        assert_eq!(grid[Coordinate::new(0, 2)], 7);
        assert_eq!(grid[Coordinate::new(1, 2)], 8);
        assert_eq!(grid[Coordinate::new(2, 2)], 9);
    }

    #[test]
    fn test_iter() {
        let grid: Grid2D<i32> = Grid2D::from_shape_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut iter = grid.iter();

        assert_eq!(iter.next(), Some((Coordinate::new(0, 0), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(1, 0), &2)));
        assert_eq!(iter.next(), Some((Coordinate::new(2, 0), &3)));
        assert_eq!(iter.next(), Some((Coordinate::new(0, 1), &4)));
        assert_eq!(iter.next(), Some((Coordinate::new(1, 1), &5)));
        assert_eq!(iter.next(), Some((Coordinate::new(2, 1), &6)));
        assert_eq!(iter.next(), Some((Coordinate::new(0, 2), &7)));
        assert_eq!(iter.next(), Some((Coordinate::new(1, 2), &8)));
        assert_eq!(iter.next(), Some((Coordinate::new(2, 2), &9)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_row_iter() {
        let grid: Grid2D<i32> = Grid2D::from_shape_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut iter = grid.row_iter();

        let row1 = iter.next().unwrap();
        let row2 = iter.next().unwrap();
        let row3 = iter.next().unwrap();

        assert_eq!(row1[0], 1);
        assert_eq!(row1[1], 2);
        assert_eq!(row1[2], 3);
        assert_eq!(row2[0], 4);
        assert_eq!(row2[1], 5);
        assert_eq!(row2[2], 6);
        assert_eq!(row3[0], 7);
        assert_eq!(row3[1], 8);
        assert_eq!(row3[2], 9);

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_col_iter() {
        let grid: Grid2D<i32> = Grid2D::from_shape_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut iter = grid.col_iter();

        let col1 = iter.next().unwrap();
        let col2 = iter.next().unwrap();
        let col3 = iter.next().unwrap();

        assert_eq!(col1[0], 1);
        assert_eq!(col1[1], 4);
        assert_eq!(col1[2], 7);

        assert_eq!(col2[0], 2);
        assert_eq!(col2[1], 5);
        assert_eq!(col2[2], 8);

        assert_eq!(col3[0], 3);
        assert_eq!(col3[1], 6);
        assert_eq!(col3[2], 9);

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_display() {
        let grid: Grid2D<i32> = Grid2D::from_shape_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(
            format!("{}", grid),
            indoc! {"

                123
                456
                789
            "}
        );
    }

    #[test]
    fn test_debug() {
        let grid: Grid2D<i32> = Grid2D::from_shape_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(
            format!("{:?}", grid),
            indoc! {"

                [1][2][3]
                [4][5][6]
                [7][8][9]
            "}
        );
    }
}
