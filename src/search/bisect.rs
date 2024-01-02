/// Binary search
///
/// Finds the first index `i` in the range `[lo, hi)` such that `pred(i)` is `true`.
///
pub fn bisect(mut lo: usize, mut hi: usize, mut pred: impl FnMut(usize) -> bool) -> usize {
    let mut mid = (lo + hi) / 2;

    while lo < hi {
        if pred(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }

        mid = (lo + hi) / 2;
    }

    mid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bisect_test() {
        let v = [1, 2, 3, 4, 5];

        assert_eq!(bisect(0, v.len(), |i| v[i] >= 0), 0);
        assert_eq!(bisect(0, v.len(), |i| v[i] >= 1), 0);
        assert_eq!(bisect(0, v.len(), |i| v[i] >= 2), 1);
        assert_eq!(bisect(0, v.len(), |i| v[i] >= 3), 2);
        assert_eq!(bisect(0, v.len(), |i| v[i] >= 4), 3);
        assert_eq!(bisect(0, v.len(), |i| v[i] >= 5), 4);
        assert_eq!(bisect(0, v.len(), |i| v[i] >= 6), 5);
        assert_eq!(bisect(0, v.len(), |i| v[i] >= 7), 5);
    }
}
