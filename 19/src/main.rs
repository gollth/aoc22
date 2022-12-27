use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use nineteenth::{blueprint::Blueprint, solve};

/// Not Enough Minerals: Solve the Aoc day 19 problem
#[derive(Parser, Debug)]
struct Options {
    /// Input file with the instructions
    #[clap(long, default_value = "sample.txt")]
    file: String,

    /// How many minutes do you have time to crack geodes
    #[clap(long, default_value_t = 24)]
    time: i32,

    /// Only take the first `n` blueprints into account
    #[clap(long)]
    blueprints: Option<usize>,
}

fn main() -> Result<()> {
    let args = Options::parse();
    let content = std::fs::read_to_string(&args.file)?;
    let blueprints = content
        .split_terminator(if content.contains("\n\n") {
            "\n\n"
        } else {
            "\n"
        })
        .map(Blueprint::from_str)
        .collect::<Result<Vec<_>>>()?;

    let n = blueprints.len();
    let solution = blueprints
        .into_iter()
        .take(args.blueprints.unwrap_or(n))
        .map(|blueprint| (blueprint.id(), solve(&blueprint, args.time)))
        .inspect(|(i, x)| println!("#{i}: {x}"))
        .collect::<Vec<_>>();

    println!(
        "Quality Level: {}",
        solution.iter().map(|(i, g)| i * g).sum::<u32>()
    );

    println!(
        "Product of all max possible geodes: {}",
        solution.iter().map(|(_, x)| x).product::<u32>()
    );

    Ok(())
}
