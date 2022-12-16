// pub mod map;
pub mod sensor;

pub type Coord = euclid::Vector2D<i32, euclid::UnknownUnit>;

pub fn manhatten(c: &Coord) -> i32 {
    c.abs().dot(Coord::one())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::sensor::Sensor;

    use anyhow::Result;
    use std::str::FromStr;

    #[test]
    fn sample_a() -> Result<()> {
        let sensors = std::fs::read_to_string("sample.txt")?
            .lines()
            .map(Sensor::from_str)
            .collect::<Result<Vec<_>>>()?;

        let (mut min, mut max) = (
            Coord::new(i32::MAX, i32::MAX),
            Coord::new(i32::MIN, i32::MIN),
        );

        let sensors = sensors.iter();
        for sensor in sensors.clone() {
            min = min.min(sensor.min);
            max = max.max(sensor.max)
        }

        let coverage = (min.x..=max.x)
            .map(|x| Coord::new(x, 10))
            .filter(|c| sensors.clone().any(|sensor| sensor.covers(c)))
            .count();

        assert_eq!(coverage, 26);
        Ok(())
    }
}
