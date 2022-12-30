use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::Parser;
use twentyfirst::{replace_human_with_x, replace_root_operation, simplify, Pack};

/// Monkey Math: Solve the Aoc day 21 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the monkey descriptions
    #[clap(long, default_value = "sample.txt")]
    file: String,

    /// Which part to solve?
    #[clap(long, default_value_t = 1)]
    part: u8,

    /// Try to simplify the expression?
    #[clap(long, action)]
    simplify: bool,
}

fn main() -> Result<()> {
    let args = Options::parse();
    let content = std::fs::read_to_string(&args.file)?;
    let content = match args.part {
        1 => Ok(content),
        2 => Ok(content
            .lines()
            .map(replace_root_operation)
            .map(|line| replace_human_with_x(&line))
            .collect::<Vec<_>>()
            .join("\n")),
        n => Err(anyhow!("Unknown part {}, only 1 or 2 support", n)),
    }?;

    let pack = Pack::from_str(&content)?;
    let expression = pack.evaluate("root")?;

    let solution = if args.simplify {
        simplify(&expression)?
    } else {
        format!("{}", expression)
    };

    println!("Monkey 'root': {}", solution);

    Ok(())
}
