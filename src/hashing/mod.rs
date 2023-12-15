use std::hash::Hash;

use ahash::RandomState;

pub fn hash_one<T>(x: T) -> u64
where
    T: Hash,
{
    let rs = RandomState::with_seed(1); // Deterministic seed, otherwise AHash randomizes
    rs.hash_one(x)
}
