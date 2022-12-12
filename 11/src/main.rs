use std::time::Duration;

use clap::{Parser, ValueEnum};
use eleventh::{
    calc_common_modulo, monkey::Item, most_active_monkeys, parse_monkeys_from_file, play_round,
    EleventhError, Monkeys,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Regulator {
    /// Letting worry levels be divided by 3 after monkey inspection (use for part 1)
    DivideBy3,

    /// Letting worry levels be regulated by greatest common modulo (use for part 2)
    CommonModulo,
}

/// Monkey Buisness: Solve the Aoc 22 day 11 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the instructions
    #[clap(long, default_value = "sample.txt")]
    file: String,

    /// How many rounds do the monkeys play?
    #[clap(short, long, default_value_t = 20)]
    rounds: u32,

    /// How many rounds per second are played? [Hz]
    #[clap(long, default_value_t = 10.0)]
    frequency: f32,

    /// Which regulator to use (choose between part 1 & part 2)
    #[clap(value_enum, default_value_t=Regulator::DivideBy3)]
    regulator: Regulator,
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

    let visualize = args.frequency >= f32::EPSILON;

    if visualize {
        render(0, &monkeys, args.frequency);
    }

    let common_modulo = calc_common_modulo(&monkeys);
    let regulator: Box<dyn Fn(Item) -> Item> = match args.regulator {
        Regulator::CommonModulo => Box::new(|x| x % common_modulo),
        Regulator::DivideBy3 => Box::new(|x| x / 3),
    };

    for round in 1..=args.rounds {
        if visualize {
            clear();
        }
        play_round(&mut monkeys, &regulator)?;
        if visualize {
            render(round, &monkeys, args.frequency);
        } else if round == args.rounds {
            let monkeys = most_active_monkeys(&monkeys);
            println!("#{}: {:#?}", round, monkeys);
            println!(
                "Solution 11: Monkey Buisness {}",
                monkeys[0].inspections() * monkeys[1].inspections()
            );
        }
    }

    Ok(())
}
