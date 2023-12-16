/// Exponential search
///
/// It starts at `lo` and a step size of one. The step size doubles each step
/// until `hi` is reached or `pred` returns `true`. Then it performs a binary
/// search on the range `[lo, hi)`.
pub fn exponential_search(
    mut lo: usize,
    hi: usize,
    mut pred: impl FnMut(usize) -> bool,
) -> Option<usize> {
    let mut step = 1;

    while lo < hi && !pred(hi) {
        lo += step;
        step *= 2;
    }

    if lo >= hi {
        return None;
    }

    Some(super::bisect::bisect(lo, hi, pred))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponential_search_test() {
        let v = [1, 2, 3, 4, 5];

        assert_eq!(exponential_search(0, v.len() - 1, |i| v[i] >= 1), Some(0));
        assert_eq!(exponential_search(0, v.len() - 1, |i| v[i] >= 2), Some(1));
        assert_eq!(exponential_search(0, v.len() - 1, |i| v[i] >= 3), Some(2));
        assert_eq!(exponential_search(0, v.len() - 1, |i| v[i] >= 4), Some(3));
        assert_eq!(exponential_search(0, v.len() - 1, |i| v[i] >= 5), Some(4));
        assert_eq!(exponential_search(0, v.len() - 1, |i| v[i] >= 6), None);
    }
}
