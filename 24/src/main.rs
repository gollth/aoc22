use std::time::Duration;

use anyhow::Result;
use christmas_eve::find_shortest_path;
use clap::Parser;
use termion::{
    clear,
    color::{Fg, Reset},
    cursor,
};

/// Blizzard Basin: Solve the Aoc day 24 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the valley
    #[clap(short, long, default_value = "sample.txt")]
    file: String,

    /// How fast should the valley traversal be visualized [Hz] (0 for not)
    #[clap(long, default_value_t = 0.)]
    fps: f32,
}

fn main() -> Result<()> {
    let args = Options::parse();

    let input = std::fs::read_to_string(&args.file)?;
    let path = find_shortest_path(&input)?;

    if args.fps > 0. {
        for state in path.iter() {
            print!("{}{}", clear::All, cursor::Goto(1, 1));
            print!("{}", state);
            print!("{}", Fg(Reset));
            std::thread::sleep(Duration::from_secs_f32(1. / args.fps));
        }
    }

    println!(
        "Solution 24a: Took {}min to get through the valley",
        path.len() - 1
    );

    Ok(())
}
