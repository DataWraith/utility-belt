/// An enum representing the four cardinal directions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    #[default]
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    /// Returns an iterator over all four directions
    pub fn all() -> impl Iterator<Item = Self> {
        [Self::Up, Self::Right, Self::Down, Self::Left].into_iter()
    }

    /// Returns the direction one would be facing after a turning left
    pub fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    /// Returns the direction one would be facing after a turning right
    pub fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    /// Returns the opposite direction
    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' | 'u' | 'N' | 'n' | '^' => Ok(Self::Up),
            'R' | 'r' | 'E' | 'e' | '>' => Ok(Self::Right),
            'D' | 'd' | 'S' | 's' | 'v' => Ok(Self::Down),
            'L' | 'l' | 'W' | 'w' | '<' => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 | b'U' | b'u' | b'N' | b'n' | b'^' => Ok(Self::Up),
            1 | b'R' | b'r' | b'E' | b'e' | b'>' => Ok(Self::Right),
            2 | b'D' | b'd' | b'S' | b's' | b'v' => Ok(Self::Down),
            3 | b'L' | b'l' | b'W' | b'w' | b'<' => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(c: usize) -> Result<Self, Self::Error> {
        TryFrom::try_from(c as u8)
    }
}

impl From<Direction> for char {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => 'U',
            Direction::Right => 'R',
            Direction::Down => 'D',
            Direction::Left => 'L',
        }
    }
}

impl From<Direction> for u8 {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

impl From<Direction> for usize {
    fn from(dir: Direction) -> Self {
        dir as usize
    }
}
