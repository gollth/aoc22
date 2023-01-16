use std::{str::FromStr, time::Duration};

use anyhow::{anyhow, Result};
use clap::Parser;
use itertools::Itertools;
use twentysecond::{grid::Grid, parse_instructions, Move, Wrappable};

/// Monkey Map: Solve the Aoc day 22 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the map & move instructions
    #[clap(short, long, default_value = "sample.txt")]
    file: String,

    /// Should the final path be rendered in the terminal?
    #[clap(long)]
    render: bool,

    /// How many movements per second should be visualized [Hz]? (0 for don't animate)
    #[clap(long, default_value_t = 0.0)]
    frequency: f32,
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn render(grid: &Grid, frequency: f32) {
    clear();
    println!("{}", grid);
    std::thread::sleep(Duration::from_secs_f32(1. / frequency));
}

fn main() -> Result<()> {
    let args = Options::parse();
    let sample = std::fs::read_to_string(&args.file)?;
    let (a, b) = sample
        .split_terminator("\n\n")
        .collect_tuple()
        .ok_or(anyhow!("no empty line detected"))?;

    let visualize = args.frequency > 0.;

    let mut grid = Grid::from_str(a)?;

    for instruction in parse_instructions(b)?
        .iter()
        .map(|cmd| match cmd {
            Move::Forward(n) => (0..*n).map(|_| Move::Forward(1)).collect::<Vec<_>>(),
            c => vec![c.clone()],
        })
        .flatten()
    {
        if instruction == Move::Forward(1) {
            continue;
        }
        if visualize {
            render(&grid, args.frequency);
        }

        grid.execute(instruction);
    }

    if args.render {
        clear();
        println!("{}", grid);
    }

    println!("Final password: {}", grid.password());
    Ok(())
}
