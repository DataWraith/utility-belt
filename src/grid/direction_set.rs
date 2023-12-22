use super::Direction;

/// A set of directions
pub struct DirectionSet(u8);

impl DirectionSet {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn all() -> Self {
        Self(0b1111)
    }

    pub fn insert(&mut self, dir: Direction) {
        self.0 |= 1 << dir as u8;
    }

    pub fn remove(&mut self, dir: Direction) {
        self.0 &= !(1 << dir as u8);
    }

    pub fn contains(&self, dir: Direction) -> bool {
        self.0 & (1 << dir as u8) != 0
    }

    pub fn iter(&self) -> impl Iterator<Item = Direction> + '_ {
        Direction::all().filter(move |dir| self.contains(*dir))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let set = DirectionSet::empty();

        assert!(!set.contains(Direction::Up));
        assert!(!set.contains(Direction::Right));
        assert!(!set.contains(Direction::Down));
        assert!(!set.contains(Direction::Left));
    }

    #[test]
    fn test_all() {
        let set = DirectionSet::all();

        assert!(set.contains(Direction::Up));
        assert!(set.contains(Direction::Right));
        assert!(set.contains(Direction::Down));
        assert!(set.contains(Direction::Left));
    }

    #[test]
    fn test_insert() {
        let mut set = DirectionSet::empty();

        set.insert(Direction::Up);
        set.insert(Direction::Right);
        set.insert(Direction::Down);
        set.insert(Direction::Left);

        assert!(set.contains(Direction::Up));
        assert!(set.contains(Direction::Right));
        assert!(set.contains(Direction::Down));
        assert!(set.contains(Direction::Left));
    }

    #[test]
    fn test_remove() {
        let mut set = DirectionSet::empty();

        set.insert(Direction::Up);
        set.insert(Direction::Right);
        set.insert(Direction::Down);
        set.insert(Direction::Left);

        set.remove(Direction::Up);
        set.remove(Direction::Right);
        set.remove(Direction::Down);
        set.remove(Direction::Left);

        assert!(!set.contains(Direction::Up));
        assert!(!set.contains(Direction::Right));
        assert!(!set.contains(Direction::Down));
        assert!(!set.contains(Direction::Left));
    }
}
