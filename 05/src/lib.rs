use std::{error::Error, fmt::Display, fs::read_to_string, str::FromStr};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{space1, u32},
    IResult,
};

type Stack<A> = Vec<A>;

#[derive(Debug, PartialEq, Eq)]
struct Ship {
    stacks: Vec<Stack<char>>,
}
#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    amount: usize,
    src: usize,
    dest: usize,
}

impl FromStr for Instruction {
    type Err = ElfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(s: &str) -> IResult<&str, Instruction> {
            let (s, _) = tag("move")(s)?;
            let (s, _) = space1(s)?;
            let (s, amount) = u32(s)?;
            let (s, _) = space1(s)?;
            let (s, _) = tag("from")(s)?;
            let (s, _) = space1(s)?;
            let (s, src) = u32(s)?;
            let (s, _) = space1(s)?;
            let (s, _) = tag("to")(s)?;
            let (s, _) = space1(s)?;
            let (s, dest) = u32(s)?;
            Ok((
                s,
                Instruction {
                    amount: amount as usize,
                    src: src as usize,
                    dest: dest as usize,
                },
            ))
        }
        parse(s)
            .map(|(_, o)| o)
            .map_err(|e| ElfError::InvalidInstruction(e.to_string()))
    }
}

#[derive(Debug)]
pub enum ElfError {
    InputDoesNotContainTwoSections,
    CannotExecuteInstructionBecauseStackAlreadyEmpty,
    InvalidInstruction(String),
}
impl Display for ElfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for ElfError {}

impl Ship {
    pub fn apply(&mut self, instruction: &Instruction) -> Result<(), ElfError> {
        for _ in 0..instruction.amount {
            let crate_ = self.stacks[instruction.src - 1]
                .pop()
                .ok_or(ElfError::CannotExecuteInstructionBecauseStackAlreadyEmpty)?;
            self.stacks[instruction.dest - 1].push(crate_);
        }
        Ok(())
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            stacks: Default::default(),
        }
    }
}

impl FromStr for Ship {
    type Err = ElfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ship = Self::default();
        for line in s.lines().rev().skip(1) {
            for (i, crate_) in line
                .chars()
                .chunks(4)
                .into_iter()
                .map(|item| item.filter(char::is_ascii_alphabetic).next())
                .enumerate()
            {
                if ship.stacks.len() <= i {
                    ship.stacks.push(Vec::new());
                }
                if let Some(c) = crate_ {
                    ship.stacks[i].push(c);
                }
            }
        }
        Ok(ship)
    }
}

impl Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(height) = self.stacks.iter().map(|stack| stack.len()).max() {
            write!(
                f,
                "{}",
                [
                    &[self
                        .stacks
                        .iter()
                        .enumerate()
                        .map(|(i, _)| format!(" {} ", i + 1))
                        .collect::<Vec<_>>()
                        .join(" ")],
                    ((0..height).map(|h| {
                        self.stacks
                            .iter()
                            .map(|stack| {
                                stack
                                    .get(h)
                                    .map_or("   ".to_owned(), |c| format!("[{}]", c))
                            })
                            .collect::<Vec<_>>()
                            .join(" ")
                    }))
                    .collect::<Vec<_>>()
                    .as_slice(),
                ]
                .concat()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\n")
            )
        } else {
            write!(f, "(empty)")
        }
    }
}

fn split_input(content: &str) -> Result<(String, String), ElfError> {
    let (a, b) = content
        .split_once("\n\n")
        .ok_or(ElfError::InputDoesNotContainTwoSections)?;
    Ok((a.to_string(), b.to_string()))
}

pub fn find_rearanged_top_of_stacks(file: &str) -> Result<Vec<char>, Box<dyn Error>> {
    let content = read_to_string(file)?;
    let (a, b) = split_input(&content)?;
    let mut ship = Ship::from_str(&a)?;

    for instruction in b.lines().flat_map(Instruction::from_str) {
        ship.apply(&instruction)?;
    }
    println!("{}", ship);

    Ok(ship
        .stacks
        .into_iter()
        .flat_map(|stack| stack.last().cloned())
        .collect())
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    fn test_sample_a_after_step(
        n: usize,
        current_ship_file: &str,
        expected_ship_file: &str,
    ) -> Result<(), Box<dyn Error>> {
        let instructions = split_input(&read_to_string("sample.txt")?)?
            .1
            .lines()
            .flat_map(Instruction::from_str)
            .collect::<Vec<_>>();

        let expected = Ship::from_str(&std::fs::read_to_string(expected_ship_file)?)?;
        let mut ship = Ship::from_str(&std::fs::read_to_string(current_ship_file)?)?;
        ship.apply(&instructions[n - 1])?;

        assert_eq!(ship, expected);
        Ok(())
    }

    #[test]
    fn sample_a_after_step_1() -> Result<(), Box<dyn Error>> {
        test_sample_a_after_step(1, "ship0.txt", "ship1.txt")
    }

    #[test]
    fn sample_a_after_step_2() -> Result<(), Box<dyn Error>> {
        test_sample_a_after_step(2, "ship1.txt", "ship2.txt")
    }

    #[test]
    fn sample_a_after_step_3() -> Result<(), Box<dyn Error>> {
        test_sample_a_after_step(3, "ship2.txt", "ship3.txt")
    }

    #[test]
    fn sample_a_after_step_4() -> Result<(), Box<dyn Error>> {
        test_sample_a_after_step(4, "ship3.txt", "ship4.txt")
    }

    #[test]
    fn sample_a() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            find_rearanged_top_of_stacks("sample.txt")?,
            &['C', 'M', 'Z']
        );
        Ok(())
    }
}
