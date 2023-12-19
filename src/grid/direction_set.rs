use super::Direction;

/// A set of directions
pub struct DirectionSet(u8);

impl DirectionSet {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn insert(&mut self, dir: Direction) {
        self.0 |= 1 << dir as u8;
    }

    pub fn contains(&self, dir: Direction) -> bool {
        self.0 & (1 << dir as u8) != 0
    }

    pub fn iter(&self) -> impl Iterator<Item = Direction> + '_ {
        Direction::all().filter(move |dir| self.contains(*dir))
    }
}
