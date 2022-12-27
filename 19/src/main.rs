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
}

fn main() -> Result<()> {
    let content = std::fs::read_to_string("sample.txt")?;
    let blueprints = content
        .split_terminator(if content.contains("\n\n") {
            "\n\n"
        } else {
            "\n"
        })
        .map(Blueprint::from_str)
        .collect::<Result<Vec<_>>>()?;

    let quality_levels = blueprints
        .into_iter()
        .map(|blueprint| {
            let geodes = solve(&blueprint, 24);
            println!("#{}: max Geodes = {}", blueprint.id(), geodes);
            blueprint.id() * geodes
        })
        .sum::<u32>();

    println!("Quality Level: {}", quality_levels);

    Ok(())
}
