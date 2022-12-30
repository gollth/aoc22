use anyhow::{anyhow, Error, Result};
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, i64},
    Finish, IResult,
};
use std::str::FromStr;

pub type Number = i64;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Plus,
    Minus,
    Times,
    Divide,
}

impl Operation {
    pub fn call(&self, a: Number, b: Number) -> Number {
        match self {
            Operation::Plus => a + b,
            Operation::Minus => a - b,
            Operation::Times => a * b,
            Operation::Divide => a / b,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Monkey {
    Lone {
        name: String,
        number: Number,
    },
    Math {
        name: String,
        a: String,
        op: Operation,
        b: String,
    },
}
impl Monkey {
    pub fn name(&self) -> String {
        match self {
            Monkey::Lone { name, number: _ } => name.clone(),
            Monkey::Math {
                name,
                a: _,
                op: _,
                b: _,
            } => name.clone(),
        }
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_monkey(s).finish().map_err(|e| anyhow!("{}", e))?.1)
    }
}

fn parse_id(s: &str) -> IResult<&str, String> {
    take_while1(|c| matches!(c, 'a'..='z'))(s).map(|(s, id)| (s, id.to_owned()))
}
fn whitespace(s: &str) -> IResult<&str, ()> {
    Ok((char(' ')(s)?.0, ()))
}
fn parse_plus(s: &str) -> IResult<&str, Operation> {
    char('+')(s).map(|(s, _)| (s, Operation::Plus))
}
fn parse_minus(s: &str) -> IResult<&str, Operation> {
    char('-')(s).map(|(s, _)| (s, Operation::Minus))
}
fn parse_times(s: &str) -> IResult<&str, Operation> {
    char('*')(s).map(|(s, _)| (s, Operation::Times))
}
fn parse_divide(s: &str) -> IResult<&str, Operation> {
    char('/')(s).map(|(s, _)| (s, Operation::Divide))
}

fn parse_lone(s: &str) -> IResult<&str, Monkey> {
    let (s, name) = parse_id(s)?;
    let (s, _) = char(':')(s)?;
    let (s, _) = whitespace(s)?;
    let (s, number) = i64(s)?;
    Ok((s, Monkey::Lone { name, number }))
}

fn parse_math(s: &str) -> IResult<&str, Monkey> {
    let (s, name) = parse_id(s)?;
    let (s, _) = char(':')(s)?;
    let (s, _) = whitespace(s)?;
    let (s, a) = parse_id(s)?;
    let (s, _) = whitespace(s)?;
    let (s, op) = alt((parse_plus, parse_minus, parse_times, parse_divide))(s)?;
    let (s, _) = whitespace(s)?;
    let (s, b) = parse_id(s)?;
    Ok((s, Monkey::Math { name, a, op, b }))
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    alt((parse_lone, parse_math))(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lone_from_str() -> Result<()> {
        assert_eq!(
            Monkey::from_str("abcd: 42")?,
            Monkey::Lone {
                name: "abcd".to_owned(),
                number: 42
            }
        );
        Ok(())
    }

    #[test]
    fn math_from_str() -> Result<()> {
        assert_eq!(
            Monkey::from_str("abcd: efgh + ijkl")?,
            Monkey::Math {
                name: "abcd".to_owned(),
                a: "efgh".to_owned(),
                op: Operation::Plus,
                b: "ijkl".to_owned()
            }
        );
        Ok(())
    }
}
