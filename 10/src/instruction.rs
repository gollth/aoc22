use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{i32, space1},
    Finish, IResult,
};

use crate::TenthError;

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

fn noop(s: &str) -> IResult<&str, Instruction> {
    let (s, _) = tag("noop")(s)?;
    Ok((s, Instruction::Noop))
}

fn add_x(s: &str) -> IResult<&str, Instruction> {
    let (s, _) = tag("addx")(s)?;
    let (s, _) = space1(s)?;
    let (s, n) = i32(s)?;
    Ok((s, Instruction::AddX(n)))
}

impl FromStr for Instruction {
    type Err = TenthError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(noop(s).or(add_x(s)).finish()?.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_noop() {
        assert_eq!(Instruction::from_str("noop"), Ok(Instruction::Noop));
        assert_eq!(Instruction::from_str("noop foo"), Ok(Instruction::Noop));
        assert!(Instruction::from_str("nopo").is_err(),);
    }

    #[test]
    fn parse_addx() {
        assert_eq!(Instruction::from_str("addx 42"), Ok(Instruction::AddX(42)));
        assert_eq!(
            Instruction::from_str("addx -69"),
            Ok(Instruction::AddX(-69))
        );
        assert!(Instruction::from_str("addx").is_err());
        assert!(Instruction::from_str("adxd").is_err());
        assert!(Instruction::from_str("addx foo").is_err());
    }
}
