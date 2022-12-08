use eighth::{count_visible, parse_forest, visible_trees};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("input.txt")?;

    let forest = parse_forest(&content)?;
    let visible_trees = count_visible(&visible_trees(&forest));
    println!("Solution 08a: {}", visible_trees);
    Ok(())
}
