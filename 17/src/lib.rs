use std::{collections::HashSet, fmt::Display};

use anyhow::{anyhow, Result};
use enum_iterator::{first, next_cycle, Sequence};

const WIDTH: i32 = 7;
const DOWN: Coord = Coord::new(0, -1);

type Coord = euclid::Vector2D<i32, euclid::UnknownUnit>;

#[derive(Debug, PartialEq, Eq, Clone, Sequence)]
enum Shape {
    Horizontal,
    Plus,
    Bend,
    Vertical,
    Block,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Rock {
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
    rocks: HashSet<Coord>,
    max: i32,
}
impl Default for Chamber {
    fn default() -> Self {
        Self {
            rock: Rock::default(),
            rocks: HashSet::new(),
            max: 3,
        }
    }
}
impl Chamber {
    pub fn spawn(&mut self) {
        self.rock = Rock::new(
            &Coord::new(2, self.max),
            next_cycle(&self.rock.shape).unwrap(),
        );
    }

    fn occupied(&self, coord: Coord) -> bool {
        self.rocks.contains(&coord)
    }

    pub fn gravity(&mut self) -> bool {
        let coords = self.rock.coords();
        if coords.iter().any(|c| self.occupied(*c + DOWN) || c.y == 0) {
            // Rock touches something solid below
            self.rocks.extend(coords.iter());
            self.max = self
                .rocks
                .iter()
                .max_by_key(|c| c.y)
                .cloned()
                .unwrap_or_default()
                .y
                + 4;
            return true;
        }
        self.rock.fall_down();
        false
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
    pub fn max_height(&self) -> u32 {
        1 + self
            .rocks
            .iter()
            .max_by_key(|c| c.y)
            .cloned()
            .unwrap_or_default()
            .y as u32
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..=(self.max + 2)).rev() {
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
            writeln!(f, "│")?;
        }
        write!(f, "     ╰")?;
        for _ in 0..WIDTH {
            write!(f, "─")?;
        }
        writeln!(f, "╯")?;
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
    use enum_iterator::next_cycle;

    use super::*;

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
        assert_eq!(Chamber::default().rocks, HashSet::new())
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
        chamber.rocks.insert(original_origin + Coord::new(-1, 0));
        chamber.rocks.insert(original_origin + Coord::new(1, 0) * 4);

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
}
