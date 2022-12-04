use std::{collections::HashSet, ops::RangeInclusive};

use nom::{
    self,
    character::complete::{char, u32},
    IResult,
};

fn range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = u32(input)?;
    let (input, _) = char('-')(input)?;
    let (input, end) = u32(input)?;
    Ok((input, start..=end))
}

fn pair(input: &str) -> IResult<&str, (HashSet<u32>, HashSet<u32>)> {
    let (input, a) = range(input)?;
    let (input, _) = char(',')(input)?;
    let (input, b) = range(input)?;
    Ok((input, (HashSet::from_iter(a), HashSet::from_iter(b))))
}

pub fn amount_of_fully_overlapping_search_assigments(input: &String) -> usize {
    input
        .lines()
        .flat_map(pair)
        .map(|(_, x)| x)
        .filter(|(a, b)| a.is_subset(&b) || b.is_subset(&a))
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample_a() -> std::io::Result<()> {
        let content = std::fs::read_to_string("sample.txt")?;
        assert_eq!(amount_of_fully_overlapping_search_assigments(&content), 2);
        Ok(())
    }
}
