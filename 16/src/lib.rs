pub mod network;
pub mod valve;

use anyhow::{anyhow, Result};
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    iter::once,
};

use crate::network::Network;

pub type Cost = i32;
pub type Name = String;

#[derive(Debug, PartialEq, Eq, Clone)]
struct State<'a> {
    network: &'a Network,
    time: i32,
    valve: String,
    pressure: i32,
    open: HashSet<Name>,
}

impl<'a> State<'a> {
    fn simulate(&self) -> i32 {
        self.open.iter().map(|v| self.network.get(v).flow()).sum()
    }
    fn stay(&self) -> Self {
        Self {
            network: self.network,
            time: self.time + 1,
            valve: self.valve.clone(),
            pressure: self.simulate(),
            open: self.open.clone(),
        }
    }
    fn travel_to(&self, valve: &str) -> Self {
        Self {
            network: self.network,
            time: self.time + 1,
            valve: valve.to_owned(),
            pressure: self.simulate(),
            open: self.open.clone(),
        }
    }
    fn travel_to_and_open(&self, valve: &str) -> Self {
        Self {
            network: self.network,
            time: self.time + 2,
            valve: valve.to_owned(),
            pressure: self.simulate(),
            open: self
                .open
                .iter()
                .cloned()
                .chain(once(valve.to_owned()))
                .collect(),
        }
    }
    pub fn initial(network: &'a Network, valve: &str) -> Self {
        Self {
            network,
            pressure: 0,
            open: once(valve.to_owned()).collect(),
            valve: valve.to_owned(),
            time: 0,
        }
    }

    fn possibilities(&self) -> Vec<Self> {
        let mut options = Vec::new();
        for connection in self.network.get(&self.valve).connections().into_iter() {
            if self.time < 29
                && !self.open.contains(&connection)
                && self.network.get(&connection).flow() > 0
            {
                options.push(self.travel_to_and_open(&connection));
            }
            if self.time < 30 {
                options.push(self.travel_to(&connection));
            }
        }
        if options.is_empty() {
            options.push(self.stay());
        }
        options
    }

    pub fn best(&self) -> (Name, i32) {
        let mut max = self.pressure;
        let mut candidate = self.valve.clone();
        let mut queue = Vec::from([self.clone()]);
        while let Some(item) = queue.pop() {
            if item.time >= 30 {
                if item.pressure > max {
                    max = item.pressure;
                    candidate = item.valve.clone();
                    println!(
                        "Found probably path to {}, p={}, queue={}, open={:?}",
                        item.valve,
                        item.pressure,
                        queue.len(),
                        item.open
                    );
                }
                continue;
            }
            queue.extend(item.possibilities());
        }
        (candidate, max)
    }
}

impl<'a> Display for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "t={}min Valve {}, total pressure {} hPa",
            self.time, self.valve, self.pressure
        )?;
        for x in self.open.iter() {
            writeln!(f, "  - {x} releasing {} hPa", self.network.get(x).flow())?;
        }
        Ok(())
    }
}

pub fn find_max_releasable_pressure(network: &Network, steps: i32) -> Result<i32> {
    let state = State::initial(network, "AA");
    let best = state.best();
    println!("Best = {:?}", best);
    Ok(best.1)

    // println!("{:?}", state.best_option()?);

    // Err(anyhow!("Could not find solution"))
}

#[cfg(test)]
mod tests {

    use super::*;

    use anyhow::Result;
    use std::str::FromStr;

    #[test]
    fn sample_state_possibilities_from_a() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let network = Network::from_str(&sample)?;
        let start = State::initial(&network, "AA");
        let options = start
            .possibilities()
            .into_iter()
            .map(|state| state.valve)
            .collect::<HashSet<_>>();
        assert_eq!(3, options.len());
        assert!(options.contains("BB"), "BB");
        assert!(options.contains("DD"), "DD");
        assert!(options.contains("II"), "II");
        Ok(())
    }

    #[test]
    fn sample_state_possibilities_from_b() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let network = Network::from_str(&sample)?;
        let a = State::initial(&network, "AA");
        let b = a
            .possibilities()
            .into_iter()
            .find(|state| state.valve == "BB")
            .unwrap();
        let options = b
            .possibilities()
            .into_iter()
            .map(|state| (state.valve, state.pressure))
            .collect::<HashSet<_>>();
        println!("{:?}", options);
        assert!(options.contains(&("CC".to_string(), 13)));
        assert_eq!(1, options.len());
        Ok(())
    }

    #[test]
    fn foo() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let network = Network::from_str(&sample)?;
        let a = State::initial(&network, "AA");
        assert_eq!(1, a.best());
        Ok(())
    }

    #[test]
    #[ignore]
    fn sample_can_release_max_1651_in_30min() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let network = Network::from_str(&sample)?;
        let solution = find_max_releasable_pressure(&network, 30)?;
        assert_eq!(1651, solution);
        Ok(())
    }
}
