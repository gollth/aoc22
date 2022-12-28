use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use twentieth::Sequence;

/// Grove Positioning System: Solve the Aoc day 20 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the encrypted coordinates
    #[clap(long, default_value = "sample.txt")]
    file: String,
}

fn main() -> Result<()> {
    let args = Options::parse();
    let mut sequence = Sequence::from_str(&std::fs::read_to_string(&args.file)?)?;

    sequence.mix();

    let coords = sequence.coords();
    println!("Decrypted coordinates: {:?}", coords);
    println!("Sum of coords:         {}", coords.0 + coords.1 + coords.2);
    Ok(())
}
