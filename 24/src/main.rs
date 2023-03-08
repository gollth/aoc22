use std::time::Duration;

use anyhow::Result;
use christmas_eve::{find_shortest_path, valley::Valley};
use clap::Parser;
use std::{rc::Rc, str::FromStr};
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

    /// How many trips from valley entry to exit (and vice versa) should be done?
    #[clap(long, default_value_t = 1)]
    trips: usize,
}

fn main() -> Result<()> {
    let args = Options::parse();

    let mut total = Vec::new();

    let mut valley = Rc::new(Valley::from_str(&std::fs::read_to_string(&args.file)?)?);
    let mut start = valley.entry();
    let mut target = valley.exit();
    let mut total_time = 0;

    for _ in 0..args.trips {
        let mut path = find_shortest_path(start, target, valley)?;
        let checkpoint = path.last().unwrap();
        valley = checkpoint.valley.clone();
        total_time += checkpoint.possibility.time();
        (start, target) = (target, start);
        total.append(&mut path);
    }

    if args.fps > 0. {
        for state in total.iter() {
            print!("{}{}", clear::All, cursor::Goto(1, 1));
            print!("{}", state);
            print!("{}", Fg(Reset));
            std::thread::sleep(Duration::from_secs_f32(1. / args.fps));
        }
    }
    println!(
        "Solution 24a: Took {}min to get through the valley",
        total_time
    );

    Ok(())
}
