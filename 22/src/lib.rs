pub mod cube;
pub mod grid;

pub type Coord = euclid::Vector2D<i32, euclid::UnknownUnit>;

use enum_iterator::{next_cycle, previous_cycle, Sequence};
use std::{
    fmt::Display,
    ops::{Div, Mul},
    str::FromStr,
};
use termion::color::{Fg, Reset, Rgb};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Sequence)]
pub enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rotation {
    None,
    Clockwise,
    CounterClockwise,
    OneEighty,
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

impl Mul<Rotation> for Direction {
    type Output = Direction;

    fn mul(self, rot: Rotation) -> Self::Output {
        match rot {
            Rotation::None => self,
            Rotation::Clockwise => next_cycle(&self).unwrap(),
            Rotation::CounterClockwise => previous_cycle(&self).unwrap(),
        }
    }
}

impl Div for Direction {
    type Output = Rotation;
    fn div(self, other: Self) -> Self::Output {
        use Direction::*;
        match (self, other) {
            (Left, Up) | (Up, Right) | (Right, Down) | (Down, Left) => Rotation::CounterClockwise,
            (Left, Down) | (Down, Right) | (Right, Up) | (Up, Left) => Rotation::Clockwise,
            _ => Rotation::None,
        }
    }
}

#[derive(Debug, Clone)]
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
            is_tip: false,
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.dir == other.dir
    }
}
impl Eq for State {}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Void,
    Free,
    Wall,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            ' ' => Cell::Void,
            '.' | '·' => Cell::Free,
            '#' | '○' => Cell::Wall,
            c => panic!("Unknown cell {}", c),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Void => write!(f, " "),
            Cell::Free => write!(f, "·"),
            Cell::Wall => write!(f, "{}●{}", Fg(Rgb(168, 123, 44)), Fg(Reset)),
        }
    }
}

pub trait Wrappable {
    fn tip(&self) -> State;
    fn tip_mut(&mut self) -> &mut State;
    // fn advance(&mut self, state: State);

    fn turn_left(&mut self);
    fn turn_right(&mut self);
    fn advance(&mut self) -> bool;
    // fn after_move(&mut self) -> bool;

    fn execute(&mut self, instruction: Move) {
        match instruction {
            Move::TurnL => self.turn_left(),
            Move::TurnR => self.turn_right(),
            Move::Forward(n) => {
                for _ in 0..n {
                    if !self.advance() {
                        break;
                    }
                }
            }
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

    #[ignore]
    #[test]
    fn direction() {
        use Direction::*;
        assert_eq!(Left / Left, Rotation::None);
        assert_eq!(Right / Right, Rotation::None);
        assert_eq!(Down / Down, Rotation::None);
        assert_eq!(Up / Up, Rotation::None);

        assert_eq!(Left / Down, Rotation::CounterClockwise);
        assert_eq!(Down / Right, Rotation::CounterClockwise);
        assert_eq!(Right / Up, Rotation::CounterClockwise);
        assert_eq!(Up / Left, Rotation::CounterClockwise);

        assert_eq!(Left / Up, Rotation::Clockwise);
        assert_eq!(Up / Right, Rotation::Clockwise);
        assert_eq!(Right / Down, Rotation::Clockwise);
        assert_eq!(Down / Left, Rotation::Clockwise);
    }
}
