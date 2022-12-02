use std::str::FromStr;

use itertools::Itertools;

pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn duell(&self, other: Self) -> u32 {
        match (self, other) {
            // draws
            (Self::Rock, Self::Rock) => 3,
            (Self::Paper, Self::Paper) => 3,
            (Self::Scissors, Self::Scissors) => 3,
            // losses
            (Self::Rock, Self::Paper) => 0,
            (Self::Paper, Self::Scissors) => 0,
            (Self::Scissors, Self::Rock) => 0,
            // wins
            (Self::Rock, Self::Scissors) => 6,
            (Self::Scissors, Self::Paper) => 6,
            (Self::Paper, Self::Rock) => 6,
        }
    }
}

impl FromStr for Shape {
    type Err = ();
    fn from_str(x: &str) -> Result<Shape, Self::Err> {
        match x {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

pub fn get_score(strategy_guide: &String) -> u32 {
    strategy_guide
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(Shape::from_str)
                .flatten()
                .collect_tuple()
        })
        .flatten()
        .map(|(opponent, yours)| yours.duell(opponent) + yours as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::get_score;

    #[test]
    fn test_sample_a() -> std::io::Result<()> {
        let content = std::fs::read_to_string("sample.txt")?;

        assert_eq!(get_score(&content), 15);
        Ok(())
    }
}
