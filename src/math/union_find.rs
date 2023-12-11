use std::hash::Hash;

use ahash::AHashMap as HashMap;

#[derive(Default)]
pub struct UnionFind<T: Hash + Eq> {
    indices: HashMap<T, usize>,
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl<T: Hash + Eq> UnionFind<T> {
    pub fn make_set(&mut self, x: T) -> usize {
        let index = self.parents.len();

        self.parents.push(index);
        self.ranks.push(0);
        self.indices.insert(x, index);

        index
    }

    pub fn find(&mut self, x: T) -> Option<usize> {
        let mut x = *self.indices.get(&x)?;

        while self.parents[x] != x {
            let new_x = self.parents[x];
            let new_parent_x = self.parents[new_x];

            let p_x = self.parents[x];
            self.parents[p_x] = new_parent_x;
            self.parents[x] = new_x;

            x = new_x;
        }

        Some(x)
    }

    pub fn union(&mut self, x: T, y: T) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root.is_none() || y_root.is_none() {
            // TODO: Make an error enum or something
            panic!("Cannot union elements that are not in the set");
        }

        if x_root == y_root {
            return;
        }

        let mut x_root = x_root.unwrap();
        let mut y_root = y_root.unwrap();

        if self.ranks[x_root] < self.ranks[y_root] {
            std::mem::swap(&mut x_root, &mut y_root);
        }

        self.parents[y_root] = x_root;

        if self.ranks[x_root] == self.ranks[y_root] {
            self.ranks[x_root] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_find_test() {
        let mut uf = UnionFind::default();

        let a = uf.make_set('a');
        let b = uf.make_set('b');
        let c = uf.make_set('c');
        let d = uf.make_set('d');
        let e = uf.make_set('e');
        let f = uf.make_set('f');

        uf.union('a', 'b');
        uf.union('b', 'c');
        uf.union('d', 'e');

        assert_eq!(uf.find('a'), uf.find('c'));
        assert_eq!(uf.find('a'), uf.find('b'));
        assert_eq!(uf.find('b'), uf.find('c'));

        assert_eq!(uf.find('d'), uf.find('e'));

        assert_ne!(uf.find('a'), uf.find('d'));
        assert_ne!(uf.find('a'), uf.find('e'));
        assert_ne!(uf.find('a'), uf.find('f'));
        assert_ne!(uf.find('b'), uf.find('d'));
        assert_ne!(uf.find('b'), uf.find('e'));
        assert_ne!(uf.find('b'), uf.find('f'));
        assert_ne!(uf.find('c'), uf.find('d'));
        assert_ne!(uf.find('c'), uf.find('e'));
        assert_ne!(uf.find('c'), uf.find('f'));
        assert_ne!(uf.find('d'), uf.find('f'));
        assert_ne!(uf.find('e'), uf.find('f'));
    }
}
