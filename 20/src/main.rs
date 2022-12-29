use anyhow::{anyhow, Result};
use clap::Parser;
use twentieth::Sequence;

/// Grove Positioning System: Solve the Aoc day 20 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the encrypted coordinates
    #[clap(long, default_value = "sample.txt")]
    file: String,

    /// Which description key to use (1 for part 1, 811589153 for part 2)
    #[clap(long, default_value_t = 1)]
    key: i64,

    /// How many rounds should the mixing happen?
    #[clap(long, default_value_t = 1)]
    rounds: usize,
}

fn main() -> Result<()> {
    let args = Options::parse();
    let numbers = std::fs::read_to_string(&args.file)?
        .lines()
        .map(|x| x.parse::<i64>().map_err(|e| anyhow!("{}", e)))
        .map(|x| Ok(x? * args.key))
        .collect::<Result<Vec<_>>>()?;

    let mut sequence = Sequence::from_iter(numbers);

    for _ in 0..args.rounds {
        sequence.mix();
    }

    let coords = sequence.coords();
    println!("Decrypted coordinates: {:?}", coords);
    println!("Sum of coords:         {}", coords.0 + coords.1 + coords.2);
    Ok(())
}
