use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::once,
};

use blueprint::Blueprint;
use solver::{Cost, A};

pub mod blueprint;
pub mod solver;

pub type Material = usize;

const ORE: Material = 0;
const CLAY: Material = 1;
const OBSIDIAN: Material = 2;
const GEODE: Material = 3;

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    time: i32,
    time_max: i32,
    materials: [Material; 4],
    robots: [Material; 4],
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}min | Cost: {:?} | Materials: {:?} | Robots: {:?}",
            self.time,
            self.cost(),
            self.materials,
            self.robots
        )
    }
}
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02} min", self.time)
    }
}
impl Cost for State {
    fn cost(&self) -> i32 {
        self.max_geodes() + self.max_geode_potential()
    }
}

impl State {
    fn remaining(&self) -> i32 {
        self.time_max - self.time
    }
    /// Maximum producable geodes with the robots available, if we wouldn't by new ones
    fn max_geodes(&self) -> i32 {
        self.materials[GEODE] as i32 + self.remaining() * self.robots[GEODE] as i32
    }

    /// Maximum producable geodes (upper limit) in the remaining time left, if we can buy each robot we want
    fn max_geode_potential(&self) -> i32 {
        let t = self.remaining();
        t * (t - 1) / 2
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost().cmp(&other.cost())
    }
}

fn buy(blueprint: &Blueprint, robot: Material, state: &State) -> State {
    let time = state.time + 1;
    let time_max = state.time_max;
    let materials = [
        state.materials[ORE] + state.robots[ORE] - blueprint.cost(robot, ORE) as usize,
        state.materials[CLAY] + state.robots[CLAY] - blueprint.cost(robot, CLAY) as usize,
        state.materials[OBSIDIAN] + state.robots[OBSIDIAN]
            - blueprint.cost(robot, OBSIDIAN) as usize,
        state.materials[GEODE] + state.robots[GEODE] - blueprint.cost(robot, GEODE) as usize,
    ];
    let mut robots = state.robots.clone();
    robots[robot] += 1;
    State {
        time,
        time_max,
        materials,
        robots,
    }
}

fn wait(state: &State) -> State {
    State {
        time: state.time + 1,
        time_max: state.time_max,
        materials: [
            state.materials[ORE] + state.robots[ORE],
            state.materials[CLAY] + state.robots[CLAY],
            state.materials[OBSIDIAN] + state.robots[OBSIDIAN],
            state.materials[GEODE] + state.robots[GEODE],
        ],
        robots: state.robots,
    }
}

fn possible_options(blueprint: &Blueprint, state: &State) -> Vec<State> {
    let mut affordable = blueprint.affordable_robots(&state.materials);

    for material in [ORE, CLAY, OBSIDIAN] {
        // Already enough robots, don't need to investigate branch to produce one more
        if state.robots[material] >= blueprint.max_robots_needed_for(material) {
            affordable.remove(&material);
        }
    }

    affordable
        .into_iter()
        .map(|robot| buy(blueprint, robot, state))
        .chain(once(wait(state)))
        .collect()
}

pub fn solve(blueprint: &Blueprint, time: i32) -> u32 {
    let mut solver = A::<State>::new();
    let path = solver.solve(
        |s| possible_options(blueprint, s),
        &State {
            time: 2,
            time_max: time,
            materials: [2, 0, 0, 0],
            robots: [1, 0, 0, 0],
        },
        |s| s.time == time,
    );
    path.last().unwrap().materials[GEODE] as u32
}

#[cfg(test)]
mod tests {

    use anyhow::Result;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_solve() -> Result<()> {
        let blueprints = std::fs::read_to_string("sample.txt")?
            .split_terminator("\n\n")
            .map(Blueprint::from_str)
            .collect::<Result<Vec<_>>>()?;
        assert_eq!(blueprints[0].quality_level(24), 1 * 9);
        assert_eq!(blueprints[1].quality_level(24), 2 * 12);
        Ok(())
    }

    #[test]
    fn test_solve_b() -> Result<()> {
        let blueprints = std::fs::read_to_string("sample.txt")?
            .split_terminator("\n\n")
            .map(Blueprint::from_str)
            .collect::<Result<Vec<_>>>()?;
        assert_eq!(solve(&blueprints[0], 32), 56);
        assert_eq!(solve(&blueprints[1], 32), 62);
        Ok(())
    }

    #[test]
    fn buy_ore_robot() -> Result<()> {
        let ore_robot_cost = 1;
        let blueprint = Blueprint::from_str(&format!(
            "Blueprint 0: Each ore robot costs {} ore.",
            ore_robot_cost
        ))?;
        let initial = State {
            time: 0,
            time_max: 10,
            materials: [17, 0, 0, 0],
            robots: [0, 0, 0, 0],
        };
        let state = buy(&blueprint, ORE, &initial);
        assert_eq!(state.time, initial.time + 1);
        assert_eq!(
            state.materials[ORE],
            initial.materials[ORE] - ore_robot_cost
        );
        Ok(())
    }

    #[test]
    fn possible_options_include_doing_nothing() -> Result<()> {
        let blueprint = Blueprint::from_str("Blueprint 0: Each ore robot costs 1 ore.")?;
        let options = possible_options(
            &blueprint,
            &State {
                time: 0,
                time_max: 10,
                materials: [0, 0, 0, 0],
                robots: [1, 0, 0, 0],
            },
        );
        println!("{:#?}", options);

        assert!(options.contains(&State {
            time: 1,
            time_max: 10,
            materials: [1, 0, 0, 0],
            robots: [1, 0, 0, 0],
        }));
        Ok(())
    }

    #[test]
    fn possible_options_include_to_buy_robot_if_affordable() -> Result<()> {
        let ore_robot_cost = 3;
        let blueprint = Blueprint::from_str(&format!(
            "Blueprint 0: Each ore robot costs {} ore.",
            ore_robot_cost
        ))?;
        let options = possible_options(
            &blueprint,
            &State {
                time: 0,
                time_max: 10,
                materials: [ore_robot_cost, 0, 0, 0],
                robots: [1, 0, 0, 0],
            },
        );

        assert!(options.contains(&State {
            time: 1,
            time_max: 10,
            materials: [1, 0, 0, 0],
            robots: [2, 0, 0, 0],
        }));
        Ok(())
    }
}
