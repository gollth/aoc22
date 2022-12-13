use std::{str::FromStr, time::Duration};

use clap::Parser;
use twelfth::{grid::Heightmap, solver::Dijkstra, TwelfthError};

/// Hill Climbing Algorithm: Solve the AoC 22 day 12 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the heightmap scan
    #[clap(long, default_value = "sample.txt")]
    file: String,

    /// How many path planning steps per second should be performed? [Hz]
    #[clap(long, default_value_t = 10.0)]
    frequency: f32,

    /// How many steps to skip in visualization (speeds up performance)?
    #[clap(long, default_value_t = 1)]
    skip: u32,

    /// Which is the letter to start from? ('S' for part 1, 'a' for part 2)
    #[clap(long, default_value_t = 'S')]
    start: char,
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
fn main() -> Result<(), TwelfthError> {
    let args = Options::parse();
    let map = Heightmap::from_str(&std::fs::read_to_string(&args.file)?)?;
    let starts = match args.start {
        'S' => vec![map.start()],
        'E' => vec![map.finish()],
        x => map
            .iter()
            .filter(|(_, elevation)| **elevation == x)
            .map(|(coord, _)| *coord)
            .collect(),
    };
    let mut solver = Dijkstra::new(&map, &starts);

    let visualize = args.frequency >= f32::EPSILON;

    let mut i = 4;
    while !solver.solve_once()? {
        if !visualize {
            continue;
        }
        i += 1;
        if i % args.skip != 0 {
            continue;
        }
        clear();
        println!("{}", solver);
        std::thread::sleep(Duration::from_secs_f32(1. / args.frequency));
    }
    if visualize {
        clear();
    }
    println!("{}", solver);
    if let Some(path) = solver.path() {
        println!("Solution 12: {:?}", path.len() - 1);
    }

    Ok(())
}
