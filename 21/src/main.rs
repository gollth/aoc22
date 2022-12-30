use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use twentyfirst::Pack;

/// Monkey Math: Solve the Aoc day 21 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the monkey descriptions
    #[clap(long, default_value = "sample.txt")]
    file: String,
}

fn main() -> Result<()> {
    let args = Options::parse();
    let mut pack = Pack::from_str(&std::fs::read_to_string(&args.file)?)?;

    println!("Monkey 'root': {:?}", pack.value("root"));

    Ok(())
}
