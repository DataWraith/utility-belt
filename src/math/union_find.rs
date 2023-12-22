use std::hash::Hash;

use ahash::AHashMap as HashMap;

/// Union-find data structure, also known as a disjoint-set data structure.
///
/// This data structure allows you to keep track of disjoint sets of elements,
/// and efficiently determine if two elements are in the same set, and to
/// efficiently merge two sets.
///
/// This is useful, for example, if you want to find connected components of a
/// graph.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Disjoint-set_data_structure) for more information.
///
#[derive(Default)]
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn make_set(&mut self) -> usize {
        let index = self.parents.len();

        self.parents.push(index);
        self.sizes.push(1);

        index
    }

    pub fn size_of_set(&mut self, x: usize) -> Option<usize> {
        let r = self.find(x)?;
        Some(self.sizes[r])
    }

    pub fn find(&mut self, x: usize) -> Option<usize> {
        if x >= self.parents.len() {
            return None;
        }

        let mut x = x;

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

    pub fn union(&mut self, x: usize, y: usize) -> Result<(), &str> {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root.is_none() || y_root.is_none() {
            return Err("Cannot union elements that are not in the data structure.");
        }

        if x_root == y_root {
            return Ok(());
        }

        let mut x_root = x_root.unwrap();
        let mut y_root = y_root.unwrap();

        if self.sizes[x_root] < self.sizes[y_root] {
            std::mem::swap(&mut x_root, &mut y_root);
        }

        self.parents[y_root] = x_root;
        self.sizes[x_root] += self.sizes[y_root];

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_find_test() {
        let mut uf = UnionFind::default();

        let a = uf.make_set();
        let b = uf.make_set();
        let c = uf.make_set();
        let d = uf.make_set();
        let e = uf.make_set();
        let f = uf.make_set();

        let _ = uf.union(a, b);
        let _ = uf.union(b, c);
        let _ = uf.union(d, e);

        assert_eq!(uf.find(a), uf.find(c));
        assert_eq!(uf.find(a), uf.find(b));
        assert_eq!(uf.find(b), uf.find(c));

        assert_eq!(uf.find(d), uf.find(e));

        assert_ne!(uf.find(a), uf.find(d));
        assert_ne!(uf.find(a), uf.find(e));
        assert_ne!(uf.find(a), uf.find(f));
        assert_ne!(uf.find(b), uf.find(d));
        assert_ne!(uf.find(b), uf.find(e));
        assert_ne!(uf.find(b), uf.find(f));
        assert_ne!(uf.find(c), uf.find(d));
        assert_ne!(uf.find(c), uf.find(e));
        assert_ne!(uf.find(c), uf.find(f));
        assert_ne!(uf.find(d), uf.find(f));
        assert_ne!(uf.find(e), uf.find(f));

        assert_eq!(uf.size_of_set(a), Some(3));
        let _ = uf.union(a, d);
        assert_eq!(uf.size_of_set(e), Some(5));
    }
}
