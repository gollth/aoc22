use first::{find_elv_carrying_most_calories, find_total_calories_of_top_three_elves};

fn main() -> std::io::Result<()> {
    let content = std::fs::read_to_string("input.txt")?;

    let (elv, calories) =
        find_elv_carrying_most_calories(&content).expect("Input does not contain any elv groups");

    println!("Solution 1a)");
    println!("Elf:            #{}", elv);
    println!("Total Calories: {}", calories);

    let total_calories = find_total_calories_of_top_three_elves(&content)
        .expect("Input does not contain any elv groups");
    println!("Solution 1b)");
    println!("Total calories of top 3 elves: {}", total_calories);

    Ok(())
}
