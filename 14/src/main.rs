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
    #[clap(short, long, default_value_t = 10.0)]
    frequency: f32,

    /// How many frames to skip in between animation to speed up?
    #[clap(long, default_value_t = 0)]
    skip: u32,

    /// Column index of the viewport's right edge during animation (use <500, omit for auto)
    #[clap(short, long)]
    left: Option<isize>,

    /// Column index of the viewport's right edge during animation (use >500, omit for auto)
    #[clap(short, long)]
    right: Option<isize>,

    /// Row index of the viewport's top edge during animation (use ~0, omit for auto)
    #[clap(short, long)]
    top: Option<isize>,

    /// Row index of the viewport's bottom edge during animation (use ~15, omit for auto)
    #[clap(short, long)]
    bottom: Option<isize>,

    /// Omit the nice visualization and just print the result
    #[clap(long)]
    dont_visualize: bool,
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

    cave.create_floor();
    args.left.map(|l| cave.left(l));
    args.right.map(|r| cave.right(r));
    args.top.map(|u| cave.top(u));
    args.bottom.map(|d| cave.bottom(d));

    if !args.dont_visualize {
        println!("{}", cave);
    }

    let mut grains = 0;
    let mut i = 0;
    while cave.simulate() {
        if !args.dont_visualize && i == args.skip {
            clear();
            render(&cave, args.frequency);
            i = 0;
        }
        i += 1;
        grains += 1;
    }
    println!("Solution 14: {}", grains);

    Ok(())
}
