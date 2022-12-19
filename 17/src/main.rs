use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use seventeenth::{Chamber, Jet};

/// Pyroclastic Flow: Solve the Aoc 22 day 17 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the instructions
    #[clap(long, default_value = "sample.txt")]
    file: String,

    /// How many steps per second are simulated? [Hz]
    #[clap(short, long, default_value_t = 5.0)]
    frequency: f32,

    /// How many rocks should be placed until the simulation stops?
    #[clap(short, long, default_value_t = 5)]
    rocks: u32,

    /// Omit the nice visualization and just print the result
    #[clap(long)]
    dont_visualize: bool,
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn render(chamber: &Chamber, direction: &str, frequency: f32) {
    println!("{}", chamber);
    println!("{}", direction);
    std::thread::sleep(Duration::from_secs_f32(1. / frequency));
}

fn main() -> Result<()> {
    let args = Options::parse();
    let mut chamber = Chamber::default();

    let jetstream = Jet::stream(&std::fs::read_to_string(&args.file)?)?;

    let mut rocks = 0;
    for jet in jetstream.iter().cycle() {
        if !args.dont_visualize {
            clear();
            render(&chamber, "Direction: ▼", args.frequency);
        }
        chamber.push(&jet.into());

        if !args.dont_visualize {
            clear();
            render(
                &chamber,
                &format!("Direction: {}", String::from(jet)),
                args.frequency,
            );
        }

        if chamber.gravity() {
            rocks += 1;
            if rocks >= args.rocks {
                break;
            }
            chamber.spawn();
        }
    }

    if !args.dont_visualize {
        clear();
    }
    render(
        &chamber,
        &format!(
            "The entire tower after the {}th rock is {} units high",
            args.rocks,
            chamber.max_height()
        ),
        args.frequency,
    );
    Ok(())
}
