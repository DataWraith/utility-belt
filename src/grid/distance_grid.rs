use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};

use num::{Bounded, Unsigned};

use super::Grid2D;

pub struct DistanceGrid<T>
where
    T: Bounded + Unsigned + Clone,
{
    cost: Grid2D<T>,
}

/// A grid of distances
///
/// This is a wrapper around Grid2D<T> that implements Display and Debug.
impl<T> DistanceGrid<T>
where
    T: Bounded + Unsigned + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cost: Grid2D::new(width, height, T::max_value()),
        }
    }
}

impl<T> Display for DistanceGrid<T>
where
    T: Bounded + Unsigned + Clone + Display + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        for y in 0..self.cost.height {
            for x in 0..self.cost.width {
                if self.cost[(x, y).into()] == T::max_value() {
                    write!(f, "[####]")?;
                } else {
                    write!(f, "[{:4}]", self.cost[(x, y).into()])?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> Debug for DistanceGrid<T>
where
    T: Bounded + Unsigned + Clone + Display + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<T> Deref for DistanceGrid<T>
where
    T: Bounded + Unsigned + Clone,
{
    type Target = Grid2D<T>;

    fn deref(&self) -> &Self::Target {
        &self.cost
    }
}

impl<T> DerefMut for DistanceGrid<T>
where
    T: Bounded + Unsigned + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cost
    }
}

impl<T> From<Grid2D<T>> for DistanceGrid<T>
where
    T: Bounded + Unsigned + Clone,
{
    fn from(grid: Grid2D<T>) -> Self {
        Self { cost: grid }
    }
}
