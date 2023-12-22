use std::fmt::Debug;

use ndarray::Array2;
use num::Num;

/// A SummedAreaTable is the 2D-equivalent of a PrefixSum.
///
/// It allows you to query the sum of the values inside of a rectangular region
/// in O(1) time after some preprocessing.
///
pub struct SummedAreaTable<T>
where
    T: Num + Clone,
{
    width: usize,
    height: usize,
    table: Array2<T>,
}

impl<T: Num + Clone> SummedAreaTable<T> {
    pub fn new(grid: Array2<T>) -> Self {
        let shape = grid.shape();

        let mut sat = Array2::zeros((shape[0] + 2, shape[1] + 2));

        for x in 1..(2 + shape[0]) {
            for y in 1..(2 + shape[1]) {
                let v = if x > shape[0] || y > shape[1] {
                    T::zero()
                } else {
                    grid[[x - 1, y - 1]].clone()
                };

                sat[[x, y]] = v.clone();
                sat[[x, y]] = sat[[x, y]].clone() + sat[[x, y - 1]].clone();
                sat[[x, y]] = sat[[x, y]].clone() + sat[[x - 1, y]].clone();
                sat[[x, y]] = sat[[x, y]].clone() - sat[[x - 1, y - 1]].clone();
            }
        }

        Self {
            table: sat,
            width: shape[0] + 2,
            height: shape[1] + 2,
        }
    }

    pub fn width(&self) -> usize {
        self.width - 2
    }

    pub fn height(&self) -> usize {
        self.height - 2
    }

    pub fn query(&self, x: usize, y: usize, w: usize, h: usize) -> T {
        let right = (x + w).min(self.width - 1);
        let bot = (y + h).min(self.height - 1);

        let a = [x, y];
        let b = [right, y];
        let c = [x, bot];
        let d = [right, bot];

        let a = self.table[a].clone();
        let b = self.table[b].clone();
        let c = self.table[c].clone();
        let d = self.table[d].clone();

        d + a - b - c
    }
}

impl<T: Num + Clone + Debug> Debug for SummedAreaTable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        for y in 0..self.height() {
            for x in 0..(self.width() - 1) {
                write!(f, "{:?} ", self.query(x, y, 1, 1))?;
            }

            write!(f, "{:?}", self.query(self.width() - 1, y, 1, 1))?;
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
    pub fn it_sums_correctly() {
        let mut grid: Array2<usize> = Array2::default((3, 3));

        grid[[0, 0]] = 1;
        grid[[0, 2]] = 1;
        grid[[1, 1]] = 1;
        grid[[2, 0]] = 1;

        let sat = SummedAreaTable::new(grid);

        // Full table
        assert_eq!(sat.query(0, 0, 3, 3), 4);

        // Single squares
        assert_eq!(sat.query(0, 0, 1, 1), 1);
        assert_eq!(sat.query(1, 0, 1, 1), 0);
        assert_eq!(sat.query(2, 0, 1, 1), 1);
        assert_eq!(sat.query(0, 1, 1, 1), 0);
        assert_eq!(sat.query(1, 1, 1, 1), 1);
        assert_eq!(sat.query(2, 1, 1, 1), 0);
        assert_eq!(sat.query(0, 2, 1, 1), 1);
        assert_eq!(sat.query(1, 2, 1, 1), 0);
        assert_eq!(sat.query(2, 2, 1, 1), 0);

        // 2x2
        assert_eq!(sat.query(0, 0, 2, 2), 2);
        assert_eq!(sat.query(1, 0, 2, 2), 2);
        assert_eq!(sat.query(0, 1, 2, 2), 2);
        assert_eq!(sat.query(1, 1, 2, 2), 1);

        // Rows
        assert_eq!(sat.query(0, 0, 3, 1), 2);
        assert_eq!(sat.query(0, 1, 3, 1), 1);
        assert_eq!(sat.query(0, 2, 3, 1), 1);

        // Columns
        assert_eq!(sat.query(0, 0, 1, 3), 2);
        assert_eq!(sat.query(1, 0, 1, 3), 1);
        assert_eq!(sat.query(2, 0, 1, 3), 1);
    }

    #[test]
    fn test_debug() {
        let mut grid: Array2<usize> = Array2::default((3, 3));

        grid[[0, 0]] = 1;
        grid[[0, 2]] = 1;
        grid[[1, 1]] = 1;
        grid[[2, 0]] = 1;

        let sat = SummedAreaTable::new(grid);

        assert_eq!(
            format!("{:?}", sat),
            indoc! {"

            1 0 1
            0 1 0
            1 0 0
        "},
        );
    }
}
