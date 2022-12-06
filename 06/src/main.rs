use std::io::Result;

use sixth::{start_marker, start_message};

fn main() -> Result<()> {
    let content = std::fs::read_to_string("input.txt")?;

    println!("Solution 06a: {:?}", start_marker(&content));
    println!("Solution 06b: {:?}", start_message(&content));

    Ok(())
}
