use std::time::Instant;

mod args;
mod distance;
mod hash;

use args::Args;

use clap::Parser;
use hex::FromHexError;

use crate::hash::FindCycleResult;

fn main() -> Result<(), FromHexError> {
    let args = Args::parse();

    let FindCycleResult {
        iter_count,
        distance,
        slow,
        fast,
        elapsed
    } = args.execute()?;

    let slow = hex::encode(slow);
    let fast = hex::encode(fast);

    println!("Found cycle: \n\t{}\n\t{}", slow, fast);
    println!("Evaluated distance: {}", distance);
    println!("iterations: {}\nExecution time: {:?}", iter_count, elapsed);

    Ok(())
}
