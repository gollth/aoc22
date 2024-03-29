use std::{collections::HashMap, fmt::Display, ops::Index, str::FromStr};

use colors_transform::{Color, Hsl};
use enum_iterator::{next_cycle, previous_cycle};
use termion::color::{Fg, Reset, Rgb};

use crate::{coord, Coord, Direction, Move, State};

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

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
    coords: HashMap<Coord, Cell>,
    path: Vec<State>,

    starting: Coord,

    top: i32,
    btm: i32,
    left: i32,
    right: i32,
}

impl Grid {
    pub fn coords(&self) -> Vec<(Coord, Direction)> {
        self.path.iter().map(Into::into).collect()
    }
    pub fn starting_position(&self) -> Coord {
        self.starting
    }

    pub fn neighbor(&self, coord: &Coord, dir: &Direction) -> (Coord, Cell) {
        let dir = Coord::from(dir.clone());
        let mut c = *coord;
        loop {
            c += dir;
            if c.x >= self.right {
                c.x -= self.right - self.left;
            }
            if c.x < self.left {
                c.x += self.right - self.left + 1;
            }
            if c.y >= self.btm {
                c.y -= self.btm - self.top;
            }
            if c.y < self.top {
                c.y += self.btm - self.top + 1;
            }
            let n = self[c];
            if n != Cell::Void {
                return (c, n);
            }
        }
    }

    pub fn execute(&mut self, instruction: Move) -> bool {
        match instruction {
            Move::TurnL => {
                let mut state = self.path.last_mut().expect("empty path");
                state.dir = previous_cycle(&state.dir).unwrap();
            }
            Move::TurnR => {
                let mut state = self.path.last_mut().expect("empty path");
                state.dir = next_cycle(&state.dir).unwrap();
            }
            Move::Forward(n) => {
                for _ in 0..n {
                    let state = self.path.last().expect("empty path").clone();
                    match self.neighbor(&state.coord, &state.dir) {
                        (_, Cell::Wall) => return false,
                        (coord, _) => {
                            self.path.push(State::new(coord, state.dir));
                        }
                    }
                }
            }
        }

        let length = self.path.len();
        if length >= 2 {
            self.path[length - 1].last = self.path[length - 2].dir;
            self.path[length - 2].is_tip = false;
        }

        true
    }

    pub fn password(&self) -> i32 {
        let state = self.path.last().expect("empty path");
        1000 * state.coord.y + 4 * state.coord.x + state.dir.clone() as usize as i32
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut top, mut btm, mut left, mut right) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
        let mut coords = HashMap::new();
        let mut starting = Coord::new(i32::MAX, i32::MAX);

        for (coord, cell) in s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (coord(x as i32 + 1, y as i32 + 1), Cell::from(c)))
            })
            .flatten()
        {
            if cell != Cell::Void {
                coords.insert(coord, cell);
            }

            top = top.min(coord.y);
            btm = btm.max(coord.y);
            left = left.min(coord.x);
            right = right.max(coord.x);

            if coord.y == 1 && coord.x < starting.x && cell == Cell::Free {
                starting = coord;
            }
        }
        Ok(Self {
            coords,
            path: vec![State::new(starting, Direction::Right)],
            starting,
            top,
            btm,
            left,
            right,
        })
    }
}
impl Index<Coord> for Grid {
    type Output = Cell;

    fn index(&self, index: Coord) -> &Self::Output {
        self.coords.get(&index).unwrap_or(&Cell::Void)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grey = Fg(Rgb(100, 100, 100));
        write!(f, "{}╭", grey)?;
        for _ in self.left..self.right {
            write!(f, "─")?;
        }
        writeln!(f, "╮")?;

        let path_len = self.path.len() as f32;
        for y in self.top..self.btm {
            write!(f, "│{}", Fg(Reset))?;
            for x in self.left..self.right {
                let c = coord(x, y);
                if let Some((i, state)) = self
                    .path
                    .iter()
                    .enumerate()
                    .find(|(_, state)| state.coord == c)
                {
                    // let x = (((path_len - i as f32) / path_len) * 220. + 35.) as u8;
                    let hsv = Hsl::from(220., 100. * (i as f32 / path_len), 65.);
                    let color = Rgb(
                        hsv.get_red() as u8,
                        hsv.get_green() as u8,
                        hsv.get_blue() as u8,
                    );
                    write!(f, "{}{}{}", Fg(color), state, Fg(Reset))?;
                } else {
                    write!(
                        f,
                        "{}",
                        self.coords.get(&coord(x, y)).unwrap_or(&Cell::Void)
                    )?;
                }
            }
            writeln!(f, "{}│", grey)?;
        }

        write!(f, "╰")?;
        for _ in self.left..self.right {
            write!(f, "─")?;
        }
        writeln!(f, "╯{}", Fg(Reset))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{anyhow, Result};
    use itertools::Itertools;
    use Cell::*;
    use Direction::*;

    #[test]
    fn from_str_square() -> Result<()> {
        let grid = Grid::from_str("..\n.#")?;
        assert_eq!(grid[coord(1, 1)], Free);
        assert_eq!(grid[coord(2, 1)], Free);
        assert_eq!(grid[coord(1, 2)], Free);
        assert_eq!(grid[coord(2, 2)], Wall);
        Ok(())
    }

    #[test]
    fn from_str_with_voids() -> Result<()> {
        let grid = Grid::from_str(" ..\n ## ")?;
        assert_eq!(grid[coord(1, 1)], Void);
        Ok(())
    }

    fn grid_from_sample() -> Result<Grid> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let (a, _) = sample
            .split_terminator("\n\n")
            .collect_tuple()
            .ok_or(anyhow!("no empty line detected"))?;
        Ok(Grid::from_str(a)?)
    }

