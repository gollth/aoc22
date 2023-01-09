pub mod grid;

pub type Coord = euclid::Vector2D<i32, euclid::UnknownUnit>;

use enum_iterator::Sequence;
use std::{fmt::Display, str::FromStr};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Sequence)]
pub enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl From<Direction> for Coord {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Left => coord(-1, 0),
            Direction::Right => coord(1, 0),
            Direction::Up => coord(0, -1),
            Direction::Down => coord(0, 1),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "◂"),
            Direction::Right => write!(f, "▸"),
            Direction::Up => write!(f, "▴"),
            Direction::Down => write!(f, "▾"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub coord: Coord,
    pub dir: Direction,
    pub last: Direction,
    pub is_tip: bool,
}
impl State {
    pub fn new(coord: Coord, dir: Direction) -> Self {
        Self {
            coord,
            dir,
            last: Direction::Right,
            is_tip: true,
        }
    }
}

impl From<&State> for (Coord, Direction) {
    fn from(state: &State) -> Self {
        (state.coord, state.dir)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;
        match (self.dir, self.last, self.is_tip) {
            (Right, Right, true) => write!(f, "→"),
            (Left, Left, true) => write!(f, "←"),
            (Up, Up, true) => write!(f, "↑"),
            (Down, Down, true) => write!(f, "↓"),

            (Right, Down, true) => write!(f, "↳"),
            (Left, Down, true) => write!(f, "↲"),
            (Right, Up, true) => write!(f, "↱"),
            (Left, Up, true) => write!(f, "↰"),

            (Down, Right, true) => write!(f, "⬎"),
            (Down, Left, true) => write!(f, "⬐"),
            (Up, Right, true) => write!(f, "⬏"),
            (Up, Left, true) => write!(f, "⬑"),

            (Right | Left, Right | Left, false) => write!(f, "─"),
            (Up | Down, Up | Down, false) => write!(f, "│"),

            (Down, Right, false) => write!(f, "╮"),
            (Right, Down, false) => write!(f, "╰"),
            (Down, Left, false) => write!(f, "╭"),
            (Left, Down, false) => write!(f, "╯"),

            (Right, Up, false) => write!(f, "╭"),
            (Up, Right, false) => write!(f, "╯"),
            (Left, Up, false) => write!(f, "╮"),
            (Up, Left, false) => write!(f, "╰"),

            (Right, Left, _) | (Left, Right, _) | (Up, Down, _) | (Down, Up, _) => {
                panic!("Nonsensical direction")
            }
        }
    }
}

/// Little helper function to create coords
pub fn coord(x: i32, y: i32) -> Coord {
    Coord::new(x, y)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Move {
    Forward(i32),
    TurnL,
    TurnR,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::TurnL),
            "R" => Ok(Self::TurnR),
            x => Ok(Self::Forward(x.parse()?)),
        }
    }
}

pub fn parse_instructions(s: &str) -> Result<Vec<Move>> {
    s.replace("L", " L ")
        .replace("R", " R ")
        .split_whitespace()
        .filter(|s| *s != " ")
        .map(|s| Move::from_str(s))
        .collect()
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use itertools::Itertools;

    use crate::grid::Grid;

    use super::*;

    #[test]
    fn sample_password() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let (a, b) = sample
            .split_terminator("\n\n")
            .collect_tuple()
            .ok_or(anyhow!("no empty line detected"))?;

        let mut grid = Grid::from_str(a)?;
        for cmd in parse_instructions(b)? {
            grid.execute(cmd);
        }

        assert_eq!(grid.password(), 6032);
        Ok(())
    }
}
