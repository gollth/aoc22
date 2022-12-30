pub mod monkey;

use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

use anyhow::{Error, Result};
use monkey::{Monkey, Number};

/// A pack of monkeys
pub struct Pack {
    monkeys: HashMap<String, Monkey>,
    cache: HashMap<String, Number>,
}
impl Pack {
    fn get(&mut self, name: &str) -> Option<Number> {
        if self.cache.contains_key(name) {
            return self.cache.get(name).cloned();
        }
        let value = self.value(name)?;
        self.cache.insert(name.to_string(), value);
        Some(value)
    }

    pub fn value(&mut self, name: &str) -> Option<Number> {
        let monkey = self.monkeys.get(name).cloned()?;
        match monkey {
            Monkey::Lone { name: _, number } => Some(number),
            Monkey::Math { name: _, a, op, b } => Some(op.call(self.get(&a)?, self.get(&b)?)),
        }
    }
}

impl FromStr for Pack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cache: HashMap::new(),
            monkeys: s
                .lines()
                .map(Monkey::from_str)
                .map_ok(|m| (m.name(), m))
                .collect::<Result<_>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn parse_sample_lone() -> Result<()> {
        let mut pack = Pack::from_str(&std::fs::read_to_string("sample.txt")?)?;
        assert_eq!(pack.value("abcd"), None);
        assert_eq!(pack.value("dbpl"), Some(5));
        assert_eq!(pack.value("zczc"), Some(2));
        assert_eq!(pack.value("dvpt"), Some(3));
        assert_eq!(pack.value("lfqf"), Some(4));
        assert_eq!(pack.value("humn"), Some(5));
        assert_eq!(pack.value("ljgn"), Some(2));
        assert_eq!(pack.value("sllz"), Some(4));
        assert_eq!(pack.value("hmdt"), Some(32));
        Ok(())
    }

    #[test]
    fn parse_sample_math() -> Result<()> {
        let mut pack = Pack::from_str(&std::fs::read_to_string("sample.txt")?)?;
        assert_eq!(pack.value("drzm"), Some(30));
        assert_eq!(pack.value("sjmn"), Some(150));
        Ok(())
    }

    #[test]
    fn parse_sample_root() -> Result<()> {
        let mut pack = Pack::from_str(&std::fs::read_to_string("sample.txt")?)?;
        assert_eq!(pack.value("root"), Some(152));
        Ok(())
    }
}
