use std::{cmp::Reverse, collections::HashMap, str::FromStr};

use monkey::Monkey;

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

pub fn play_round(monkeys: &mut Monkeys) -> Result<(), EleventhError> {
    for i in 0..(monkeys.len() as u32) {
        // Turns
        loop {
            let monkey = monkeys
                .get_mut(&i)
                .ok_or(EleventhError::NoSuchMonkeyWithId(i))?;
            match monkey.inspect() {
                None => break,
                Some(item) => {
                    let worry_level = monkey.operation(item) / 3;
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

    fn sample_after_round(round: usize, items: [Vec<u32>; 4]) -> Result<(), EleventhError> {
        let mut monkeys = parse_monkeys_from_file("sample.txt")?;
        for _ in 0..round {
            play_round(&mut monkeys)?;
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

    #[test]
    fn sample_after_round_20_has_0_and_3_most_active_monkeys() -> Result<(), EleventhError> {
        let mut monkeys = parse_monkeys_from_file("sample.txt")?;
        for _ in 0..20 {
            play_round(&mut monkeys)?;
        }
        let monkeys = most_active_monkeys(&monkeys);
        assert_eq!(monkeys[0].id(), 3);
        assert_eq!(monkeys[0].inspections(), 105);
        assert_eq!(monkeys[1].id(), 0);
        assert_eq!(monkeys[1].inspections(), 101);

        Ok(())
    }
}
