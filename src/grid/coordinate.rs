use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use num::Num;

use super::Direction;

/// A coordinate in a 2D grid.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate<T = i32>
where
    T: Num + Copy + PartialOrd + PartialEq + Neg,
{
    pub x: T,
    pub y: T,
}

impl<T> Display for Coordinate<T>
where
    T: Num + Copy + PartialOrd + PartialEq + Neg + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Debug for Coordinate<T>
where
    T: Num + Copy + PartialOrd + PartialEq + Neg + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Add<Coordinate<T>> for Coordinate<T>
where
    T: Num + Neg<Output = T> + Copy + PartialOrd + PartialEq,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T> Add<Direction> for Coordinate<T>
where
    T: Num + Neg<Output = T> + Copy + PartialOrd + PartialEq,
{
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        let offset: Coordinate<T> = dir.into();
        self + offset
    }
}

impl AddAssign<Coordinate> for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl AddAssign<Direction> for Coordinate {
    fn add_assign(&mut self, dir: Direction) {
        let offset: Coordinate = dir.into();
        *self += offset;
    }
}

impl Sub<Coordinate> for Coordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<Direction> for Coordinate {
    type Output = Self;

    fn sub(self, dir: Direction) -> Self {
        let offset: Coordinate = dir.into();
        self - offset
    }
}

impl SubAssign<Coordinate> for Coordinate {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl SubAssign<Direction> for Coordinate {
    fn sub_assign(&mut self, dir: Direction) {
        let offset: Coordinate = dir.into();
        *self -= offset;
    }
}

impl<T> Mul<T> for Coordinate<T>
where
    T: Num + Neg<Output = T> + Copy + PartialOrd + PartialEq,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self::new(self.x * other, self.y * other)
    }
}

impl<T> MulAssign<T> for Coordinate<T>
where
    T: Num + Neg<Output = T> + Copy + PartialOrd + PartialEq,
{
    fn mul_assign(&mut self, other: T) {
        *self = *self * other;
    }
}

impl Rem<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn rem(self, other: Coordinate) -> Self {
        Self::new(self.x.rem_euclid(other.x), self.y.rem_euclid(other.y))
    }
}

impl RemAssign<Coordinate> for Coordinate {
    fn rem_assign(&mut self, other: Coordinate) {
        *self = *self % other;
    }
}

impl<T> Coordinate<T>
where
    T: Num + Copy + PartialOrd + PartialEq + Neg<Output = T>,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
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
    pub fn mirror_x_wrap(self, width: T) -> Self {
        Self::new(width - T::one() - self.x, self.y)
    }

    /// Mirror the coordinate along the Y axis.
    pub fn mirror_y(self) -> Self {
        Self::new(self.x, -self.y)
    }

    /// Mirror the coordinate along the Y axis, wrapping it so that it stays within the given height.
    pub fn mirror_y_wrap(self, height: T) -> Self {
        Self::new(self.x, height - T::one() - self.y)
    }

    /// Returns neighboring Coordinate in the given direction
    pub fn neighbor(self, dir: Direction) -> Self {
        self + dir
    }

    /// Return a list of all neighboring coordinates (alias of `von_neumann_neighbors`)
    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        Direction::cardinal().map(move |dir| self + dir)
    }

    /// Return a list of all neighboring coordinates (von Neumann Neighborhood)
    pub fn von_neumann_neighbors(self) -> impl Iterator<Item = Self> {
        self.neighbors()
    }

    /// Return a list of all neighboring coordinates (Moore Neighborhood)
    pub fn moore_neighbors(self) -> impl Iterator<Item = Self> {
        Direction::all().map(move |dir| self + dir)
    }

    /// Return a list of all coordinates reachable from self by a knight's move
    pub fn knight_move_neighbors(self) -> impl Iterator<Item = Self> {
        use Direction::*;

        [
            self + Up + Up + Right,
            self + Up + Up + Left,
            self + Right + Right + Up,
            self + Right + Right + Down,
            self + Down + Down + Right,
            self + Down + Down + Left,
            self + Left + Left + Up,
            self + Left + Left + Down,
        ]
        .into_iter()
    }

    /// Returns whether the two coordinates are adjacent
    pub fn adjacent(self, other: Self) -> bool {
        self.manhattan_distance(other) == T::one()
    }

    /// Returns the Manhattan distance between the two coordinates
    pub fn manhattan_distance(self, other: Self) -> T {
        let x_max = if self.x > other.x { self.x } else { other.x };
        let x_min = if self.x < other.x { self.x } else { other.x };
        let y_max = if self.y > other.y { self.y } else { other.y };
        let y_min = if self.y < other.y { self.y } else { other.y };

        x_max - x_min + y_max - y_min
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
}

