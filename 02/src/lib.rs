use std::str::FromStr;

use enum_iterator::{next_cycle, previous_cycle, Sequence};
use itertools::Itertools;

#[derive(Clone, Sequence)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl Shape {
    fn duell(&self, other: Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Rock) => Outcome::Draw,
            (Self::Paper, Self::Paper) => Outcome::Draw,
            (Self::Scissors, Self::Scissors) => Outcome::Draw,

            (Self::Rock, Self::Paper) => Outcome::Loss,
            (Self::Paper, Self::Scissors) => Outcome::Loss,
            (Self::Scissors, Self::Rock) => Outcome::Loss,

            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
        }
    }

    fn opponent_plays_to_achieve(&self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Draw => self.clone(),
            Outcome::Win => next_cycle(self).unwrap(),
            Outcome::Loss => previous_cycle(self).unwrap(),
        }
    }
}

impl FromStr for Shape {
    type Err = ();
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

impl FromStr for Outcome {
    type Err = ();
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
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
        .map(|(opponent, yours)| yours.duell(opponent) as u32 + yours as u32)
        .sum()
}

pub fn get_score_b(strategy_guide: &String) -> u32 {
    strategy_guide
        .lines()
        .map(|line| line.split_ascii_whitespace().collect_tuple())
        .flatten()
        .map(|(opponent, outcome)| {
            (
                Shape::from_str(opponent).unwrap(),
                Outcome::from_str(outcome).unwrap(),
            )
        })
        .map(|(opponent, outcome)| {
            (
                opponent.clone(),
                opponent.opponent_plays_to_achieve(outcome),
            )
        })
        .map(|(opponent, yours)| yours.duell(opponent) as u32 + yours as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{get_score, get_score_b};

    #[test]
    fn test_sample_a() -> std::io::Result<()> {
        let content = std::fs::read_to_string("sample.txt")?;
        assert_eq!(get_score(&content), 15);
        Ok(())
    }

    #[test]
    fn test_sample_b() -> std::io::Result<()> {
        let content = std::fs::read_to_string("sample.txt")?;
        assert_eq!(get_score_b(&content), 12);
        Ok(())
    }
}
