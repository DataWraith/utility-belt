use std::{collections::VecDeque, ops::Deref};

/// Breadth-first search
///
/// It's spelled with an 'r' because it is breadth-first, as opposed to
/// best-first.
pub struct BrFS<N>
where
    N: Clone,
{
    queue: VecDeque<N>,
}

impl<N> BrFS<N>
where
    N: Clone,
{
    pub fn new<IN>(start: IN) -> Self
    where
        IN: IntoIterator<Item = N>,
    {
        let mut queue = VecDeque::new();

        for s in start.into_iter() {
            queue.push_back(s.clone());
        }

        Self { queue }
    }

    pub fn next<S, IN>(&mut self, mut successors: S) -> Option<N>
    where
        S: FnMut(&N) -> IN,
        IN: IntoIterator<Item = N>,
    {
        if let Some(cur) = self.queue.pop_front() {
            for next in successors(&cur) {
                self.queue.push_back(next.clone());
            }

            return Some(cur);
        }

        None
    }
}

impl<N> Deref for BrFS<N>
where
    N: Clone,
{
    type Target = VecDeque<N>;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

#[cfg(test)]
mod tests {
    use crate::misc::MiniBitset;

    use super::*;

    #[test]
    fn test_brfs() {
        let mut seen = MiniBitset::<u16>::default();
        seen.insert(6);

        let mut successors = |n: &i32| {
            let mut result = Vec::new();

            if n.abs() < 5 {
                if seen.insert((6 + n + 1) as usize) {
                    result.push(n + 1);
                }

                if seen.insert((6 + n - 1) as usize) {
                    result.push(n - 1);
                }
            }

            result
        };

        let mut brfs = BrFS::new(vec![0]);
        let mut visited = Vec::new();

        while let Some(cur) = brfs.next(&mut successors) {
            visited.push(cur);
        }

        assert_eq!(visited, vec![0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5]);
    }
}
