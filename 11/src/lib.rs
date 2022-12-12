use std::{cmp::Reverse, collections::HashMap, str::FromStr};

use monkey::{Item, Monkey};

pub mod monkey;

#[derive(Debug, PartialEq)]
pub enum EleventhError {
    InputInvalid(String),
    FileProblem(String),
    NoSuchMonkeyWithId(u32),
}
pub type Monkeys = HashMap<u32, Monkey>;

impl From<nom::error::Error<&str>> for EleventhError {
    fn from(e: nom::error::Error<&str>) -> Self {
        Self::InputInvalid(format!("{}", e))
    }
}

impl From<std::io::Error> for EleventhError {
    fn from(e: std::io::Error) -> Self {
        Self::FileProblem(format!("{}", e))
    }
}

pub fn parse_monkeys_from_file(file: &str) -> Result<Monkeys, EleventhError> {
    std::fs::read_to_string(file)?
        .split_terminator("\n\n")
        .map(|desc| Monkey::from_str(desc).map(|monkey| (monkey.id(), monkey)))
        .collect()
}

pub fn play_round<F>(monkeys: &mut Monkeys, regulator: F) -> Result<(), EleventhError>
where
    F: Fn(Item) -> Item,
{
    for i in 0..(monkeys.len() as u32) {
        // Turns
        loop {
            let monkey = monkeys
                .get_mut(&i)
                .ok_or(EleventhError::NoSuchMonkeyWithId(i))?;
            match monkey.inspect() {
                None => break,
                Some(item) => {
                    let worry_level = regulator(monkey.operation(item));
                    let next_monkey_id = monkey.test(worry_level);
                    monkeys
                        .get_mut(&next_monkey_id)
                        .ok_or(EleventhError::NoSuchMonkeyWithId(next_monkey_id))?
                        .catch(worry_level);
                }
            }
        }
    }
    Ok(())
}

pub fn most_active_monkeys(monkeys: &Monkeys) -> Vec<&Monkey> {
    let mut monkeys = monkeys.values().collect::<Vec<_>>();
    monkeys
        .as_mut_slice()
        .sort_by_key(|monkey| Reverse(monkey.inspections()));
    monkeys
}

