use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use fifteenth::{map::Map, sensor::Sensor};

/// Beacon Exclusion Zone: Solve the Aoc 22 day 15 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the instructions
    #[clap(long, default_value = "sample.txt")]
    file: String,

    /// Column index of the viewport's right edge during animation (use <500, omit for auto)
    #[clap(short, long)]
    left: Option<i32>,

    /// Column index of the viewport's right edge during animation (use >500, omit for auto)
    #[clap(short, long)]
    right: Option<i32>,

    /// Row index of the viewport's top edge during animation (use ~0, omit for auto)
    #[clap(short, long)]
    top: Option<i32>,

    /// Row index of the viewport's bottom edge during animation (use ~15, omit for auto)
    #[clap(short, long)]
    bottom: Option<i32>,

    /// Row index to which to check for coverage (use 10 for sample, 2000000 for input)
    #[clap(long, default_value_t = 10)]
    check: i32,

    /// Omit the nice visualization and just print the result
    #[clap(long)]
    dont_visualize: bool,
}

fn main() -> Result<()> {
    let args = Options::parse();

    let mut map = Map::new(
        std::fs::read_to_string(&args.file)?
            .lines()
            .map(Sensor::from_str)
            .collect::<Result<_>>()?,
    );

    args.left.map(|l| map.left(l));
    args.right.map(|r| map.right(r));
    args.top.map(|t| map.top(t));
    args.bottom.map(|b| map.bottom(b));

    if !args.dont_visualize {
        println!("{}", map);
    }

    println!(
        "Coverage in row {}: {}",
        args.check,
        map.coverage_row(args.check).count()
    );
    Ok(())
}
