pub use num_modular::{ModularCoreOps, ModularInteger, ModularPow, ModularUnaryOps};

#[cfg(test)]
mod tests {
    use super::*;

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
