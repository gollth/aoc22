use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use fifteenth::{sensor::Sensor, Coord};

/// Beacon Exclusion Zone: Solve the Aoc 22 day 15 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the instructions
    #[clap(long, default_value = "sample.txt")]
    file: String,

    /// Row index to which to check for coverage (use 10 for sample, 2000000 for input)
    #[clap(long, default_value_t = 10)]
    check: i32,
}

fn main() -> Result<()> {
    let args = Options::parse();

    let sensors = std::fs::read_to_string(&args.file)?
        .lines()
        .map(Sensor::from_str)
        .collect::<Result<Vec<_>>>()?;

    let (mut min, mut max) = (
        Coord::new(i32::MAX, i32::MAX),
        Coord::new(i32::MIN, i32::MIN),
    );

    let sensors = sensors.iter();
    for sensor in sensors.clone() {
        min = min.min(sensor.min);
        max = max.max(sensor.max)
    }

    let coverage = (min.x..=max.x)
        .map(|x| Coord::new(x, args.check))
        .filter(|c| sensors.clone().any(|sensor| sensor.covers(c)))
        .count();

    println!("Area from {:?} .. {:?}", min, max);
    println!("Coverage in row {}: {}", args.check, coverage);

    Ok(())
}
