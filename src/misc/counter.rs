//! A simple counter for counting the frequency of items in a collection by storing them in a hash map.

use std::{hash::Hash, ops::Index};

use ahash::AHashMap;

pub struct Counter<T> {
    counts: AHashMap<T, usize>,
    count_sum: usize,
}

impl<T: Hash + Eq> Counter<T> {
    pub fn new() -> Self {
        Self {
            counts: AHashMap::new(),
            count_sum: 0,
        }
    }

    pub fn add(&mut self, item: T) {
        self.count_sum += 1;
        *self.counts.entry(item).or_insert(0) += 1;
    }

    pub fn get(&self, item: &T) -> usize {
        *self.counts.get(item).unwrap_or(&0)
    }

    pub fn count_sum(&self) -> usize {
        self.count_sum
    }

    pub fn frequency(&self, item: &T) -> f64 {
        self.get(item) as f64 / self.count_sum.max(1) as f64
    }
}

impl<T: Hash + Eq> Default for Counter<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Hash + Eq, K: IntoIterator<Item = T>> From<K> for Counter<T> {
    fn from(iter: K) -> Self {
        Self::from_iter(iter)
    }
}

impl<T: Hash + Eq> FromIterator<T> for Counter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut counter = Self::new();

        for item in iter {
            counter.add(item);
        }

        counter
    }
}

impl<T: Hash + Eq> Index<T> for Counter<T> {
    type Output = usize;

    fn index(&self, index: T) -> &Self::Output {
        self.counts.get(&index).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut counter = Counter::from_iter(vec![1, 2, 2, 3, 3, 3]);

        assert_eq!(counter.get(&1), 1);
        assert_eq!(counter.get(&2), 2);
        assert_eq!(counter.get(&3), 3);

        counter.add(4);
        assert_eq!(counter[4], 1);

        assert_eq!(counter[5], 0);
    }

    #[test]
    fn test_frequency() {
        let counter: Counter<i32> = vec![1, 2, 2, 3, 3, 3].into();

        assert_eq!(counter.frequency(&0), 0.0);
        assert_eq!(counter.frequency(&1), 1.0 / 6.0);
        assert_eq!(counter.frequency(&2), 2.0 / 6.0);
        assert_eq!(counter.frequency(&3), 3.0 / 6.0);
        assert_eq!(counter.count_sum(), 6);
    }
}
