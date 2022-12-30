pub mod monkey;

use itertools::Itertools;
use savage_core::{expression::Expression, helpers::int};
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};
use urlencoding::encode;

use anyhow::{anyhow, Error, Result};
use monkey::Monkey;

/// A pack of monkeys
#[derive(Debug)]
pub struct Pack {
    monkeys: HashMap<String, Monkey>,
}
impl Pack {
    pub fn evaluate(&self, name: &str) -> Result<Expression> {
        let context = self
            .monkeys
            .iter()
            .map(|(name, monkey)| (name.clone(), monkey.expression().clone()))
            .collect();
        let expression = self
            .monkeys
            .get(name)
            .ok_or(anyhow!("No solution found for '{}'", name))?
            .expression()
            .evaluate(context)
            .map_err(|e| anyhow!("No solution found for '{name:}': {e:?}'"))?;

        if let Expression::Equal(a, b) = expression.clone() {
            // try to simplify a bit
            let expr = match (*a.clone(), *b.clone()) {
                (Expression::Integer(a), b) => b - int(a),
                (a, Expression::Integer(b)) => a - int(b),
                (a, b) => Expression::Equal(Box::new(a), Box::new(b)),
            };
            return Ok(expr);
        }
        Ok(expression)
    }
}

impl FromStr for Pack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            monkeys: s
                .lines()
                .map(Monkey::from_str)
                .map_ok(|m| (m.name(), m))
                .collect::<Result<_>>()?,
        })
    }
}

pub fn replace_root_operation(line: &str) -> String {
    if line.starts_with("root:") {
        line.replace(&['+', '-', '*', '/'], "=")
    } else {
        line.to_string()
    }
}
pub fn replace_human_with_x(line: &str) -> String {
    if line.starts_with("humn:") {
        "humn: x".to_owned()
    } else {
        line.to_string()
    }
}

pub fn simplify(expr: &Expression) -> Result<String> {
    #[derive(Deserialize, Debug)]
    struct Response {
        result: String,
    }
    let url = format!(
        "https://newton.now.sh/api/v2/simplify/{}",
        encode(&expr.to_string())
    );
    Ok(reqwest::blocking::get(url)?.json::<Response>()?.result)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use savage_core::helpers::int;

    use super::*;

    #[test]
    fn parse_sample_lone() -> Result<()> {
        let pack = Pack::from_str(&std::fs::read_to_string("sample.txt")?)?;
        assert!(pack.evaluate("abcd").is_err());
        assert_eq!(pack.evaluate("dbpl")?, int(5));
        assert_eq!(pack.evaluate("zczc")?, int(2));
        assert_eq!(pack.evaluate("dvpt")?, int(3));
        assert_eq!(pack.evaluate("lfqf")?, int(4));
        assert_eq!(pack.evaluate("humn")?, int(5));
        assert_eq!(pack.evaluate("ljgn")?, int(2));
        assert_eq!(pack.evaluate("sllz")?, int(4));
        assert_eq!(pack.evaluate("hmdt")?, int(32));
        Ok(())
    }

    #[test]
    fn parse_sample_math() -> Result<()> {
        let pack = Pack::from_str(&std::fs::read_to_string("sample.txt")?)?;
        assert_eq!(pack.evaluate("drzm")?, int(30));
        assert_eq!(pack.evaluate("sjmn")?, int(150));
        Ok(())
    }

    #[test]
    fn parse_sample_root() -> Result<()> {
        let pack = Pack::from_str(&std::fs::read_to_string("sample.txt")?)?;
        assert_eq!(pack.evaluate("root")?, int(152));
        Ok(())
    }

    #[test]
    fn parse_sample_root_part2() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let sample = sample
            .lines()
            .map(|line| replace_root_operation(line))
            .map(|line| replace_human_with_x(&line))
            .collect::<Vec<_>>()
            .join("\n");
        let pack = Pack::from_str(&sample)?;
        let solution = "(4 + 2 * (x - 3)) / 4 - 150".parse::<Expression>().unwrap();
        let expression = pack.evaluate("root")?;
        println!("{}", expression);
        assert_eq!(expression, solution);
        Ok(())
    }

    #[test]
    fn parse_sample_root_part2_simplified() -> Result<()> {
        let original = "(4 + 2 * (x - 3)) / 4 - 150".parse::<Expression>().unwrap();
        let simplified = simplify(&original)?;
        assert_eq!(simplified, "1/2 x - 301/2");
        Ok(())
    }
}
