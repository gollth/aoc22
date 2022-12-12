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
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
fn main() -> Result<(), TwelfthError> {
    let args = Options::parse();
    let map = Heightmap::from_str(&std::fs::read_to_string(&args.file)?)?;
    let mut solver = Dijkstra::new(&map);

    let visualize = args.frequency >= f32::EPSILON;

    while !solver.solve_once()? {
        if !visualize {
            continue;
        }
        clear();
        println!("{}", solver);
        std::thread::sleep(Duration::from_secs_f32(1. / args.frequency));
    }
    clear();
    println!("{}", solver);
    if let Some(path) = solver.path() {
        println!("Solution 12a: {:?}", path.len() - 1);
    }

    Ok(())
}
