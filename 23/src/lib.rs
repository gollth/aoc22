use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    ops::Add,
    str::FromStr,
};

use enum_iterator::{all, Sequence};
use itertools::Itertools;

pub type Coord = euclid::Vector2D<i32, euclid::UnknownUnit>;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
    elves: HashSet<Coord>,
    preferences: VecDeque<([Direction; 3], Direction)>,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        Ok(Grid {
            elves: s
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '#')
                        .map(move |(x, _)| Coord::new(x as i32, y as i32))
                })
                .flatten()
                .collect(),
            preferences: [
                ([N, NE, NW], N),
                ([S, SE, SW], S),
                ([W, NW, SW], W),
                ([E, NE, SE], E),
            ]
            .into_iter()
            .collect(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Sequence, Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, dir: Direction) -> Self::Output {
        self + Coord::from(dir)
    }
}
impl From<Direction> for Coord {
    fn from(c: Direction) -> Self {
        match c {
            Direction::N => Coord::new(0, -1),
            Direction::NE => Coord::new(1, -1),
            Direction::NW => Coord::new(-1, -1),
            Direction::E => Coord::new(1, 0),
            Direction::W => Coord::new(-1, 0),
            Direction::S => Coord::new(0, 1),
            Direction::SE => Coord::new(1, 1),
            Direction::SW => Coord::new(-1, 1),
        }
    }
}

impl Grid {
    pub fn contains(&self, elf: &Coord) -> bool {
        self.elves.contains(elf)
    }

    fn propose(&self, elf: &Coord) -> Coord {
        if !all::<Direction>().any(|dir| self.contains(&(*elf + dir))) {
            // Alone, no one near by
            return *elf;
        }
        for (prefs, choice) in &self.preferences {
            if !prefs.iter().any(|dir| self.contains(&(*elf + *dir))) {
                return *elf + *choice;
            }
        }
        *elf
    }
    pub fn rotate_preferences(&mut self) {
        let dir = self.preferences.pop_front().unwrap();
        self.preferences.push_back(dir);
    }

    pub fn motion(&mut self) {
        let grid = self
            .elves
            .iter()
            .enumerate()
            .map(|(id, elf)| (id, elf))
            .collect::<HashMap<_, _>>();

        let propositions = grid
            .iter()
            .map(|(id, elf)| (*id, self.propose(elf)))
            .collect_vec();

        let mut xs = HashSet::new();

        for (key, group) in propositions
            .into_iter()
            .map(|(id, p)| (p, id))
            .into_group_map()
        {
            if group.len() > 1 {
                // Clash, keep original positions
                for id in group {
                    xs.insert(**grid.get(&id).unwrap());
                }
                continue;
            }
            xs.insert(key);
        }
        self.elves = xs;
    }

    pub fn bounding_box(&self) -> (i32, i32, i32, i32) {
        let minx = self.elves.iter().min_by_key(|c| c.x).expect("empty grid").x;
        let maxx = self.elves.iter().max_by_key(|c| c.x).expect("empty grid").x;
        let miny = self.elves.iter().min_by_key(|c| c.y).expect("empty grid").y;
        let maxy = self.elves.iter().max_by_key(|c| c.y).expect("empty grid").y;
        (minx, maxx, miny, maxy)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (minx, maxx, miny, maxy) = self.bounding_box();
        for y in miny..=maxy {
            for x in minx..=maxx {
                write!(
                    f,
                    "{}",
                    if self.contains(&Coord::new(x, y)) {
                        "#"
                    } else {
                        "."
                    }
                )?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;

    fn round(scenario: &str, n: usize) -> Result<()> {
        // Arange
        let mut grid = Grid::from_str(&std::fs::read_to_string(format!("{}/0.txt", scenario))?)?;
        for _ in 0..n {
            grid.motion();
            grid.rotate_preferences();
        }

        // Act

        // Assert
        let solution =
            Grid::from_str(&std::fs::read_to_string(format!("{}/{}.txt", scenario, n))?)?;
        assert_eq!(solution.elves, grid.elves, "\n{}", grid);
        Ok(())
    }

    #[test]
    fn sample_into_grid() -> Result<()> {
        let sample = std::fs::read_to_string("sample/0.txt")?;
        let grid = Grid::from_str(&sample)?;
        assert_eq!(grid.elves.len(), 5);

        assert!(grid.contains(&Coord::new(2, 1)));
        assert!(grid.contains(&Coord::new(3, 1)));
        assert!(grid.contains(&Coord::new(2, 2)));
        assert!(grid.contains(&Coord::new(2, 4)));
        assert!(grid.contains(&Coord::new(3, 4)));
        Ok(())
    }

    #[test]
    fn sample_round1() -> Result<()> {
        round("sample", 1)
    }
    #[test]
    fn sample_round2() -> Result<()> {
        round("sample", 2)
    }

    #[test]
    fn sample_round3() -> Result<()> {
        round("sample", 3)
    }

    #[test]
    fn example_round1() -> Result<()> {
        round("example", 1)
    }
    #[test]
    fn example_round2() -> Result<()> {
        round("example", 2)
    }
    #[test]
    fn example_round3() -> Result<()> {
        round("example", 3)
    }
    #[test]
    fn example_round4() -> Result<()> {
        round("example", 4)
    }
    #[test]
    fn example_round5() -> Result<()> {
        round("example", 5)
    }
    #[test]
    fn example_round10() -> Result<()> {
        round("example", 10)
    }
    #[test]
    fn example_empty_squares() -> Result<()> {
        let mut grid = Grid::from_str(&std::fs::read_to_string("example/0.txt")?)?;
        for _ in 0..10 {
            grid.motion();
            grid.rotate_preferences();
        }

        let (ax, bx, ay, by) = grid.bounding_box();
        let empties = (ax..=bx)
            .cartesian_product(ay..=by)
            .filter(|(x, y)| !grid.contains(&Coord::new(*x, *y)))
            .count();

        assert_eq!(110, empties);

        Ok(())
    }
}
