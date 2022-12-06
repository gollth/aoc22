use std::error::Error;

use fifth::{find_rearanged_top_of_stacks, Ship};

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Solution 05a: {}",
        find_rearanged_top_of_stacks("input.txt", Ship::crate_mover9000)?
            .iter()
            .collect::<String>()
    );

    println!(
        "Solution 05b: {}",
        find_rearanged_top_of_stacks("input.txt", Ship::crate_mover9001)?
            .iter()
            .collect::<String>()
    );

    Ok(())
}
