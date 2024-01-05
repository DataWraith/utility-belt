/// CmpEq wraps any type T and implements Ord and PartialOrd such that any two
/// values of T are considered equal.
///
/// This is useful for implementing A* search, where we order nodes by their
/// f-value (and thus don't care about the order of the T's), but we want to be
/// able to store the state T in the same priority queue. Since that requires an
/// Ord implementation, we can wrap T with CmpEq to get a dummy implementation
/// of Ord and PartialOrd.
#[derive(PartialEq, Eq)]
pub struct CmpEq<T>(pub T);

impl<T: PartialEq + Eq> Ord for CmpEq<T> {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl<T: PartialEq> PartialOrd for CmpEq<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}
