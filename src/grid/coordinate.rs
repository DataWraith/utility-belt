use std::fmt::{Display, Formatter};

use derive_more::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use glam::IVec2;

use super::Direction;

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
)]
pub struct Coordinate(IVec2);

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0.x, self.0.y)
    }
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }

    pub fn x(self) -> i32 {
        self.0.x
    }

    pub fn y(self) -> i32 {
        self.0.y
    }

    fn rotate_right(self) -> Self {
        Self::new(self.y(), -self.x())
    }

    fn rotate_left(self) -> Self {
        Self::new(-self.y(), self.x())
    }

    fn rotate_180(self) -> Self {
        Self::new(-self.x(), -self.y())
    }

    fn mirror_x(self) -> Self {
        Self::new(-self.x(), self.y())
    }

    fn mirror_x_wrap(self, width: i32) -> Self {
        Self::new(width - self.x(), self.y())
    }

    fn mirror_y(self) -> Self {
        Self::new(self.x(), -self.y())
    }

    fn miror_y_wrap(self, height: i32) -> Self {
        Self::new(self.x(), height - self.y())
    }

    pub fn neighbor(self, dir: Direction) -> Self {
        self + dir.into()
    }

    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        Direction::all().map(move |dir| self + dir.into())
    }

    pub fn von_neumann_neighbors(self) -> impl Iterator<Item = Self> {
        self.neighbors()
    }

    pub fn moore_neighbors(self) -> impl Iterator<Item = Self> {
        [
            self + Direction::Up.into(),
            self + Direction::Right.into(),
            self + Direction::Down.into(),
            self + Direction::Left.into(),
            self + Direction::Up.into() + Direction::Right.into(),
            self + Direction::Up.into() + Direction::Left.into(),
            self + Direction::Down.into() + Direction::Right.into(),
            self + Direction::Down.into() + Direction::Left.into(),
        ]
        .into_iter()
    }

    pub fn knight_move_neighbors(self) -> impl Iterator<Item = Self> {
        [
            self + Direction::Up.into() + Direction::Up.into() + Direction::Right.into(),
            self + Direction::Up.into() + Direction::Up.into() + Direction::Left.into(),
            self + Direction::Right.into() + Direction::Right.into() + Direction::Up.into(),
            self + Direction::Right.into() + Direction::Right.into() + Direction::Down.into(),
            self + Direction::Down.into() + Direction::Down.into() + Direction::Right.into(),
            self + Direction::Down.into() + Direction::Down.into() + Direction::Left.into(),
            self + Direction::Left.into() + Direction::Left.into() + Direction::Up.into(),
            self + Direction::Left.into() + Direction::Left.into() + Direction::Down.into(),
        ]
        .into_iter()
    }

    pub fn adjacent(self, other: Self) -> bool {
        self.manhattan_distance(other) == 1
    }

    pub fn manhattan_distance(self, other: Self) -> i32 {
        (self.x().abs_diff(other.x()) + self.y().abs_diff(other.y())) as i32
    }

    pub fn towards(self, other: Self) -> Direction {
        if other.x() < self.x() {
            Direction::Left
        } else if other.x() > self.x() {
            Direction::Right
        } else if other.y() < self.y() {
            Direction::Up
        } else {
            Direction::Down
        }
    }
}

impl From<IVec2> for Coordinate {
    fn from(vec: IVec2) -> Self {
        Self(vec)
    }
}

impl From<Direction> for Coordinate {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::new(0, -1),
            Direction::Right => Self::new(1, 0),
            Direction::Down => Self::new(0, 1),
            Direction::Left => Self::new(-1, 0),
        }
    }
}
