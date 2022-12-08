use seventh::{FileSystem, SeventhError};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let fs = FileSystem::new("input.txt")?;

    let total_size = fs
        .folders_with(|size| size <= 100_000)
        .iter()
        .map(|(_, size)| size)
        .sum::<usize>();
    println!("{}", fs);
    println!("Solution 07a: {}", total_size);

    let total_fs_size = 70_000_000;
    let required_free_space = 30_000_000;

    let free_space = total_fs_size - fs.disk_usage();
    let min_space_to_free = required_free_space - free_space;

    let mut candidates = fs.folders_with(|size| size >= min_space_to_free);
    candidates.sort_by_key(|(_, size)| *size);

    let candidate = candidates.first().ok_or(SeventhError::NoCandidateFound)?;
    println!("Solution 07b:");
    println!(
        "Deleting '{}' found free up {} B of memory",
        candidate.0, candidate.1
    );
    println!(
        "This would leave {} B of free memory",
        candidate.1 + free_space
    );
    println!(
        "Which is sufficient for the update (>= {})? {}",
        required_free_space,
        candidate.1 + free_space >= required_free_space
    );

    Ok(())
}
