use std::{fmt::Display, ops::Range};

use anyhow::{anyhow, Result};
use enum_iterator::{first, next_cycle, Sequence};
use num_derive::FromPrimitive;

const WIDTH: i32 = 7;
const DOWN: Coord = Coord::new(0, -1);

type Coord = euclid::Vector2D<i32, euclid::UnknownUnit>;

#[derive(Debug, PartialEq, Eq, Clone, Sequence, FromPrimitive)]
enum Shape {
    Horizontal = 0,
    Plus,
    Bend,
    Vertical,
    Block,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rock {
    origin: Coord,
    shape: Shape,
    offsets: Vec<Coord>,
}
impl Default for Rock {
    fn default() -> Self {
        Self::new(&Coord::new(2, 3), first::<Shape>().unwrap())
    }
}
impl Rock {
    fn new(origin: &Coord, shape: Shape) -> Self {
        use Shape::*;
        let offsets = match shape {
            Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Bend => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Vertical => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Block => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
        .into_iter()
        .map(|c| c.into())
        .collect();
        Self {
            origin: *origin,
            shape,
            offsets,
        }
    }

    fn coords(&self) -> Vec<Coord> {
        self.offsets.iter().map(|c| self.origin + c).collect()
    }

    fn fall_down(&mut self) {
        self.origin += DOWN;
    }
    fn push(&mut self, jet: &Coord) {
        self.origin += *jet;
    }
}

pub struct Chamber {
    rock: Rock,
    rocks: Vec<u8>,
    pub history: Vec<usize>,
    min: i32,
    max: i32,
}
impl Default for Chamber {
    fn default() -> Self {
        Self {
            rock: Rock::default(),
            rocks: Vec::new(),
            history: Vec::new(),
            min: 0,
            max: 0,
        }
    }
}
impl Chamber {
    pub fn new(rock: Rock) -> Self {
        Self {
            rock,
            rocks: Vec::new(),
            history: Vec::new(),
            min: 0,
            max: 0,
        }
    }
    pub fn spawn(&mut self) {
        self.rock = Rock::new(
            &Coord::new(2, self.max + 3),
            next_cycle(&self.rock.shape).unwrap(),
        );
    }

    fn occupied(&self, coord: Coord) -> bool {
        match self.rocks.get(coord.y as usize) {
            Some(row) => 1 << coord.x & row != 0,
            None => false,
        }
    }

    pub fn total_rocks_within(&self, range: Range<usize>) -> usize {
        self.history
            .iter()
            .filter(|y| range.start <= **y && **y < range.end)
            .count()
    }

    fn place(&mut self) {
        for c in self.rock.coords() {
            let y = c.y as usize;
            if self.rocks.len() <= y {
                self.rocks.push(0);
            }
            self.rocks[y] |= 1 << c.x;
        }

        self.history.push(
            self.rock
                .coords()
                .iter()
                .map(|c| c.y as usize)
                .max()
                .unwrap_or_default(),
        );
    }

    pub fn gravity(&mut self) -> bool {
        let coords = self.rock.coords();
        if coords.iter().any(|c| self.occupied(*c + DOWN) || c.y == 0) {
            // Rock touches something solid below
            self.place();
            self.max = self.rocks.len() as i32;
            self.min = 0.max(self.max - 20);
            return true;
        }
        self.rock.fall_down();
        false
    }

    fn auto_correlation(&self, i: usize) -> usize {
        let mut cor = 0;
        for n in 0..self.rocks.len() {
            if n as isize - i as isize >= 0 {
                cor += (self.rocks[n] as usize) * (self.rocks[n - i] as usize);
            }
        }
        cor
    }

    pub fn find_repeating_frequency(&self, threshold: f32) -> Option<usize> {
        let autocor = (0..self.rocks.len())
            .map(|i| self.auto_correlation(i as usize))
            .collect::<Vec<_>>();
        let max = autocor[0] as f32;
        autocor
            .into_iter()
            .map(|c| c as f32 / max)
            .enumerate()
            .filter(|(i, ac)| *i != 0 && *ac > threshold)
            .map(|(i, _)| i)
            .next()
    }

    pub fn find_repeating_offset(&self, frequency: usize) -> Option<usize> {
        if 2 * frequency > self.rocks.len() {
            return None;
        }
        for i in 0..frequency {
            let first_half = self.rocks[i..frequency].iter().collect::<Vec<_>>();
            let second_half = self.rocks[frequency + i..2 * frequency]
                .iter()
                .collect::<Vec<_>>();
            if first_half == second_half {
                return Some(i);
            }
        }
        None
    }

