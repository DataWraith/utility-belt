use std::ops::{Add, Sub};

use num::Num;

/// Binary search
///
/// Finds the first index `i` in the range `[lo, hi)` such that `pred(i)` is
/// `false` and pred(i+1) is `true`, assuming that there is exactly one pair of
/// indices between which the predicate flips.
///
/// Returns None if no such index exists (e.g. if `pred(lo)` is true or
/// `pred(hi)` is false).
///
pub fn bisect<T: Num + Copy + PartialOrd + Add + Sub>(
    lo: T,
    hi: T,
    pred: impl Fn(&T) -> bool,
) -> Option<T> {
    if let Some((left, _right)) = bsearch(
        |lo, hi| {
            if *lo + T::one() < *hi {
                Some(*lo + (*hi - *lo) / (T::one() + T::one()))
            } else {
                None
            }
        },
        pred,
        lo,
        hi,
    ) {
        Some(left)
    } else {
        None
    }
}

/// A general bisection search.
///
/// It searches the interval between `lo` and `hi` for the two values between
/// which the predicate switches from false to true and returns them, assuming
/// that exactly one such a pair exists.
///
/// You have to provide a midpoint function that, conceptually, returns the
/// midpoint between two values (although you can also implement e.g. a linear
/// search by always incrementing the lower bound by 1 instead of bisecting the
/// interval). If there is no midpoint, e.g. because lo and hi are consecutive
/// integers, the midpoint function should return None, which signals that the
/// search is complete.
///
/// Returns None if there is no such pair, e.g. if `predicate(lo)` is true or
/// `predicate(hi)` is false from the start.
///
pub fn bsearch<T>(
    midpoint_fn: impl Fn(&T, &T) -> Option<T>,
    predicate: impl Fn(&T) -> bool,
    lo: T,
    hi: T,
) -> Option<(T, T)> {
    if predicate(&lo) || !predicate(&hi) {
        // Invariant: predicate(lo) is always false and predicate(hi) is always
        // true. The search function finds the two values, lo and hi, between
        // which the predicate switches from false to true and returns them.
        //
        // If the predicate does not hold, then there's no point in running the
        // search, so we'll just return None.
        return None;
    }

    fn inner<T>(
        midpoint_fn: impl Fn(&T, &T) -> Option<T>,
        predicate: impl Fn(&T) -> bool,
        lo: T,
        hi: T,
    ) -> Option<(T, T)> {
        if let Some(mid) = midpoint_fn(&lo, &hi) {
            if predicate(&mid) {
                inner(midpoint_fn, predicate, lo, mid)
            } else {
                inner(midpoint_fn, predicate, mid, hi)
            }
        } else {
            Some((lo, hi))
        }
    }

    inner(midpoint_fn, predicate, lo, hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bisect_test() {
        let v = [1, 2, 3, 4, 5];

        assert_eq!(bisect(0, v.len() - 1, |&i| v[i] >= 1), None); // pred(lo) is true -> None
        assert_eq!(bisect(0, v.len() - 1, |&i| v[i] >= 2).unwrap(), 0);
        assert_eq!(bisect(0, v.len() - 1, |&i| v[i] >= 3).unwrap(), 1);
        assert_eq!(bisect(0, v.len() - 1, |&i| v[i] >= 4).unwrap(), 2);
        assert_eq!(bisect(0, v.len() - 1, |&i| v[i] >= 5).unwrap(), 3);
        assert_eq!(bisect(0, v.len() - 1, |&i| v[i] >= 6), None); // No switch from false to true -> None
    }
}
