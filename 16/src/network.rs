use crate::{valve::Valve, Name};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashMap},
    iter::once,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Path {
    valves: Vec<Name>,
    distance: i32,
}
impl Path {
    fn new(start: Name) -> Self {
        Self {
            valves: vec![start],
            distance: 0,
        }
    }
    fn append(&self, valve: &Name) -> Self {
        Self {
            valves: self.valves.iter().chain(once(valve)).cloned().collect(),
            distance: self.distance + 1,
        }
    }
    pub fn len(&self) -> i32 {
        self.distance
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Network {
    network: HashMap<Name, Valve>,
    paths: HashMap<(Name, Name), Path>,
}

impl FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let network = s
            .lines()
            .map(Valve::from_str)
            .map_ok(|valve| (valve.name(), valve))
            .collect::<Result<HashMap<Name, Valve>>>()?;

        let mut paths = HashMap::new();
        for (start, target) in network
            .keys()
            .permutations(2)
            .filter(|xs| xs[0] <= xs[1])
            .map(|xs| (xs[0].to_owned(), xs[1].to_owned()))
        {
            // println!("Investigate path from {start} -> {target}");
            // Investigate best path to come from pair[0] -> pair[1]
            let mut queue = BinaryHeap::from([Path::new(start.to_owned())]);
            while let Some(path) = queue.pop() {
                // for each possible candidate path...
                let tip = path.valves.last().unwrap();
                if tip == &target {
                    // println!(
                    //     "Found target with cost {} via {:?}",
                    //     path.distance, path.valves
                    // );
                    paths
                        .entry((start.clone(), target.clone()))
                        .and_modify(|p: &mut Path| {
                            if path.distance < p.distance {
                                *p = path.clone();
                            }
                        })
                        .or_insert(path.clone());
                }

                for valve in network[tip].connections() {
                    if path.valves.contains(&valve) {
                        continue;
                    }
                    let x = path.append(&valve);
                    // println!(" >> {start}->{target}: {x:?}");
                    queue.push(x);
                }
            }
        }

        // for x in paths.keys() {
        // println!("{x:?} -> {:?}", paths[x].valves);
        // }
        Ok(Self { network, paths })
    }
}

