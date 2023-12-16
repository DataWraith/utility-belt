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
        let mut height = 0;
        let mut elems = Vec::new();

        for c in input.chars() {
            if c == '\n' {
                height += 1;
            } else {
                width += 1;
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

    pub fn get_mut(&mut self, coord: Coordinate) -> Option<&mut T> {
        if coord.x() < 0 || coord.y() < 0 {
            return None;
        }

        self.data.get_mut((coord.y() as usize, coord.x() as usize))
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

        if self.cur.y() >= self.grid.height {
            return None;
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

        if self.cur.x() >= self.grid.width {
            return None;
        }

        Some(result.clone())
    }
}
