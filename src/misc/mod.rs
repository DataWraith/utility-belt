pub mod union_find;
pub use union_find::*;

pub mod bitset;
pub use bitset::*;

pub mod cmp_eq;
pub use cmp_eq::CmpEq;

pub mod counter;
pub use counter::*;

use std::hash::Hash;

use crate::prelude::HashMap;

/// Path contraction
///
/// Some Advent of Code puzzles involve finding the result of applying, say, one
/// billion operations to a data structure. Since this kind of problem would be
/// impossible otherwise, the problems usually contain a cycle we can find using,
/// for example, the `pathfinding` crate and Brent's algorithm.
///
/// Path contraction is an alternative way to solve such problems. The idea is
/// to make short-cuts in the state-space, similar to the `Contraction
/// Hierarchies` idea in pathfinding.
///
/// For example, given a path from A to E, `A -> B -> C -> D -> E`, we can start
/// by moving from `A` to `B`, and then from `B` to `C`. Now that we know where
/// the transitions lead, we can add a shortcut from `A` to `C`, skipping `B`.
///
/// Short-cuts are themselves subject to being short-cut: When we're at `A` again,
/// we move through the short-cut `A--->C`. If there is already a short-cut
/// `C--->E`, we can combine the shortcuts to form a new shortcut `A--->E`.
///
/// We also store the length of the path that the shortcut shortcuts in order to
/// be able to track how many operations we've done already. If a shortcut would
/// overshoot the target, we clear all shortcuts and continue building new,
/// shorter, shortcuts from the current position until we reach our target
/// number of iterations.
///
pub fn path_contraction<N, FN>(start: &N, mut successor: FN, iterations: usize) -> N
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> N,
{
    let mut shortcuts: HashMap<N, (N, usize)> = HashMap::default();

    let mut cur = start.clone();
    let mut cur_iter = 0;

    loop {
        if cur_iter == iterations {
            return cur;
        }

        // Step 1
        let (next1, iters_step1) = if let Some((next1, length1)) = shortcuts.get(&cur) {
            (next1.clone(), *length1)
        } else {
            let next1 = successor(&cur);
            (next1, 1)
        };

        // Step 2
        let (next2, iters_step2) = if let Some((next2, length2)) = shortcuts.get(&next1) {
            (next2.clone(), *length2)
        } else {
            let next2 = successor(&next1);
            (next2, 1)
        };

        // Combine
        if cur_iter + iters_step1 + iters_step2 <= iterations {
            shortcuts.insert(cur, (next2.clone(), iters_step1 + iters_step2));
            cur = next2;
            cur_iter += iters_step1 + iters_step2;
        } else if cur_iter + iters_step1 <= iterations {
            shortcuts.insert(cur, (next1.clone(), iters_step1));
            cur = next1;
            cur_iter += iters_step1;
        } else {
            let next = successor(&cur);
            cur = next;
            cur_iter += 1;

            shortcuts.clear();
        }
    }
}

/// Iterates a state function once.
///
/// The idea is to have a HashMap containing the current states. Then a
/// transition function (which may take an input value) is applied to each
/// state, and the resulting state(s) are collected in a new HashMap.
///
/// The HashMap keeps track of how often a given state has occurred. This can be
/// used to, for example, count how often a state is visited in a finite state
/// machine after n iterations.
///
/// For example, given a non-deterministic finite state machine, you can count
/// how often you end up in the `accept` state after iterating the machine for
/// n steps. This was useful for AoC 2023, day 12 in order to count the number of
/// valid strings.
///
pub fn state_iteration<S, FN, IS, IN>(
    states: &HashMap<S, usize>,
    mut transition: FN,
    input: IN,
) -> HashMap<S, usize>
where
    S: Eq + Hash + Clone,
    FN: FnMut(&S, &IN) -> IS,
    IS: IntoIterator<Item = S>,
{
    let mut new_states = HashMap::default();

    for (state, count) in states.iter() {
        for new_state in transition(state, &input) {
            new_states
                .entry(new_state)
                .and_modify(|c| *c += count)
                .or_insert(*count);
        }
    }

    new_states
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_contraction() {
        let start = 0;
        let iterations = 101;

        let result = path_contraction(&start, |n| (n + 1) % 10, iterations);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_state_iteration() {
        let mut states = HashMap::default();
        states.insert(0, 1);

        let result = state_iteration(&states, |n, _| vec![n + 1, n + 1, n + 2], 0);
        assert_eq!(result.get(&1), Some(&2));
        assert_eq!(result.get(&2), Some(&1));
    }
}
