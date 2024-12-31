use std::ops::Neg;

use num::{Bounded, Num};

pub mod coordinate;
pub mod direction;
pub mod grid;

pub use coordinate::*;
pub use direction::*;
pub use grid::*;

pub fn bounding_box<T: Num + Bounded + Copy + PartialOrd + PartialEq + Neg<Output = T>>(
    points: impl Iterator<Item = Coordinate<T>>,
) -> (Coordinate<T>, Coordinate<T>) {
    let (min_x, min_y, max_x, max_y) = points.fold(
        (
            T::max_value(),
            T::max_value(),
            T::min_value(),
            T::min_value(),
        ),
        |(min_x, min_y, max_x, max_y), point| {
            (
                if min_x < point.x { min_x } else { point.x },
                if min_y < point.y { min_y } else { point.y },
                if max_x > point.x { max_x } else { point.x },
                if max_y > point.y { max_y } else { point.y },
            )
        },
    );

    (Coordinate::new(min_x, min_y), Coordinate::new(max_x, max_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box() {
        let points = vec![(-1, 0), (1, 1), (2, 2)];
        let (min, max) = bounding_box(points.into_iter().map(|(x, y)| Coordinate::new(x, y)));
        assert_eq!(min, Coordinate::new(-1, 0));
        assert_eq!(max, Coordinate::new(2, 2));
    }
}
