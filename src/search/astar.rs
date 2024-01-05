use std::{cmp::Reverse, collections::BinaryHeap, hash::Hash};

use ahash::HashMap;
use num::Num;

use crate::misc::CmpEq;

/// A* search
pub fn astar<N, S, C, IN, G, H>(
    start: &N,
    mut successors: S,
    mut goal: G,
    mut heuristic: H,
) -> Option<(Vec<N>, C)>
where
    N: Hash + Eq + Clone,
    C: Num + Clone + Ord,
    S: FnMut(&N) -> IN,
    G: FnMut(&N) -> bool,
    H: FnMut(&N) -> C,
    IN: IntoIterator<Item = (N, C)>,
{
    fn reconstruct_path<N: Clone + Hash + Eq>(parents: HashMap<N, N>, current: N) -> Vec<N> {
        let mut result = Vec::new();

        let mut cur = current;
        result.push(cur.clone());

        while let Some(next) = parents.get(&cur) {
            result.push(next.clone());
            cur = next.clone();
        }

        result.reverse();
        result
    }

    let mut open_set = BinaryHeap::new();
    let mut parents = HashMap::default();

    open_set.push((Reverse(C::zero()), Reverse(C::zero()), CmpEq(start.clone())));

    while let Some((_, Reverse(g), CmpEq(current))) = open_set.pop() {
        if goal(&current) {
            return Some((reconstruct_path(parents, current), g));
        }

        for (neighbor, cost) in successors(&current) {
            if parents.contains_key(&neighbor) {
                // We already have a path to the neighbor, so we don't need to
                // consider this path -- cheaper paths are checked before more
                // expensive paths, at least if the heuristic is admissable and
                // consistent.
                continue;
            }

            let new_g = g.clone() + cost;

            let h = heuristic(&neighbor);
            let f = new_g.clone() + h;

            parents.insert(neighbor.clone(), current.clone());
            open_set.push((Reverse(f), Reverse(new_g), CmpEq(neighbor)));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astar() {
        let goal_state = 17i32;

        let successors = |n: &i32| vec![(n + 5, 1), (n - 1, 4)];
        let heuristic = |n: &i32| goal_state.abs_diff(*n);
        let goal = |n: &i32| *n == goal_state;

        let (path, cost) = astar(&0, successors, goal, heuristic).unwrap();

        assert_eq!(path, vec![0, 5, 10, 15, 20, 19, 18, 17]);
        assert_eq!(cost, 4 + 3 * 4)
    }
}
