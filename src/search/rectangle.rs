use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::hash::Hash;

use crate::prelude::HashMap;
use crate::prelude::CmpEq;

/// Rectangle search
/// https://arxiv.org/abs/2312.12554
pub struct RectangleSearch<N, C>
where
    N: Clone + Eq + Hash,
    C: Ord + Clone,
{
    open_lists: VecDeque<BinaryHeap<(Reverse<C>, CmpEq<N>)>>,
    closed: HashMap<N, C>,
    incumbent: Option<(N, C)>,
    depth: usize,
}

impl<N, C> RectangleSearch<N, C>
where
    N: Clone + Eq + Hash,
    C: Ord + Clone,
{
    pub fn new<IN>(start: IN) -> Self
    where
        IN: IntoIterator<Item = (N, C)>,
    {
        let mut first_list = BinaryHeap::new();

        for (n, c) in start {
            first_list.push((Reverse(c), CmpEq(n)));
        }

        let open_lists = VecDeque::from(vec![first_list]);
        let closed = HashMap::new();
        let incumbent = None;
        let depth = 1;

        Self {
            open_lists,
            closed,
            incumbent,
            depth,
        }
    }

    pub fn next<S, IN, F>(&mut self, mut successors: S, mut heuristic: F) -> Result<(N, C), ()>
    where
        S: FnMut(&N, &C) -> IN,
        F: FnMut(&N, &C) -> C,
        IN: IntoIterator<Item = (N, C, bool)>,
    {
        let mut incumbent_changed = false;

        loop {
            if self.open_lists.iter().all(|list| list.is_empty()) {
                return Err(());
            }

            for i in 0..(self.open_lists.len() - 1) {
                incumbent_changed |= self.select_and_expand(i, &mut heuristic, &mut successors);
            }

            let i = self.open_lists.len() - 1;
            self.open_lists.push_back(BinaryHeap::new());

            for j in i..(self.open_lists.len() - 1) {
                for _k in 1..=self.depth {
                    incumbent_changed |= self.select_and_expand(j, &mut heuristic, &mut successors);
                }
            }

            self.depth += 1;

            while let Some(q) = self.open_lists.front() {
                if q.is_empty() {
                    self.open_lists.pop_front();
                } else {
                    break;
                }
            }

            while let Some(q) = self.open_lists.back() {
                if q.is_empty() {
                    self.open_lists.pop_back();
                } else {
                    break;
                }
            }

            if incumbent_changed {
                return Ok(self.incumbent.as_ref().unwrap().clone());
            }
        }
    }

    fn select_and_expand<S, F, IN>(&mut self, i: usize, mut heuristic: F, mut successors: S) -> bool
    where
        S: FnMut(&N, &C) -> IN,
        F: FnMut(&N, &C) -> C,
        IN: IntoIterator<Item = (N, C, bool)>,
    {
        let mut incumbent_changed = false;

        loop {
            let Some((Reverse(c), CmpEq(n))) = self.open_lists[i].pop() else {
                return incumbent_changed;
            };

            if let Some(best) = &self.incumbent {
                if heuristic(&n, &c) >= best.1 {
                    continue;
                }
            }

            self.closed.insert(n.clone(), c.clone());

            for (next_n, next_c, is_goal) in successors(&n, &c) {
                if let Some(best) = &self.incumbent {
                    if heuristic(&next_n, &next_c) >= best.1 {
                        continue;
                    }
                }

                if is_goal {
                    self.incumbent = Some((next_n, next_c));
                    incumbent_changed = true;
                } else {
                    if let Some(dup) = self.closed.get(&next_n) {
                        if *dup <= next_c {
                            continue;
                        }
                    }

                    self.open_lists[i + 1].push((Reverse(next_c), CmpEq(next_n)));
                }
            }

            break;
        }

        incumbent_changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beam_search() {
        let goal = 16;

        let mut successors = |n: &Vec<i32>, c: &i32| {
            if n.last().unwrap().abs() < goal {
                let mut n1 = n.to_vec();
                let mut n2 = n.to_vec();
                let mut n3 = n.to_vec();

                n1.push(n1.last().unwrap() + 5);
                n2.push(n2.last().unwrap() - 1);
                n3.push(n3.last().unwrap() - 2);

                vec![
                    (n1.clone(), c + 2, *n1.last().unwrap() == goal),
                    (n2.clone(), c + 1, *n2.last().unwrap() == goal),
                    (n3.clone(), c + 5, *n3.last().unwrap() == goal),
                ]
            } else {
                vec![]
            }
        };

        let mut bs = RectangleSearch::new(vec![(vec![0], 0)]);
        let mut visited_states = Vec::new();

        while let Ok(cur) = bs.next(&mut successors, |n, c| {
            ((26 - n.last().unwrap()).abs() + 4) / 5 + c
        }) {
            visited_states.push(cur);
        }

        assert_eq!(
            visited_states,
            vec![(vec![0, -1, -2, -3, -4, 1, 6, 11, 16], 12)]
        );
    }
}
