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
    #[clap(short, long, default_value_t = 25.0)]
    frequency: f32,

    /// How many rocks should be placed until the simulation stops?
    #[clap(short, long, default_value_t = 100)]
    rocks: usize,

    /// Omit the nice visualization and just print the result
    #[clap(long)]
    dont_visualize: bool,

    /// Provide a detected pattern cycle cycle, i.e. after how many rocks the
    /// sequence repeats again. Can be found out with the jupyter notebook
    /// (53 for sample and 2626 for input)
    #[clap(long)]
    cycle: Option<usize>,
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
            if let Some(cycle) = args.cycle {
                if let Some(offset) = chamber.find_repeating_offset(cycle) {
                    let rocks_up_to_offset = chamber.total_rocks_within(0..offset);
                    let rocks_per_cycle = chamber.total_rocks_within(offset..offset + cycle);

                    let n = (args.rocks - rocks_up_to_offset) / rocks_per_cycle;
                    let rocks_up_to_last_cycle = rocks_up_to_offset + n * rocks_per_cycle;
                    let rocks_remaining = args.rocks - rocks_up_to_last_cycle;

                    let height_of_last_rocks = chamber
                        .history
                        .iter()
                        .skip(rocks_up_to_offset)
                        .take(rocks_remaining)
                        .cloned()
                        .map(|y| y - offset)
                        .max()
                        .unwrap_or_default()
                        + 1;

                    println!(
                        "The entire tower after the {}th rock is {} units high",
                        args.rocks,
                        offset + n * cycle + height_of_last_rocks
                    );
                    return Ok(());
                }
            }
            if rocks >= args.rocks {
                break;
            }
            chamber.spawn();
        }
    }

    if !args.dont_visualize {
        clear();
    }
    println!("{:?}", chamber);
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
