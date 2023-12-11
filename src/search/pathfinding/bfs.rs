use std::collections::VecDeque;
use std::hash::Hash;

use ahash::AHashSet as HashSet;

// TODO: Make a struct for the return type.
// TODO: We may want to return the actual path...
pub fn breadth_first_search<T: Hash + Eq + Clone>(
    start: T,
    is_goal: impl Fn(&T) -> bool,
    successors: impl Fn(&T) -> Vec<T>,
) -> (Option<T>, HashSet<T>) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if is_goal(&node) {
            // Explicitly visit the goal node
            visited.insert(node.clone());

            return (Some(node), visited);
        }

        if !visited.insert(node.clone()) {
            continue;
        }

        for next_node in successors(&node) {
            queue.push_back(next_node);
        }
    }

    (None, visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_test() {
        let (result, visited) = breadth_first_search(
            0,
            |n| *n == 5,
            |n| {
                let mut successors = Vec::new();

                if *n < 5 {
                    successors.push(n + 1);
                }

                if *n > 0 {
                    successors.push(n - 1);
                }

                successors
            },
        );

        assert_eq!(result, Some(5));
        assert_eq!(visited.len(), 6);
    }
}
