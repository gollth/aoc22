use std::{
    fmt::{Debug, Formatter},
    {collections::VecDeque, str::FromStr},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, u32},
    multi::separated_list0,
    Finish, IResult,
};

use crate::EleventhError;

pub struct Monkey {
    id: u32,
    items: VecDeque<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    test: Box<dyn Fn(u32) -> u32>,
    inspections: u32,
}

impl Monkey {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn inspections(&self) -> u32 {
        self.inspections
    }

    pub fn operation(&self, x: u32) -> u32 {
        (self.operation)(x)
    }

    pub fn items(&self) -> VecDeque<u32> {
        self.items.clone()
    }

    pub fn test(&self, x: u32) -> u32 {
        (self.test)(x)
    }

    pub fn inspect(&mut self) -> Option<u32> {
        let item = self.items.pop_front();
        if item.is_some() {
            self.inspections += 1;
        }
        item
    }

    pub fn catch(&mut self, item: u32) {
        self.items.push_back(item)
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Monkey {{ id: {}, items: {:?} }}", self.id, self.items)
    }
}

fn parse_addition(s: &str) -> IResult<&str, Box<dyn Fn(u32) -> u32>> {
    let (s, _) = tag("+")(s)?;
    let (s, _) = multispace0(s)?;
    if let Ok((s, _)) = tag::<_, _, (_, _)>("old")(s) {
        return Ok((s, Box::new(move |x| x + x)));
    }
    let (s, y) = u32(s)?;
    Ok((s, Box::new(move |x| x + y)))
}
fn parse_multiplication(s: &str) -> IResult<&str, Box<dyn Fn(u32) -> u32>> {
    let (s, _) = tag("*")(s)?;
    let (s, _) = multispace0(s)?;
    if let Ok((s, _)) = tag::<_, _, (_, _)>("old")(s) {
        return Ok((s, Box::new(move |x| x * x)));
    }
    let (s, y) = u32(s)?;
    Ok((s, Box::new(move |x| x * y)))
}

fn parse_function(s: &str) -> IResult<&str, Box<dyn Fn(u32) -> u32>> {
    let (s, _) = tag("new = old ")(s)?;
    let (s, operation) = alt((parse_addition, parse_multiplication))(s)?;
    Ok((s, operation))
}

fn parse_predicate(s: &str) -> IResult<&str, Box<dyn Fn(u32) -> u32>> {
    let (s, _) = tag("divisible by ")(s)?;
    let (s, div) = u32(s)?;
    let (s, _) = newline(s)?;
    let (s, _) = multispace1(s)?;

    let (s, _) = tag("If true: throw to monkey ")(s)?;
    let (s, t) = u32(s)?;
    let (s, _) = newline(s)?;
    let (s, _) = multispace1(s)?;

    let (s, _) = tag("If false: throw to monkey ")(s)?;
    let (s, f) = u32(s)?;
    Ok((s, Box::new(move |x| if x % div == 0 { t } else { f })))
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    let (s, _) = tag("Monkey ")(s)?;
    let (s, id) = u32(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = newline(s)?;

    let (s, _) = multispace1(s)?;
    let (s, _) = tag("Starting items: ")(s)?;
    let (s, items) = separated_list0(tag(", "), u32)(s)?;
    let (s, _) = newline(s)?;

    let (s, _) = multispace1(s)?;
    let (s, _) = tag("Operation: ")(s)?;
    let (s, operation) = parse_function(s)?;
    let (s, _) = newline(s)?;

    let (s, _) = multispace1(s)?;
    let (s, _) = tag("Test: ")(s)?;
    let (s, test) = parse_predicate(s)?;

    Ok((
        s,
        Monkey {
            id,
            items: items.into_iter().collect::<VecDeque<_>>(),
            operation,
            test,
            inspections: 0,
        },
    ))
}

impl FromStr for Monkey {
    type Err = EleventhError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_monkey(s).finish()?.1)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parses_monkey_0() -> Result<(), EleventhError> {
        let content = std::fs::read_to_string("sample.txt")?;

        let monkeys = content.split_terminator("\n\n").collect::<Vec<_>>();
        let monkey = Monkey::from_str(monkeys[0])?;
        assert_eq!(monkey.id, 0);
        assert_eq!(monkey.items, vec![79, 98]);
        assert_eq!((monkey.operation)(1), 19);
        assert_eq!((monkey.operation)(2), 2 * 19);
        assert_eq!((monkey.test)(23), 2);
        assert_eq!((monkey.test)(24), 3);

        Ok(())
    }

    #[test]
    fn parses_monkey_1() -> Result<(), EleventhError> {
        let content = std::fs::read_to_string("sample.txt")?;

        let monkeys = content.split_terminator("\n\n").collect::<Vec<_>>();
        let monkey = Monkey::from_str(monkeys[1])?;
        assert_eq!(monkey.id, 1);
        assert_eq!(monkey.items, vec![54, 65, 75, 74]);
        assert_eq!(monkey.operation(1), 1 + 6);
        assert_eq!(monkey.operation(2), 2 + 6);
        assert_eq!(monkey.test(19), 2);
        assert_eq!(monkey.test(20), 0);

        Ok(())
    }

    #[test]
    fn parses_monkey_2() -> Result<(), EleventhError> {
        let content = std::fs::read_to_string("sample.txt")?;

        let monkeys = content.split_terminator("\n\n").collect::<Vec<_>>();
        let monkey = Monkey::from_str(monkeys[2])?;
        assert_eq!(monkey.id, 2);
        assert_eq!(monkey.items, vec![79, 60, 97]);
        assert_eq!(monkey.operation(1), 1 * 1);
        assert_eq!(monkey.operation(2), 2 * 2);
        assert_eq!(monkey.operation(42), 42 * 42);
        assert_eq!(monkey.test(13), 1);
        assert_eq!(monkey.test(14), 3);

        Ok(())
    }
}
