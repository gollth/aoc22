use itertools::process_results;
use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use twentyfifth::Snafu;

/// Full of Hot air: Solve the Aoc day 25 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the valley
    #[clap(short, long, default_value = "sample.txt")]
    file: String,
}

fn main() -> Result<()> {
    let args = Options::parse();

    let file = std::fs::read_to_string(&args.file)?;
    let sum = process_results(file.lines().map(Snafu::from_str), |snafus| {
        snafus.map(i64::from).sum::<i64>()
    })?;
    println!("Sum of all Snafu Numbers: {}", Snafu::from(sum));

    Ok(())
}
