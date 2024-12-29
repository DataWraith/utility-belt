use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use rand::prelude::*;
use thunderdome::{Arena, Index};

type Priority = u64;

/// A Cartesian tree is a binary tree that satisfies the heap property.
pub struct CartesianTree<K, V> {
    root: Option<Index>,
    arena: Arena<CTNode<K, V>>,
}

pub struct CTNode<K, V> {
    key: K,
    value: V,
    priority: Priority,
    left: Option<Index>,
    right: Option<Index>,
    count: usize,
}

impl<K, V> Default for CartesianTree<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> CartesianTree<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            root: None,
            arena: Arena::new(),
        }
    }

    /// Returns true if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> usize {
        if let Some(root) = self.root {
            self.arena.get(root).unwrap().count
        } else {
            0
        }
    }

    /// Returns the value and priority associated with a key (if any).
    pub fn get(&self, key: &K) -> Option<(&V, Priority)> {
        self.search(self.root, key)
    }

    // Returns the index the key would have in a sorted list of the tree's keys.
    pub fn index(&self, key: K) -> Option<usize> {
        self.index_of(self.root, key)
    }

    /// Returns the element that would be at index `k` in a list sorted by the
    /// tree's keys. This is the kth-largest element in the tree.
    pub fn get_index(&self, k: usize) -> Option<(&K, &V, Priority)> {
        self.kth_largest_of(self.root, k)
    }

    /// Insert a key-value pair into the tree with a given priority.
    pub fn insert(&mut self, key: K, value: V, priority: Priority) {
        self.root = self.insert_kv(self.root, key, value, priority);
    }

    /// Remove a key from the tree.
    pub fn remove(&mut self, key: &K) -> Option<(V, Priority)> {
        let (new_root, value_and_priority) = self.remove_key(self.root, key);
        self.root = new_root;

        value_and_priority
    }

    pub fn remove_index(&mut self, k: usize) -> Option<(K, V, Priority)> {
        let (new_root, value_and_priority) = self.remove_by_index(self.root, k);
        self.root = new_root;

        value_and_priority
    }

    // Searching for a key is just a standard binary search on the tree.
    fn search(&self, root: Option<Index>, key: &K) -> Option<(&V, Priority)> {
        if let Some(root) = root {
            let subtree_root = self.arena.get(root).unwrap();

            match key.cmp(&subtree_root.key) {
                Ordering::Equal => {
                    return Some((&subtree_root.value, subtree_root.priority));
                }
                Ordering::Greater => {
                    return self.search(subtree_root.right, key);
                }
                Ordering::Less => {
                    return self.search(subtree_root.left, key);
                }
            }
        }

        None
    }

    // Returns the index the key would have in a sorted list of the subtree's keys.
    fn index_of(&self, root: Option<Index>, key: K) -> Option<usize> {
        let root_idx = root?;
        let root_node = self.arena.get(root_idx).unwrap();

        let left_count = root_node
            .left
            .map_or(0, |l| self.arena.get(l).unwrap().count);

        match key.cmp(&root_node.key) {
            Ordering::Equal => {
                // If the key is the root, return the size of the left subtree
                // -- those are the elements that come before it in a sorted
                // list.
                Some(left_count)
            }

            Ordering::Greater => {
                // Key is in right subtree, so we need to add the size of the
                // left subtree and the root to the recursive result.
                self.index_of(root_node.right, key)
                    .map(|i| i + 1 + left_count)
            }

            Ordering::Less => {
                // Key is in left subtree, so we just need to recurse.
                self.index_of(root_node.left, key)
            }
        }
    }

    fn kth_largest_of(&self, root: Option<Index>, k: usize) -> Option<(&K, &V, Priority)> {
        let root_idx = root?;
        let root_node = self.arena.get(root_idx).unwrap();

        let left_count = root_node
            .left
            .map_or(0, |l| self.arena.get(l).unwrap().count);

        // If exactly k elements are smaller than the root, the root is the
        // k-th largest element.
        if k == left_count {
            return Some((&root_node.key, &root_node.value, root_node.priority));
        }

        if k < left_count {
            // We know that the k-th largest element is in the left subtree.
            self.kth_largest_of(root_node.left, k)
        } else {
            // We know that the k-th largest element is in the right subtree.
            // However, we need to account for the size of the left subtree and
            // the current node.
            self.kth_largest_of(root_node.right, k - left_count - 1)
        }
    }

    fn update_count(&mut self, root: Option<Index>) {
        if let Some(root) = root {
            let root_node = self.arena.get(root).unwrap();

            let left_count = root_node
                .left
                .map_or(0, |l| self.arena.get(l).unwrap().count);

            let right_count = root_node
                .right
                .map_or(0, |r| self.arena.get(r).unwrap().count);

            let updated_count = 1 + left_count + right_count;

            let root_node = self.arena.get_mut(root).unwrap();
            root_node.count = updated_count;
        }
    }

    fn insert_kv(&mut self, root: Option<Index>, key: K, value: V, priority: u64) -> Option<Index> {
        if root.is_none() {
            let node = CTNode {
                key,
                value,
                priority,
                left: None,
                right: None,
                count: 1,
            };

            return Some(self.arena.insert(node));
        }

        let mut root = root;
        let root_node = self.arena.get(root.unwrap()).unwrap();

        match key.cmp(&root_node.key) {
            // Overwrite the value if the key already exists.
            Ordering::Equal => {
                let root_node = self.arena.get_mut(root.unwrap()).unwrap();
                root_node.value = value;
            }

            Ordering::Less => {
                // Recursively insert into the left subtree.
                let left = self.insert_kv(root_node.left, key, value, priority);

                let root_priority = {
                    let root_node = self.arena.get_mut(root.unwrap()).unwrap();
                    root_node.left = left;
                    root_node.priority
                };

                // Fix heap property
                let left_node = self.arena.get(left.unwrap()).unwrap();

                if left_node.priority > root_priority {
                    root = self.rotate_right(root);
                }

                self.update_count(root);
            }

            Ordering::Greater => {
                // Recursively insert into the right subtree.
                let right = self.insert_kv(root_node.right, key, value, priority);

                let root_priority = {
                    let root_node = self.arena.get_mut(root.unwrap()).unwrap();
                    root_node.right = right;
                    root_node.priority
                };

                // Fix heap property
                let right_node = self.arena.get(right.unwrap()).unwrap();

                if right_node.priority > root_priority {
                    root = self.rotate_left(root);
                }

                self.update_count(root);
            }
        }

        root
    }

    fn remove_key(
        &mut self,
        root: Option<Index>,
        key: &K,
    ) -> (Option<Index>, Option<(V, Priority)>) {
        let Some(root_idx) = root else {
            return (None, None);
        };

        let root_node = self.arena.get(root_idx).unwrap();

        match key.cmp(&root_node.key) {
            Ordering::Less => {
                // The key is in the left subtree if it exists.
                let (left, value_and_priority) = self.remove_key(root_node.left, key);
                let root_node = self.arena.get_mut(root_idx).unwrap();
                root_node.left = left;

                self.update_count(root);

                return (Some(root_idx), value_and_priority);
            }

            Ordering::Greater => {
                // The key is in the right subtree if it exists.
                let (right, value_and_priority) = self.remove_key(root_node.right, key);
                let root_node = self.arena.get_mut(root_idx).unwrap();
                root_node.right = right;

                self.update_count(root);

                return (Some(root_idx), value_and_priority);
            }

            Ordering::Equal => {}
        }

        // We found the key.
        let left = root_node.left;
        let right = root_node.right;

        match (left.is_some(), right.is_some()) {
            (false, false) => {
                // If we don't have any children, we can just remove the node.
                let root_node = self.arena.remove(root_idx).unwrap();
                return (None, Some((root_node.value, root_node.priority)));
            }

            (false, true) => {
                // If we only have a right child, we can just replace the node with the right child.
                let root_node = self.arena.remove(root_idx).unwrap();
                return (right, Some((root_node.value, root_node.priority)));
            }

            (true, false) => {
                // If we only have a left child, we can just replace the node with the left child.
                let root_node = self.arena.remove(root_idx).unwrap();
                return (left, Some((root_node.value, root_node.priority)));
            }

            (true, true) => {}
        }

        // We have both children.
        let left_node = self.arena.get(left.unwrap()).unwrap();
        let right_node = self.arena.get(right.unwrap()).unwrap();

        // Now we need to decide which child to rotate to fix the heap property.
        if left_node.priority < right_node.priority {
            let Some(root_idx) = self.rotate_right(root) else {
                unreachable!("We know that the left node and the root node exist.");
            };

            let root_node = self.arena.get(root_idx).unwrap();
            let (new_right, value_and_priority) = self.remove_key(root_node.right, key);
            let root_node = self.arena.get_mut(root_idx).unwrap();
            root_node.right = new_right;

            self.update_count(Some(root_idx));

            (Some(root_idx), value_and_priority)
        } else {
            let Some(root_idx) = self.rotate_left(root) else {
                unreachable!("We know that the right node and the root node exist.");
            };

            let root_node = self.arena.get(root_idx).unwrap();
            let (new_left, value_and_priority) = self.remove_key(root_node.left, key);
            let root_node = self.arena.get_mut(root_idx).unwrap();
            root_node.left = new_left;

            self.update_count(Some(root_idx));

            (Some(root_idx), value_and_priority)
        }
    }

    fn remove_by_index(
        &mut self,
        root: Option<Index>,
        k: usize,
    ) -> (Option<Index>, Option<(K, V, Priority)>) {
        let Some(root_idx) = root else {
            return (None, None);
        };

        let root_node = self.arena.get(root_idx).unwrap();

        let left_count = root_node
            .left
            .map_or(0, |l| self.arena.get(l).unwrap().count);

        match k.cmp(&left_count) {
            Ordering::Less => {
                // Element is in left subtree
                let (left, value_and_priority) = self.remove_by_index(root_node.left, k);
                let root_node = self.arena.get_mut(root_idx).unwrap();
                root_node.left = left;

                self.update_count(root);

                return (Some(root_idx), value_and_priority);
            }

            Ordering::Greater => {
                // Element is in right subtree
                let (right, value_and_priority) =
                    self.remove_by_index(root_node.right, k - left_count - 1);
                let root_node = self.arena.get_mut(root_idx).unwrap();
                root_node.right = right;

                self.update_count(root);

                return (Some(root_idx), value_and_priority);
            }

            Ordering::Equal => {}
        };

        // We found the kth element - it's the root
        let left = root_node.left;
        let right = root_node.right;

        match (left.is_some(), right.is_some()) {
            (false, false) => {
                // If we don't have any children, we can just remove the node
                let root_node = self.arena.remove(root_idx).unwrap();
                return (
                    None,
                    Some((root_node.key, root_node.value, root_node.priority)),
                );
            }

            (false, true) => {
                // If we only have the right child, replace with that child
                let root_node = self.arena.remove(root_idx).unwrap();
                return (
                    root_node.right,
                    Some((root_node.key, root_node.value, root_node.priority)),
                );
            }

            (true, false) => {
                let root_node = self.arena.remove(root_idx).unwrap();
                return (
                    root_node.left,
                    Some((root_node.key, root_node.value, root_node.priority)),
                );
            }

            (true, true) => {}
        }

        // We have both children - rotate based on priority
        let left_node = self.arena.get(left.unwrap()).unwrap();
        let right_node = self.arena.get(right.unwrap()).unwrap();

        if left_node.priority < right_node.priority {
            let new_root = self.rotate_left(Some(root_idx));
            let (new_root, result) = self.remove_by_index(new_root, k);
            self.update_count(Some(new_root.unwrap()));
            (new_root, result)
        } else {
            let new_root = self.rotate_right(Some(root_idx));
            let new_root_node = self.arena.get(new_root.unwrap()).unwrap();

            let new_left_count = new_root_node
                .left
                .map_or(0, |l| self.arena.get(l).unwrap().count);

            let (new_root, result) = self.remove_by_index(new_root, k - new_left_count - 1);
            self.update_count(Some(new_root.unwrap()));
            (new_root, result)
        }
    }

    fn rotate_right(&mut self, root: Option<Index>) -> Option<Index> {
        let root_idx = root?;
        let root_node = self.arena.get(root_idx).unwrap();
        let left_idx = root_node.left?;

        // Get the left child's right subtree
        let left_node = self.arena.get(left_idx).unwrap();
        let left_right = left_node.right;

        // Update the left child's right pointer to point to root
        let left_node = self.arena.get_mut(left_idx).unwrap();
        left_node.right = Some(root_idx);

        // Update root's left pointer to point to left child's right subtree
        let root_node = self.arena.get_mut(root_idx).unwrap();
        root_node.left = left_right;

        self.update_count(root);

        Some(left_idx)
    }

    fn rotate_left(&mut self, root: Option<Index>) -> Option<Index> {
        let root_idx = root?;
        let root_node = self.arena.get(root_idx).unwrap();
        let right_idx = root_node.right?;

        // Get the right child's left subtree
        let right_node = self.arena.get(right_idx).unwrap();
        let right_left = right_node.left;

        // Update the right child's left pointer to point to root
        let right_node = self.arena.get_mut(right_idx).unwrap();
        right_node.left = Some(root_idx);

        // Update root's right pointer to point to right child's left subtree
        let root_node = self.arena.get_mut(root_idx).unwrap();
        root_node.right = right_left;

        self.update_count(root);

        Some(right_idx)
    }
}