pub fn calc_common_modulo(monkeys: &Monkeys) -> u128 {
    monkeys.values().map(|monkey| monkey.modulo()).product()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn sample_amount_of_monkeys_is_four() {
        assert_eq!(
            parse_monkeys_from_file("sample.txt").map(|ms| ms.len()),
            Ok(4)
        );
    }

    fn sample_after_round(round: usize, items: [Vec<Item>; 4]) -> Result<(), EleventhError> {
        let mut monkeys = parse_monkeys_from_file("sample.txt")?;
        let regulator = |x| x / 3;
        for _ in 0..round {
            play_round(&mut monkeys, regulator)?;
        }
        for i in 0..items.len() {
            assert_eq!(monkeys[&(i as u32)].items(), items[i]);
        }
        Ok(())
    }

    #[test]
    fn sample_after_round_1() -> Result<(), EleventhError> {
        sample_after_round(
            1,
            [
                vec![20, 23, 27, 26],
                vec![2080, 25, 167, 207, 401, 1046],
                vec![],
                vec![],
            ],
        )
    }

    #[test]
    fn sample_after_round_2() -> Result<(), EleventhError> {
        sample_after_round(
            2,
            [
                vec![695, 10, 71, 135, 350],
                vec![43, 49, 58, 55, 362],
                vec![],
                vec![],
            ],
        )
    }

    #[test]
    fn sample_after_round_3() -> Result<(), EleventhError> {
        sample_after_round(
            3,
            [
                vec![16, 18, 21, 20, 122],
                vec![1468, 22, 150, 286, 739],
                vec![],
                vec![],
            ],
        )
    }

    #[test]
    fn sample_after_round_4() -> Result<(), EleventhError> {
        sample_after_round(
            4,
            [
                vec![491, 9, 52, 97, 248, 34],
                vec![39, 45, 43, 258],
                vec![],
                vec![],
            ],
        )
    }

    #[test]
    fn sample_after_round_5() -> Result<(), EleventhError> {
        sample_after_round(
            5,
            [
                vec![15, 17, 16, 88, 1037],
                vec![20, 110, 205, 524, 72],
                vec![],
                vec![],
            ],
        )
    }

    #[test]
    fn sample_after_round_20() -> Result<(), EleventhError> {
        sample_after_round(
            20,
            [
                vec![10, 12, 14, 26, 34],
                vec![245, 93, 53, 199, 115],
                vec![],
                vec![],
            ],
        )
    }

    struct ActiveMonkey {
        id: u32,
        inspections: u64,
    }
    fn sample_after_n_rounds_has_most_active_monkeys<F>(
        monkeys: &mut Monkeys,
        round: u32,
        regulator: F,
        most_active: Vec<ActiveMonkey>,
    ) -> Result<(), EleventhError>
    where
        F: Fn(Item) -> Item,
    {
        for _ in 0..round {
            play_round(monkeys, &regulator)?;
        }
        let monkeys = most_active_monkeys(&monkeys);
        for (monkey, ActiveMonkey { id, inspections }) in monkeys.iter().zip(most_active.iter()) {
            assert_eq!(monkey.id(), *id);
            assert_eq!(monkey.inspections(), *inspections);
        }
        Ok(())
    }

    #[test]
    fn part_a_sample_after_round_20_has_0_and_3_most_active_monkeys() -> Result<(), EleventhError> {
        let mut monkeys = parse_monkeys_from_file("sample.txt")?;
        sample_after_n_rounds_has_most_active_monkeys(
            &mut monkeys,
            20,
            |x| x / 3,
            vec![
                ActiveMonkey {
                    id: 3,
                    inspections: 105,
                },
                ActiveMonkey {
                    id: 0,
                    inspections: 101,
                },
            ],
        )
    }

    #[test]
    fn part_b_sample_monkey_buisness_after_round_1() -> Result<(), EleventhError> {
        let mut monkeys = parse_monkeys_from_file("sample.txt")?;
        sample_after_n_rounds_has_most_active_monkeys(
            &mut monkeys,
            1,
            |x| x % 1000,
            vec![
                ActiveMonkey {
                    id: 3,
                    inspections: 6,
                },
                ActiveMonkey {
                    id: 1,
                    inspections: 4,
                },
                ActiveMonkey {
                    id: 2,
                    inspections: 3,
                },
                ActiveMonkey {
                    id: 0,
                    inspections: 2,
                },
            ],
        )
    }

    #[test]
    fn part_b_sample_monkey_buisness_after_round_20() -> Result<(), EleventhError> {
        let mut monkeys = parse_monkeys_from_file("sample.txt")?;
        let common_modulo = calc_common_modulo(&monkeys);
        sample_after_n_rounds_has_most_active_monkeys(
            &mut monkeys,
            20,
            |x| x % common_modulo,
            vec![
                ActiveMonkey {
                    id: 3,
                    inspections: 103,
                },
                ActiveMonkey {
                    id: 0,
                    inspections: 99,
                },
                ActiveMonkey {
                    id: 1,
                    inspections: 97,
                },
                ActiveMonkey {
                    id: 2,
                    inspections: 8,
                },
            ],
        )
    }

    #[test]
    fn part_b_sample_monkey_buisness_after_round_1000() -> Result<(), EleventhError> {
        let mut monkeys = parse_monkeys_from_file("sample.txt")?;
        let common_modulo = calc_common_modulo(&monkeys);
        sample_after_n_rounds_has_most_active_monkeys(
            &mut monkeys,
            1000,
            |x| x % common_modulo,
            vec![
                ActiveMonkey {
                    id: 0,
                    inspections: 5204,
                },
                ActiveMonkey {
                    id: 3,
                    inspections: 5192,
                },
                ActiveMonkey {
                    id: 1,
                    inspections: 4792,
                },
                ActiveMonkey {
                    id: 2,
                    inspections: 199,
                },
            ],
        )
    }
}
