use std::{collections::VecDeque, hash::Hash};

use ahash::HashSet;

/// Breadth-first search
///
/// It's spelled with an 'r' because it is breadth-first, as opposed to
/// best-first.
pub struct BrFS<N>
where
    N: Hash + Eq + Clone,
{
    pub queue: VecDeque<N>,
    pub seen: HashSet<N>,
}

impl<N> BrFS<N>
where
    N: Hash + Eq + Clone,
{
    pub fn new<IN>(start: IN) -> Self
    where
        IN: IntoIterator<Item = N>,
    {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::default();

        for s in start.into_iter() {
            queue.push_back(s.clone());
            seen.insert(s);
        }

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

        let mut brfs = BrFS::new(vec![0]);
        let mut seen = Vec::new();

        while let Some(cur) = brfs.next(&successors) {
            seen.push(cur);
        }

        assert_eq!(seen, vec![0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5]);
    }
}