impl<K: Debug, V: Debug> std::fmt::Debug for CTNode<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CTNode {{ key: {:?}, value: {:?}, priority: {:?}, count: {:?}, left: {:?}, right: {:?} }}",
            self.key, self.value, self.priority, self.count, self.left, self.right
        )?;

        Ok(())
    }
}

impl<K: Debug, V: Debug> std::fmt::Debug for CartesianTree<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CartesianTree {{")?;

        if let Some(root) = self.root {
            writeln!(f, "  root: {:?}", self.arena.get(root).unwrap().key)?;

            for (_, node) in self.arena.iter() {
                writeln!(f, "   {:?}", node)?;
            }
        } else {
            writeln!(f, "[Empty]")?;
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Treap<K, V> {
    tree: CartesianTree<K, V>,
}

impl<K: Ord, V> Default for Treap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> Treap<K, V> {
    pub fn new() -> Self {
        Self {
            tree: CartesianTree::new(),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.tree.get(key).map(|(v, _)| v)
    }

    pub fn get_index(&self, k: usize) -> Option<(&K, &V)> {
        self.tree.get_index(k).map(|(key, value, _)| (key, value))
    }

    pub fn insert(&mut self, key: K, value: V) {
        let priority = thread_rng().gen();
        self.tree.insert(key, value, priority);
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.tree.remove(key).map(|(v, _)| v)
    }
}

impl<K: Ord, V> Deref for Treap<K, V> {
    type Target = CartesianTree<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl<K: Ord, V> DerefMut for Treap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_small_tree() {
        let mut tree = CartesianTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);

        tree.insert(1, 'a', 14);
        tree.insert(2, 'b', 2);
        tree.insert(5, 'c', 6);
        tree.insert(6, 'd', 15);
        tree.insert(10, 'e', 1);
        tree.insert(14, 'f', 14);
        tree.insert(18, 'g', 11);
        tree.insert(19, 'h', 2);

        assert_eq!(tree.len(), 8);

        assert_eq!(tree.get(&1), Some((&'a', 14)));
        assert_eq!(tree.get(&2), Some((&'b', 2)));
        assert_eq!(tree.get(&5), Some((&'c', 6)));
        assert_eq!(tree.get(&6), Some((&'d', 15)));
        assert_eq!(tree.get(&10), Some((&'e', 1)));
        assert_eq!(tree.get(&14), Some((&'f', 14)));
        assert_eq!(tree.get(&18), Some((&'g', 11)));
        assert_eq!(tree.get(&19), Some((&'h', 2)));

        assert_eq!(tree.get(&11), None);

        assert_eq!(tree.index(1), Some(0));
        assert_eq!(tree.index(2), Some(1));
        assert_eq!(tree.index(5), Some(2));
        assert_eq!(tree.index(6), Some(3));
        assert_eq!(tree.index(10), Some(4));
        assert_eq!(tree.index(14), Some(5));
        assert_eq!(tree.index(18), Some(6));
        assert_eq!(tree.index(19), Some(7));
        assert_eq!(tree.index(11), None);

        assert_eq!(tree.get_index(0), Some((&1, &'a', 14)));
        assert_eq!(tree.get_index(1), Some((&2, &'b', 2)));
        assert_eq!(tree.get_index(2), Some((&5, &'c', 6)));
        assert_eq!(tree.get_index(3), Some((&6, &'d', 15)));
        assert_eq!(tree.get_index(4), Some((&10, &'e', 1)));
        assert_eq!(tree.get_index(5), Some((&14, &'f', 14)));
        assert_eq!(tree.get_index(6), Some((&18, &'g', 11)));
        assert_eq!(tree.get_index(7), Some((&19, &'h', 2)));
        assert_eq!(tree.get_index(8), None);

        tree.remove(&1);
        assert_eq!(tree.get(&1), None);
        tree.remove(&10);
        assert_eq!(tree.get(&10), None);

        tree.insert(2, 'z', 99);

        // The priority of an existing key is not changed.
        assert_eq!(tree.get(&2), Some((&'z', 2)));

        assert_eq!(tree.len(), 6);
    }

    #[test]
    fn test_treap() {
        let mut treap = Treap::new();

        for i in 0..100 {
            treap.insert(i, i * i);
        }

        assert_eq!(treap.len(), 100);

        for i in 0..100 {
            assert_eq!(treap.get(&i), Some(&(i * i)));
        }

        for i in 0..100 {
            assert_eq!(treap.get_index(i), Some((&i, &(i * i))));
        }

        for i in 0..100 {
            assert_eq!(treap.index(i), Some(i));
        }

        assert_eq!(treap.remove(&3), Some(9));
        assert_eq!(treap.get(&3), None);
        assert_eq!(treap.get_index(3), Some((&4, &16)));
        assert_eq!(treap.len(), 99);

        for i in 0..10 {
            treap.remove_index(i);
        }

        assert_eq!(treap.len(), 89);
    }
}
