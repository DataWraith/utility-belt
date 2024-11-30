use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use num::{
    traits::{Euclid, Pow},
    Num,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modular<T>
where
    T: Num + Euclid + Clone + std::fmt::Debug,
{
    value: T,
    modulus: T,
}

impl<T> Modular<T>
where
    T: Num + Euclid + Clone + std::fmt::Debug + PartialOrd,
{
    pub fn new(value: T, modulus: T) -> Self {
        assert!(modulus > T::zero());

        Self {
            value: value.rem_euclid(&modulus),
            modulus,
        }
    }
}

impl<T> Add<T> for Modular<T>
where
    T: Num + Euclid + Clone + std::fmt::Debug + PartialOrd,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::new(self.value + rhs, self.modulus)
    }
}

impl<T> AddAssign<T> for Modular<T>
where
    T: Num + Euclid + Clone + std::fmt::Debug + PartialOrd,
{
    fn add_assign(&mut self, rhs: T) {
        *self = self.clone().add(rhs);
    }
}

impl<T> Sub<T> for Modular<T>
where
    T: Num + Euclid + Clone + std::fmt::Debug + PartialOrd,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        if self.value < rhs {
            let rhs_value = rhs.rem_euclid(&self.modulus);
            Self::new(self.value + self.modulus.clone() - rhs_value, self.modulus)
        } else {
            Self::new(self.value - rhs, self.modulus)
        }
    }
}

impl<T> SubAssign<T> for Modular<T>
where
    T: Num + Euclid + Clone + std::fmt::Debug + PartialOrd,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = self.clone().sub(rhs);
    }
}

impl<T> Mul<T> for Modular<T>
where
    T: Num + Euclid + Clone + std::fmt::Debug + PartialOrd,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.value * rhs, self.modulus)
    }
}

impl<T> MulAssign<T> for Modular<T>
where
    T: Num + Euclid + Clone + std::fmt::Debug + PartialOrd,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = self.clone().mul(rhs);
    }
}

impl<T> Pow<T> for Modular<T>
where
    T: Num + Euclid + Clone + std::fmt::Debug + PartialOrd,
{
    type Output = Self;

    fn pow(self, exp: T) -> Self::Output {
        let mut base = self.clone();
        let mut exp = exp;
        let mut result = Self::new(T::one(), self.modulus);
        let two = T::one() + T::one();

        // Exponentiation by squaring
        while exp > T::zero() {
            if exp.clone() % two.clone() == T::one() {
                result *= base.value.clone();
            }

            base *= base.value.clone();
            exp = exp / two.clone();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = Modular::new(100usize, 10);
        assert_eq!(m.value, 0);
        assert_eq!(m.modulus, 10);

        let m = Modular::new(-1i32, 10);
        assert_eq!(m.value, 9);
        assert_eq!(m.modulus, 10);
    }

    #[test]
    fn test_add() {
        let m = Modular::new(111usize, 10);
        assert_eq!(m + 222usize, Modular::new(3usize, 10));

        let m = Modular::new(-1i32, 10);
        assert_eq!(m + 2i32, Modular::new(1i32, 10));

        let m = Modular::new(-1i32, 10);
        assert_eq!(m + -11i32, Modular::new(8i32, 10));
    }

    #[test]
    fn test_sub() {
        let m = Modular::new(111usize, 10);
        assert_eq!(m - 222usize, Modular::new(9usize, 10));

        let m = Modular::new(-1i32, 10);
        assert_eq!(m - -11i32, Modular::new(0i32, 10));
    }

    #[test]
    fn test_mul() {
        let m = Modular::new(111usize, 10);
        assert_eq!(m * 222usize, Modular::new(2usize, 10));

        let m = Modular::new(-1i32, 10);
        assert_eq!(m * -11i32, Modular::new(1i32, 10));
    }

    #[test]
    fn test_pow() {
        let m = Modular::new(2u64, 1_000_000_007);
        assert_eq!(m.pow(99u64), Modular::new(988_185_646, 1_000_000_007));
    }
}
