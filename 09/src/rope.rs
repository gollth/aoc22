use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use crate::{Coord, Direction};

#[derive(Debug, PartialEq, Eq)]
pub struct Rope {
    pub head: Coord,
    pub tail: Coord,
    pub visited_positions: HashSet<Coord>,
}

impl Default for Rope {
    fn default() -> Self {
        Self {
            head: Coord::default(),
            tail: Coord::default(),
            visited_positions: HashSet::from_iter([Coord::default()]),
        }
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
            .min(self.tail.x);
        let right = self
            .visited_positions
            .iter()
            .map(|c| c.x)
            .max()
            .unwrap_or_default()
            .max(self.head.x)
            .max(self.tail.x)
            .max(1);
        let bottom = self
            .visited_positions
            .iter()
            .map(|c| c.y)
            .min()
            .unwrap_or_default()
            .min(self.head.y)
            .min(self.tail.y);
        let top = self
            .visited_positions
            .iter()
            .map(|c| c.y)
            .max()
            .unwrap_or_default()
            .max(self.head.y)
            .max(self.tail.y)
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
                if c == self.head && c == self.tail {
                    write!(f, "⊕")?;
                } else if c == self.head {
                    write!(f, "○")?;
                } else if c == self.tail {
                    write!(f, "+")?;
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
    pub fn pull_tail(&mut self) {
        let spring = self.head - self.tail;
        let distance = spring.x.abs() + spring.y.abs();
        if distance <= 1 {
            // head & tail are touching, do nothing
            return;
        }
        if distance == 2 && (spring.x == 0 || spring.y == 0) {
            // head is exactly two steps directly up, down, left or right from tail
            self.tail += spring.clamp(-Direction::one(), Direction::one());
            return;
        }
        if distance == 2 && spring.x != 0 && spring.y != 0 {
            // Head and tail one off but still diagonally touching
            return;
        }

        // Head and tail are not touching and are offset diagonally
        self.tail += spring.clamp(-Direction::one(), Direction::one());
    }

    pub fn step(&mut self, dir: Direction) {
        self.head += dir;
        self.pull_tail();
        self.visited_positions.insert(self.tail);
    }
}
