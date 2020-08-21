use std::hash::{Hash, Hasher, BuildHasher};

fn compute_hash<B, T>(builder: &B, value: &T) -> u64
    where B: BuildHasher, T: Hash
{
    let mut hasher = builder.build_hasher(); // 1. start the algorithm
    value.hash(&mut hasher);                 // 2. feed it data
    hasher.finish()                          // 3. finish, producing a u64
}

fn main() {
}
