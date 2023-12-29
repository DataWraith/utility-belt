use num::Num;

pub mod cumsum;
pub mod line_line_intersection;
pub mod polygons;
pub mod polynomials;
pub mod union_find;

pub use cumsum::*;
pub use line_line_intersection::*;
pub use polygons::*;
pub use polynomials::*;
pub use union_find::*;

/// Greatest Common Divisor.
///
/// Remember that you can fold this along a Vec as well.
pub fn gcd<T: Num + Copy>(a: T, b: T) -> T {
    let mut a = a;
    let mut b = b;

    while b != T::zero() {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

/// Least Common Multiple.
///
/// Remember that you can fold this along a Vec as well.
pub fn lcm<T: Num + Copy>(a: T, b: T) -> T {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(4, 2), 2);
        assert_eq!(gcd(4, 6), 2);
        assert_eq!(gcd(6, 4), 2);
        assert_eq!(gcd(6, 9), 3);
        assert_eq!(gcd(9, 6), 3);
    }

    #[test]
    fn lcm_works() {
        assert_eq!(lcm(2, 2), 2);
        assert_eq!(lcm(2, 4), 4);
        assert_eq!(lcm(4, 2), 4);
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(6, 4), 12);
        assert_eq!(lcm(6, 9), 18);
        assert_eq!(lcm(9, 6), 18);
    }
}
