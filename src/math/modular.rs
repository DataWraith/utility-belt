use num::integer::gcd;
use num::{Integer, Unsigned};

pub use num_modular::{ModularCoreOps, ModularInteger, ModularPow, ModularUnaryOps};

pub struct Congruence<T: Integer + Unsigned> {
    pub a: T,
    pub m: T,
}

pub fn chinese_remainder_theorem<
    'a,
    T: Clone + Integer + Unsigned + ModularUnaryOps<&'a T, Output = T>,
>(
    congruences: &'a [Congruence<T>],
) -> Option<T> {
    let mut n = T::one();

    for congruence in congruences.iter() {
        assert!(congruence.m != T::zero());
        debug_assert!(gcd(n.clone(), congruence.m.clone()) == T::one());

        n = n * congruence.m.clone();
    }

    let mut solution = T::zero();

    for congruence in congruences.iter() {
        let a_i = congruence.a.clone();
        let m_i = n.clone() / congruence.m.clone();
        let n_i = m_i.clone().invm(&congruence.m)?;

        solution = (solution + a_i * (m_i % n.clone()) * n_i) % n.clone();
    }

    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_remainder_theorem() {
        let congruences = vec![
            Congruence {
                a: 83u64,
                m: 101u64,
            },
            Congruence {
                a: 60u64,
                m: 103u64,
            },
        ];

        // 2024 Day 14
        assert_eq!(chinese_remainder_theorem(&congruences), Some(6446));
    }

    #[test]
    fn test_add() {
        let (x, y, m) = (100usize, 200usize, 11usize);
        assert_eq!(x.addm(y, &m), 3);
    }

    #[test]
    fn test_sub() {
        let (x, y, m) = (100usize, 200usize, 11usize);
        assert_eq!(x.subm(y, &m), 10);
    }

    #[test]
    fn test_mul() {
        let (x, y, m) = (11usize, 38usize, 17usize);
        assert_eq!(x.mulm(y, &m), 10);
    }

    #[test]
    fn test_pow() {
        let (x, y, m) = (2u64, 99u64, 1_000_000_007u64);
        assert_eq!(x.powm(y, &m), 988_185_646);
    }

    #[test]
    fn test_inv() {
        let (x, m) = (11usize, 17usize);
        assert_eq!(x.invm(&m), Some(14));
    }
}
