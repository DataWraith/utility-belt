use num::integer::gcd;
use num::{Integer, Unsigned};

use num_modular::ModularRefOps;

pub struct Congruence<T: Integer + Unsigned> {
    pub a: T,
    pub m: T,
}

pub fn chinese_remainder_theorem<T: Clone + Integer + Unsigned + ModularRefOps>(
    congruences: &[Congruence<T>],
) -> Option<T> {
    let mut n = T::one();

    for congruence in congruences.iter() {
        assert!(congruence.m != T::zero());
        assert!(gcd(n.clone(), congruence.m.clone()) == T::one());

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
}
