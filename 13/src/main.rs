use clap::Parser;
use thirteenth::{sum_of_right_ordered_packet_indices, ThirteenthError};

/// Distress Signal: Solve the AoC 22 day 13 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the packet pairs
    #[clap(long, default_value = "sample.txt")]
    file: String,
}

fn main() -> Result<(), ThirteenthError> {
    let args = Options::parse();
    let content = std::fs::read_to_string(args.file)?;

    println!(
        "Solution 13a: {}",
        sum_of_right_ordered_packet_indices(&content)?
    );
    Ok(())
}
