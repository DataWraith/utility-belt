use std::ops::{Deref, DerefMut};

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
        if x > -self.border_size && x < 0 || y > -self.border_size && y < 0 {
            return Some(&self.default);
        }

        // Bottom/Right border
        if x >= self.grid.width() as i32 && x < self.grid.width() as i32 + self.border_size
            || y >= self.grid.height() as i32 && y < self.grid.height() as i32 + self.border_size
        {
            return Some(&self.default);
        }

        self.grid.get(coord)
    }

    pub fn get_wrap(&self, coord: Coordinate) -> &T {
        let x = coord.x() % self.width();
        let y = coord.y() % self.height();
        let c = Coordinate::new(x, y);

        self.get(c).unwrap()
    }

    pub fn set(&mut self, coord: Coordinate, value: T) {
        let x = coord.x() - self.border_size;
        let y = coord.y() - self.border_size;
        let c = Coordinate::new(x, y);

        self.grid.set(c, value)
    }

    pub fn indexed_iter(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        (0..self.height()).flat_map(move |y| {
            (0..self.width()).map(move |x| {
                let c = Coordinate::new(x, y);
                (c, self.get(c).unwrap())
            })
        })
    }

    pub fn row_iter(&self) -> impl Iterator<Item = &T> {
        (0..self.height()).flat_map(move |y| {
            (0..self.width()).map(move |x| {
                let c = Coordinate::new(x, y);
                self.get(c).unwrap()
            })
        })
    }

    pub fn col_iter(&self) -> impl Iterator<Item = &T> {
        (0..self.width()).flat_map(move |x| {
            (0..self.height()).map(move |y| {
                let c = Coordinate::new(x, y);
                self.get(c).unwrap()
            })
        })
    }
}
