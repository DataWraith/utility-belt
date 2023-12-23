use std::{collections::VecDeque, hash::Hash};

use ahash::HashSet;

/// Breadth-first search
///
/// It's spelled with an 'r' because it is breadth-first, as opposed to
/// best-first.
pub struct BrFS<N>
where
    N: Hash + Eq,
{
    pub queue: VecDeque<N>,
    pub seen: HashSet<N>,
}

impl<N: Hash + Eq + Clone> BrFS<N> {
    pub fn new(start: N) -> Self {
        let queue = VecDeque::from([start.clone()]);

        let seen = {
            let mut seen = HashSet::default();
            seen.insert(start);
            seen
        };

        Self { queue, seen }
    }

    pub fn next<S, IN>(&mut self, mut successors: S) -> Option<N>
    where
        S: FnMut(&N) -> IN,
        IN: IntoIterator<Item = N>,
    {
        if let Some(cur) = self.queue.pop_front() {
            for next in successors(&cur) {
                if self.seen.insert(next.clone()) {
                    self.queue.push_back(next.clone());
                }
            }

            return Some(cur);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brfs() {
        let successors = |n: &i32| {
            if n.abs() < 5 {
                vec![n + 1, n - 1]
            } else {
                vec![]
            }
        };

        let mut brfs = BrFS::new(0);
        let mut seen = Vec::new();

        while let Some(cur) = brfs.next(&successors) {
            seen.push(cur);
        }

        assert_eq!(seen, vec![0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5]);
    }
}
