use std::str::FromStr;

use anyhow::Result;
use sixteenth::{find_max_releasable_pressure, network::Network};

fn main() -> Result<()> {
    let sample = std::fs::read_to_string("sample.txt")?;
    let network = Network::from_str(&sample)?;
    let solution = find_max_releasable_pressure(&network, 30)?;
    println!("Solution A: {solution}");
    Ok(())
}
