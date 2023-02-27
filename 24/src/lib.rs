pub mod valley;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    iter::once,
    rc::Rc,
    str::FromStr,
};

use anyhow::{anyhow, Result};
use colors_transform::{Color, Hsl};
use termion::color::{Fg, Rgb, White};
use valley::Valley;

pub type Coord = euclid::Vector2D<i32, euclid::UnknownUnit>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Direction::Right),
            '<' => Ok(Direction::Left),
            'v' => Ok(Direction::Down),
            '^' => Ok(Direction::Up),
            c => Err(anyhow!("Unknown direction {}", c)),
        }
    }
}

impl From<Direction> for Coord {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Coord::new(0, -1),
            Direction::Down => Coord::new(0, 1),
            Direction::Left => Coord::new(-1, 0),
            Direction::Right => Coord::new(1, 0),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Possibility {
    time: usize,
    coord: Coord,
    target: Coord,
    parent: Option<Rc<Possibility>>,
}
impl PartialOrd for Possibility {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heureristic().partial_cmp(&other.heureristic())
    }
}
impl Ord for Possibility {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.manhattan().cmp(&other.manhattan())
    }
}
impl Possibility {
    fn heureristic(&self) -> i32 {
        self.time as i32 + self.manhattan()
    }
    fn manhattan(&self) -> i32 {
        let diff = (self.target - self.coord).abs();
        diff.x + diff.y
    }
    fn options(parent: &Rc<Possibility>) -> Vec<Rc<Possibility>> {
        use Direction::*;

        [Up, Down, Left, Right]
            .into_iter()
            // Move
            .map(|direction| Possibility {
                time: parent.time + 1,
                coord: parent.coord + Coord::from(direction),
                target: parent.target,
                parent: Some(parent.clone()),
            })
            // Wait
            .chain(once(Possibility {
                time: parent.time + 1,
                coord: parent.coord,
                target: parent.target,
                parent: Some(parent.clone()),
            }))
            .map(Rc::new)
            .collect()
    }
}

pub struct State {
    possibility: Rc<Possibility>,
    valley: Rc<Valley>,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;
        let dims = self.valley.dimensions();
        let grey = Rgb(120, 120, 120);
        let mut path = Vec::new();
        let mut item = self.possibility.clone();
        while let Some(parent) = &item.parent {
            path.push(parent.coord);
            item = parent.clone();
        }

