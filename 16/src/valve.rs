use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};

use crate::Name;

#[derive(PartialEq, Eq, Debug)]
pub struct Valve {
    name: Name,
    flow: i32,
    connections: HashSet<Name>,
}

impl Valve {
    fn new(name: &str, flow: u32, connections: Vec<&str>) -> Self {
        Self {
            name: name.to_owned(),
            flow: flow as i32,
            connections: connections.into_iter().map(String::from).collect(),
        }
    }
    pub fn name(&self) -> Name {
        self.name.clone()
    }

    pub fn flow(&self) -> i32 {
        self.flow
    }
    pub fn connections(&self) -> HashSet<Name> {
        self.connections.clone()
    }

    pub fn connects(&self, other: &str) -> bool {
        self.connections.contains(other)
    }
}

impl From<(&str, u32, Vec<&str>)> for Valve {
    fn from((name, flow, connections): (&str, u32, Vec<&str>)) -> Self {
        Self::new(name, flow, connections)
    }
}

impl FromStr for Valve {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(s: &str) -> IResult<&str, Valve> {
            map(
                tuple((
                    preceded(tag("Valve "), alpha1),
                    preceded(tag(" has flow rate="), u32),
                    preceded(
                        alt((
                            tag("; tunnel leads to valve "),
                            tag("; tunnels lead to valves "),
                        )),
                        separated_list1(tag(", "), alpha1),
                    ),
                )),
                Valve::from,
            )(s)
        }
        Ok(parse(s)
            .finish()
            .map_err(|e| anyhow!("{}", e.to_string()))?
            .1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valve_from_str() -> Result<()> {
        let valve = Valve::from_str("Valve A has flow rate=42; tunnels lead to valves B, C, D")?;
        assert_eq!("A", valve.name);
        assert_eq!(42, valve.flow);
        assert!(valve.connects("B"));
        assert!(valve.connects("C"));
        assert!(valve.connects("D"));
        Ok(())
    }

    #[test]
    fn valve_from_str_sample() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let valves = sample
            .lines()
            .map(Valve::from_str)
            .collect::<Result<Vec<_>>>()?;

        let expectations = vec![
            Valve::new("AA", 0, vec!["DD", "II", "BB"]),
            Valve::new("BB", 13, vec!["CC", "AA"]),
            Valve::new("CC", 2, vec!["DD", "BB"]),
            Valve::new("DD", 20, vec!["CC", "AA", "EE"]),
            Valve::new("EE", 3, vec!["FF", "DD"]),
            Valve::new("FF", 0, vec!["EE", "GG"]),
            Valve::new("GG", 0, vec!["FF", "HH"]),
            Valve::new("HH", 22, vec!["GG"]),
            Valve::new("II", 0, vec!["AA", "JJ"]),
            Valve::new("JJ", 21, vec!["II"]),
        ];

        assert_eq!(expectations, valves);

        Ok(())
    }
}
