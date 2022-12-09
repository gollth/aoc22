use eighth::{count_visible, find_most_scenic_place, parse_forest, visible_trees};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("input.txt")?;

    let forest = parse_forest(&content)?;
    let visible_trees = count_visible(&visible_trees(&forest));
    println!("Solution 08a: {}", visible_trees);

    if let Some(best) = find_most_scenic_place(&forest) {
        println!(
            "Solution 08b: best spot {:?}, which scores {}",
            best.0, best.1
        );
    } else {
        println!("No best place found, all equally bad =(");
    }
    Ok(())
}
