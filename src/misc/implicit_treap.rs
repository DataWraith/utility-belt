use rand::prelude::*;
use thunderdome::{Arena, Index};

struct Node<T> {
    value: T,
    priority: u64,
    size: usize,
    left: Option<Index>,
    right: Option<Index>,
}

impl<T> Node<T> {
    fn new(value: T, priority: u64) -> Self {
        Self {
            value,
            priority,
            size: 1,
            left: None,
            right: None,
        }
    }
}

/// An implicit treap is a data structure that can represent a sequence that
/// supports insertions and removals at any position in O(log n) time.
///
/// Since it relies on randomness for this property, you need to provide a
/// random number generator for its operations.
pub struct ImplicitTreap<T> {
    arena: Arena<Node<T>>,
    root: Option<Index>,
}

impl<T> ImplicitTreap<T> {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            root: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn len(&self) -> usize {
        self.size(self.root)
    }

    pub fn push_back(&mut self, value: T, rng: impl Rng) {
        self.insert(self.len(), value, rng);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.remove(self.len() - 1)
    }

    pub fn push_front(&mut self, value: T, rng: impl Rng) {
        self.insert(0, value, rng);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.remove(0)
    }

    pub fn insert(&mut self, pos: usize, value: T, mut rng: impl Rng) -> Index {
        let priority = rng.gen();
        let node = self.arena.insert(Node::new(value, priority));

        let (l, r) = if pos == 0 {
            (None, self.root)
        } else {
            self.split(self.root, pos - 1, 0)
        };

        let left_treap = self.merge(l, Some(node));
        let result = self.merge(left_treap, r);

        self.root = result;

        result.unwrap()
    }

    pub fn remove(&mut self, pos: usize) -> Option<T> {
        let (l, r1) = if pos == 0 {
            (None, self.root)
        } else {
            self.split(self.root, pos - 1, 0)
        };

        let (m, r2) = self.split(r1, 0, 0);

        self.root = self.merge(l, r2);

        m.and_then(|node| self.arena.remove(node))
            .map(|node| node.value)
    }

    fn size(&self, node: Option<Index>) -> usize {
        if let Some(node) = node {
            self.arena[node].size
        } else {
            0
        }
    }

    fn find(&self, t: Option<Index>, pos: usize) -> Option<Index> {
        let Some(t) = t else {
            return None;
        };

        let current_pos = self.size(self.arena[t].left) + 1;

        if pos < current_pos {
            self.find(self.arena[t].left, pos)
        } else if pos > current_pos {
            self.find(self.arena[t].right, pos - current_pos)
        } else {
            Some(t)
        }
    }

    fn split(
        &mut self,
        t: Option<Index>,
        pos: usize,
        add: usize,
    ) -> (Option<Index>, Option<Index>) {
        let Some(t) = t else {
            return (None, None);
        };

        let current_pos = self.size(self.arena[t].left) + add;

        let (l, r) = if current_pos <= pos {
            let (new_left, new_right) = self.split(self.arena[t].right, pos, current_pos + 1);
            self.arena[t].right = new_left;

            (Some(t), new_right)
        } else {
            let (new_left, new_right) = self.split(self.arena[t].left, pos, add);
            self.arena[t].left = new_right;

            (new_left, Some(t))
        };

        self.update_size(Some(t));

        (l, r)
    }

    fn merge(&mut self, l: Option<Index>, r: Option<Index>) -> Option<Index> {
        if l.is_none() || r.is_none() {
            return l.or(r);
        }

        let (l, r) = (l.unwrap(), r.unwrap());

        let merged = if self.arena[l].priority > self.arena[r].priority {
            let new_l = self.merge(self.arena[l].right, Some(r));
            self.arena[l].right = new_l;
            l
        } else {
            let new_r = self.merge(Some(l), self.arena[r].left);
            self.arena[r].left = new_r;
            r
        };

        self.update_size(Some(merged));

        Some(merged)
    }

    fn update_size(&mut self, node: Option<Index>) {
        if let Some(node) = node {
            let size = self.size(self.arena[node].left) + 1 + self.size(self.arena[node].right);
            self.arena[node].size = size;
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for ImplicitTreap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "[]");
        }

