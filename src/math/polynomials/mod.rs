use num::{Num, Signed};

/// Returns the difference between each successive value of the given slice.
pub fn differences<T: Num + Signed + Clone>(xs: &[T]) -> Vec<T> {
    xs.windows(2).map(|x| x[1].clone() - x[0].clone()).collect()
}

/// Determines the degree of the polynomial that generates the given sequence.
/// Assumes that the sequence does not contain noise.
pub fn polynomial_degree<T: Num + Signed + Clone>(xs: &[T]) -> usize {
    let mut cur = differences(xs);

    for degree in 0..(xs.len() - 1) {
        if cur.iter().all(|x| *x == T::zero()) {
            return degree;
        }

        cur = differences(&cur);
    }

    xs.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn constant_sequence() -> impl Iterator<Item = i8> {
        std::iter::repeat(17)
    }

    fn linear_sequence() -> impl Iterator<Item = i32> {
        let mut x = 0;

        std::iter::from_fn(move || {
            x += 1;
            Some(3 * x - 7)
        })
    }

    fn quadratic_sequence() -> impl Iterator<Item = isize> {
        let f = |x: usize| (3 * x * x + 7 * x + 3) as isize;

        let mut x = 0;

        std::iter::from_fn(move || {
            x += 1;
            Some(f(x))
        })
    }

    #[test]
    fn test_differences() {
        let xs: Vec<isize> = quadratic_sequence().take(10).collect();

        let fd = differences(&xs);
        let sd = differences(&fd);
        let td = differences(&sd);

        assert_eq!(td.iter().sum::<isize>(), 0);
    }

    #[test]
    fn test_polynomial_degree() {
        let xs: Vec<i8> = constant_sequence().take(10).collect();
        assert_eq!(polynomial_degree(&xs), 0);

        let xs: Vec<i32> = linear_sequence().take(10).collect();
        assert_eq!(polynomial_degree(&xs), 1);

        let xs: Vec<isize> = quadratic_sequence().take(10).collect();
        assert_eq!(polynomial_degree(&xs), 2);
    }
}
