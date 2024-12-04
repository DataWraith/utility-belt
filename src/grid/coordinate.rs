use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

use derive_more::{Add, AddAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use glam::IVec2;

use super::Direction;

/// A coordinate in a 2D grid.
///
/// This is a wrapper around `IVec2` that implements some useful methods.
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

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.y.cmp(&other.0.y).then(self.0.x.cmp(&other.0.x))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Deref for Coordinate {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }

    /// Rotate the coordinate 90 degrees clockwise. The anchor point is at the bottom left.
    pub fn rotate_right(self) -> Self {
        Self::new(self.y, -self.x)
    }

    /// Rotate the coordinate 90 degrees counter-clockwise. The anchor point is at the bottom left.
    pub fn rotate_left(self) -> Self {
        Self::new(-self.y, self.x)
    }

    /// Rotate the coordinate 180 degrees. The anchor point is at the bottom left.
    pub fn rotate_180(self) -> Self {
        Self::new(-self.x, -self.y)
    }

    /// Mirror the coordinate along the X axis. The anchor point is at the bottom left.
    pub fn mirror_x(self) -> Self {
        Self::new(-self.x, self.y)
    }

    /// Mirror the coordinate along the X axis, wrapping it so that it stays within the given width.
    pub fn mirror_x_wrap(self, width: i32) -> Self {
        Self::new(width - 1 - self.x, self.y)
    }

    /// Mirror the coordinate along the Y axis.
    pub fn mirror_y(self) -> Self {
        Self::new(self.x, -self.y)
    }

    /// Mirror the coordinate along the Y axis, wrapping it so that it stays within the given height.
    pub fn mirror_y_wrap(self, height: i32) -> Self {
        Self::new(self.x, height - 1 - self.y)
    }

    /// Returns neighboring Coordinate in the given direction
    pub fn neighbor(self, dir: Direction) -> Self {
        self + dir.into()
    }

    /// Return a list of all neighboring coordinates (alias of `von_neumann_neighbors`)
    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        Direction::all().map(move |dir| self + dir.into())
    }

    /// Return a list of all neighboring coordinates (von Neumann Neighborhood)
    pub fn von_neumann_neighbors(self) -> impl Iterator<Item = Self> {
        self.neighbors()
    }

    /// Return a list of all neighboring coordinates (Moore Neighborhood)
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

    /// Return a list of all coordinates reachable from self by a knight's move
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

    /// Returns whether the two coordinates are adjacent
    pub fn adjacent(self, other: Self) -> bool {
        self.manhattan_distance(other) == 1
    }

    /// Returns the Manhattan distance between the two coordinates
    pub fn manhattan_distance(self, other: Self) -> i32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i32
    }

    /// Returns the direction from self towards other
    pub fn towards(self, other: Self) -> Direction {
        if other.x < self.x {
            Direction::Left
        } else if other.x > self.x {
            Direction::Right
        } else if other.y < self.y {
            Direction::Up
        } else {
            Direction::Down
        }
    }

    /// Returns the Z-order curve value of the coordinate
    pub fn z_index(self) -> u64 {
        fn spread(x: u32) -> u64 {
            let mut x = x as u64;
            x = (x | (x << 16)) & 0x0000ffff0000ffff;
            x = (x | (x << 8)) & 0x00ff00ff00ff00ff;
            x = (x | (x << 4)) & 0x0f0f0f0f0f0f0f0f;
            x = (x | (x << 2)) & 0x3333333333333333;
            x = (x | (x << 1)) & 0x5555555555555555;
            x
        }

        let unsigned_x = self.x as u32;
        let unsigned_y = self.y as u32;

        spread(unsigned_x) | (spread(unsigned_y) << 1)
    }

    /// Converts a Z-order curve index into to x and y coordinates
    pub fn from_z_index(z_index: u64) -> Self {
        fn compact(x: u64) -> u32 {
            let mut x = x & 0x5555555555555555;
            x = (x | (x >> 1)) & 0x3333333333333333;
            x = (x | (x >> 2)) & 0x0f0f0f0f0f0f0f0f;
            x = (x | (x >> 4)) & 0x00ff00ff00ff00ff;
            x = (x | (x >> 8)) & 0x0000ffff0000ffff;
            x = (x | (x >> 16)) & 0x00000000ffffffff;
            x as u32
        }

        let x = compact(z_index);
        let y = compact(z_index >> 1);

        Self::new(x as i32, y as i32)
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

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Coordinate::new(1, 2)), "(1, 2)");
    }

    #[test]
    fn test_adjacent() {
        assert!(Coordinate::new(0, 0).adjacent(Coordinate::new(0, 1)));
        assert!(Coordinate::new(0, 0).adjacent(Coordinate::new(1, 0)));
        assert!(Coordinate::new(0, 0).adjacent(Coordinate::new(0, -1)));
        assert!(Coordinate::new(0, 0).adjacent(Coordinate::new(-1, 0)));
        assert!(!Coordinate::new(0, 0).adjacent(Coordinate::new(1, 1)));
        assert!(!Coordinate::new(0, 0).adjacent(Coordinate::new(-1, -1)));
    }

    #[rstest]
    #[case((0, 0), (11, 11))]
    #[case((0, 0), (0, 2))]
    #[case((0, 0), (2, 0))]
    #[case((0, 0), (0, -2))]
    #[case((0, 0), (-2, 0))]
    fn test_not_adjacent(#[case] a: (i32, i32), #[case] b: (i32, i32)) {
        assert!(!Coordinate::from(a).adjacent(Coordinate::from(b)));
    }

    #[rstest]
    #[case((0, 0), 0)]
    #[case((1, 0), 1)]
    #[case((0, 1), 2)]
    #[case((1, 1), 3)]
    #[case((2, 0), 4)]
    #[case((0, 2), 8)]
    #[case((7, 7), 63)]
    #[case((8, 0), 64)]
    #[case((0, 8), 128)]
    #[case((-1, -1), 18446744073709551615)]
    fn test_z_order(#[case] input: (i32, i32), #[case] expected: u64) {
        assert_eq!(Coordinate::from(input).z_index(), expected);
    }

    #[test]
    fn test_from_z_index() {
        assert_eq!(Coordinate::from_z_index(0), Coordinate::new(0, 0));
        assert_eq!(Coordinate::from_z_index(1), Coordinate::new(1, 0));
        assert_eq!(Coordinate::from_z_index(2), Coordinate::new(0, 1));

        let x = 12345;
        let y = 67890;
        let z = Coordinate::new(x, y).z_index();

        assert_eq!(Coordinate::from_z_index(z), Coordinate::new(x, y));
    }
}
