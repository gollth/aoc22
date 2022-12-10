use clap::Parser;
use std::{str::FromStr, time::Duration};

use tenth::{cpu::Cpu, crt::Screen, instruction::Instruction, TenthError};

/// Cathode-Ray Tube: Solve the AoC 22 day 10 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the instructions
    #[clap(long, default_value = "input.txt")]
    file: String,

    /// How many cycles per second does the Clock tick? [Hz]
    #[clap(long, default_value = "30")]
    frequency: f32,

    /// Omit the nice visualization and just print the result
    #[clap(long, action)]
    dont_visualize: bool,
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn render(screen: &mut Screen, frequency: f32) {
    clear();
    println!("{}", screen);
    std::thread::sleep(Duration::from_secs_f32(1. / frequency));
}

fn main() -> Result<(), TenthError> {
    let args = Options::parse();

    let mut cpu = Cpu::default();
    let mut screen = Screen::default();

    for instruction in std::fs::read_to_string(args.file)?
        .lines()
        .map(Instruction::from_str)
    {
        if !args.dont_visualize {
            render(&mut screen, args.frequency);
        }
        cpu.execute(&instruction?, &mut screen);
    }

    if args.dont_visualize {
        println!("Solution 10a: {}", cpu.signal_strength());
        println!("Solution 10b");
        println!("{}", screen);
    } else {
        render(&mut screen, args.frequency);
    }

    Ok(())
}