        for y in 0..=dims.y {
            for x in 0..=dims.x {
                let c = Coord::new(x, y);
                let bs = self.valley.blizzards(&c);

                let idx =
                    path.iter().position(|item| *item == c).unwrap_or(0) as f32 / path.len() as f32;
                let hsv = Hsl::from(220. * (1. - idx), 100., 50.);
                let color = Rgb(
                    hsv.get_red() as u8,
                    hsv.get_green() as u8,
                    hsv.get_blue() as u8,
                );
                let is_player = x == self.possibility.coord.x && y == self.possibility.coord.y;
                match (x, y) {
                    (0, 0) => write!(f, "{}╭", Fg(grey)),
                    (x, 0) if x == dims.x => write!(f, "{}╮", Fg(grey)),
                    (x, y) if x == dims.x && y == dims.y => write!(f, "{}╯", Fg(grey)),
                    (0, y) if y == dims.y => write!(f, "{}╰", Fg(grey)),
                    (_, _) if self.valley.entry() == c && is_player => write!(f, "{}┰", Fg(grey)),
                    (_, _) if self.valley.exit() == c && is_player => write!(f, "{}┸", Fg(grey)),
                    (_, _) if self.valley.entry() == c => write!(f, "{}╥", Fg(grey)),
                    (_, _) if self.valley.exit() == c => write!(f, "{}╨", Fg(grey)),
                    (_, 0) => write!(f, "{}─", Fg(grey)),
                    (_, y) if y == dims.y => write!(f, "{}─", Fg(grey)),
                    (0, _) => write!(f, "{}│", Fg(grey)),
                    (x, _) if x == dims.x => write!(f, "{}│", Fg(grey)),
                    (_, _) if is_player => write!(f, "{}●", Fg(White)),
                    (_, _) if path.contains(&c) => write!(f, "{}·", Fg(color)),
                    (_, _) if bs.len() >= 3 => write!(f, "{}↺", Fg(grey)),
                    (_, _) if bs.contains(&Up) && bs.contains(&Down) => {
                        write!(f, "{}↕", Fg(grey))
                    }
                    (_, _) if bs.contains(&Left) && bs.contains(&Right) => {
                        write!(f, "{}↔", Fg(grey))
                    }
                    (_, _) if bs.contains(&Up) && bs.contains(&Left) => {
                        write!(f, "{}↖", Fg(grey))
                    }
                    (_, _) if bs.contains(&Up) && bs.contains(&Right) => {
                        write!(f, "{}↗", Fg(grey))
                    }
                    (_, _) if bs.contains(&Down) && bs.contains(&Left) => {
                        write!(f, "{}↙", Fg(grey))
                    }
                    (_, _) if bs.contains(&Down) && bs.contains(&Right) => {
                        write!(f, "{}↘", Fg(grey))
                    }
                    (_, _) if bs.contains(&Up) => write!(f, "{}↑", Fg(grey)),
                    (_, _) if bs.contains(&Down) => write!(f, "{}↓", Fg(grey)),
                    (_, _) if bs.contains(&Left) => write!(f, "{}←", Fg(grey)),
                    (_, _) if bs.contains(&Right) => write!(f, "{}→", Fg(grey)),
                    (_, _) => write!(f, " "),
                }?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

pub fn find_shortest_path(input: &str) -> Result<Vec<State>> {
    let valley = Rc::new(Valley::from_str(input)?);
    let entry = Rc::new(Possibility {
        time: 0,
        coord: valley.entry(),
        target: valley.exit(),
        parent: None,
    });
    let mut seen = HashSet::new();
    let mut valleys = vec![valley];
    let mut queue = [Reverse(entry)].into_iter().collect::<BinaryHeap<_>>();

    while let Some(item) = queue.pop() {
        for option in Possibility::options(&item.0) {
            if option.time >= valleys.len() {
                println!(
                    ">> Depth: {} min, current queue {}",
                    option.time,
                    queue.len()
                );
                valleys.push(Rc::new(valleys.last().unwrap().clone().simulate()));
            }
            let valley = &valleys[option.time];

            if valley.exit() == option.coord {
                // println!("");
                return Ok(reconstruct_path(option, &valleys));
            }
            if !valley.inside(&option.coord) {
                // Motion would lead outside of valley, discard option
                continue;
            }
            if !valley.blizzards(&option.coord).is_empty() {
                // Motion would lead inside a blizzard, discard option
                continue;
            }

            if seen.insert((option.time, option.coord)) {
                // Never been here, should investigate this option next
                queue.push(Reverse(option));
            }
        }
    }
    Err(anyhow!("Could not find solution"))
}

fn reconstruct_path(exit: Rc<Possibility>, valleys: &Vec<Rc<Valley>>) -> Vec<State> {
    let mut path = vec![State {
        possibility: exit.clone(),
        valley: valleys[exit.time].clone(),
    }];
    while let Some(parent) = path.last().unwrap().possibility.parent.clone() {
        path.push(State {
            possibility: parent.clone(),
            valley: valleys[parent.time].clone(),
        });
    }
    path.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn sample_requires_18_min_to_reach_exit() -> Result<()> {
        let path = find_shortest_path(&std::fs::read_to_string("sample.txt")?)?;
        let exit = &path.last().unwrap().possibility;
        assert_eq!(18, exit.time);
        assert_eq!(path[0].valley.exit(), exit.coord);
        Ok(())
    }

    #[test]
    fn sample_solution_has_continuous_path() -> Result<()> {
        let path = find_shortest_path(&std::fs::read_to_string("sample.txt")?)?;

        for (time, (actual, expected)) in path
            .into_iter()
            .map(|state| state.possibility.coord)
            .zip(
                [
                    (1, 0),
                    (1, 1),
                    (1, 2),
                    (1, 2),
                    (1, 1),
                    (2, 1),
                    (3, 1),
                    (3, 2),
                    (2, 2),
                    (2, 1),
                    (3, 1),
                    (3, 1),
                    (3, 2),
                    (3, 3),
                    (4, 3),
                    (5, 3),
                    (6, 3),
                    (6, 4),
                    (6, 5),
                ]
                .into_iter()
                .map(Coord::from),
            )
            .enumerate()
        {
            assert_eq!(expected, actual, "t = {}min", time);
        }

        Ok(())
    }

    #[test]
    fn possibilities_are_ord_by_manhatten_distance() {
        let target = Coord::new(10, 10);
        let c1 = Possibility {
            coord: Coord::new(5, 5),
            time: 0,
            target,
            parent: None,
        };
        let c2 = Possibility {
            coord: Coord::new(4, 5),
            time: 0,
            target,
            parent: None,
        };
        let c3 = Possibility {
            coord: Coord::new(6, 5),
            time: 0,
            target,
            parent: None,
        };
        let c4 = Possibility {
            coord: Coord::new(4, 6),
            time: 0,
            target,
            parent: None,
        };
        assert_eq!(c1.cmp(&c2), Ordering::Less);
        assert_eq!(c1.cmp(&c3), Ordering::Greater);
        assert_eq!(c2.cmp(&c3), Ordering::Greater);
        assert_eq!(c1.cmp(&c4), Ordering::Equal);
    }
}
