use num::Num;

/// Prefix sum
///
/// The prefix sum is useful to be able to query the sum of a range of values.
///
/// # Examples
///
/// ```
/// use utility_belt::math::cumsum::PrefixSum;
///
/// let v = [1, 2, 3, 4, 5];
/// let psum = PrefixSum::new(&v);
///
/// assert_eq!(psum.query(0..1), 1); // 1
/// assert_eq!(psum.query(0..3), 6); // 1 + 2 + 3
/// assert_eq!(psum.query(1..4), 9); // 2 + 3 + 4
/// ```
pub struct PrefixSum<T>
where
    T: Num + Clone,
{
    prefix_sums: Vec<T>,
}

impl<T: Num + Clone> PrefixSum<T> {
    pub fn new(values: &[T]) -> Self {
        let mut cumsum = T::zero();
        let mut prefix_sums = Vec::with_capacity(values.len());

        for v in values {
            cumsum = cumsum + v.clone();
            prefix_sums.push(cumsum.clone());
        }

        Self { prefix_sums }
    }

    pub fn query(&self, r: std::ops::Range<usize>) -> T {
        let x1 = r.start;
        let x2 = r.end;

        let lower = if x1 == 0 {
            T::zero()
        } else {
            self.prefix_sums[x1 - 1].clone()
        };

        let upper = if x2 == 0 {
            T::zero()
        } else {
            self.prefix_sums[x2 - 1].clone()
        };

        upper - lower
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_sum() {
        let cumsum = PrefixSum::new(&[1, 2, 3]);

        assert_eq!(cumsum.query(0..1), 1);
        assert_eq!(cumsum.query(1..2), 2);
        assert_eq!(cumsum.query(2..3), 3);

        assert_eq!(cumsum.query(0..2), 3);
        assert_eq!(cumsum.query(0..3), 6);

        assert_eq!(cumsum.query(1..3), 5);
    }

    #[test]
    fn prefix_sum_float() {
        let cumsum = PrefixSum::new(&[1.0, 2.0, 3.0]);

        assert_eq!(cumsum.query(0..1), 1.0);
        assert_eq!(cumsum.query(1..2), 2.0);
        assert_eq!(cumsum.query(2..3), 3.0);

        assert_eq!(cumsum.query(0..2), 3.0);
        assert_eq!(cumsum.query(0..3), 6.0);

        assert_eq!(cumsum.query(1..3), 5.0);
    }
}
