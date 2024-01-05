// TODO: Maybe implement Deref, so that we can, e.g. count_ones()

/// A set of integers in the range [0, 32).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Default)]
pub struct Set32(u32);

impl Set32 {
    pub fn new(val: u32) -> Self {
        Self(val)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn contains(&self, i: usize) -> bool {
        assert!(i < 32);

        self.0 & (1 << i) != 0
    }

    pub fn insert(&mut self, i: usize) {
        assert!(i < 32);

        self.0 |= 1 << i
    }

    pub fn remove(&mut self, i: usize) {
        assert!(i < 32);
        self.0 &= !(1 << i)
    }
}

/// A set of integers in the range [0, 64).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Default)]
pub struct Set64(u64);

impl Set64 {
    pub fn new(val: u64) -> Self {
        Self(val)
    }

    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn contains(&self, i: usize) -> bool {
        assert!(i < 64);

        self.0 & (1 << i) != 0
    }

    pub fn insert(&mut self, i: usize) {
        assert!(i < 64);

        self.0 |= 1 << i
    }

    pub fn remove(&mut self, i: usize) {
        assert!(i < 64);
        self.0 &= !(1 << i)
    }
}

/// A set of integers in the range [0, 128).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Default)]
pub struct Set128(u128);

impl Set128 {
    pub fn new(val: u128) -> Self {
        Self(val)
    }

    pub fn contains(&self, i: usize) -> bool {
        assert!(i < 128);

        self.0 & (1 << i) != 0
    }

    pub fn insert(&mut self, i: usize) {
        assert!(i < 128);

        self.0 |= 1 << i
    }

    pub fn remove(&mut self, i: usize) {
        assert!(i < 128);
        self.0 &= !(1 << i)
    }
}
