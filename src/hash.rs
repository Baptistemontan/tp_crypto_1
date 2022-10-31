use digest::{Digest, Output as DigestOutput};

use crate::distance::Distance;

pub struct FindCycleResult {
    pub iter_count: usize,
    pub distance: usize,
    pub slow: Vec<u8>,
    pub fast: Vec<u8>,
}

pub fn find_cycle<H: Digest, D: Distance>(
    seed: impl AsRef<[u8]>,
    threshold: usize,
) -> FindCycleResult {
    // hash the seed once
    let mut slow = hash::<H>(seed);
    // hash slow, (= hashing seed twice)
    let mut fast = hash::<H>(&slow);

    let mut iter_count = 1; // seed is already hashed, can count it as an iteration
    let mut distance;


    // continue iterating as long as the distance
    // is above the threshold
    while {
        distance = D::distance(&slow, &fast);
        distance > threshold
    } {
        iter_count += 1;
        slow = hash::<H>(&slow);
        let intermidiate = hash::<H>(&fast);
        fast = hash::<H>(&intermidiate);
    }

    let (slow, fast) = (slow.to_vec(), fast.to_vec());

    FindCycleResult {
        iter_count,
        distance,
        slow,
        fast,
    }
}

fn hash<H: Digest>(data: impl AsRef<[u8]>) -> DigestOutput<H> {
    H::new_with_prefix(data).finalize()
}
