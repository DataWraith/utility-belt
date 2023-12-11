use std::collections::VecDeque;
use std::hash::Hash;

use ahash::AHashSet as HashSet;

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
            return (Some(node), visited);
        }

        if visited.contains(&node) {
            continue;
        }

        visited.insert(node.clone());

        for next_node in successors(&node) {
            queue.push_back(next_node);
        }
    }

    (None, visited)
}
