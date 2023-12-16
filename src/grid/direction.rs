#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn all() -> impl Iterator<Item = Self> {
        [Self::Up, Self::Right, Self::Down, Self::Left]
            .iter()
            .copied()
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

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
