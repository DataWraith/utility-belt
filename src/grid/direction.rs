/// An enum representing the eight directions in a grid
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    #[default]
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
    UpLeft = 4,
    UpRight = 5,
    DownLeft = 6,
    DownRight = 7,
}

impl Direction {
    /// Returns an iterator over the four cardinal directions
    pub fn cardinal() -> impl Iterator<Item = Self> {
        [Self::Up, Self::Right, Self::Down, Self::Left].into_iter()
    }

    /// Returns an iterator over the four diagonal directions
    pub fn diagonal() -> impl Iterator<Item = Self> {
        [Self::UpLeft, Self::UpRight, Self::DownLeft, Self::DownRight].into_iter()
    }

    /// Returns an iterator over all eight directions
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Up,
            Self::Right,
            Self::Down,
            Self::Left,
            Self::UpLeft,
            Self::UpRight,
            Self::DownLeft,
            Self::DownRight,
        ]
        .into_iter()
    }

    /// Returns the direction one would be facing after a turning left by 90 degrees
    pub fn turn_left_90(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,

            // This isn't really well-defined, but follows the pattern of the
            // other directions.
            Self::UpRight => Self::UpLeft,
            Self::UpLeft => Self::DownLeft,
            Self::DownLeft => Self::DownRight,
            Self::DownRight => Self::UpRight,
        }
    }

    /// Returns the direction one would be facing after turning left by 45 degrees
    pub fn turn_left_45(self) -> Self {
        match self {
            Self::Up => Self::UpLeft,
            Self::UpLeft => Self::Left,
            Self::Left => Self::DownLeft,
            Self::DownLeft => Self::Down,
            Self::Down => Self::DownRight,
            Self::DownRight => Self::Right,
            Self::Right => Self::UpRight,
            Self::UpRight => Self::Up,
        }
    }

    /// Returns the direction one would be facing after a turning right by 90 degrees
    pub fn turn_right_90(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,

            Self::UpRight => Self::DownRight,
            Self::DownRight => Self::DownLeft,
            Self::DownLeft => Self::UpLeft,
            Self::UpLeft => Self::UpRight,
        }
    }

    /// Returns the direction one would be facing after turning right by 45 degrees
    pub fn turn_right_45(self) -> Self {
        match self {
            Self::Up => Self::UpRight,
            Self::UpRight => Self::Right,
            Self::Right => Self::DownRight,
            Self::DownRight => Self::Down,
            Self::Down => Self::DownLeft,
            Self::DownLeft => Self::Left,
            Self::Left => Self::UpLeft,
            Self::UpLeft => Self::Up,
        }
    }

    /// Returns the opposite direction
    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,

            Self::UpRight => Self::DownLeft,
            Self::DownLeft => Self::UpRight,
            Self::DownRight => Self::UpLeft,
            Self::UpLeft => Self::DownRight,
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
            4 => Ok(Self::UpLeft),
            5 => Ok(Self::UpRight),
            6 => Ok(Self::DownLeft),
            7 => Ok(Self::DownRight),
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

impl TryFrom<Direction> for char {
    type Error = ();

    fn try_from(dir: Direction) -> Result<Self, Self::Error> {
        match dir {
            Direction::Up => Ok('U'),
            Direction::Right => Ok('R'),
            Direction::Down => Ok('D'),
            Direction::Left => Ok('L'),
            _ => Err(()),
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
            Direction::UpLeft => 4,
            Direction::UpRight => 5,
            Direction::DownLeft => 6,
            Direction::DownRight => 7,
        }
    }
}

impl From<Direction> for usize {
    fn from(dir: Direction) -> Self {
        dir as usize
    }
}
