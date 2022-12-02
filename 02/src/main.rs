use second::get_score;

fn main() -> std::io::Result<()> {
    let strategy_guide = std::fs::read_to_string("input.txt")?;

    println!("Solution A");
    println!("Your score: {}", get_score(&strategy_guide));
    Ok(())
}
