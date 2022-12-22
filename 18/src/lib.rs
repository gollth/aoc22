use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use anyhow::Result;
use itertools::Itertools;

pub type Coord = euclid::Vector3D<i32, euclid::UnknownUnit>;

pub struct Lavablob {
    volume: HashSet<Coord>,
}
const RIGHT: Coord = Coord::new(1, 0, 0);
const LEFT: Coord = Coord::new(-1, 0, 0);
const UP: Coord = Coord::new(0, 1, 0);
const DOWN: Coord = Coord::new(0, -1, 0);
const FORE: Coord = Coord::new(0, 0, 1);
const REAR: Coord = Coord::new(0, 0, -1);

const DIRECTIONS: [Coord; 6] = [RIGHT, LEFT, UP, DOWN, FORE, REAR];

impl Lavablob {
    pub fn surface_area(&self) -> usize {
        self.volume.iter().map(|c| self.boundaries(c)).sum()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Coord> {
        self.volume.iter()
    }

    pub fn bounds(&self) -> (Coord, Coord) {
        let mut min = self.volume.iter().next().cloned().unwrap_or(Coord::one());
        let mut max = min;
        for coord in self.volume.iter() {
            min.x = min.x.min(coord.x);
            min.y = min.y.min(coord.y);
            min.z = min.z.min(coord.z);
            max.x = max.x.max(coord.x + 1);
            max.y = max.y.max(coord.y + 1);
            max.z = max.z.max(coord.z + 1);
        }

        (min, max)
    }

    pub fn region_around(&self, start: &Coord, min: &Coord, max: &Coord) -> HashSet<Coord> {
        let mut region = HashSet::new();

        let containing = self.volume.contains(start);
        let mut queue = VecDeque::new();
        queue.push_back(*start);
        while let Some(item) = queue.pop_back() {
            if self.volume.contains(&item) == containing {
                region.insert(item);
            }
            queue.extend(
                DIRECTIONS
                    .iter()
                    .map(|dir| item + dir)
                    .filter(|c| !region.contains(c))
                    .filter(|c| !c.lower_than(*min).any())
                    .filter(|c| !c.greater_than(*max - Coord::one()).any())
                    .filter(|c| self.volume.contains(c) == containing),
            );
        }

        region
    }

    fn boundaries(&self, c: &Coord) -> usize {
        if !self.volume.contains(c) {
            return 0;
        }
        DIRECTIONS
            .into_iter()
            .map(|dir| *c + dir)
            .filter(|c| !self.volume.contains(c))
            .count()
    }
}

impl FromStr for Lavablob {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let volume = s
            .lines()
            .flat_map(|line| {
                line.split_terminator(",")
                    .flat_map(|n| n.parse::<i32>())
                    .collect_tuple()
            })
            .map(|(x, y, z)| Coord::new(x, y, z))
            .collect::<HashSet<_>>();
        Ok(Self { volume })
    }
}

impl FromIterator<Coord> for Lavablob {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Coord>,
    {
        let mut volume = HashSet::new();
        for item in iter {
            volume.insert(item);
        }
        Self { volume }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lavablob_from_single_cube() {
        let lava = Lavablob::from_str("1,2,3");
        assert!(lava.is_ok());
        let lava = lava.unwrap();
        assert!(lava.volume.contains(&Coord::new(1, 2, 3)));
        assert_eq!(lava.bounds(), (Coord::new(1, 2, 3), Coord::new(2, 3, 4)));
    }

    #[test]
    fn lavablob_sample_dimensions() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let lava = Lavablob::from_str(&sample)?;
        assert_eq!(lava.volume.iter().count(), sample.lines().count());
        Ok(())
    }

    #[test]
    fn lavablob_simple_surface_area() -> Result<()> {
        let lava = Lavablob::from_str("1,1,1\n2,1,1")?;
        println!("{:?}", lava.volume);
        assert_eq!(lava.surface_area(), 10);
        Ok(())
    }

    #[test]
    fn lavablob_sample_surface_area() -> Result<()> {
        let lava = Lavablob::from_str(&std::fs::read_to_string("sample.txt")?)?;
        println!("{:?}", lava.volume);
        assert_eq!(lava.surface_area(), 64);
        Ok(())
    }

    #[test]
    fn lavablob_sample_surface_area_without_inside_cave() -> Result<()> {
        let lava = Lavablob::from_str(&std::fs::read_to_string("sample.txt")?)?;
        let (mut min, mut max) = lava.bounds();
        min -= Coord::one();
        max += Coord::one();
        let cube = max - min;
        let cube_area = 2 * cube.x * cube.y + 2 * cube.x * cube.z + 2 * cube.y * cube.z;
        let water = Lavablob::from_iter(lava.region_around(&min, &min, &max).iter().cloned());

        assert_eq!(water.surface_area() as i32 - cube_area, 58);
        Ok(())
    }
}
