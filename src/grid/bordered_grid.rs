use std::ops::{Deref, DerefMut, Index};

use super::{Coordinate, Grid2D};

/// A bordered grid is a wrapper around Grid2D that has an implicit border
/// consisting of a default value. The original retains its coordinates but the
/// border is added around it.
///
/// For example, a 3x3 grid with a border of 1 (of value 0) would look like this:
///
/// >  A 0 0 0 0
/// >  0 1 1 1 0
/// >  0 1 1 1 0
/// >  0 1 1 1 0
/// >  0 0 0 0 B
///
/// The element marked A is at (-1, -1) and the B is at (3, 3). Both A and B
/// have, of course, the border-element value, 0.
#[derive(PartialEq, Eq, Hash)]
pub struct BorderedGrid2D<'a, T: Clone> {
    grid: &'a mut Grid2D<T>,
    border_size: i32,
    default: T,
}

impl<'a, T: Clone> BorderedGrid2D<'a, T> {
    /// Create a new bordered grid
    #[must_use]
    pub fn new(border_size: usize, border_element: T, grid: &'a mut Grid2D<T>) -> Self {
        Self {
            grid,
            default: border_element,
            border_size: border_size as i32,
        }
    }

    /// Returns the width of the grid, including the border
    #[must_use]
    pub fn width(&self) -> usize {
        self.grid.width() + self.border_size as usize * 2
    }

    /// Returns the height of the grid, including the border
    #[must_use]
    pub fn height(&self) -> usize {
        self.grid.height() + self.border_size as usize * 2
    }

    /// Returns the size of the grid, including the border
    #[must_use]
    pub fn area(&self) -> usize {
        self.width() * self.height()
    }

    #[must_use]
    pub fn contains(&self, coord: Coordinate) -> bool {
        let x = coord.x;
        let y = coord.y;
        let w = self.grid.width() as i32;
        let h = self.grid.height() as i32;

        x >= -self.border_size
            && x < w + self.border_size
            && y >= -self.border_size
            && y < h + self.border_size
    }

    /// Accesses the element at the given coordinate
    #[must_use]
    pub fn get(&self, coord: Coordinate) -> Option<&T> {
        let x = coord.x;
        let y = coord.y;
        let w = self.grid.width() as i32;
        let h = self.grid.height() as i32;

        if x < -self.border_size
            || x >= w + self.border_size
            || y < -self.border_size
            || y >= h + self.border_size
        {
            return None;
        }

        // Top/Left border
        if x >= -self.border_size && x < 0 || y >= -self.border_size && y < 0 {
            return Some(&self.default);
        }

        // Bottom/Right border
        if x >= w && x < w + self.border_size || y >= h && y < h + self.border_size {
            return Some(&self.default);
        }

        self.grid.get(coord)
    }

    /// Sets the element at the given coordinate
    pub fn set(&mut self, coord: Coordinate, value: T) -> Option<T> {
        let x = coord.x + 1 - self.border_size;
        let y = coord.y + 1 - self.border_size;
        let c = Coordinate::new(x, y);

        self.grid.set(c, value)
    }

    /// Returns an iterator over all elements in the grid, including the border
    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        ((-self.border_size)..(self.height() as i32 - self.border_size)).flat_map(move |y| {
            ((-self.border_size)..(self.width() as i32 - self.border_size)).map(move |x| {
                let c = Coordinate::new(x, y);

                (c, self.get(c).unwrap())
            })
        })
    }

    /// Returns an iterator over all elements in the grid (including the border)
    /// in row-major order
    #[must_use]
    pub fn row_iter(&self) -> impl Iterator<Item = &T> {
        (0..self.height()).flat_map(move |y| {
            (0..self.width()).map(move |x| {
                let x = x as i32;
                let y = y as i32;
                let c = Coordinate::new(x - self.border_size, y - self.border_size);

                self.get(c).unwrap()
            })
        })
    }

    /// Returns an iterator over all elements in the grid (including the border)
    /// in column-major order
    #[must_use]
    pub fn col_iter(&self) -> impl Iterator<Item = &T> {
        (0..self.width()).flat_map(move |x| {
            (0..self.height()).map(move |y| {
                let x = x as i32;
                let y = y as i32;
                let c = Coordinate::new(x - self.border_size, y - self.border_size);

                self.get(c).unwrap()
            })
        })
    }
}

