use std::{str::FromStr, time::Duration};

use anyhow::Result;
use clap::Parser;
use fourteenth::cave::Cave;

/// Regolith Reservoir: Solve the Aoc 22 day 14 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the instructions
    #[clap(long, default_value = "sample.txt")]
    file: String,

    /// How many rounds per second are played? [Hz]
    #[clap(long, default_value_t = 10.0)]
    frequency: f32,
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn render(cave: &Cave, frequency: f32) {
    println!("{}", cave);
    std::thread::sleep(Duration::from_secs_f32(1. / frequency));
}
fn main() -> Result<()> {
    let args = Options::parse();
    let mut cave = Cave::from_str(&std::fs::read_to_string(&args.file)?)?;

    println!("{}", cave);
    let mut grains = 0;
    while cave.simulate() {
        clear();
        render(&cave, args.frequency);
        grains += 1;
    }
    println!("Solution 14a: {}", grains);

    Ok(())
}
