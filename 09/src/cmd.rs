use std::str::FromStr;

use crate::{Direction, NinthError};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Command {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl FromStr for Command {
    type Err = NinthError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(" ")
            .ok_or(NinthError::InputParseError(s.to_owned()))?;

        let n = b
            .parse::<u32>()
            .map_err(|e| NinthError::InputParseError(e.to_string()))?;
        match a {
            "U" => Ok(Self::Up(n)),
            "D" => Ok(Self::Down(n)),
            "L" => Ok(Self::Left(n)),
            "R" => Ok(Self::Right(n)),
            x => Err(NinthError::InputParseError(format!(
                "Unknown direction {}",
                x
            ))),
        }
    }
}

impl From<Command> for Direction {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Up(_) => Direction::new(0, 1),
            Command::Down(_) => Direction::new(0, -1),
            Command::Left(_) => Direction::new(-1, 0),
            Command::Right(_) => Direction::new(1, 0),
        }
    }
}

impl From<Command> for i32 {
    fn from(cmd: Command) -> Self {
        (match cmd {
            Command::Up(n) => n,
            Command::Down(n) => n,
            Command::Left(n) => n,
            Command::Right(n) => n,
        }) as i32
    }
}
