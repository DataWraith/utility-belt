use std::ops::Deref;

use num::{PrimInt, Unsigned};

/// A simple bitset implementation generic over the primitive integer used for storage.
#[derive(Clone, PartialEq, Eq, Hash, Copy, Default, PartialOrd, Ord)]
pub struct MiniBitset<T: PrimInt + Unsigned> {
    data: T,
}

impl<T: PrimInt + Unsigned> MiniBitset<T> {
    /// Create a new bitset from the given bit value.
    pub fn new(data: T) -> Self {
        Self { data }
    }

    /// Return the cardinality of the stored set
    pub fn len(&self) -> u32 {
        self.data.count_ones()
    }

    /// Checks if the bitset is empty.
    pub fn is_empty(&self) -> bool {
        self.data == T::zero()
    }

    /// Check if the bitset contains the given index.
    pub fn contains(&self, i: usize) -> bool {
        self.data & (T::one() << i) != T::zero()
    }

    /// Insert the given index into the bitset.
    pub fn insert(&mut self, i: usize) -> bool {
        if self.contains(i) {
            return false;
        }

        self.data = self.data | (T::one() << i);

        true
    }

    /// Remove the given index from the bitset.
    pub fn remove(&mut self, i: usize) {
        self.data = self.data & !(T::one() << i)
    }

    /// Iterate over the members of the bitset
    pub fn iter(&self) -> impl Iterator<Item = usize> {
        let mut data = self.data;

        std::iter::from_fn(move || {
            if data != T::zero() {
                let index = data.trailing_zeros();

                // Clear least significant one bit
                data = data & (data - T::one());

                return Some(index as usize);
            }

            None
        })
    }
}

impl<T: PrimInt + Unsigned> Deref for MiniBitset<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: PrimInt + Unsigned> std::fmt::Debug for MiniBitset<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        let num_bits = T::zero().count_zeros() as usize;

        for i in (0..num_bits).rev() {
            if self.contains(i) {
                write!(f, "1")?;
            } else {
                write!(f, "0")?;
            }
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset() {
        let mut bs = MiniBitset::new(0b1010u8);

        assert!(!bs.is_empty());
        assert_eq!(bs.len(), 2);

        assert!(!bs.contains(0));
        assert!(bs.contains(1));
        assert!(!bs.contains(2));
        assert!(bs.contains(3));

        bs.remove(1);

        assert!(!bs.contains(1));
        assert!(bs.contains(3));
        assert!(!bs.contains(0));
        assert!(!bs.contains(2));

        bs.insert(0);

        assert!(bs.contains(0));
        assert!(!bs.contains(1));
        assert!(!bs.contains(2));
        assert!(bs.contains(3));

        // Deref
        assert!(bs.count_ones() == 2);
        assert!(bs.count_zeros() == 6);
    }

    #[test]
    fn test_debug() {
        let bs = MiniBitset::new(0b1010_u8);
        assert_eq!(format!("{:?}", bs), "{00001010}");

        let bs = MiniBitset::new(0xcafe_u16);
        assert_eq!(format!("{:?}", bs), "{1100101011111110}");
    }

    #[test]
    fn test_iter() {
        let bs = MiniBitset::new(0b1010_u8);
        assert_eq!(bs.iter().collect::<Vec<_>>(), vec![1, 3]);

        let bs = MiniBitset::new(0xcafe_u16);

        assert_eq!(
            bs.iter().collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5, 6, 7, 9, 11, 14, 15]
        );
    }
}
