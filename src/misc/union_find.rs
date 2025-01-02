use std::collections::HashSet;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DisjointSetIndex(usize);

impl From<DisjointSetIndex> for usize {
    fn from(value: DisjointSetIndex) -> Self {
        value.0
    }
}

impl UnionFind {
    /// Creates a new UnionFind data structure with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parents: Vec::with_capacity(capacity),
            sizes: Vec::with_capacity(capacity),
        }
    }

    /// Adds a singleton set to the data structure and returns the index of the
    /// set.
    pub fn add_set(&mut self) -> DisjointSetIndex {
        let index = self.parents.len();

        self.parents.push(index);
        self.sizes.push(1);

        DisjointSetIndex(index)
    }

    /// Adds `n` singleton sets to the data structure.
    pub fn extend(&mut self, n: usize) -> Vec<DisjointSetIndex> {
        let start = self.parents.len();

        self.parents.extend(start..(start + n));
        self.sizes.extend(std::iter::repeat(1).take(n));

        (start..start + n).map(DisjointSetIndex).collect()
    }

    /// Returns the size of the set that `x` belongs to.
    pub fn size_of_set(&mut self, x: DisjointSetIndex) -> Option<usize> {
        self.find(x).map(|r| self.sizes[r.0])
    }

    /// Returns the indices of all distinct sets.
    pub fn roots(&mut self) -> HashSet<DisjointSetIndex> {
        (0..self.parents.len())
            .map(DisjointSetIndex)
            .filter_map(|x| self.find(x))
            .collect()
    }

    /// Returns the index of the set that `x` belongs to.
    pub fn find(&mut self, x: DisjointSetIndex) -> Option<DisjointSetIndex> {
        if x.0 >= self.parents.len() {
            return None;
        }

        let mut x = x.0;

        while self.parents[x] != x {
            let new_x = self.parents[x];
            let new_parent_x = self.parents[new_x];

            let p_x = self.parents[x];
            self.parents[p_x] = new_parent_x;
            self.parents[x] = new_x;

            x = new_x;
        }

        Some(DisjointSetIndex(x))
    }

    /// Unions the sets that `x` and `y` belong to.
    pub fn union(&mut self, x: DisjointSetIndex, y: DisjointSetIndex) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root.is_none() || y_root.is_none() {
            panic!("Cannot union elements that are not in the data structure.");
        }

        if x_root == y_root {
            return;
        }

        let mut x_root = x_root.unwrap();
        let mut y_root = y_root.unwrap();

        if self.sizes[x_root.0] < self.sizes[y_root.0] {
            std::mem::swap(&mut x_root, &mut y_root);
        }

        self.parents[y_root.0] = x_root.0;
        self.sizes[x_root.0] += self.sizes[y_root.0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_find_test() {
        let mut uf = UnionFind::default();

        let a = uf.add_set();
        let b = uf.add_set();
        let c = uf.add_set();

        let def = uf.extend(3);

        let d = def[0];
        let e = def[1];
        let f = def[2];

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
