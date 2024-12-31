use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use crate::spatial::grid::Grid2D;

/// A grid of booleans
///
/// This is a wrapper around Grid2D<bool> that implements Display and Debug.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BoolGrid2D {
    grid: Grid2D<bool>,
}

impl BoolGrid2D {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid2D::new(width, height, false),
        }
    }

    pub fn invert(&mut self) {
        for value in self.grid.data.iter_mut() {
            *value = !*value;
        }
    }
}

impl Display for BoolGrid2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                if self.grid[(x, y).into()] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Debug for BoolGrid2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Deref for BoolGrid2D {
    type Target = Grid2D<bool>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl DerefMut for BoolGrid2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl From<Grid2D<bool>> for BoolGrid2D {
    fn from(grid: Grid2D<bool>) -> Self {
        Self { grid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let mut grid = BoolGrid2D::new(3, 3);
        grid.set((1, 1).into(), true);

        let expected = "\n...\n.#.\n...\n";
        let expected_inverted = "\n###\n#.#\n###\n";

        assert_eq!(format!("{}", grid), expected);
        grid.invert();
        assert_eq!(format!("{}", grid), expected_inverted);
    }
}
