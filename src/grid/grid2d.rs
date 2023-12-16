use std::ops::{Index, IndexMut};

use ndarray::Array2;

use super::{Coordinate, Direction};

pub struct Grid2D<T>
where
    T: Clone,
{
    pub width: i32,
    pub height: i32,
    pub data: Array2<T>,
}

impl<T: Clone + From<char>> Grid2D<T> {
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

        let data = Array2::from_shape_vec((height, width), elems).unwrap();

        Self {
            width: width as i32,
            height: height as i32,
            data,
        }
    }
}

impl<T: Clone> Grid2D<T> {
    pub fn new(width: i32, height: i32, default: T) -> Self {
        Self {
            width,
            height,
            data: Array2::from_elem((height as usize, width as usize), default),
        }
    }

    pub fn from_shape_vec(width: i32, height: i32, data: Vec<T>) -> Self {
        Self {
            width,
            height,
            data: Array2::from_shape_vec((height as usize, width as usize), data).unwrap(),
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn get(&self, coord: Coordinate) -> Option<&T> {
        if coord.x() < 0 || coord.y() < 0 {
            return None;
        }

        self.data.get((coord.y() as usize, coord.x() as usize))
    }

    pub fn get_wrap(&self, coord: Coordinate) -> Option<&T> {
        let x = coord.x() % self.width;
        let y = coord.y() % self.height;

        self.data.get((y as usize, x as usize))
    }

    pub fn get_mut(&mut self, coord: Coordinate) -> Option<&mut T> {
        if coord.x() < 0 || coord.y() < 0 {
            return None;
        }

        self.data.get_mut((coord.y() as usize, coord.x() as usize))
    }

    pub fn get_wrap_mut(&mut self, coord: Coordinate) -> Option<&mut T> {
        let x = coord.x() % self.width;
        let y = coord.y() % self.height;

        self.data.get_mut((y as usize, x as usize))
    }

    pub fn set(&mut self, coord: Coordinate, value: T) {
        if coord.x() < 0 || coord.y() < 0 {
            return;
        }

        if coord.x() >= self.width || coord.y() >= self.height {
            return;
        }

        self.data[(coord.y() as usize, coord.x() as usize)] = value;
    }

    pub fn row_iter(&self) -> impl Iterator<Item = T> + '_ {
        RowIter {
            grid: self,
            cur: Coordinate::new(0, 0),
        }
    }

    pub fn col_iter(&self) -> impl Iterator<Item = T> + '_ {
        ColIter {
            grid: self,
            cur: Coordinate::new(0, 0),
        }
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

pub struct RowIter<'a, T>
where
    T: Clone,
{
    grid: &'a Grid2D<T>,
    cur: Coordinate,
}

impl<T: Clone> Iterator for RowIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get(self.cur)?;

        self.cur += Direction::Right.into();

        if self.cur.x() >= self.grid.width {
            self.cur = Coordinate::new(0, self.cur.y() + 1);
        }

        Some(result.clone())
    }
}

pub struct ColIter<'a, T>
where
    T: Clone,
{
    grid: &'a Grid2D<T>,
    cur: Coordinate,
}

impl<T: Clone> Iterator for ColIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get(self.cur)?;

        self.cur += Direction::Down.into();

        if self.cur.y() >= self.grid.height {
            self.cur = Coordinate::new(self.cur.x() + 1, 0);
        }

        Some(result.clone())
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

        assert_eq!(grid.get_wrap(Coordinate::new(0, 0)), Some(&1));
        assert_eq!(grid.get_wrap(Coordinate::new(1, 0)), Some(&2));
        assert_eq!(grid.get_wrap(Coordinate::new(2, 0)), Some(&3));
        assert_eq!(grid.get_wrap(Coordinate::new(3, 0)), Some(&1));

        assert_eq!(grid.get_wrap(Coordinate::new(0, 0)), Some(&1));
        assert_eq!(grid.get_wrap(Coordinate::new(0, 1)), Some(&4));
        assert_eq!(grid.get_wrap(Coordinate::new(0, 2)), Some(&7));
        assert_eq!(grid.get_wrap(Coordinate::new(0, 3)), Some(&1));
    }

    #[test]
    fn get_wrap_mut_test() {
        let mut grid: Grid2D<i32> = Grid2D::from_shape_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(grid.get_wrap_mut(Coordinate::new(0, 0)), Some(&mut 1));
        assert_eq!(grid.get_wrap_mut(Coordinate::new(1, 0)), Some(&mut 2));
        assert_eq!(grid.get_wrap_mut(Coordinate::new(2, 0)), Some(&mut 3));
        assert_eq!(grid.get_wrap_mut(Coordinate::new(3, 0)), Some(&mut 1));

        assert_eq!(grid.get_wrap_mut(Coordinate::new(0, 0)), Some(&mut 1));
        assert_eq!(grid.get_wrap_mut(Coordinate::new(0, 1)), Some(&mut 4));
        assert_eq!(grid.get_wrap_mut(Coordinate::new(0, 2)), Some(&mut 7));
        assert_eq!(grid.get_wrap_mut(Coordinate::new(0, 3)), Some(&mut 1));
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
    fn test_row_iter() {
        let grid: Grid2D<i32> = Grid2D::from_shape_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut iter = grid.row_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_col_iter() {
        let grid: Grid2D<i32> = Grid2D::from_shape_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut iter = grid.col_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), None);
    }
}
