use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use itertools::Itertools;
use ndarray::prelude::*;
use nom::{
    bytes::complete::tag,
    character::complete::{char, u32},
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};

type Coord = euclid::Vector2D<isize, euclid::UnknownUnit>;

const CAVE_SAND_ENTRY: Coord = Coord::new(500, 0);
const DOWN: Coord = Coord::new(0, 1);
const LEFT: Coord = Coord::new(-1, 0);
const RIGHT: Coord = Coord::new(1, 0);

#[derive(Debug, PartialEq, Eq, Clone)]
enum Material {
    Air,
    Rock,
    Sand,
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, " "),
            Self::Rock => write!(f, "█"),
            Self::Sand => write!(f, "░"),
        }
    }
}

fn parse_coord(s: &str) -> IResult<&str, Coord> {
    let (s, (x, y)) = separated_pair(u32, char(','), u32)(s)?;
    Ok((s, Coord::new(x as isize, y as isize)))
}

fn parse_structure(s: &str) -> IResult<&str, Vec<Coord>> {
    let (s, st) = separated_list1(tag(" -> "), parse_coord)(s)?;
    Ok((s, st))
}

pub struct Cave {
    cave: Array2<Material>,
    region: (Coord, Coord),
    has_ground: bool,
}

fn index(c: Coord) -> [usize; 2] {
    [c.x as usize, c.y as usize]
}

impl Cave {
    pub fn simulate(&mut self) -> bool {
        let mut grain = CAVE_SAND_ENTRY;

        loop {
            if self.has_ground && self.cave[index(CAVE_SAND_ENTRY)] == Material::Sand {
                // sand is blocking entry
                return false;
            }
            match self.cave.get(index(grain + DOWN)) {
                None if !self.has_ground => return false, // grain free falling, no ground
                Some(Material::Air) => {
                    grain += DOWN;
                    continue;
                }
                _ => {}
            };
            if self.cave[index(grain + DOWN + LEFT)] == Material::Air {
                grain += DOWN + LEFT;
                continue;
            }
            if self.cave[index(grain + DOWN + RIGHT)] == Material::Air {
                grain += DOWN + RIGHT;
                continue;
            }

            // Grain came to rest
            self.cave[index(grain)] = Material::Sand;
            return true;
        }
    }

    pub fn create_floor(&mut self) {
        let height = self.region.1.y + 2;
        self.cave
            .slice_mut(s![.., height..=height])
            .fill(Material::Rock);
        self.region.1.y = height;
        self.has_ground = true;
    }

    pub fn viewport(&mut self, left: isize, right: isize, top: isize, bottom: isize) {
        self.region = (Coord::new(left, top), Coord::new(right, bottom));
    }

    pub fn left(&mut self, left: isize) -> &mut Self {
        self.region.0.x = left;
        self
    }
    pub fn right(&mut self, right: isize) -> &mut Self {
        self.region.1.x = right;
        self
    }
    pub fn top(&mut self, top: isize) -> &mut Self {
        self.region.0.y = top;
        self
    }
    pub fn bottom(&mut self, bottom: isize) -> &mut Self {
        self.region.1.y = bottom;
        self
    }
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let structures = s
            .lines()
            .map(|line| {
                parse_structure(line)
                    .finish()
                    .map(|(_, r)| r.into_iter().tuple_windows())
                    .map_err(|e| nom::error::Error::new(e.input.to_string(), e.code).into())
            })
            .collect::<Result<Vec<_>>>()?;

        let (mut min, mut max) = (
            Coord::new(isize::MAX, isize::MAX).min(CAVE_SAND_ENTRY),
            Coord::new(isize::MIN, isize::MIN).max(CAVE_SAND_ENTRY),
        );
        let mut cave = Array2::from_elem((1000, 200), Material::Air);
        for structure in structures {
            for (a, b) in structure {
                let (a, b) = (a.min(b), a.max(b));
                min = min.min(a);
                max = max.max(b);
                for x in a.x..=b.x {
                    for y in a.y..=b.y {
                        cave[[x as usize, y as usize]] = Material::Rock;
                    }
                }
            }
        }

        Ok(Self {
            cave,
            region: (min, max),
            has_ground: false,
        })
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_digits = self.region.1.x.to_string().len();
        let y_digits = self.region.1.y.to_string().len();
        let y_range = (1..=y_digits).rev().map(|n| 10usize.pow(n as u32));
        for _ in 0..=y_digits {
            write!(f, " ")?;
        }
        write!(f, "╭")?;
        for _ in self.region.0.x..=self.region.1.x {
            write!(f, "─")?;
        }
        writeln!(f, "╮")?;

