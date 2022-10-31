use clap::{Parser, ValueEnum};
use hex::FromHexError;
use sha2::{Sha256, Sha512};

const DEFAULT_RANDOM_SEED_LEN: usize = 32;

use crate::{
    distance::{BitDiff, ByteDiff, CommonSubSeq, Distance},
    hash::{find_cycle, FindCycleResult},
};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum HashFunction {
    Sha256,
    Sha512,
}

impl HashFunction {
    pub fn exec<D: Distance>(self, seed: impl AsRef<[u8]>, limit: usize) -> FindCycleResult {
        match self {
            HashFunction::Sha256 => find_cycle::<Sha256, D>(seed, limit),
            HashFunction::Sha512 => find_cycle::<Sha512, D>(seed, limit),
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DistanceFunction {
    BitDiff,
    ByteDiff,
    CommonSubSeq,
}

impl DistanceFunction {
    pub fn exec(
        self,
        hash: HashFunction,
        seed: impl AsRef<[u8]>,
        threshold: usize,
    ) -> FindCycleResult {
        match self {
            DistanceFunction::BitDiff => hash.exec::<BitDiff>(seed, threshold),
            DistanceFunction::ByteDiff => hash.exec::<ByteDiff>(seed, threshold),
            DistanceFunction::CommonSubSeq => hash.exec::<CommonSubSeq>(seed, threshold),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    /// Hash function to use
    #[arg(long)]
    digest: HashFunction,

    /// Distance function to use for Floyd's algorithm
    #[arg(long)]
    distance: DistanceFunction,

    /// Threshold for the distance function.
    /// Smaller for stricter (0 means equal)
    #[arg(short, long)]
    threshold: usize,

    /// Optionnal seed, UTF8 by default, in hex if the hex flag is set.
    /// Will be randomly generated if none
    #[arg(short, long)]
    seed: Option<String>,

    /// Optional length for the random seed, no effect if a seed is given
    #[arg(short, long)]
    random_size: Option<usize>,

    /// Flag used to tell the seed is in hex.
    /// No effect if no seed is given
    #[arg(long)]
    hex: bool,
}

impl Args {
    pub fn execute(self) -> Result<FindCycleResult, FromHexError> {
        let Args {
            digest,
            distance,
            threshold: limit,
            seed,
            random_size,
            hex,
        } = self;
        let seed = seed
            .map(|seed| {
                if hex {
                    hex::decode(seed)
                } else {
                    Ok(seed.as_bytes().to_vec())
                }
            })
            .unwrap_or_else(|| Ok(Self::random_seed(random_size.unwrap_or(DEFAULT_RANDOM_SEED_LEN))))?;

        println!("seed: 0x{}", hex::encode(&seed));

        let result = distance.exec(digest, seed, limit);

        Ok(result)
    }

    fn random_seed(len: usize) -> Vec<u8> {
        std::iter::repeat(())
            .take(len)
            .map(|_| rand::random())
            .collect()
    }
}
