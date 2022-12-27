use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{multispace1, newline, space1, u32},
    combinator::{fail, opt},
    multi::separated_list0,
    Finish, IResult,
};

use crate::{solve, Material, CLAY, GEODE, OBSIDIAN, ORE};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Robot {
    costs: [Material; 4],
    mining: Material,
}
impl Robot {
    fn new(mining: Material) -> Self {
        Self {
            mining,
            costs: [0; 4],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Blueprint {
    id: u32,
    robots: [Robot; 4],
}

impl Blueprint {
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn affordable_robots(&self, materials: &[Material; 4]) -> HashSet<Material> {
        self.robots
            .iter()
            .filter(|robot| {
                robot
                    .costs
                    .iter()
                    .zip(materials.iter())
                    .all(|(c, m)| m >= c)
            })
            .map(|robot| robot.mining)
            .collect()
    }

    /// How much of a certain `material` type does a certain `robot` type cost?
    pub fn cost(&self, robot: Material, material: Material) -> u32 {
        self.robots[robot as usize].costs[material as usize] as u32
    }

    /// How many robots of `material` are needed to constantly produce any robot
    pub fn max_robots_needed_for(&self, material: Material) -> usize {
        self.robots
            .iter()
            .map(|robot| robot.costs[material])
            .max()
            .unwrap_or_default()
    }

    pub fn quality_level(&self, time_limit: i32) -> u32 {
        self.id() * solve(self, time_limit)
    }
}

fn whitespace(s: &str) -> IResult<&str, ()> {
    let (s, _) = opt(newline)(s)?;
    let (s, _) = multispace1(s)?;
    Ok((s, ()))
}

fn parse_material(s: &str) -> IResult<&str, Material> {
    let (s, word) = take_while1(char::is_alphabetic)(s)?;
    if word == "ore" {
        return Ok((s, ORE));
    }
    if word == "clay" {
        return Ok((s, CLAY));
    }
    if word == "obsidian" {
        return Ok((s, OBSIDIAN));
    }
    if word == "geode" {
        return Ok((s, GEODE));
    }
    fail(s)
}
fn parse_cost(s: &str) -> IResult<&str, (Material, u32)> {
    let (s, amount) = u32(s)?;
    let (s, _) = space1(s)?;
    let (s, unit) = parse_material(s)?;
    Ok((s, (unit, amount)))
}
fn parse_costs(s: &str) -> IResult<&str, [Material; 4]> {
    let (s, list) = separated_list0(tag(" and "), parse_cost)(s)?;
    let mut costs = [0; 4];
    for (unit, amount) in list {
        costs[unit as usize] = amount as Material;
    }
    Ok((s, costs))
}

fn parse_robot(s: &str) -> IResult<&str, Robot> {
    let (s, _) = tag("Each ")(s)?;
    let (s, mining) = parse_material(s)?;
    let (s, _) = tag(" robot costs ")(s)?;
    let (s, costs) = parse_costs(s)?;
    let (s, _) = tag(".")(s)?;
    Ok((s, Robot { costs, mining }))
}

fn parse_blueprint(s: &str) -> IResult<&str, Blueprint> {
    let (s, _) = tag("Blueprint ")(s)?;
    let (s, id) = u32(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = whitespace(s)?;
    let mut robots = [
        Robot::new(ORE),
        Robot::new(CLAY),
        Robot::new(OBSIDIAN),
        Robot::new(GEODE),
    ];
    let (s, rs) = separated_list0(whitespace, parse_robot)(s)?;
    for robot in rs {
        robots[robot.mining as usize] = robot.clone();
    }

    Ok((s, Blueprint { id, robots }))
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, blueprint) = parse_blueprint(s)
            .finish()
            .map_err(|e| anyhow!("{}", e.to_string()))?;
        Ok(blueprint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blueprint_from_str() -> Result<()> {
        let blueprint = Blueprint::from_str("Blueprint 0: Each ore robot costs 1 ore.")?;

        assert_eq!(
            blueprint,
            Blueprint {
                id: 0,
                robots: [
                    Robot {
                        mining: ORE,
                        costs: [1, 0, 0, 0]
                    },
                    Robot::new(CLAY),
                    Robot::new(OBSIDIAN),
                    Robot::new(GEODE),
                ]
            }
        );
        Ok(())
    }

    #[test]
    fn blueprint_from_str_sample() -> Result<()> {
        let blueprints = std::fs::read_to_string("sample.txt")?
            .split_terminator("\n\n")
            .map(Blueprint::from_str)
            .collect::<Result<Vec<_>>>()?;

        assert_eq!(blueprints.len(), 2);
        assert_eq!(blueprints[0].id, 1);
        assert_eq!(
            blueprints[0].robots[ORE],
            Robot {
                mining: ORE,
                costs: [4, 0, 0, 0],
            }
        );
        assert_eq!(
            blueprints[0].robots[CLAY],
            Robot {
                mining: CLAY,
                costs: [2, 0, 0, 0],
            }
        );
        assert_eq!(
            blueprints[0].robots[OBSIDIAN],
            Robot {
                mining: OBSIDIAN,
                costs: [3, 14, 0, 0],
            }
        );
        assert_eq!(
            blueprints[0].robots[GEODE],
            Robot {
                mining: GEODE,
                costs: [2, 0, 7, 0],
            }
        );

        Ok(())
    }

    #[test]
    fn blueprint_can_affort() -> Result<()> {
        let blueprints = std::fs::read_to_string("sample.txt")?
            .split_terminator("\n\n")
            .map(Blueprint::from_str)
            .collect::<Result<Vec<_>>>()?;

        assert_eq!(
            blueprints[0].affordable_robots(&[3, 0, 0, 0]),
            HashSet::from([CLAY])
        );
        assert_eq!(
            blueprints[0].affordable_robots(&[4, 0, 0, 0]),
            HashSet::from([CLAY, ORE])
        );
        assert_eq!(
            blueprints[0].affordable_robots(&[4, 20, 0, 0]),
            HashSet::from([CLAY, ORE, OBSIDIAN])
        );
        assert_eq!(
            blueprints[0].affordable_robots(&[2, 0, 10, 0]),
            HashSet::from([CLAY, GEODE])
        );
        Ok(())
    }
}
