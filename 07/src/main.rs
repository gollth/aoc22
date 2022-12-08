use seventh::FileSystem;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let fs = FileSystem::new("input.txt")?;

    let total_size = fs
        .folders_with_size_below(100_000)
        .iter()
        .map(|(_, size)| size)
        .sum::<usize>();
    println!("{}", fs);
    println!("Solution 07a: {}", total_size);

    Ok(())
}
