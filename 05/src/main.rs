use std::error::Error;

use fifth::find_rearanged_top_of_stacks;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Solution 05a: {}",
        find_rearanged_top_of_stacks("input.txt")?
            .iter()
            .collect::<String>()
    );

    Ok(())
}