    #[test]
    fn sample_from_str() -> Result<()> {
        let grid = grid_from_sample();
        assert!(grid.is_ok());
        assert_eq!(
            grid?.coords.values().filter(|k| **k != Void).count(),
            4 * 4 + 12 * 4 + 8 * 4
        );

        Ok(())
    }

    #[test]
    fn sample_starting_position() -> Result<()> {
        let grid = grid_from_sample()?;
        assert_eq!(grid.starting_position(), coord(9, 1));
        Ok(())
    }

    #[test]
    fn sample_neighbor() -> Result<()> {
        let grid = grid_from_sample()?;
        assert_eq!(
            grid.neighbor(&(grid.starting_position() + coord(2, 0)), &Right),
            (grid.starting_position() + coord(3, 0), Wall)
        );
        Ok(())
    }

    #[test]
    fn sample_neightbor_wrapping() -> Result<()> {
        let grid = grid_from_sample()?;

        assert_eq!(
            grid.neighbor(&grid.starting_position(), &Left),
            (coord(12, 1), Wall)
        );

        assert_eq!(
            grid.neighbor(&grid.starting_position(), &Up),
            (coord(9, 12), Free)
        );

        let a = coord(12, 7);
        let b = coord(1, 7);
        assert_eq!(grid.neighbor(&a, &Right), (b, Free));
        assert_eq!(grid.neighbor(&b, &Left), (a, Free));

        let c = coord(6, 8);
        let d = coord(6, 5);
        assert_eq!(grid.neighbor(&c, &Down), (d, Cell::Free));
        assert_eq!(grid.neighbor(&d, &Up), (c, Cell::Free));
        Ok(())
    }

    #[test]
    fn sample_move_forward1() -> Result<()> {
        let mut grid = grid_from_sample()?;

        grid.execute(Move::Forward(1));
        assert_eq!(
            grid.coords(),
            vec![
                (grid.starting_position(), Right),
                (grid.starting_position() + coord(1, 0), Right)
            ]
        );
        Ok(())
    }

    #[test]
    fn sample_move_turn_left() -> Result<()> {
        let mut grid = grid_from_sample()?;
        grid.execute(Move::TurnL);
        assert_eq!(grid.coords(), vec![(grid.starting_position(), Up)]);
        Ok(())
    }

    #[test]
    fn sample_move_turn_right() -> Result<()> {
        let mut grid = grid_from_sample()?;
        grid.execute(Move::TurnR);
        assert_eq!(grid.coords(), vec![(grid.starting_position(), Down)]);
        Ok(())
    }

    #[test]
    fn sample_move_forward_into_rock() -> Result<()> {
        let mut grid = grid_from_sample()?;

        grid.execute(Move::Forward(4));
        assert_eq!(
            grid.coords(),
            vec![
                (grid.starting_position(), Right),
                (grid.starting_position() + coord(1, 0), Right),
                (grid.starting_position() + coord(2, 0), Right),
            ]
        );
        Ok(())
    }

    #[test]
    fn sample_move_forward_wrapping_free_on_other_side() -> Result<()> {
        let mut grid = grid_from_sample()?;
        grid.path = vec![State::new(grid.starting_position(), Up)]; // adjust starting orientation
        grid.execute(Move::Forward(2));
        assert_eq!(
            grid.coords(),
            vec![
                (grid.starting_position(), Up),
                (coord(9, 12), Up),
                (coord(9, 11), Up),
            ]
        );
        Ok(())
    }

    #[test]
    fn sample_move_forward_wrapping_wall_on_other_side() -> Result<()> {
        let mut grid = grid_from_sample()?;
        grid.path = vec![State::new(grid.starting_position(), Left)]; // adjust starting orientation
        grid.execute(Move::Forward(2));
        assert_eq!(grid.path, vec![State::new(grid.starting_position(), Left)]);
        Ok(())
    }
}
