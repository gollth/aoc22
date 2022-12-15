use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::{sensor::Sensor, Coord};

#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    Sensor,
    Beacon,
    Coverage,
    Air,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Sensor => "⊚",
                Self::Beacon => "✕",
                Self::Coverage => "·",
                Self::Air => " ",
            }
        )
    }
}

pub struct Map {
    coverage: HashMap<Coord, Object>,
    viewport: (Coord, Coord),
}

fn manhatten(c: &Coord) -> i32 {
    c.abs().dot(Coord::one())
}

impl Map {
    pub fn new(sensors: Vec<Sensor>) -> Self {
        let (mut min, mut max) = (
            Coord::new(i32::MAX, i32::MAX),
            Coord::new(i32::MIN, i32::MIN),
        );

        let mut coverage = HashMap::new();
        for sensor in sensors {
            min = min.min(sensor.location).min(sensor.beacon);
            max = max.max(sensor.location).max(sensor.beacon);
            coverage.insert(sensor.location, Object::Sensor);
            coverage.insert(sensor.beacon, Object::Beacon);

            let range = manhatten(&(sensor.location - sensor.beacon));
            for coord in (-range..=range)
                .cartesian_product(-range..=range)
                .map(Coord::from)
                .filter(|c| manhatten(c) <= range)
                .map(|c| c + sensor.location)
            {
                coverage.entry(coord).or_insert(Object::Coverage);
            }
        }
        Self {
            coverage,
            viewport: (min, max),
        }
    }

    pub fn coverage_row(&self, y: i32) -> impl Iterator<Item = &Coord> {
        println!("Checking {} entries", self.coverage.len());
        self.coverage
            .iter()
            .filter(move |(coord, obj)| coord.y == y && **obj == Object::Coverage)
            .map(|(coord, _)| coord)
    }

    pub fn left(&mut self, left: i32) {
        self.viewport.0.x = left;
    }
    pub fn right(&mut self, right: i32) {
        self.viewport.1.x = right;
    }
    pub fn top(&mut self, top: i32) {
        self.viewport.0.y = top;
    }
    pub fn bottom(&mut self, bottom: i32) {
        self.viewport.1.y = bottom;
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_digits = self.viewport.1.x.to_string().len();
        let y_digits = self.viewport.1.y.to_string().len();
        let y_range = (1..=y_digits).rev().map(|n| 10usize.pow(n as u32));
        for _ in 0..=y_digits {
            write!(f, " ")?;
        }
        write!(f, "╭")?;
        for _ in self.viewport.0.x..=self.viewport.1.x {
            write!(f, "─")?;
        }
        writeln!(f, "╮")?;

        for n in (1..=x_digits).rev().map(|n| 10usize.pow(n as u32)) {
            for _ in 0..=y_digits {
                write!(f, " ")?;
            }
            write!(f, "│")?;
            for x in self.viewport.0.x..=self.viewport.1.x {
                // let x = x as usize;
                let digit = (x as usize) % n / (n / 10);
                if x % 5 == 0 && x >= 0 {
                    write!(f, "{}", digit)?;
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
        for _ in self.viewport.0.x..=self.viewport.1.x {
            write!(f, "─")?;
        }
        writeln!(f, "┤")?;

        for y in self.viewport.0.y..=self.viewport.1.y {
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

            for x in self.viewport.0.x..=self.viewport.1.x {
                write!(
                    f,
                    "{}",
                    self.coverage.get(&Coord::new(x, y)).unwrap_or(&Object::Air)
                )?;
            }
            writeln!(f, "│")?;
        }
        write!(f, "╰")?;
        for _ in 0..y_digits {
            write!(f, "─")?;
        }
        write!(f, "┴")?;
        for _ in self.viewport.0.x..=self.viewport.1.x {
            write!(f, "─")?;
        }
        writeln!(f, "╯")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;
    use std::str::FromStr;

    #[test]
    fn sample_a() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let sensors = sample
            .lines()
            .map(Sensor::from_str)
            .collect::<Result<_>>()?;
        let mut map = Map::new(sensors);

        // Visulaization on error
        map.top(9);
        map.bottom(11);
        println!("{}", map);

        assert_eq!(map.coverage_row(10).count(), 26);

        Ok(())
    }
}
