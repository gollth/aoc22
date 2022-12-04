use fourth::{
    amount_of_fully_overlapping_search_assigments,
    amount_of_partially_overlapping_search_assigments,
};

fn main() -> std::io::Result<()> {
    let content = std::fs::read_to_string("input.txt")?;

    println!(
        "Solution 04a: {}",
        amount_of_fully_overlapping_search_assigments(&content)
    );

    println!(
        "Solution 04b: {}",
        amount_of_partially_overlapping_search_assigments(&content)
    );

    Ok(())
}
