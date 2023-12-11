use std::{cmp::Reverse, hash::Hash};

use ahash::AHashSet as HashSet;
use radix_heap::RadixHeapMap;

/// Uniform cost search
///
/// # Arguments
/// * `start` - The starting node
/// * `is_goal` - A function that returns true if the node is a goal
/// * `successors` - A function that returns the successors of a node
///
/// # Returns
/// * `Option<(T, usize)>` - The goal node and the cumulative cost of the path
/// * `HashSet<T>` - The set of visited nodes
///
// TODO: Currently costs are usize, but they should be generic
pub fn uniform_cost_search<T: Hash + Eq + Clone>(
    start: T,
    is_goal: impl Fn(&T) -> bool,
    successors: impl Fn(&T) -> Vec<(T, usize)>,
) -> (Option<(T, usize)>, HashSet<T>) {
    let mut queue = RadixHeapMap::new();
    let mut visited = HashSet::new();

    queue.push(Reverse(0), start);

    while let Some((cost, node)) = queue.pop() {
        if is_goal(&node) {
            // Explicitly visit the goal node
            visited.insert(node.clone());

            return (Some((node, cost.0)), visited);
        }

        if !visited.insert(node.clone()) {
            continue;
        }

        for (next_node, next_cost) in successors(&node) {
            queue.push(Reverse(cost.0 + next_cost), next_node);
        }
    }

    (None, visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ucs_test() {
        let (result, visited) = uniform_cost_search(
            15,
            |n| *n == 25,
            |n| {
                let mut successors = Vec::new();

                if *n < 25 {
                    successors.push((n + 1, 1));
                }

                if *n > 0 {
                    successors.push((n - 1, 1));
                }

                successors
            },
        );

        assert_eq!(result, Some((25, 10)));
        assert_eq!(visited.len(), 20);
    }
}
