use std::{fmt::Debug, str::FromStr};

use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, error::Error,
    multi::separated_list0, sequence::delimited, Finish, IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ThirteenthError {
    InputInvalid(String),
    FileProblem(String),
}

impl From<Error<&str>> for ThirteenthError {
    fn from(e: Error<&str>) -> Self {
        Self::InputInvalid(format!("{}", e))
    }
}

impl From<std::io::Error> for ThirteenthError {
    fn from(e: std::io::Error) -> Self {
        Self::FileProblem(format!("{}", e))
    }
}

#[derive(Clone)]
pub enum Packet {
    Number(u32),
    List(Vec<Packet>),
}
impl Packet {
    pub fn divider(x: u32) -> Self {
        use Packet::*;
        List(vec![List(vec![Number(x)])])
    }
}

fn parse_number(s: &str) -> IResult<&str, Packet> {
    u32(s).map(|(s, x)| (s, Packet::Number(x)))
}

fn parse_list(s: &str) -> IResult<&str, Packet> {
    let (s, list) = delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]"))(s)?;
    Ok((s, Packet::List(list)))
}

fn parse_packet(s: &str) -> IResult<&str, Packet> {
    alt((parse_number, parse_list))(s)
}

impl FromStr for Packet {
    type Err = ThirteenthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_packet(s).finish()?.1)
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{}", x),
            Self::List(xs) => write!(f, "{:?}", xs),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::List(a), Self::List(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Packet::*;
        match (self, other) {
            (Number(a), Number(b)) => a.cmp(&b),
            (List(a), List(b)) => a.cmp(&b),
            (Number(a), List(b)) => vec![Number(*a)].cmp(b),
            (List(a), Number(b)) => a.cmp(&vec![Number(*b)]),
        }
    }
}

pub fn sum_of_right_ordered_packet_indices(content: &str) -> Result<usize, ThirteenthError> {
    content
        .split_terminator("\n\n")
        .flat_map(|two_lines| {
            two_lines
                .split_once("\n")
                .map(|(a, b)| Ok((Packet::from_str(a)?, Packet::from_str(b)?)))
        })
        .enumerate()
        .map(|(i, pair): (_, Result<_, ThirteenthError>)| Ok((i, pair?)))
        .filter_ok(|(_, (a, b))| a < b)
        .map_ok(|(i, _)| i + 1)
        .collect::<Result<Vec<_>, _>>()
        .map(|indices| indices.iter().sum())
}

