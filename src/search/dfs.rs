use std::ops::Deref;

/// Depth-first search
pub struct DFS<N>
where
    N: Clone,
{
    stack: Vec<N>,
}

impl<N: Clone> DFS<N> {
    pub fn new(start: N) -> Self {
        let stack = vec![start.clone()];

        Self { stack }
    }

    pub fn next<S, IN>(&mut self, mut successors: S) -> Option<N>
    where
        S: FnMut(&N) -> IN,
        IN: IntoIterator<Item = N>,
    {
        if let Some(cur) = self.stack.pop() {
            for next in successors(&cur) {
                self.stack.push(next.clone());
            }

            return Some(cur);
        }

        None
    }
}

impl<N> Deref for DFS<N>
where
    N: Clone,
{
    type Target = Vec<N>;

    fn deref(&self) -> &Self::Target {
        &self.stack
    }
}

#[cfg(test)]
mod tests {
    use crate::misc::MiniBitset;

    use super::*;

    #[test]
    fn test_dfs() {
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

        let mut dfs = DFS::new(0);
        let mut visited = Vec::new();

        while let Some(cur) = dfs.next(&mut successors) {
            visited.push(cur);
        }

        assert_eq!(visited, vec![0, -1, -2, -3, -4, -5, 1, 2, 3, 4, 5]);
    }
}
