use itertools::Itertools;

pub fn find_elv_carrying_most_calories(input: &String) -> Option<(usize, u32)> {
    input
        .lines()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .map(|(_, lines)| {
            lines
                .map(|line| line.parse::<u32>())
                .flatten()
                .collect::<Vec<_>>()
        })
        .filter(|group| !group.is_empty())
        .map(|group| group.iter().sum::<u32>())
        .enumerate()
        .max_by_key(|(_, sum)| *sum)
}

#[cfg(test)]
mod tests {
    use crate::find_elv_carrying_most_calories;

    #[test]
    fn sample() -> Result<(), ()> {
        let content = std::fs::read_to_string("sample.txt").map_err(|_| ())?;
        let (elv, calories) = find_elv_carrying_most_calories(&content).ok_or(())?;
        assert_eq!(elv, 3);
        assert_eq!(calories, 24_000);
        Ok(())
    }
}
