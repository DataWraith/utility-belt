use std::{
    fmt::{Debug, Formatter},
    ops::Range,
};

use num::Num;

// Least significant one bit
fn lso(x: usize) -> usize {
    x & x.wrapping_neg()
}

/// FenwickTree allows you to efficiently query the sum of a range of values.
///
/// Contrary to PrefixSum, it also allows you to efficiently update the values
/// at the cost of slightly slower queries.
pub struct FenwickTree<T>
where
    T: Num + Clone,
{
    tree: Vec<T>,
}

impl<T: Num + Clone> FenwickTree<T> {
    /// Initialize a Fenwick tree of the given size.
    pub fn new(size: usize) -> Self {
        Self {
            tree: vec![T::zero(); size + 1],
        }
    }

    /// Construct a Fenwick tree from the given values.
    //
    // NOTE: This is naive and inefficient, but simple to implement.
    pub fn from_values(values: &[T]) -> Self {
        let mut tree = Self::new(values.len());

        for (i, v) in values.iter().enumerate() {
            tree.add(i, v.clone());
        }

        tree
    }

    /// Add `delta` to the value at `idx`.
    pub fn add(&mut self, idx: usize, delta: T) {
        let mut i = idx + 1;

        assert!(i < self.tree.len(), "index out of bounds");

        while i < self.tree.len() {
            self.tree[i] = self.tree[i].clone() + delta.clone();
            i += lso(i);
        }
    }

    /// Get the value at `idx`.
    pub fn get(&self, idx: usize) -> Option<T> {
        if idx < self.tree.len() {
            Some(self.range_sum(idx..(idx + 1)))
        } else {
            None
        }
    }

    /// Get the sum of the values in the range [start, end).
    fn range_sum(&self, rng: Range<usize>) -> T {
        let start = rng.start;
        let end = rng.end;

        if start == 0 {
            self.query(end)
        } else {
            self.query(end) - self.query(start)
        }
    }

    /// Query the sum of the values in the range [0, idx).
    fn query(&self, idx: usize) -> T {
        assert!(idx < self.tree.len(), "index out of bounds");

        let mut i = idx;
        let mut sum = T::zero();

        while i != 0 {
            sum = sum + self.tree[i].clone();
            i -= lso(i);
        }

        sum
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.tree.len() - 1
    }
}

impl Debug for FenwickTree<i32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries((0..self.len()).map(|i| self.get(i).unwrap()))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let t = FenwickTree::from_values(&[1, 2, 3, 4]);

        assert_eq!(t.get(0), Some(1));
        assert_eq!(t.get(1), Some(2));
        assert_eq!(t.get(2), Some(3));
        assert_eq!(t.get(3), Some(4));
    }

    #[test]
    fn test_add() {
        let mut t = FenwickTree::new(4);

        t.add(0, 1);
        t.add(1, 2);
        t.add(2, 3);
        t.add(3, 4);

        assert_eq!(t.get(0), Some(1));
        assert_eq!(t.get(1), Some(2));
        assert_eq!(t.get(2), Some(3));
        assert_eq!(t.get(3), Some(4));

        t.add(2, 17);

        assert_eq!(t.get(2), Some(20));
        assert_eq!(t.range_sum(1..4), 26);
    }

    #[test]
    fn test_range_sum() {
        let t = FenwickTree::from_values(&[1, 2, 3, 4]);

        assert_eq!(t.range_sum(0..0), 0);
        assert_eq!(t.range_sum(0..1), 1);
        assert_eq!(t.range_sum(0..2), 3);
        assert_eq!(t.range_sum(0..3), 6);
        assert_eq!(t.range_sum(0..4), 10);

        assert_eq!(t.range_sum(1..2), 2);
        assert_eq!(t.range_sum(1..3), 5);
        assert_eq!(t.range_sum(1..4), 9);

        assert_eq!(t.range_sum(2..3), 3);
        assert_eq!(t.range_sum(2..4), 7);

        assert_eq!(t.range_sum(3..4), 4);
    }

    #[test]
    fn test_query() {
        let t = FenwickTree::from_values(&[1, 2, 3, 4]);

        assert_eq!(t.query(0), 0);
        assert_eq!(t.query(1), 1);
        assert_eq!(t.query(2), 3);
        assert_eq!(t.query(3), 6);
        assert_eq!(t.query(4), 10);
    }

    #[test]
    fn test_debug() {
        let t = FenwickTree::new(4);
        assert_eq!(format!("{:?}", t), "[0, 0, 0, 0]");

        let t = FenwickTree::from_values(&[1, 2, 3, 4]);
        assert_eq!(format!("{:?}", t), "[1, 2, 3, 4]");
    }
}
