

use super::{Coordinate, Grid2D};

pub struct BorderedGrid2D<'a, T: Clone> {
    grid: &'a mut Grid2D<T>,
    border_size: i32,
    default: T,
}

impl<'a, T: Clone> BorderedGrid2D<'a, T> {
    pub fn new(border: usize, default: T, grid: &'a mut Grid2D<T>) -> Self {
        Self {
            grid,
            default,
            border_size: border as i32,
        }
    }

    pub fn width(&self) -> i32 {
        self.grid.width() + self.border_size * 2
    }

    pub fn height(&self) -> i32 {
        self.grid.height() + self.border_size * 2
    }

    pub fn get(&self, coord: Coordinate) -> Option<&T> {
        let x = coord.x();
        let y = coord.y();

        if x < -self.border_size
            || x >= self.grid.width() + self.border_size
            || y < -self.border_size
            || y >= self.grid.height() + self.border_size
        {
            return None;
        }

        // Top/Left border
        if x >= -self.border_size && x < 0 || y >= -self.border_size && y < 0 {
            return Some(&self.default);
        }

        // Bottom/Right border
        if x >= self.grid.width() && x < self.grid.width() + self.border_size
            || y >= self.grid.height() && y < self.grid.height() + self.border_size
        {
            return Some(&self.default);
        }

        self.grid.get(coord)
    }

    pub fn set(&mut self, coord: Coordinate, value: T) {
        let x = coord.x() + 1 - self.border_size;
        let y = coord.y() + 1 - self.border_size;
        let c = Coordinate::new(x, y);

        self.grid.set(c, value)
    }

    pub fn indexed_iter(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        (0..self.height()).flat_map(move |y| {
            (0..self.width()).map(move |x| {
                let c = Coordinate::new(x, y);
                let c2 = Coordinate::new(x - self.border_size, y - self.border_size);
                (c, self.get(c2).unwrap())
            })
        })
    }

    pub fn row_iter(&self) -> impl Iterator<Item = &T> {
        (0..self.height()).flat_map(move |y| {
            (0..self.width()).map(move |x| {
                let c = Coordinate::new(x - self.border_size, y - self.border_size);
                self.get(c).unwrap()
            })
        })
    }

    pub fn col_iter(&self) -> impl Iterator<Item = &T> {
        (0..self.width()).flat_map(move |x| {
            (0..self.height()).map(move |y| {
                let c = Coordinate::new(x - self.border_size, y - self.border_size);
                self.get(c).unwrap()
            })
        })
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
    fn test_indexed_iter() {
        let mut grid = Grid2D::new(3, 3, 1);
        let bordered = BorderedGrid2D::new(1, 0, &mut grid);

        let mut iter = bordered.indexed_iter();

        assert_eq!(iter.next(), Some((Coordinate::new(0, 0), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(1, 0), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(2, 0), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(3, 0), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(4, 0), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(0, 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(1, 1), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(2, 1), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(3, 1), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(4, 1), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(0, 2), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(1, 2), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(2, 2), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(3, 2), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(4, 2), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(0, 3), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(1, 3), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(2, 3), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(3, 3), &1)));
        assert_eq!(iter.next(), Some((Coordinate::new(4, 3), &0)));
        assert_eq!(iter.next(), Some((Coordinate::new(0, 4), &0)));
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
