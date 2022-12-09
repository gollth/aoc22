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

    #[test]
    fn sample_a() -> Result<(), Box<dyn Error>> {
        let cmds = read_commands("sample.txt")?;

        let mut rope = Rope::default();
        for cmd in cmds {
            for _ in 0i32..cmd.into() {
                rope.step(cmd.into());
            }
        }
        println!("{}", rope);

        assert_eq!(rope.head, Coord::new(2, 2));
        assert_eq!(rope.tail, Coord::new(1, 2));
        assert_eq!(rope.visited_positions.len(), 13);

        Ok(())
    }
}
