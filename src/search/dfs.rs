use std::{collections::VecDeque, hash::Hash};

use ahash::HashSet;

/// Depth-first search
pub struct DFS<N>
where
    N: Hash + Eq + Clone,
{
    pub stack: Vec<N>,
    pub seen: HashSet<N>,
}

impl<N: Hash + Eq + Clone> DFS<N> {
    pub fn new(start: N) -> Self {
        let stack = vec![start.clone()];

        let seen = {
            let mut seen = HashSet::default();
            seen.insert(start);
            seen
        };

        Self { stack, seen }
    }

    pub fn next<S, IN>(&mut self, mut successors: S) -> Option<N>
    where
        S: FnMut(&N) -> IN,
        IN: IntoIterator<Item = N>,
    {
        if let Some(cur) = self.stack.pop() {
            for next in successors(&cur) {
                if self.seen.insert(next.clone()) {
                    self.stack.push(next.clone());
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
    fn test_dfs() {
        let successors = |n: &i32| {
            if n.abs() < 5 {
                vec![n + 1, n - 1]
            } else {
                vec![]
            }
        };

        let mut dfs = DFS::new(0);
        let mut seen = Vec::new();

        while let Some(cur) = dfs.next(&successors) {
            seen.push(cur);
        }

        assert_eq!(seen, vec![0, -1, -2, -3, -4, -5, 1, 2, 3, 4, 5]);
    }
}
