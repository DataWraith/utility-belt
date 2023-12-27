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

impl std::fmt::Debug for DirectionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        if self.0 == 0 {
            return write!(f, "∅}}");
        }

        if self.contains(Direction::Up) {
            write!(f, "^")?;
        } else {
            write!(f, " ")?;
        }

        if self.contains(Direction::Right) {
            write!(f, ">")?;
        } else {
            write!(f, " ")?;
        }

        if self.contains(Direction::Down) {
            write!(f, "v")?;
        } else {
            write!(f, " ")?;
        }

        if self.contains(Direction::Left) {
            write!(f, "<")?;
        } else {
            write!(f, " ")?;
        }

        write!(f, "}}")
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

    #[test]
    fn test_debug() {
        let mut set = DirectionSet::empty();

        assert_eq!(format!("{:?}", set), "{∅}");

        set.insert(Direction::Up);
        assert_eq!(format!("{:?}", set), "{^   }");

        set.insert(Direction::Right);
        assert_eq!(format!("{:?}", set), "{^>  }");

        set.insert(Direction::Down);
        assert_eq!(format!("{:?}", set), "{^>v }");

        set.insert(Direction::Left);
        assert_eq!(format!("{:?}", set), "{^>v<}");

        set.remove(Direction::Up);
        assert_eq!(format!("{:?}", set), "{ >v<}");
    }
}