impl Network {
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Valve> {
        self.network.iter()
    }
    pub fn keys(&self) -> std::collections::hash_map::Keys<String, Valve> {
        self.network.keys()
    }
    pub fn get<'a>(&'a self, valve: &str) -> &'a Valve {
        &self.network[valve]
    }
    pub fn path(&self, start: &str, target: &str) -> Result<&Path> {
        self.paths
            .get(&(start.to_owned(), target.to_owned()))
            .or(self.paths.get(&(target.to_owned(), start.to_owned())))
            .ok_or(anyhow!("No start '{start}' or target '{target}' valve"))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use anyhow::Result;
    use std::str::FromStr;

    fn sample_path_between(a: &str, b: &str, distance: i32) -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let network = Network::from_str(&sample)?;
        let path = network.path(a, b)?;
        assert_eq!(distance, path.distance);
        Ok(())
    }

    #[test]
    fn sample_path_from_a_to_b() -> Result<()> {
        sample_path_between("AA", "BB", 1)
    }
    #[test]
    fn sample_path_from_a_to_c() -> Result<()> {
        sample_path_between("AA", "CC", 2)
    }
    #[test]
    fn sample_path_from_a_to_d() -> Result<()> {
        sample_path_between("AA", "DD", 1)
    }
    #[test]
    fn sample_path_from_a_to_e() -> Result<()> {
        sample_path_between("AA", "EE", 2)
    }
    #[test]
    fn sample_path_from_a_to_f() -> Result<()> {
        sample_path_between("AA", "FF", 3)
    }
    #[test]
    fn sample_path_from_a_to_g() -> Result<()> {
        sample_path_between("AA", "GG", 4)
    }
    #[test]
    fn sample_path_from_a_to_h() -> Result<()> {
        sample_path_between("AA", "HH", 5)
    }
    #[test]
    fn sample_path_from_a_to_i() -> Result<()> {
        sample_path_between("AA", "II", 1)
    }
    #[test]
    fn sample_path_from_a_to_j() -> Result<()> {
        sample_path_between("AA", "JJ", 2)
    }

    #[test]
    fn sample_path_from_b_to_c() -> Result<()> {
        sample_path_between("BB", "CC", 1)
    }
    #[test]
    fn sample_path_from_b_to_d() -> Result<()> {
        sample_path_between("BB", "DD", 2)
    }
    #[test]
    fn sample_path_from_b_to_e() -> Result<()> {
        sample_path_between("BB", "EE", 3)
    }
    #[test]
    fn sample_path_from_b_to_f() -> Result<()> {
        sample_path_between("BB", "FF", 4)
    }
    #[test]
    fn sample_path_from_b_to_g() -> Result<()> {
        sample_path_between("BB", "GG", 5)
    }
    #[test]
    fn sample_path_from_b_to_h() -> Result<()> {
        sample_path_between("BB", "HH", 6)
    }
    #[test]
    fn sample_path_from_b_to_i() -> Result<()> {
        sample_path_between("BB", "II", 2)
    }
    #[test]
    fn sample_path_from_b_to_j() -> Result<()> {
        sample_path_between("BB", "JJ", 3)
    }

    #[test]
    fn sample_path_from_c_to_d() -> Result<()> {
        sample_path_between("CC", "DD", 1)
    }
    #[test]
    fn sample_path_from_c_to_e() -> Result<()> {
        sample_path_between("CC", "EE", 2)
    }
    #[test]
    fn sample_path_from_c_to_f() -> Result<()> {
        sample_path_between("CC", "FF", 3)
    }
    #[test]
    fn sample_path_from_c_to_g() -> Result<()> {
        sample_path_between("CC", "GG", 4)
    }
    #[test]
    fn sample_path_from_c_to_h() -> Result<()> {
        sample_path_between("CC", "HH", 5)
    }
    #[test]
    fn sample_path_from_c_to_i() -> Result<()> {
        sample_path_between("CC", "II", 3)
    }
    #[test]
    fn sample_path_from_c_to_j() -> Result<()> {
        sample_path_between("CC", "JJ", 4)
    }

    #[test]
    fn sample_path_from_d_to_e() -> Result<()> {
        sample_path_between("DD", "EE", 1)
    }
    #[test]
    fn sample_path_from_d_to_f() -> Result<()> {
        sample_path_between("DD", "FF", 2)
    }
    #[test]
    fn sample_path_from_d_to_g() -> Result<()> {
        sample_path_between("DD", "GG", 3)
    }
    #[test]
    fn sample_path_from_d_to_h() -> Result<()> {
        sample_path_between("DD", "HH", 4)
    }
    #[test]
    fn sample_path_from_d_to_i() -> Result<()> {
        sample_path_between("DD", "II", 2)
    }
    #[test]
    fn sample_path_from_d_to_j() -> Result<()> {
        sample_path_between("DD", "JJ", 3)
    }

    #[test]
    fn sample_path_from_e_to_f() -> Result<()> {
        sample_path_between("EE", "FF", 1)
    }
    #[test]
    fn sample_path_from_e_to_g() -> Result<()> {
        sample_path_between("EE", "GG", 2)
    }
    #[test]
    fn sample_path_from_e_to_h() -> Result<()> {
        sample_path_between("EE", "HH", 3)
    }
    #[test]
    fn sample_path_from_e_to_i() -> Result<()> {
        sample_path_between("EE", "II", 3)
    }
    #[test]
    fn sample_path_from_e_to_j() -> Result<()> {
        sample_path_between("EE", "JJ", 4)
    }

    #[test]
    fn sample_path_from_f_to_g() -> Result<()> {
        sample_path_between("FF", "GG", 1)
    }
    #[test]
    fn sample_path_from_f_to_h() -> Result<()> {
        sample_path_between("FF", "HH", 2)
    }
    #[test]
    fn sample_path_from_f_to_i() -> Result<()> {
        sample_path_between("FF", "II", 4)
    }
    #[test]
    fn sample_path_from_f_to_j() -> Result<()> {
        sample_path_between("FF", "JJ", 5)
    }

    #[test]
    fn sample_path_from_g_to_h() -> Result<()> {
        sample_path_between("GG", "HH", 1)
    }
    #[test]
    fn sample_path_from_g_to_i() -> Result<()> {
        sample_path_between("GG", "II", 5)
    }
    #[test]
    fn sample_path_from_g_to_j() -> Result<()> {
        sample_path_between("GG", "JJ", 6)
    }

    #[test]
    fn sample_path_from_h_to_i() -> Result<()> {
        sample_path_between("HH", "II", 6)
    }
    #[test]
    fn sample_path_from_h_to_j() -> Result<()> {
        sample_path_between("HH", "JJ", 7)
    }

    #[test]
    fn sample_path_from_i_to_j() -> Result<()> {
        sample_path_between("II", "JJ", 1)
    }
}
