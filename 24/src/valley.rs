use anyhow::anyhow;
use std::{collections::HashSet, str::FromStr};

use crate::{Coord, Direction};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Blizzard {
    coord: Coord,
    direction: Direction,
}
impl Blizzard {
    fn new(coord: Coord, direction: Direction) -> Self {
        Self { coord, direction }
    }

    fn blow(&self, dimensions: &Coord) -> Self {
        let mut target = self.coord + Coord::from(self.direction.clone());
        if target.x < 1 {
            target.x += dimensions.x - 1;
        }
        if target.x >= dimensions.x {
            target.x -= dimensions.x - 1;
        }
        if target.y < 1 {
            target.y += dimensions.y - 1;
        }
        if target.y >= dimensions.y {
            target.y -= dimensions.y - 1;
        }
        Self {
            coord: target,
            direction: self.direction.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Valley {
    entry: Coord,
    exit: Coord,
    dims: Coord,
    blizzards: HashSet<Blizzard>,
}

fn is_opening((_, c): &(usize, char)) -> bool {
    *c == '.'
}

impl Valley {
    pub fn entry(&self) -> Coord {
        self.entry
    }
    pub fn exit(&self) -> Coord {
        self.exit
    }
    pub fn dimensions(&self) -> Coord {
        self.dims
    }
    pub fn blizzards(&self, coord: &Coord) -> HashSet<Direction> {
        use Direction::*;
        self.blizzards
            .intersection(
                &[Up, Down, Left, Right]
                    .into_iter()
                    .map(|dir| Blizzard {
                        coord: *coord,
                        direction: dir,
                    })
                    .collect(),
            )
            .into_iter()
            .map(|b| b.direction)
            .collect()
    }

    pub fn inside(&self, coord: &Coord) -> bool {
        coord.x > 0 && coord.x < self.dims.x && coord.y > 0 && coord.y < self.dims.y
            || *coord == self.entry
            || *coord == self.exit
    }

    pub fn simulate(&self) -> Self {
        Self {
            entry: self.entry,
            exit: self.exit,
            dims: self.dims,
            blizzards: self.blizzards.iter().map(|b| b.blow(&self.dims)).collect(),
        }
    }
}

impl FromStr for Valley {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        if lines.is_empty() {
            return Err(anyhow!("Grid empty"));
        }
        if lines.len() == 1 {
            return Err(anyhow!("Valley empty, only two walls"));
        }

        let dims = Coord::new(lines[0].chars().count() as i32 - 1, lines.len() as i32 - 1);
        let entry = Coord::new(
            lines[0]
                .chars()
                .enumerate()
                .find(is_opening)
                .ok_or(anyhow!("No entry found"))?
                .0 as i32,
            0,
        );
        let exit = Coord::new(
            lines[(dims.y) as usize]
                .chars()
                .enumerate()
                .find(is_opening)
                .ok_or(anyhow!("No exit found"))?
                .0 as i32,
            dims.y,
        );

        let mut blizzards = HashSet::new();
        for x in 1..dims.x {
            for y in 1..dims.y {
                if let Some(d) = Direction::try_from(
                    lines[y as usize]
                        .chars()
                        .nth(x as usize)
                        .ok_or(anyhow!("Line {} has not enough items", y))?,
                )
                .ok()
                {
                    blizzards.insert(Blizzard::new(Coord::new(x, y), d));
                }
            }
        }

        Ok(Self {
            dims,
            entry,
            exit,
            blizzards,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn blizzards_blowing_wraps_around_walls() {
        let dimensions = Coord::new(6, 6);
        let r = Blizzard::new(Coord::new(5, 2), Direction::Right);
        let l = Blizzard::new(Coord::new(1, 2), Direction::Left);
        let u = Blizzard::new(Coord::new(4, 1), Direction::Up);
        let d = Blizzard::new(Coord::new(4, 5), Direction::Down);
        assert_eq!(Blizzard::new(l.coord, r.direction), r.blow(&dimensions));
        assert_eq!(Blizzard::new(r.coord, l.direction), l.blow(&dimensions));
        assert_eq!(Blizzard::new(u.coord, d.direction), d.blow(&dimensions));
        assert_eq!(Blizzard::new(d.coord, u.direction), u.blow(&dimensions));
    }

    #[test]
    fn sample_from_str() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        assert!(Valley::from_str(&sample).is_ok());
        Ok(())
    }

    #[test]
    fn sample_inside() -> Result<()> {
        let valley = Valley::from_str(&std::fs::read_to_string("sample.txt")?)?;
        assert!(!valley.inside(&Coord::new(0, 0)));
        assert!(!valley.inside(&Coord::new(0, 1)));
        assert!(valley.inside(&Coord::new(1, 1)));
        assert!(valley.inside(&Coord::new(6, 4)));
        assert!(!valley.inside(&Coord::new(5, 5)));
        assert!(!valley.inside(&Coord::new(7, 4)));
        assert!(!valley.inside(&Coord::new(7, 5)));
        Ok(())
    }

    #[test]
    fn sample_entry_is_still_inside_valley() -> Result<()> {
        let valley = Valley::from_str(&std::fs::read_to_string("sample.txt")?)?;
        assert!(valley.inside(&valley.entry()));
        Ok(())
    }

    #[test]
    fn sample_exit_is_still_inside_valley() -> Result<()> {
        let valley = Valley::from_str(&std::fs::read_to_string("sample.txt")?)?;
        assert!(valley.inside(&valley.exit()));
        Ok(())
    }

    #[test]
    fn sample_has_dimensions_7_5() -> Result<()> {
        let valley = Valley::from_str(&std::fs::read_to_string("sample.txt")?)?;
        assert_eq!(valley.dims, Coord::new(7, 5));
        Ok(())
    }

    #[test]
    fn sample_has_entry_at_1_0() -> Result<()> {
        let valley = Valley::from_str(&std::fs::read_to_string("sample.txt")?)?;
        assert_eq!(valley.entry(), Coord::new(1, 0));
        Ok(())
    }

    #[test]
    fn sample_has_exit_at_6_5() -> Result<()> {
        let valley = Valley::from_str(&std::fs::read_to_string("sample.txt")?)?;
        assert_eq!(valley.exit(), Coord::new(6, 5));
        Ok(())
    }

    #[test]
    fn sample_has_blizzards_at_correct_places() -> Result<()> {
        use Direction::*;
        let valley = Valley::from_str(&std::fs::read_to_string("sample.txt")?)?;
        let blizzards = vec![
            (1, 1, [Right]),
            (2, 1, [Right]),
            (4, 1, [Left]),
            (5, 1, [Up]),
            (6, 1, [Left]),
            (2, 2, [Left]),
            (5, 2, [Left]),
            (6, 2, [Left]),
            (1, 3, [Right]),
            (2, 3, [Down]),
            (4, 3, [Right]),
            (5, 3, [Left]),
            (6, 3, [Right]),
            (1, 4, [Left]),
            (2, 4, [Up]),
            (3, 4, [Down]),
            (4, 4, [Up]),
            (5, 4, [Up]),
            (6, 4, [Right]),
        ];
        for (x, y, dirs) in blizzards {
            assert_eq!(
                valley.blizzards(&Coord::new(x, y)),
                dirs.into_iter().collect(),
                "Coord {},{}",
                x,
                y
            );
        }
        Ok(())
    }

    #[test]
    fn simple_simulate_blizzards() -> Result<()> {
        let valley = Valley::from_str(&std::fs::read_to_string("simple.txt")?)?;
        let valley = valley.simulate();
        assert_eq!(
            valley.blizzards,
            [(4, 2, Direction::Down), (4, 2, Direction::Right)]
                .into_iter()
                .map(|(x, y, d)| Blizzard::new(Coord::new(x, y), d))
                .collect()
        );

        let valley = valley.simulate();
        assert_eq!(
            valley.blizzards,
            [(4, 3, Direction::Down), (5, 2, Direction::Right)]
                .into_iter()
                .map(|(x, y, d)| Blizzard::new(Coord::new(x, y), d))
                .collect()
        );

        Ok(())
    }
}
