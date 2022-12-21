use anyhow::Result;
use std::str::FromStr;

use eighteenth::Lavablob;

fn main() -> Result<()> {
    let lava = Lavablob::from_str(&std::fs::read_to_string("input.txt")?)?;
    println!("Surface area: {}", lava.surface_area());
    Ok(())
}