    pub fn push(&mut self, jet: &Coord) {
        let coords = self.rock.coords();
        if coords.iter().any(|c| {
            let dest = *c + jet;
            dest.x < 0 || dest.x >= WIDTH || self.occupied(dest)
        }) {
            return;
        }
        self.rock.push(jet);
    }
    pub fn max_height(&self) -> i32 {
        self.max
    }
}

impl std::fmt::Debug for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for y in 0..self.max {
            if y != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", self.rocks[y as usize])?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0.max(self.min - 2)..=(self.max + 5)).rev() {
            if y % 5 == 0 {
                write!(f, "{y:>4} ┤")?;
            } else {
                write!(f, "     │")?;
            }

            for x in 0..WIDTH {
                let coord = Coord::new(x, y);
                if self.occupied(coord) {
                    write!(f, "█")?;
                } else if self.rock.coords().contains(&coord) {
                    write!(f, "░")?;
                } else {
                    write!(f, "·")?;
                }
            }
            if y == self.min {
                writeln!(f, "┤")?;
            } else if y == self.max {
                writeln!(f, "┥")?;
            } else {
                writeln!(f, "│")?;
            }
        }
        if self.min > 2 {
            write!(f, "     ⋮")?;
            for _ in 0..WIDTH {
                write!(f, " ")?;
            }
            writeln!(f, "⋮")?;
        } else {
            write!(f, "     ╰")?;
            for _ in 0..WIDTH {
                write!(f, "─")?;
            }
            writeln!(f, "╯")?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
pub enum Jet {
    Left,
    Right,
}
impl Jet {
    pub fn stream(s: &str) -> Result<Vec<Jet>> {
        s.chars()
            .filter(|c| *c != '\n')
            .map(Jet::try_from)
            .collect()
    }
}
impl From<&Jet> for Coord {
    fn from(jet: &Jet) -> Self {
        match jet {
            Jet::Left => Coord::new(-1, 0),
            Jet::Right => Coord::new(1, 0),
        }
    }
}
impl TryFrom<char> for Jet {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),
            c => Err(anyhow!(
                "Character '{c:}' does not denote a jet stream direction"
            )),
        }
    }
}
impl From<&Jet> for String {
    fn from(jet: &Jet) -> Self {
        match jet {
            Jet::Right => "▶".to_owned(),
            Jet::Left => "◀".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enum_iterator::next_cycle;

    #[test]
    fn parse_sample_instructions() {
        let stream = std::fs::read_to_string("sample.txt").and_then(|line| Ok(Jet::stream(&line)));
        assert!(stream.is_ok());
    }

    #[test]
    fn parse_char_other_than_angle_brackets_is_error() {
        assert!(Jet::stream(">>><<<x>>").is_err())
    }

    #[test]
    fn chamber_default_is_empty() {
        assert_eq!(Chamber::default().rocks, Vec::new())
    }

    #[test]
    fn shape_order_repeats() {
        let mut shape = first::<Shape>().unwrap();
        assert_eq!(shape, Shape::Horizontal);
        shape = next_cycle(&shape).unwrap();
        assert_eq!(shape, Shape::Plus);
        shape = next_cycle(&shape).unwrap();
        assert_eq!(shape, Shape::Bend);
        shape = next_cycle(&shape).unwrap();
        assert_eq!(shape, Shape::Vertical);
        shape = next_cycle(&shape).unwrap();
        assert_eq!(shape, Shape::Block);
        shape = next_cycle(&shape).unwrap();
        assert_eq!(shape, Shape::Horizontal);
    }

    #[test]
    fn chamber_gravity_lets_rock_fall_down_until_on_ground() {
        let mut chamber = Chamber::default();
        assert_eq!(chamber.gravity(), false);
        assert_eq!(chamber.gravity(), false);
        assert_eq!(chamber.gravity(), false);
        assert_eq!(chamber.gravity(), true);
        assert_eq!(chamber.rocks.is_empty(), false);
    }

    #[test]
    fn chamber_push_lets_rock_move_sideways() {
        let mut chamber = Chamber::default();
        chamber.push(&Coord::from(&Jet::Left));
        assert_eq!(chamber.rock.origin, Coord::new(1, 3));
        chamber.push(&Coord::from(&Jet::Right));
        assert_eq!(chamber.rock.origin, Coord::new(2, 3));
    }

    #[test]
    fn chamber_push_does_nothing_if_rock_is_already_against_wall() {
        let mut chamber = Chamber::default();
        chamber.rock.origin = Coord::new(0, 3);
        chamber.push(&Coord::from(&Jet::Left));
        assert_eq!(chamber.rock.origin, Coord::new(0, 3));
    }

    #[test]
    fn chamber_push_does_nothing_if_rock_in_way() {
        let mut chamber = Chamber::default();
        let original_origin = chamber.rock.origin;
        // Place some rocks directly next to currently falling rock
        chamber.rocks.push(0);
        chamber.rocks.push(0);
        chamber.rocks.push(0);
        chamber.rocks.push(1 << 6 | 1 << 1);

        chamber.push(&Coord::from(&Jet::Left));
        assert_eq!(chamber.rock.origin, original_origin);
        chamber.push(&Coord::from(&Jet::Right));
        assert_eq!(chamber.rock.origin, original_origin);
    }

    #[test]
    fn chamber_sample_a() -> Result<()> {
        let jetstream = Jet::stream(&std::fs::read_to_string("sample.txt")?)?;
        let mut chamber = Chamber::default();

        let mut rocks = 0;
        for jet in jetstream.iter().cycle() {
            chamber.push(&jet.into());
            if chamber.gravity() {
                rocks += 1;
                if rocks >= 2022 {
                    break;
                }
                chamber.spawn();
            }
        }
        assert_eq!(chamber.max_height(), 3068);
        Ok(())
    }

    #[test]
    fn chamber_sample_a_repeating_frequency() -> Result<()> {
        let jetstream = Jet::stream(&std::fs::read_to_string("sample.txt")?)?;
        let mut chamber = Chamber::default();

        let mut rocks = 0;
        for jet in jetstream.iter().cycle() {
            chamber.push(&jet.into());
            if chamber.gravity() {
                rocks += 1;
                if let Some(f) = chamber.find_repeating_frequency(0.75) {
                    assert_eq!(f, 53);
                    return Ok(());
                }
                if rocks >= 2022 {
                    break;
                }
                chamber.spawn();
            }
        }
        panic!("find_repeating_frequency() didn't find any pattern");
    }

    #[test]
    fn chamber_sample_a_repeating_offset() -> Result<()> {
        let jetstream = Jet::stream(&std::fs::read_to_string("sample.txt")?)?;
        let mut chamber = Chamber::default();

        let mut rocks = 0;
        let mut cycle = None;
        for jet in jetstream.iter().cycle() {
            chamber.push(&jet.into());
            if chamber.gravity() {
                rocks += 1;
                if cycle.is_none() {
                    cycle = chamber.find_repeating_frequency(0.75);
                }
                if let Some(offset) = cycle.and_then(|f| chamber.find_repeating_offset(f)) {
                    assert_eq!(offset, 25);
                    return Ok(());
                }
                if rocks >= 2022 {
                    break;
                }
                chamber.spawn();
            }
        }
        panic!("find_repeating_frequency() and/or find_repeating_offset didn't find any pattern");
    }

    #[test]
    fn chamber_sample_a_cyclic_solution() -> Result<()> {
        let jetstream = Jet::stream(&std::fs::read_to_string("sample.txt")?)?;
        let mut chamber = Chamber::default();

        let target = 2022;
        let mut rocks = 0;
        let mut cycle = None;
        for jet in jetstream.iter().cycle() {
            chamber.push(&jet.into());
            if chamber.gravity() {
                rocks += 1;
                if cycle.is_none() {
                    cycle = chamber.find_repeating_frequency(0.75);
                }
                if let Some(frequency) = cycle {
                    if let Some(offset) = chamber.find_repeating_offset(frequency) {
                        println!("Height of offset: {}", offset);
                        println!("Height per cycle: {}", frequency);
                        let rocks_up_to_offset = chamber.total_rocks_within(0..offset);
                        let rocks_per_cycle =
                            chamber.total_rocks_within(offset..offset + frequency);

                        assert_eq!(rocks_up_to_offset, 15);
                        assert_eq!(rocks_per_cycle, 35);
                        let n = (target - rocks_up_to_offset) / rocks_per_cycle;
                        println!("Amount of cycles: {}", n);
                        let rocks_up_to_last_cycle = rocks_up_to_offset + n * rocks_per_cycle;
                        let rocks_remaining = target - rocks_up_to_last_cycle;

                        println!("Rocks of offset: {}", rocks_up_to_offset);
                        println!("Rocks per cycle: {}", rocks_per_cycle);
                        println!("Remaining rocks: {}", rocks_remaining);
                        println!(
                            "Total rocks:     {}",
                            rocks_up_to_offset + n * rocks_per_cycle + rocks_remaining
                        );
                        assert_eq!(rocks_remaining, 12);
                        assert!(rocks_remaining < rocks_per_cycle);

                        let height_of_last_rocks = chamber
                            .history
                            .iter()
                            .skip(rocks_up_to_offset)
                            .take(rocks_remaining)
                            .cloned()
                            .map(|y| y - offset)
                            .max()
                            .unwrap_or_default()
                            + 1;
                        println!("Height of last rocks {}", height_of_last_rocks);

                        assert_eq!(rocks_up_to_last_cycle, 2010);
                        assert_eq!(offset + n * frequency + height_of_last_rocks, 3068);
                        return Ok(());
                    }
                }
                if rocks >= target {
                    break;
                }
                chamber.spawn();
            }
        }
        panic!("find_repeating_frequency() and/or find_repeating_offset didn't find any pattern");
    }
}
