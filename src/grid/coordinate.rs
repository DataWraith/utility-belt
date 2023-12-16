use std::fmt::{Display, Formatter};

use derive_more::{Add, AddAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use glam::IVec2;

use super::Direction;

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Rem,
    RemAssign,
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

    pub fn rotate_right(self) -> Self {
        Self::new(self.y(), -self.x())
    }

    pub fn rotate_left(self) -> Self {
        Self::new(-self.y(), self.x())
    }

    pub fn rotate_180(self) -> Self {
        Self::new(-self.x(), -self.y())
    }

    pub fn mirror_x(self) -> Self {
        Self::new(-self.x(), self.y())
    }

    pub fn mirror_x_wrap(self, width: i32) -> Self {
        Self::new(width - 1 - self.x(), self.y())
    }

    pub fn mirror_y(self) -> Self {
        Self::new(self.x(), -self.y())
    }

    pub fn mirror_y_wrap(self, height: i32) -> Self {
        Self::new(self.x(), height - 1 - self.y())
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
            self + Direction::Up.into() + Direction::Right.into(),
            self + Direction::Right.into(),
            self + Direction::Down.into() + Direction::Right.into(),
            self + Direction::Down.into(),
            self + Direction::Down.into() + Direction::Left.into(),
            self + Direction::Left.into(),
            self + Direction::Up.into() + Direction::Left.into(),
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

impl From<(i32, i32)> for Coordinate {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

// TODO: Impl Display

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_rotate_right() {
        assert_eq!(Coordinate::new(1, 2).rotate_right(), Coordinate::new(2, -1));
    }

    #[test]
    fn test_rotate_left() {
        assert_eq!(Coordinate::new(1, 2).rotate_left(), Coordinate::new(-2, 1));
    }

    #[test]
    fn test_rotate_180() {
        assert_eq!(Coordinate::new(1, 2).rotate_180(), Coordinate::new(-1, -2));
    }

    #[test]
    fn test_mirror_x() {
        assert_eq!(Coordinate::new(1, 2).mirror_x(), Coordinate::new(-1, 2));
    }

    #[rstest]
    // 0 1 2 -> 2 1 0
    #[case((0, 0), (2, 0))]
    #[case((1, 0), (1, 0))]
    #[case((2, 0), (0, 0))]
    fn test_mirror_x_wrap(#[case] input: (i32, i32), #[case] expected: (i32, i32)) {
        assert_eq!(
            Coordinate::from(input).mirror_x_wrap(3),
            Coordinate::from(expected)
        );
    }

    #[test]
    fn test_mirror_y() {
        assert_eq!(Coordinate::new(1, 2).mirror_y(), Coordinate::new(1, -2));
    }

    #[rstest]
    // 0    2
    // 1 -> 1
    // 2    0
    #[case((0, 0), (0, 2))]
    #[case((0, 1), (0, 1))]
    #[case((0, 2), (0, 0))]
    fn test_mirror_y_wrap(#[case] input: (i32, i32), #[case] expected: (i32, i32)) {
        assert_eq!(
            Coordinate::from(input).mirror_y_wrap(3),
            Coordinate::from(expected)
        );
    }

    #[test]
    fn test_neighbor() {
        assert_eq!(
            Coordinate::new(1, 2).neighbor(Direction::Up),
            Coordinate::new(1, 1)
        );
        assert_eq!(
            Coordinate::new(1, 2).neighbor(Direction::Right),
            Coordinate::new(2, 2)
        );
        assert_eq!(
            Coordinate::new(1, 2).neighbor(Direction::Down),
            Coordinate::new(1, 3)
        );
        assert_eq!(
            Coordinate::new(1, 2).neighbor(Direction::Left),
            Coordinate::new(0, 2)
        );
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(
            Coordinate::new(1, 2).neighbors().collect::<Vec<_>>(),
            vec![
                Coordinate::new(1, 1),
                Coordinate::new(2, 2),
                Coordinate::new(1, 3),
                Coordinate::new(0, 2),
            ]
        );
    }

    #[test]
    fn test_von_neumann_neighbors() {
        assert_eq!(
            Coordinate::new(1, 2)
                .von_neumann_neighbors()
                .collect::<Vec<_>>(),
            vec![
                Coordinate::new(1, 1),
                Coordinate::new(2, 2),
                Coordinate::new(1, 3),
                Coordinate::new(0, 2),
            ]
        );
    }

    #[test]
    fn test_moore_neighbors() {
        assert_eq!(
            Coordinate::new(1, 1).moore_neighbors().collect::<Vec<_>>(),
            vec![
                Coordinate::new(1, 0),
                Coordinate::new(2, 0),
                Coordinate::new(2, 1),
                Coordinate::new(2, 2),
                Coordinate::new(1, 2),
                Coordinate::new(0, 2),
                Coordinate::new(0, 1),
                Coordinate::new(0, 0),
            ]
        );
    }

    #[test]
    fn test_knight_move_neighbors() {
        assert_eq!(
            Coordinate::new(0, 0)
                .knight_move_neighbors()
                .collect::<Vec<_>>(),
            vec![
                Coordinate::new(1, -2),
                Coordinate::new(-1, -2),
                Coordinate::new(2, -1),
                Coordinate::new(2, 1),
                Coordinate::new(1, 2),
                Coordinate::new(-1, 2),
                Coordinate::new(-2, -1),
                Coordinate::new(-2, 1),
            ]
        );
    }
}