        for n in (1..=x_digits).rev().map(|n| 10usize.pow(n as u32)) {
            for _ in 0..=y_digits {
                write!(f, " ")?;
            }
            write!(f, "│")?;
            for x in self.region.0.x..=self.region.1.x {
                let x = x as usize;
                let d = x % n / (n / 10);
                if x % 5 == 0 {
                    write!(f, "{}", d)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "│")?;
        }

        write!(f, "╭")?;
        for _ in 0..y_digits {
            write!(f, "─")?;
        }
        write!(f, "┼")?;
        for _ in self.region.0.x..=self.region.1.x {
            write!(f, "─")?;
        }
        writeln!(f, "┤")?;

        for y in self.region.0.y..=self.region.1.y {
            write!(f, "│")?;

            for n in y_range.clone() {
                let digit = (y as usize) % n / (n / 10);
                if y % 5 == 0 {
                    write!(f, "{}", digit)?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "│")?;

            for x in self.region.0.x..=self.region.1.x {
                if Coord::new(x, y) == CAVE_SAND_ENTRY {
                    write!(f, "●")?;
                    continue;
                }
                write!(f, "{}", self.cave[index(Coord::new(x, y))])?;
            }
            writeln!(f, "│")?;
        }
        write!(f, "╰")?;
        for _ in 0..y_digits {
            write!(f, "─")?;
        }
        write!(f, "┴")?;
        for _ in self.region.0.x..=self.region.1.x {
            write!(f, "─")?;
        }
        writeln!(f, "╯")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Material::*;
    use super::*;

    #[test]
    fn cave_from_str_single_line() -> Result<()> {
        let cave = Cave::from_str("10,2 -> 10,5")?;
        assert_eq!(
            cave.cave.slice(s![10..=10, 0..8]),
            array![[Air, Air, Rock, Rock, Rock, Rock, Air, Air]]
        );
        Ok(())
    }

    #[test]
    fn cave_from_str_single_line_rev() -> Result<()> {
        let cave = Cave::from_str("42,20 -> 42,18")?;
        assert_eq!(
            cave.cave.slice(s![42..=42, 15..=21]),
            array![[Air, Air, Air, Rock, Rock, Rock, Air]]
        );
        Ok(())
    }

    #[test]
    fn cave_from_str_two_lines() -> Result<()> {
        let cave = Cave::from_str(&vec!["10,5 -> 10,7", "10,6 -> 14,6"].iter().join("\n"))?;
        assert_eq!(
            cave.cave.slice(s![10..=14, 5..=7]),
            array![
                [Rock, Air, Air, Air, Air],
                [Rock, Rock, Rock, Rock, Rock],
                [Rock, Air, Air, Air, Air]
            ]
            .t()
        );
        Ok(())
    }
    #[test]
    fn cave_from_str_sample() -> Result<()> {
        let cave = Cave::from_str(&std::fs::read_to_string("sample.txt")?)?;
        #[rustfmt::skip]
        assert_eq!(
            cave.cave.slice(s![494..=503, 3..=10]),
            array![
                [ Air,  Air,  Air,  Air,  Air,  Air,  Air,  Air,  Air,  Air],
                [ Air,  Air,  Air,  Air, Rock,  Air,  Air,  Air, Rock, Rock],
                [ Air,  Air,  Air,  Air, Rock,  Air,  Air,  Air, Rock,  Air],
                [ Air,  Air, Rock, Rock, Rock,  Air,  Air,  Air, Rock,  Air],
                [ Air,  Air,  Air,  Air,  Air,  Air,  Air,  Air, Rock,  Air],
                [ Air,  Air,  Air,  Air,  Air,  Air,  Air,  Air, Rock,  Air],
                [Rock, Rock, Rock, Rock, Rock, Rock, Rock, Rock, Rock,  Air],
                [ Air,  Air,  Air,  Air,  Air,  Air,  Air,  Air,  Air,  Air],
            ].t()
        );
        Ok(())
    }

    fn cave_sample_simulate(coords: Vec<[usize; 2]>) -> Result<()> {
        let mut cave = Cave::from_str(&std::fs::read_to_string("sample.txt")?)?;
        for _ in 0..coords.len() {
            cave.simulate();
        }
        for i in coords {
            assert_eq!(cave.cave[i], Sand);
        }
        Ok(())
    }

    #[test]
    fn cave_sample_simulate_sand_1() -> Result<()> {
        cave_sample_simulate(vec![[500, 8]])
    }
    #[test]
    fn cave_sample_simulate_sand_2() -> Result<()> {
        cave_sample_simulate(vec![[500, 8], [499, 8]])
    }

    #[test]
    fn cave_sample_simulate_sand_5() -> Result<()> {
        cave_sample_simulate(vec![[500, 8], [499, 8], [501, 8], [498, 8], [500, 7]])
    }

    #[test]
    fn cave_sample_simulate_until_settled() -> Result<()> {
        let mut cave = Cave::from_str(&std::fs::read_to_string("sample.txt")?)?;
        let mut i = 0;
        while cave.simulate() {
            i += 1;
        }
        assert_eq!(i, 24);
        Ok(())
    }

    #[test]
    fn cave_sample_with_floor() -> Result<()> {
        let mut cave = Cave::from_str(&std::fs::read_to_string("sample.txt")?)?;
        cave.create_floor();
        assert_eq!(cave.region.1.y, 11);
        Ok(())
    }

    #[test]
    fn cave_sample_simulate_until_settled_with_floor() -> Result<()> {
        let mut cave = Cave::from_str(&std::fs::read_to_string("sample.txt")?)?;
        cave.create_floor();
        let mut i = 0;
        while cave.simulate() {
            i += 1;
        }
        assert_eq!(i, 93);
        Ok(())
    }
}
