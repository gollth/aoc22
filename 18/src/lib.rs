use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::Result;
use itertools::Itertools;
use ndarray::prelude::*;

#[derive(PartialEq, Eq, Clone)]
enum Material {
    Air,
    // Boundary,
    Lava,
}
impl Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "·"),
            Self::Lava => write!(f, "█"),
        }
    }
}

pub struct Lavablob {
    volume: Array3<Material>,
}

impl Lavablob {
    pub fn surface_area(&self) -> usize {
        let mut boundaries = 0;
        let dim = self.volume.dim();
        for x in 0..dim.0 {
            for y in 0..dim.1 {
                for z in 0..dim.2 {
                    let c = [x, y, z];
                    boundaries += self.boundaries(&arr1(&c));
                }
            }
        }
        boundaries
    }

    fn boundaries(&self, c: &Array1<usize>) -> usize {
        use Material::*;
        let dim = self.volume.dim();
        let right = arr1(&[1, 0, 0]);
        let up = arr1(&[0, 1, 0]);
        let fore = arr1(&[0, 0, 1]);
        if self.volume[index(c.clone())] != Lava {
            return 0;
        }
        let mut n = 0;
        if c[0] == dim.0 - 1 || self.volume[index(c + right.clone())] == Air {
            // Right side is boundary
            n += 1;
        }
        if c[0] == 0 || self.volume[index(c - right)] == Air {
            // Left side is boundary
            n += 1;
        }

        if c[1] == dim.1 - 1 || self.volume[index(c + up.clone())] == Air {
            // Top side is boundary
            n += 1;
        }
        if c[1] == 0 || self.volume[index(c - up)] == Air {
            // Bottom side is boundary
            n += 1;
        }

        if c[2] == dim.2 - 1 || self.volume[index(c + fore.clone())] == Air {
            // Front side is boundary
            n += 1;
        }
        if 0 < c[2] && self.volume[index(c - fore)] == Air {
            // Rear side is boundary
            n += 1;
        }

        n
    }
}

fn index(c: Array1<usize>) -> [usize; 3] {
    [c[0], c[1], c[2]]
}

impl FromStr for Lavablob {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blobs = s
            .lines()
            .flat_map(|line| {
                line.split_terminator(",")
                    .flat_map(|n| n.parse::<usize>())
                    .collect_tuple()
            })
            .map(|(x, y, z)| arr1(&[x, y, z]))
            .collect::<Vec<_>>();
        let dim = (
            blobs.iter().map(|c| c[0]).max().unwrap_or_default() + 1,
            blobs.iter().map(|c| c[1]).max().unwrap_or_default() + 1,
            blobs.iter().map(|c| c[2]).max().unwrap_or_default() + 1,
        );
        let mut volume = Array3::from_elem(dim, Material::Air);
        for c in blobs {
            volume[index(c)] = Material::Lava;
        }
        Ok(Self { volume })
    }
}

impl Display for Lavablob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let dim = self.volume.dim();
        write!(f, "{:?}", self.volume.slice(s![.., .., 0]))?;
        Ok(())
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
        assert_eq!(lava.volume.dim(), (2, 3, 4));
        assert_eq!(lava.volume[[1, 2, 3]], Material::Lava);
    }

    #[test]
    fn lavablob_sample_dimensions() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let lava = Lavablob::from_str(&sample)?;
        assert_eq!(lava.volume.dim(), (4, 4, 7));
        assert_eq!(
            lava.volume.iter().filter(|v| **v == Material::Lava).count(),
            sample.lines().count()
        );
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
}
