use std::time::Duration;

use clap::Parser;
use eleventh::{most_active_monkeys, parse_monkeys_from_file, play_round, EleventhError, Monkeys};

/// Monkey Buisness: Solve the Aoc 22 day 11 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the instructions
    #[clap(long, default_value = "sample.txt")]
    file: String,

    /// How many rounds do the monkeys play?
    #[clap(short, long, default_value = "20")]
    rounds: u32,

    /// How many rounds per second are played? [Hz]
    #[clap(long, default_value = "10")]
    frequency: f32,
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn render(round: u32, monkeys: &Monkeys, frequency: f32) {
    println!("#{}: {:#?}", round, monkeys);
    std::thread::sleep(Duration::from_secs_f32(1. / frequency));
}
fn main() -> Result<(), EleventhError> {
    let args = Options::parse();

    let mut monkeys = parse_monkeys_from_file(&args.file)?;

    for round in 1..=args.rounds {
        clear();
        play_round(&mut monkeys)?;
        render(round, &monkeys, args.frequency);
        let monkeys = most_active_monkeys(&monkeys);
        println!(
            "Solution 11a: Monkey Buisness {}",
            monkeys[0].inspections() * monkeys[1].inspections()
        );
    }

    Ok(())
}
