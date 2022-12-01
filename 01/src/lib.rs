use itertools::Itertools;

pub fn find_elv_carrying_most_calories(input: &String) -> Option<(usize, u32)> {
    parse_calories(input)
        .into_iter()
        .enumerate()
        .max_by_key(|(_, calories)| *calories)
}

pub fn parse_calories(input: &String) -> Vec<u32> {
    input
        .lines()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .map(|(_, lines)| lines.map(|line| line.parse::<u32>()).flatten().sum())
        .filter(|calories| *calories > 0)
        .collect()
}

pub fn find_total_calories_of_top_three_elves(input: &String) -> Option<u32> {
    parse_calories(input).iter().sorted().rev().take(3).sum1()
}

#[cfg(test)]
mod tests {
    use crate::{find_elv_carrying_most_calories, find_total_calories_of_top_three_elves};

    #[test]
    fn sample() -> Result<(), ()> {
        let content = std::fs::read_to_string("sample.txt").map_err(|_| ())?;
        let (elv, calories) = find_elv_carrying_most_calories(&content).ok_or(())?;
        assert_eq!(elv, 3);
        assert_eq!(calories, 24_000);
        Ok(())
    }

    #[test]
    fn sample2() -> Result<(), ()> {
        let content = std::fs::read_to_string("sample.txt").map_err(|_| ())?;
        let total_calories = find_total_calories_of_top_three_elves(&content).ok_or(())?;
        assert_eq!(total_calories, 45_000);
        Ok(())
    }
}