impl<T> From<Direction> for Coordinate<T>
where
    T: Num + Neg<Output = T> + Copy + PartialOrd + PartialEq,
{
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::new(T::zero(), T::one().neg()),
            Direction::Right => Self::new(T::one(), T::zero()),
            Direction::Down => Self::new(T::zero(), T::one()),
            Direction::Left => Self::new(T::one().neg(), T::zero()),
            Direction::UpLeft => Self::new(T::one().neg(), T::one().neg()),
            Direction::UpRight => Self::new(T::one(), T::one().neg()),
            Direction::DownLeft => Self::new(T::one().neg(), T::one()),
            Direction::DownRight => Self::new(T::one(), T::one()),
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
    use num::Rational64;
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

    #[rstest]
    #[case(Direction::Up, (1, 1))]
    #[case(Direction::Right, (2, 2))]
    #[case(Direction::Down, (1, 3))]
    #[case(Direction::Left, (0, 2))]
    #[case(Direction::UpLeft, (0, 1))]
    #[case(Direction::UpRight, (2, 1))]
    #[case(Direction::DownLeft, (0, 3))]
    #[case(Direction::DownRight, (2, 3))]
    fn test_neighbor(#[case] dir: Direction, #[case] expected: (i32, i32)) {
        assert_eq!(
            Coordinate::new(1, 2).neighbor(dir),
            Coordinate::from(expected)
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
                Coordinate::new(1, 1) + Direction::Up,
                Coordinate::new(1, 1) + Direction::Right,
                Coordinate::new(1, 1) + Direction::Down,
                Coordinate::new(1, 1) + Direction::Left,
                Coordinate::new(1, 1) + Direction::UpLeft,
                Coordinate::new(1, 1) + Direction::UpRight,
                Coordinate::new(1, 1) + Direction::DownLeft,
                Coordinate::new(1, 1) + Direction::DownRight,
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

    #[rstest]
    #[case((0, 0), (0, 1))]
    #[case((0, 0), (1, 0))]
    #[case((0, 0), (0, -1))]
    #[case((0, 0), (-1, 0))]
    fn test_adjacent(#[case] a: (i32, i32), #[case] b: (i32, i32)) {
        assert!(Coordinate::from(a).adjacent(Coordinate::from(b)));
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

    #[test]
    fn test_rem() {
        let a = Coordinate::new(11, 38);
        let b = Coordinate::new(7, 9);

        assert_eq!(a % b, Coordinate::new(11 % 7, 38 % 9));
    }

    #[test]
    fn test_rem_assign() {
        let mut a = Coordinate::new(11, 38);
        let b = Coordinate::new(7, 9);
        a %= b;
        assert_eq!(a, Coordinate::new(11 % 7, 38 % 9));
    }

    #[test]
    fn test_add() {
        let a = Coordinate::new(11, 38);
        let b = Coordinate::new(7, 9);
        assert_eq!(a + b, Coordinate::new(11 + 7, 38 + 9));
    }

    #[test]
    fn test_add_assign() {
        let mut a = Coordinate::new(11, 38);
        let b = Coordinate::new(7, 9);
        a += b;

        assert_eq!(a, Coordinate::new(11 + 7, 38 + 9));
    }

    #[test]
    fn test_sub() {
        let a = Coordinate::new(11, 38);
        let b = Coordinate::new(7, 9);
        assert_eq!(a - b, Coordinate::new(11 - 7, 38 - 9));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Coordinate::new(11, 38);
        let b = Coordinate::new(7, 9);
        a -= b;

        assert_eq!(a, Coordinate::new(11 - 7, 38 - 9));
    }

    #[test]
    fn test_mul() {
        let a = Coordinate::new(11, 38);
        let b = 7;
        assert_eq!(a * b, Coordinate::new(11 * 7, 38 * 7));
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Coordinate::new(11, 38);
        let b = 7;
        a *= b;
        assert_eq!(a, Coordinate::new(11 * 7, 38 * 7));
    }

    #[test]
    fn test_coord_can_be_generic() {
        let a = Coordinate::new(Rational64::from(1), Rational64::from(2));
        let b = Coordinate::new(Rational64::from(3), Rational64::from(4));

        assert_eq!(
            a + b,
            Coordinate::new(Rational64::from(4), Rational64::from(6))
        );
    }
}
