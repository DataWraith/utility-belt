use num::{Num, Signed};

/// Returns the difference between each successive value of the given slice.
pub fn differences<T: Num + Signed + Clone>(xs: &[T]) -> Vec<T> {
    xs.windows(2).map(|x| x[1].clone() - x[0].clone()).collect()
}

/// Determines the degree of the polynomial that generates the given sequence.
/// Assumes that the sequence does not contain noise.
pub fn polynomial_degree<T: Num + Signed + Clone>(ys: &[T]) -> usize {
    let mut cur = differences(ys);

    for degree in 0..(ys.len() - 1) {
        if cur.iter().all(|x| *x == T::zero()) {
            return degree;
        }

        cur = differences(&cur);
    }

    ys.len() - 1
}

/// Determines the coefficients of a quadratic polynomial from data. Assumes
/// that there is no noise in the data. Assumes that x is 1, 2, 3, ... and the
/// corresponding y is given in the `ys` argument.
///
/// The return type is ordered from highest degree to lowest degree, so that the
/// returned coefficients (a, b, c) correspond to the polynomial `ax² + bx + c`.
///
/// Reference: https://www.radfordmathematics.com/algebra/sequences-series/difference-method-sequences/quadratic-sequences.html
pub fn fit_quadratic<T: Num + Signed + Clone>(ys: &[T]) -> Option<(T, T, T)> {
    let first_diffs = differences(ys);
    let second_diffs = differences(&first_diffs);
    let third_diffs = differences(&second_diffs);

    if third_diffs.is_empty() || third_diffs.iter().any(|d| *d != T::zero()) {
        return None;
    }

    let a = second_diffs[0].clone() / (T::one() + T::one());
    let b = first_diffs[0].clone() - a.clone() - a.clone() - a.clone();
    let c = ys[0].clone() - a.clone() - b.clone();

    Some((a, b, c))
}

/// Determines the coefficients of a cubic polynomial from data. Assumes
/// that there is no noise in the data. Assumes that x is 1, 2, 3, ... and the
/// corresponding y is given in the `ys` argument.
///
/// The return type is ordered from highest degree to lowest degree, so that the
/// returned coefficients (a, b, c, d) correspond to the polynomial `ax³ + bx² +
/// cx + d.`
///
/// Reference: https://www.radfordmathematics.com/algebra/sequences-series/difference-method-sequences/cubic-sequences.html
pub fn fit_cubic<T: Num + Signed + Clone>(ys: &[T]) -> Option<(T, T, T, T)> {
    let first_diffs = differences(ys);
    let second_diffs = differences(&first_diffs);
    let third_diffs = differences(&second_diffs);
    let fourth_diffs = differences(&third_diffs);

    if fourth_diffs.is_empty() || fourth_diffs.iter().any(|d| *d != T::zero()) {
        return None;
    }

    let two = T::one() + T::one();
    let three = two.clone() + T::one();
    let six = three.clone() + three.clone();
    let twelve = six.clone() + six.clone();

    let a = third_diffs[0].clone() / six.clone();
    let b = (second_diffs[0].clone() - twelve.clone() * a.clone()) / two.clone();
    let c =
        first_diffs[0].clone() - three.clone() * b.clone() - (six.clone() + T::one()) * a.clone();
    let d = ys[0].clone() - a.clone() - b.clone() - c.clone();

    Some((a, b, c, d))
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
        let ys: Vec<isize> = quadratic_sequence().take(10).collect();

        let fd = differences(&ys);
        let sd = differences(&fd);
        let td = differences(&sd);

        assert_eq!(td.iter().sum::<isize>(), 0);
    }

    #[test]
    fn test_polynomial_degree() {
        let ys: Vec<i8> = constant_sequence().take(10).collect();
        assert_eq!(polynomial_degree(&ys), 0);

        let ys: Vec<i32> = linear_sequence().take(10).collect();
        assert_eq!(polynomial_degree(&ys), 1);

        let ys: Vec<isize> = quadratic_sequence().take(10).collect();
        assert_eq!(polynomial_degree(&ys), 2);
    }

    #[test]
    fn test_fit_quadratic() {
        let ys = vec![3699, 91951, 297707, 620967];
        let (a, b, c) = fit_quadratic(&ys).unwrap();
        let f = |x: isize| a * x * x + b * x + c;

        assert_eq!(f(1), ys[0]);
        assert_eq!(f(2), ys[1]);
        assert_eq!(f(3), ys[2]);
        assert_eq!(f(4), ys[3]);
    }

    #[test]
    fn test_fit_cubic() {
        let ys = vec![4, 14, 40, 88, 164];
        let (a, b, c, d) = fit_cubic(&ys).unwrap();

        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, -3);
        assert_eq!(d, 4);

        let ys = quadratic_sequence().take(10).collect::<Vec<_>>();
        let (a, b, c, d) = fit_cubic(&ys).unwrap();

        assert_eq!(a, 0);
    }
}
