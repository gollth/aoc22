use second::{get_score, get_score_b};

fn main() -> std::io::Result<()> {
    let strategy_guide = std::fs::read_to_string("input.txt")?;

    println!("Solution A");
    println!("Your score: {}", get_score(&strategy_guide));

    println!("Solution B");
    println!("Your score: {}", get_score_b(&strategy_guide));

    Ok(())
}
