use crate::misc::MiniBitset;

use super::Direction;

/// A set of directions
#[derive(Clone, Copy)]
pub struct DirectionSet(MiniBitset<u8>);

impl DirectionSet {
    pub fn empty() -> Self {
        Self(MiniBitset::default())
    }

    pub fn cardinal() -> Self {
        Self(MiniBitset::new(0b00001111))
    }

    pub fn diagonal() -> Self {
        Self(MiniBitset::new(0b11110000))
    }

    pub fn all() -> Self {
        Self(MiniBitset::new(0b11111111))
    }

    pub fn insert(&mut self, dir: Direction) -> bool {
        self.0.insert(dir.into())
    }

    pub fn remove(&mut self, dir: Direction) {
        self.0.remove(dir.into())
    }

    pub fn contains(&self, dir: Direction) -> bool {
        self.0.contains(dir.into())
    }

    pub fn iter(&self) -> impl Iterator<Item = Direction> {
        self.0.iter().map(|i| (i as u8).try_into().unwrap())
    }
}

impl From<Direction> for DirectionSet {
    fn from(dir: Direction) -> Self {
        let mut set = Self::empty();
        set.insert(dir);
        set
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
    fn test_cardinal() {
        let set = DirectionSet::cardinal();

        assert!(set.contains(Direction::Up));
        assert!(set.contains(Direction::Right));
        assert!(set.contains(Direction::Down));
        assert!(set.contains(Direction::Left));
    }

    #[test]
    fn test_diagonal() {
        let set = DirectionSet::diagonal();

        assert!(set.contains(Direction::UpLeft));
        assert!(set.contains(Direction::UpRight));
        assert!(set.contains(Direction::DownLeft));
        assert!(set.contains(Direction::DownRight));
    }

    #[test]
    fn test_all() {
        let set = DirectionSet::all();

        assert!(set.contains(Direction::Up));
        assert!(set.contains(Direction::Right));
        assert!(set.contains(Direction::Down));
        assert!(set.contains(Direction::Left));
        assert!(set.contains(Direction::UpLeft));
        assert!(set.contains(Direction::UpRight));
        assert!(set.contains(Direction::DownLeft));
        assert!(set.contains(Direction::DownRight));
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
        let mut set = DirectionSet::cardinal();

        set.remove(Direction::Up);
        set.remove(Direction::Right);
        set.remove(Direction::Down);
        set.remove(Direction::Left);

        assert!(!set.contains(Direction::Up));
        assert!(!set.contains(Direction::Right));
        assert!(!set.contains(Direction::Down));
        assert!(!set.contains(Direction::Left));
    }

    #[test]
    fn test_from() {
        let set = DirectionSet::from(Direction::Up);

        assert!(set.contains(Direction::Up));
        assert!(!set.contains(Direction::Right));
        assert!(!set.contains(Direction::Down));
        assert!(!set.contains(Direction::Left));
    }
}
