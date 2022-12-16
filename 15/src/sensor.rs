use std::str::FromStr;

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::{manhatten, Coord};

pub struct Sensor {
    location: Coord,
    beacon: Coord,
    range: i32,
    pub min: Coord,
    pub max: Coord,
}

impl Sensor {
    pub fn covers(&self, coord: &Coord) -> bool {
        *coord != self.location
            && *coord != self.beacon
            && manhatten(&(self.location - *coord)) <= self.range
    }
}

impl FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(
            r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)",
        )?;
        let captures = regex
            .captures(s)
            .ok_or(anyhow!("Sensor not in correct format: '{}'", s))?;

        let group = |name| captures.name(name).unwrap().as_str().parse::<i32>();

        let location = Coord::new(group("sx")?, group("sy")?);
        let beacon = Coord::new(group("bx")?, group("by")?);
        let range = manhatten(&(location - beacon));
        let min = Coord::new(location.x - range, location.y - range);
        let max = Coord::new(location.x + range, location.y + range);
        Ok(Self {
            location,
            beacon,
            range,
            min,
            max,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensor_from_str() {
        assert!(Sensor::from_str("Sensor at x=2, y=18: closest beacon is at x=-2, y=15").is_ok())
    }

    #[test]
    fn sensor_from_str_sample() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let sensors = sample
            .lines()
            .map(Sensor::from_str)
            .collect::<Result<Vec<_>>>();
        assert!(sensors.is_ok());
        Ok(())
    }
}
