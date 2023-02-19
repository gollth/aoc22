use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use twentythird::{Coord, Grid};

/// Unstable Diffusion: Solve the Aoc day 23 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the map & move instructions
    #[clap(short, long, default_value = "example/0.txt")]
    file: String,

    #[clap(short, long, default_value_t = 10)]
    rounds: u32,
}

fn main() -> Result<()> {
    let args = Options::parse();
    let mut grid = Grid::from_str(&std::fs::read_to_string(&args.file)?)?;

    for _ in 0..args.rounds {
        grid.motion();
        grid.rotate_preferences();
    }

    let (ax, bx, ay, by) = grid.bounding_box();
    let empties = (ax..=bx)
        .cartesian_product(ay..=by)
        .filter(|(x, y)| !grid.contains(&Coord::new(*x, *y)))
        .count();
    println!("Solution 23a: {} empty squares", empties);

    Ok(())
}
