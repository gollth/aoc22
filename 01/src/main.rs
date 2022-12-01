use first::find_elv_carrying_most_calories;

fn main() -> std::io::Result<()> {
    let content = std::fs::read_to_string("input.txt")?;

    let (elv, calories) =
        find_elv_carrying_most_calories(&content).expect("Input does not contain any elv groups");

    println!("Solution 1a)");
    println!("Elf:            #{}", elv);
    println!("Total Calories: {}", calories);

    Ok(())
}
