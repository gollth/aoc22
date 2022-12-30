use anyhow::{anyhow, Error, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::i64,
    Finish, IResult,
};
use savage_core::{
    expression::Expression,
    helpers::{eq, int, var},
};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monkey {
    name: String,
    expr: Expression,
}
impl Monkey {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn expression(&self) -> &Expression {
        &self.expr
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
fn parse_plus(s: &str) -> IResult<&str, Expression> {
    let (s, a) = parse_id(s)?;
    let (s, _) = tag(" + ")(s)?;
    let (s, b) = parse_id(s)?;
    Ok((s, var(a) + var(b)))
}
fn parse_minus(s: &str) -> IResult<&str, Expression> {
    let (s, a) = parse_id(s)?;
    let (s, _) = tag(" - ")(s)?;
    let (s, b) = parse_id(s)?;
    Ok((s, var(a) - var(b)))
}
fn parse_times(s: &str) -> IResult<&str, Expression> {
    let (s, a) = parse_id(s)?;
    let (s, _) = tag(" * ")(s)?;
    let (s, b) = parse_id(s)?;
    Ok((s, var(a) * var(b)))
}
fn parse_divide(s: &str) -> IResult<&str, Expression> {
    let (s, a) = parse_id(s)?;
    let (s, _) = tag(" / ")(s)?;
    let (s, b) = parse_id(s)?;
    Ok((s, var(a) / var(b)))
}
fn parse_equals(s: &str) -> IResult<&str, Expression> {
    let (s, a) = parse_id(s)?;
    let (s, _) = tag(" = ")(s)?;
    let (s, b) = parse_id(s)?;
    Ok((s, eq(var(a), var(b))))
}

fn parse_constant(s: &str) -> IResult<&str, Expression> {
    i64(s).map(|(s, x)| (s, int(x)))
}
fn parse_variable(s: &str) -> IResult<&str, Expression> {
    parse_id(s).map(|(s, x)| (s, var(x)))
}
fn parse_binary_operation(s: &str) -> IResult<&str, Expression> {
    alt((
        parse_plus,
        parse_minus,
        parse_times,
        parse_divide,
        parse_equals,
    ))(s)
}
fn parse_expression(s: &str) -> IResult<&str, Expression> {
    alt((parse_constant, parse_binary_operation, parse_variable))(s)
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    let (s, name) = parse_id(s)?;
    let (s, _) = tag(": ")(s)?;
    let (s, expr) = parse_expression(s)?;
    Ok((s, Monkey { name, expr }))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn expr_constant_from_str() -> Result<()> {
        assert_eq!(
            Monkey::from_str("abcd: 42")?,
            Monkey {
                name: "abcd".to_owned(),
                expr: int(42),
            }
        );
        Ok(())
    }

    #[test]
    fn expr_binary_from_str() -> Result<()> {
        assert_eq!(
            Monkey::from_str("abcd: efgh + ijkl")?,
            Monkey {
                name: "abcd".to_owned(),
                expr: var("efgh") + var("ijkl")
            }
        );
        Ok(())
    }

    #[test]
    fn expr_variable_from_str() -> Result<()> {
        assert_eq!(
            Monkey::from_str("humn: x")?,
            Monkey {
                name: "humn".to_owned(),
                expr: var("x")
            }
        );
        Ok(())
    }

    #[test]
    fn expr_equals_from_str() -> Result<()> {
        assert_eq!(
            Monkey::from_str("foo: bar = baz")?,
            Monkey {
                name: "foo".to_owned(),
                expr: eq(var("bar"), var("baz")),
            }
        );
        Ok(())
    }
}
