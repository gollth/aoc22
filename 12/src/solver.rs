use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    iter::once,
};

use colors_transform::{Color, Hsl};
use termion::color::{Fg, Reset, Rgb};

use crate::{grid::Heightmap, Coord, TwelfthError};

pub type Path = Vec<Coord>;

#[derive(Debug)]
pub struct Dijkstra<'a> {
    unvisited: VecDeque<Coord>,
    visited: HashMap<Coord, Option<Coord>>,
    map: &'a Heightmap,
    path: Option<Path>,
}

impl<'a> Display for Dijkstra<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (w, h) = self.map.dimension();

        write!(f, "╭")?;
        for _ in 0..w {
            write!(f, "─")?;
        }
        writeln!(f, "╮")?;

        for y in 0..h {
            write!(f, "│")?;
            for x in 0..w {
                let coord = Coord::new(x as i32, y as i32);
                if coord == self.map.start() {
                    write!(f, "{}⚑{}", Fg(Rgb(0, 85, 255)), Fg(Reset))?;
                } else if coord == self.map.finish() {
                    write!(f, "{}⚑{}", Fg(Rgb(255, 0, 0)), Fg(Reset))?;
                } else if self.unvisited.contains(&coord) {
                    write!(f, "{}●{}", Fg(Rgb(160, 160, 160)), Fg(Reset))?;
                } else if self.path.as_ref().unwrap_or(&vec![]).contains(&coord) {
                    let elevation = self.map.elevation(coord).unwrap().1;
                    let hsv = Hsl::from(
                        220. * (1. - (elevation as u8 - 'a' as u8) as f32 / 26.),
                        100.,
                        50.,
                    );
                    let color = Rgb(
                        hsv.get_red() as u8,
                        hsv.get_green() as u8,
                        hsv.get_blue() as u8,
                    );

                    write!(f, "{}{}{}", Fg(color), elevation, Fg(Reset))?;
                } else if self.visited.contains_key(&coord) {
                    let elevation = self.map.elevation(coord).unwrap().1;
                    write!(f, "{}", elevation)?;
                } else {
                    write!(f, "{}·{}", Fg(Rgb(128, 128, 128)), Fg(Reset))?;
                }
            }
            writeln!(f, "{}│", Fg(Reset))?;
        }

        write!(f, "╰")?;
        for _ in 0..w {
            write!(f, "─")?;
        }
        writeln!(f, "╯")?;
        Ok(())
    }
}

impl<'a> Dijkstra<'a> {
    pub fn new(map: &'a Heightmap) -> Self {
        Self {
            map,
            unvisited: once(map.start()).collect(),
            visited: HashMap::from([(map.start(), None)]),
            path: None,
        }
    }
    pub fn solve_once(&mut self) -> Result<bool, TwelfthError> {
        let current = self
            .unvisited
            .pop_front()
            .ok_or(TwelfthError::SolverCouldNotFindASolutionToTarget)?;

        if self.map.finish() == current {
            let mut path = vec![current];
            let mut x = current;
            while let Some(parent) = self.visited.get(&x).expect("Finish not yet visited") {
                path.push(*parent);
                x = *parent;
            }
            self.path = Some(path.into_iter().rev().collect());
            return Ok(true);
        }

        let (_, current_cost) = self.map.elevation(current).unwrap();

        for (neighbor, cost) in [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(x, y)| current + Coord::new(x, y))
            .flat_map(|coord| self.map.elevation(coord))
        {
            if cost as u8 <= current_cost as u8 + 1 && !self.visited.contains_key(&neighbor) {
                self.visited.insert(neighbor, Some(current));
                self.unvisited.push_back(neighbor);
                continue;
            }
        }

        Ok(false)
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref()
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn dijkstra_new_assigns_zero_distance_to_start_node() -> Result<(), TwelfthError> {
        let map = Heightmap::from_str(&std::fs::read_to_string("sample.txt")?)?;
        let solver = Dijkstra::new(&map);
        assert_eq!(solver.unvisited, vec![map.start()]);
        Ok(())
    }

    #[test]
    fn dijkstra_solve_1st() -> Result<(), TwelfthError> {
        let map = Heightmap::from_str(&std::fs::read_to_string("sample.txt")?)?;
        let mut solver = Dijkstra::new(&map);

        solver.solve_once()?;

        assert!(solver.visited.contains_key(&map.start()));
        assert_eq!(solver.unvisited, vec![Coord::new(1, 0), Coord::new(0, 1)]);

        Ok(())
    }

    #[test]
    fn dijkstra_solve_2nd() -> Result<(), TwelfthError> {
        let map = Heightmap::from_str(&std::fs::read_to_string("sample.txt")?)?;
        let mut solver = Dijkstra::new(&map);

        solver.solve_once()?;
        solver.solve_once()?;

        assert!(solver.visited.contains_key(&Coord::new(1, 0)));
        assert_eq!(
            solver.unvisited,
            vec![Coord::new(0, 1), Coord::new(2, 0), Coord::new(1, 1)]
        );

        Ok(())
    }

    #[test]
    fn dijkstra_solve_3rd() -> Result<(), TwelfthError> {
        let map = Heightmap::from_str(&std::fs::read_to_string("sample.txt")?)?;
        let mut solver = Dijkstra::new(&map);

        solver.solve_once()?;
        solver.solve_once()?;
        solver.solve_once()?;

        assert!(solver.visited.contains_key(&Coord::new(0, 1)));
        assert_eq!(
            solver.unvisited,
            vec![Coord::new(2, 0), Coord::new(1, 1), Coord::new(0, 2)]
        );

        Ok(())
    }

    #[test]
    fn dijkstra_solve_4th() -> Result<(), TwelfthError> {
        let map = Heightmap::from_str(&std::fs::read_to_string("sample.txt")?)?;
        let mut solver = Dijkstra::new(&map);

        solver.solve_once()?;
        solver.solve_once()?;
        solver.solve_once()?;
        solver.solve_once()?;

        assert!(solver.visited.contains_key(&Coord::new(2, 0)));
        assert_eq!(
            solver.unvisited,
            vec![Coord::new(1, 1), Coord::new(0, 2), Coord::new(2, 1)]
        );

        Ok(())
    }

    #[test]
    fn dijkstra_solve_sample() -> Result<(), TwelfthError> {
        let map = Heightmap::from_str(&std::fs::read_to_string("sample.txt")?)?;
        let mut solver = Dijkstra::new(&map);

        while !solver.solve_once()? {}

        assert_eq!(solver.path().map(|p| p.len()), Some(32));
        Ok(())
    }
}
