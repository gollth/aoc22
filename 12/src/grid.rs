use std::{collections::HashMap, str::FromStr};

use crate::{Coord, TwelfthError};

#[derive(Debug)]
pub struct Heightmap {
    start: Coord,
    finish: Coord,
    grid: HashMap<Coord, char>,
    width: usize,
    height: usize,
}

impl FromStr for Heightmap {
    type Err = TwelfthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut finish = None;
        let mut grid = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                let coord = Coord::new(x as i32, y as i32);
                if c == 'S' {
                    start = Some(coord);
                }
                if c == 'E' {
                    finish = Some(coord);
                }
                grid.insert(
                    coord,
                    match c {
                        'S' => 'a',
                        'E' => 'z',
                        x => x,
                    },
                );
            }
        }

        if start.is_none() {
            return Err(TwelfthError::InputDoesNotContainAnyStart);
        }
        if finish.is_none() {
            return Err(TwelfthError::InputDoesNotContainAnyFinish);
        }
        Ok(Self {
            start: start.unwrap(),
            finish: finish.unwrap(),
            grid,
            width: max_x + 1,
            height: max_y + 1,
        })
    }
}

impl Heightmap {
    pub fn start(&self) -> Coord {
        self.start
    }
    pub fn finish(&self) -> Coord {
        self.finish
    }
    pub fn elevation(&self, coord: Coord) -> Option<(Coord, char)> {
        self.grid.get(&coord).map(|c| (coord, *c))
    }

    pub fn dimension(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_heightmap_1d() -> Result<(), TwelfthError> {
        let map = Heightmap::from_str("SabcdeE")?;
        assert_eq!(map.start, Coord::new(0, 0));
        assert_eq!(map.finish, Coord::new(6, 0));
        assert_eq!(
            map.grid,
            HashMap::from([
                (Coord::new(0, 0), 'a'),
                (Coord::new(1, 0), 'a'),
                (Coord::new(2, 0), 'b'),
                (Coord::new(3, 0), 'c'),
                (Coord::new(4, 0), 'd'),
                (Coord::new(5, 0), 'e'),
                (Coord::new(6, 0), 'z'),
            ])
        );
        Ok(())
    }

    #[test]
    fn parse_heightmap_2d() -> Result<(), TwelfthError> {
        let content = std::fs::read_to_string("sample.txt")?;
        let map = Heightmap::from_str(&content)?;

        assert_eq!(map.start, Coord::new(0, 0));
        assert_eq!(map.finish, Coord::new(5, 2));
        assert_eq!(map.grid.get(&Coord::new(7, 0)), Some(&'m'));
        assert_eq!(map.grid.get(&Coord::new(0, 4)), Some(&'a'));
        assert_eq!(map.grid.get(&Coord::new(7, 4)), Some(&'i'));
        Ok(())
    }
}
