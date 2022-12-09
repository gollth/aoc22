pub mod cmd;
pub mod rope;

use std::{
    error::Error,
    fmt::{Display, Formatter},
    str::FromStr,
};

use euclid::UnknownUnit;

use crate::cmd::Command;

pub type Coord = euclid::Vector2D<i32, UnknownUnit>;
pub type Direction = euclid::Vector2D<i32, UnknownUnit>;

#[derive(Debug)]
pub enum NinthError {
    InputParseError(String),
}

impl Display for NinthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
impl Error for NinthError {}

pub fn parse_input(content: &str) -> Result<Vec<Command>, NinthError> {
    content.lines().map(Command::from_str).collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::rope::Rope;

    fn read_commands(file: &str) -> Result<Vec<Command>, Box<dyn Error>> {
        Ok(parse_input(&std::fs::read_to_string(file)?)?)
    }

    fn simulate(file: &str, knots: usize) -> Result<Rope, Box<dyn Error>> {
        let cmds = read_commands(file)?;

        let mut rope = Rope::new(knots);
        for cmd in cmds {
            for _ in 0i32..cmd.into() {
                rope.step(cmd.into());
            }
        }
        println!("{}", rope);
        Ok(rope)
    }

    #[test]
    fn sample_a() -> Result<(), Box<dyn Error>> {
        let rope = simulate("sample.txt", 2)?;
        assert_eq!(rope.visited_positions.len(), 13);
        Ok(())
    }

    #[test]
    fn sample_b() -> Result<(), Box<dyn Error>> {
        let rope = simulate("sample.txt", 10)?;
        assert_eq!(rope.visited_positions.len(), 1);
        Ok(())
    }

    #[test]
    fn sample_b_bigger() -> Result<(), Box<dyn Error>> {
        let rope = simulate("sample-big.txt", 10)?;
        assert_eq!(rope.visited_positions.len(), 36);
        Ok(())
    }
}
