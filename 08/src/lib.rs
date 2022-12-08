use ndarray::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    num::ParseIntError,
};
type Coord = Array1<isize>;
pub type Forest = Array2<i8>;
pub type VisibilityGrid = Array2<i8>;

#[derive(Debug)]
pub enum EigthError {
    CannotParseTreeHeight(ParseIntError),
    EmptyGrid,
}

impl Error for EigthError {}
impl Display for EigthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

pub fn parse_forest(content: &str) -> Result<Forest, EigthError> {
    let grid = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_string()
                        .parse::<i8>()
                        .map_err(EigthError::CannotParseTreeHeight)
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let h = grid.len();
    let w = grid
        .get(0)
        .ok_or(EigthError::EmptyGrid)
        .map(|row| row.len())?;

    let mut forest = Forest::default((w, h));
    for (i, mut row) in forest.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = grid[i][j];
        }
    }
    Ok(forest)
}

const NOT_VISIBLE: i8 = -1;

const N: (isize, isize) = (0, -1);
const S: (isize, isize) = (0, 1);
const E: (isize, isize) = (1, 0);
const W: (isize, isize) = (-1, 0);

pub fn visible_trees(forest: &Forest) -> Forest {
    let dim = forest.shape()[0]; // square
    let idim = dim as isize;

    let rays = vec![N, S, E, W]
        .iter()
        .map(|dir| {
            (1..(dim - 1)).map(|i| {
                (
                    arr1(&[dir.0, dir.1]),
                    Coord::from_iter(match *dir {
                        N => [i as isize, idim - 2],
                        S => [i as isize, 1],
                        E => [1, i as isize],
                        W => [idim - 2, i as isize],
                        _ => unreachable!(),
                    }),
                )
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut visibility = forest.clone();

    // Mark inner rect with "not visible" yet
    visibility
        .slice_mut(s![1..(dim - 1), 1..(dim - 1)])
        .fill(NOT_VISIBLE);

    for (dir, point) in rays {
        // Raycast
        for i in 0..dim - 2 {
            let highest = (0..=i)
                .map(|j| forest[index(&point, ((j as isize) - 1) * &dir).unwrap()])
                .max()
                .unwrap_or(0);
            let ux = index(&point, (i as isize) * &dir).unwrap();
            if forest[ux] > highest {
                visibility[ux] = forest[ux];
            }
        }
    }

    visibility
}

fn index(point: &Coord, dir: Coord) -> Option<(usize, usize)> {
    let ix = point + dir;
    if ix[0] < 0 || ix[1] < 0 {
        None
    } else {
        Some((ix[0] as usize, ix[1] as usize))
    }
}

pub fn count_visible(visibile_trees: &VisibilityGrid) -> usize {
    visibile_trees
        .as_slice()
        .unwrap()
        .into_iter()
        .cloned()
        .filter(|h| *h != NOT_VISIBLE)
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample_a() -> Result<(), Box<dyn Error>> {
        let content = std::fs::read_to_string("sample.txt")?;

        let forest = parse_forest(&content)?;
        assert_eq!(forest.shape(), [5, 5]);

        let visible_trees = visible_trees(&forest);
        assert_eq!(visible_trees.shape(), [5, 5]);
        assert_eq!(count_visible(&visible_trees), 21);

        Ok(())
    }
}
