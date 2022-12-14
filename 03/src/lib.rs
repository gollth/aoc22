use std::collections::HashSet;

use itertools::Itertools;

fn prio(c: char) -> u32 {
    match c {
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        _ => 0,
    }
}

pub fn sum_of_priorities_of_duplicate_items(input: &String) -> u32 {
    input
        .clone()
        .lines()
        .map(|line| line.chars().map(prio).collect::<Vec<u32>>())
        .flat_map(|line| {
            line.clone()
                .chunks(line.len() / 2)
                .map(|chunk| chunk.iter().cloned().collect::<HashSet<u32>>())
                .collect_tuple()
        })
        .flat_map(|(a, b)| a.intersection(&b).next().cloned())
        .sum()
}

pub fn sum_of_priorities_of_badges(input: &String) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .flat_map(|elves| {
            elves
                .map(|rucksack| rucksack.chars().collect::<HashSet<_>>())
                .reduce(|a, x| a.intersection(&x).cloned().collect())
                .and_then(|i| i.into_iter().next())
        })
        .map(prio)
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample_a() -> std::io::Result<()> {
        let content = std::fs::read_to_string("sample.txt")?;
        assert_eq!(sum_of_priorities_of_duplicate_items(&content), 157);
        Ok(())
    }

    #[test]
    fn sample_b() -> std::io::Result<()> {
        let content = std::fs::read_to_string("sample.txt")?;
        assert_eq!(sum_of_priorities_of_badges(&content), 70);
        Ok(())
    }
}
