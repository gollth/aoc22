use itertools::Itertools;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use crate::{Coord, Direction};

#[derive(Debug, PartialEq, Eq)]
pub struct Rope {
    head: Coord,
    tails: Vec<Coord>,
    pub visited_positions: HashSet<Coord>,
}

impl Default for Rope {
    fn default() -> Self {
        Self::new(2)
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let left = self
            .visited_positions
            .iter()
            .map(|c| c.x)
            .min()
            .unwrap_or_default()
            .min(self.head.x)
            .min(self.tails.iter().map(|t| t.x).min().unwrap_or(0));
        let right = self
            .visited_positions
            .iter()
            .map(|c| c.x)
            .max()
            .unwrap_or_default()
            .max(self.head.x)
            .max(self.tails.iter().map(|t| t.x).max().unwrap_or(0))
            .max(1);
        let bottom = self
            .visited_positions
            .iter()
            .map(|c| c.y)
            .min()
            .unwrap_or_default()
            .min(self.head.y)
            .min(self.tails.iter().map(|t| t.y).min().unwrap_or(0));
        let top = self
            .visited_positions
            .iter()
            .map(|c| c.y)
            .max()
            .unwrap_or_default()
            .max(self.head.y)
            .max(self.tails.iter().map(|t| t.y).max().unwrap_or(0))
            .max(1);

        write!(f, "╭")?;
        for _ in left..=right {
            write!(f, "─")?;
        }
        writeln!(f, "╮")?;
        for j in (bottom..=top).rev() {
            write!(f, "│")?;
            for i in left..=right {
                let c = Coord::new(i, j);
                let c_on_tail = self.tails.iter().enumerate().find(|(_, t)| **t == c);
                if c == self.head && c_on_tail.is_some() {
                    write!(f, "●")?;
                } else if c == self.head {
                    write!(f, "○")?;
                } else if let Some((i, _)) = c_on_tail {
                    write!(f, "{}", i + 1)?;
                } else if self.visited_positions.contains(&c) {
                    write!(f, "•")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "│")?;
        }
        write!(f, "╰")?;
        for _ in left..=right {
            write!(f, "─")?;
        }
        writeln!(f, "╯")?;
        Ok(())
    }
}

impl Rope {
    pub fn new(knots: usize) -> Self {
        Self {
            head: Coord::default(),
            tails: vec![Coord::default(); knots - 1],
            visited_positions: HashSet::from_iter([Coord::default()]),
        }
    }

    pub fn pull(&mut self, head: Coord, i: usize) {
        let spring = head - self.tails[i];
        let distance = spring.x.abs() + spring.y.abs();
        if distance <= 1 {
            // head & tail are touching, do nothing
            return;
        }
        if distance == 2 && (spring.x == 0 || spring.y == 0) {
            // head is exactly two steps directly up, down, left or right from tail
            self.tails[i] += spring.clamp(-Direction::one(), Direction::one());
            return;
        }
        if distance == 2 && spring.x != 0 && spring.y != 0 {
            // Head and tail one off but still diagonally touching
            return;
        }

        // Head and tail are not touching and are offset diagonally
        self.tails[i] += spring.clamp(-Direction::one(), Direction::one());
    }

    pub fn step(&mut self, dir: Direction) {
        self.head += dir; // move head

        // Pull first tail
        self.pull(self.head, 0);

        // If there exist further tails, pull them also
        if self.tails.len() > 1 {
            for (a, b) in (0..self.tails.len()).tuple_windows() {
                self.pull(self.tails[a], b);
            }
        }
        self.visited_positions.insert(*self.tails.last().unwrap());
    }
}
