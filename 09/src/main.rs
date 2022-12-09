use std::{error::Error, time::Duration};

use ninth::{parse_input, rope::Rope};

use clap::Parser;

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

/// Rope Bridge: Solve the AoC 22 day 09 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the commands
    #[clap(long, default_value = "input.txt")]
    file: String,

    /// How many steps per seconds to simulate [Hz]
    #[clap(long, default_value = "50")]
    frequency: f32,

    /// Should the simulation be visualized in terminal?
    #[clap(long, action)]
    visualize: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Options::parse();
    let cmds = parse_input(&std::fs::read_to_string(args.file)?)?;

    let sleep_time = Duration::from_secs_f32(1. / args.frequency);
    let mut rope = Rope::default();
    for cmd in cmds {
        for _ in 0i32..cmd.into() {
            if args.visualize {
                clear();
                println!("{}", rope);
                std::thread::sleep(sleep_time);
            }
            rope.step(cmd.into());
        }
    }
    if args.visualize {
        clear();
        println!("{}", rope);
    } else {
        println!("Solution 09a: {}", rope.visited_positions.len());
    }

    Ok(())
}