impl<'a, T: Clone> Deref for BorderedGrid2D<'a, T> {
    type Target = Grid2D<T>;

    fn deref(&self) -> &Self::Target {
        self.grid
    }
}

impl<'a, T: Clone> DerefMut for BorderedGrid2D<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.grid
    }
}

impl<'a, T: Clone> Index<Coordinate> for BorderedGrid2D<'a, T> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        self.get(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut grid = Grid2D::new(3, 3, 1);
        let bordered = BorderedGrid2D::new(1, 0, &mut grid);

        assert_eq!(bordered.width(), 5);
        assert_eq!(bordered.height(), 5);

        assert_eq!(bordered.get(Coordinate::new(-2, -2)), None);
        assert_eq!(bordered.get(Coordinate::new(-1, -1)), Some(&0));
        assert_eq!(bordered.get(Coordinate::new(0, 0)), Some(&1));
        assert_eq!(bordered.get(Coordinate::new(1, 1)), Some(&1));
        assert_eq!(bordered.get(Coordinate::new(2, 2)), Some(&1));
        assert_eq!(bordered.get(Coordinate::new(3, 3)), Some(&0));
        assert_eq!(bordered.get(Coordinate::new(4, 4)), None);
    }

    #[test]
    fn test_set() {
        let mut grid = Grid2D::new(3, 3, 1);
        let mut bordered = BorderedGrid2D::new(1, 0, &mut grid);

        bordered.set(Coordinate::new(-1, -1), 2);
        bordered.set(Coordinate::new(0, 0), 3);
        bordered.set(Coordinate::new(1, 1), 4);
        bordered.set(Coordinate::new(2, 2), 5);
        bordered.set(Coordinate::new(3, 3), 6);

        assert_eq!(bordered.get(Coordinate::new(-1, -1)), Some(&0));
        assert_eq!(bordered.get(Coordinate::new(0, 0)), Some(&3));
        assert_eq!(bordered.get(Coordinate::new(1, 1)), Some(&4));
        assert_eq!(bordered.get(Coordinate::new(2, 2)), Some(&5));
        assert_eq!(bordered.get(Coordinate::new(3, 3)), Some(&0));
    }

    #[test]
    fn test_iter() {
        let mut grid = Grid2D::new(3, 3, 1);
        let bordered = BorderedGrid2D::new(1, 0, &mut grid);

        let mut iter = bordered.iter();

        assert_eq!(iter.next(), Some((Coordinate::new(0 - 1, 0 - 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(1 - 1, 0 - 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(2 - 1, 0 - 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(3 - 1, 0 - 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(4 - 1, 0 - 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(0 - 1, 1 - 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(1 - 1, 1 - 1), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(2 - 1, 1 - 1), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(3 - 1, 1 - 1), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(4 - 1, 1 - 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(0 - 1, 2 - 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(1 - 1, 2 - 1), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(2 - 1, 2 - 1), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(3 - 1, 2 - 1), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(4 - 1, 2 - 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(0 - 1, 3 - 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(1 - 1, 3 - 1), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(2 - 1, 3 - 1), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(3 - 1, 3 - 1), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(4 - 1, 3 - 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(0 - 1, 4 - 1), &0)));
    }

    #[test]
    fn test_row_iter() {
        let mut grid = Grid2D::new(1, 1, 1);
        let bordered = BorderedGrid2D::new(2, 0, &mut grid);
        let mut iter = bordered.row_iter();

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_col_iter() {
        let mut grid = Grid2D::new(2, 2, 1);
        let bordered = BorderedGrid2D::new(1, 0, &mut grid);
        let mut iter = bordered.col_iter();

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&0));

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&0));

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));

        assert_eq!(iter.next(), None);
    }
}