pub fn divider_packet_indices(
    content: &str,
    dividers: &[Packet],
) -> Result<Vec<usize>, ThirteenthError> {
    let mut packets = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(Packet::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    packets.extend(dividers.iter().cloned());
    packets.as_mut_slice().sort();

    Ok(packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| dividers.contains(&packet))
        .map(|(i, _)| i + 1)
        .collect())
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;
    use Packet::*;

    #[test]
    fn parse_integer() -> Result<(), ThirteenthError> {
        assert_eq!(Packet::from_str("42")?, Number(42));
        Ok(())
    }
    #[test]
    fn parse_empty_list() -> Result<(), ThirteenthError> {
        assert_eq!(Packet::from_str("[]")?, List(Vec::new()));
        Ok(())
    }

    #[test]
    fn parse_singleton_list() -> Result<(), ThirteenthError> {
        assert_eq!(Packet::from_str("[42]")?, List(vec![Number(42)]));
        Ok(())
    }

    #[test]
    fn parse_list_with_two_integers() -> Result<(), ThirteenthError> {
        assert_eq!(
            Packet::from_str("[42,43]")?,
            List(vec![Number(42), Number(43)])
        );
        Ok(())
    }

    #[test]
    fn parse_nested_list_with_integer_and_list() -> Result<(), ThirteenthError> {
        assert_eq!(
            Packet::from_str("[42,[1,10,100]]")?,
            List(vec![
                Number(42),
                List(vec![Number(1), Number(10), Number(100)])
            ]),
        );
        Ok(())
    }

    #[test]
    fn parse_trippely_nested_list() -> Result<(), ThirteenthError> {
        assert_eq!(
            Packet::from_str("[[[]]]")?,
            List(vec![List(vec![List(vec![])])])
        );
        Ok(())
    }

    #[test]
    fn parse_missing_bracket_is_parse_error() {
        assert!(Packet::from_str("[[]").is_err());
    }

    #[test]
    fn parse_extra_bracket_is_parse_error() {
        assert!(Packet::from_str("[1[]").is_err());
    }

    #[test]
    fn parse_extra_comma_is_parse_error() {
        assert!(Packet::from_str("[1,2,,3]").is_err());
    }

    #[test]
    fn compare_integers() {
        let a = Number(42);
        let b = Number(43);
        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn compare_empty_list() {
        let a = List(vec![]);
        let b = List(vec![]);
        assert_eq!(a.cmp(&b), Ordering::Equal);
    }

    #[test]
    fn compare_two_lists_different_length() {
        let a = List((1..=5).map(Number).collect());
        let b = List((1..=6).map(Number).collect());

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }

    #[test]
    fn compare_list_and_integer() {
        let a = List([0, 0, 0].into_iter().map(Number).collect());
        let b = Number(2);
        let c = List(vec![b.clone()]);
        assert_eq!(a.cmp(&b), a.cmp(&c));
    }

    #[test]
    fn compare_sample_pair_1() {
        let a = List([1, 1, 3, 1, 1].into_iter().map(Number).collect());
        let b = List([1, 1, 3, 1, 1].into_iter().map(Number).collect());
        let c = List([1, 1, 5, 1, 1].into_iter().map(Number).collect());
        assert_eq!(a.cmp(&b), Ordering::Equal);
        assert_eq!(a.cmp(&c), Ordering::Less);
        assert_eq!(c.cmp(&a), Ordering::Greater);
    }

    #[test]
    fn compare_sample_pair_2() {
        let a = List(vec![
            List(vec![Number(1)]),
            List(vec![Number(2), Number(3), Number(4)]),
        ]);
        let b = List(vec![List(vec![Number(1)]), Number(4)]);
        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn compare_sample_pair_3() {
        let a = List(vec![Number(9)]);
        let b = List(vec![List(vec![Number(8), Number(7), Number(6)])]);
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn compare_sample_pair_4() {
        let a = List(vec![List(vec![Number(4), Number(4)]), Number(4), Number(4)]);
        let b = List(vec![
            List(vec![Number(4), Number(4)]),
            Number(4),
            Number(4),
            Number(4),
        ]);
        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn compare_sample_pair_5() {
        let a = List([7, 7, 7, 7].into_iter().map(Number).collect());
        let b = List([7, 7, 7].into_iter().map(Number).collect());
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn compare_sample_pair_6() {
        let a = List(vec![]);
        let b = List(vec![Number(3)]);
        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn compare_sample_pair_7() {
        let a = List(vec![List(vec![List(vec![])])]);
        let b = List(vec![List(vec![])]);
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn compare_sample_pair_8() -> Result<(), ThirteenthError> {
        let a = Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]")?;
        let b = Packet::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]")?;
        assert_eq!(a.cmp(&b), Ordering::Greater);
        Ok(())
    }

    #[test]
    fn sample_a() -> Result<(), ThirteenthError> {
        let sample = std::fs::read_to_string("sample.txt")?;
        assert_eq!(sum_of_right_ordered_packet_indices(&sample)?, 13);
        Ok(())
    }

    #[test]
    fn sample_b() -> Result<(), ThirteenthError> {
        let sample = std::fs::read_to_string("sample.txt")?;
        assert_eq!(
            divider_packet_indices(&sample, &[Packet::divider(2), Packet::divider(6)])?,
            vec![10, 14]
        );
        Ok(())
    }
}