        write!(f, "[")?;
        for i in 0..(self.len() - 1) {
            write!(f, "{:?}, ", self[i])?;
        }
        write!(f, "{:?}]", self[self.len() - 1])
    }
}

impl<T> std::ops::Index<usize> for ImplicitTreap<T> {
    type Output = T;

    fn index(&self, pos: usize) -> &T {
        self.find(self.root, pos + 1)
            .map(|node| &self.arena[node].value)
            .expect("index out of bounds")
    }
}

impl<T> std::ops::IndexMut<usize> for ImplicitTreap<T> {
    fn index_mut(&mut self, pos: usize) -> &mut T {
        self.find(self.root, pos + 1)
            .map(|node| &mut self.arena[node].value)
            .expect("index out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_rng() -> StdRng {
        StdRng::seed_from_u64(123)
    }

    #[test]
    fn test_empty_treap() {
        let treap = ImplicitTreap::<i32>::new();
        assert_eq!(treap.len(), 0);
        assert!(treap.is_empty());
    }

    #[test]
    fn test_push_back_and_indexing() {
        let mut treap = ImplicitTreap::new();
        let mut rng = new_rng();

        treap.push_back(1, &mut rng);
        treap.push_back(2, &mut rng);
        treap.push_back(3, &mut rng);

        assert_eq!(treap.len(), 3);
        assert!(!treap.is_empty());

        assert_eq!(treap[0], 1);
        assert_eq!(treap[1], 2);
        assert_eq!(treap[2], 3);
    }

    #[test]
    fn test_pop_operations() {
        let mut treap = ImplicitTreap::new();
        let mut rng = new_rng();

        treap.push_back(1, &mut rng);
        treap.push_back(2, &mut rng);
        treap.push_back(3, &mut rng);

        assert_eq!(treap.pop_back(), Some(3));
        assert_eq!(treap.len(), 2);
        assert_eq!(treap.pop_front(), Some(1));
        assert_eq!(treap.len(), 1);
        assert_eq!(treap.pop_back(), Some(2));
        assert_eq!(treap.len(), 0);
    }

    #[test]
    fn test_insert_operations() {
        let mut treap = ImplicitTreap::new();
        let mut rng = new_rng();

        treap.insert(0, 17, &mut rng);
        treap.insert(1, 18, &mut rng);
        treap.insert(2, 19, &mut rng);
        treap.insert(1, 20, &mut rng);

        assert_eq!(treap.len(), 4);
        assert_eq!(treap[0], 17);
        assert_eq!(treap[1], 20);
        assert_eq!(treap[2], 18);
        assert_eq!(treap[3], 19);
    }

    #[test]
    fn test_remove_operations() {
        let mut treap = ImplicitTreap::new();
        let mut rng = new_rng();

        // Setup initial state
        treap.insert(0, 17, &mut rng);
        treap.insert(1, 20, &mut rng);
        treap.insert(2, 18, &mut rng);
        treap.insert(3, 19, &mut rng);

        // Test removing from different positions
        assert_eq!(treap.remove(1138), None);
        assert_eq!(treap.remove(0), Some(17));
        assert_eq!(treap.len(), 3);
        assert_eq!(treap[0], 20);

        assert_eq!(treap.remove(1), Some(18));
        assert_eq!(treap.len(), 2);
        assert_eq!(treap[0], 20);
        assert_eq!(treap[1], 19);

        treap.remove(1);
        assert_eq!(treap.len(), 1);
        assert_eq!(treap[0], 20);

        treap.remove(0);
        assert_eq!(treap.len(), 0);
    }

    #[test]
    fn test_empty_treap_remove() {
        let mut treap = ImplicitTreap::<i32>::new();
        assert_eq!(treap.remove(0), None);
        assert_eq!(treap.remove(11), None);
    }

    #[test]
    fn test_modification() {
        let mut treap = ImplicitTreap::new();
        let mut rng = new_rng();

        treap.insert(0, 17, &mut rng);
        treap[0] = 18;
        assert_eq!(treap[0], 18);
    }
}
