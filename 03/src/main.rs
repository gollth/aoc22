use third::sum_of_priorities_of_duplicate_items;

fn main() -> std::io::Result<()> {
    let content = std::fs::read_to_string("input.txt")?;

    println!(
        "Solution 03a: {}",
        sum_of_priorities_of_duplicate_items(&content)
    );

    Ok(())
}
