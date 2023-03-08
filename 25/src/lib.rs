use anyhow::{anyhow, Result};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Snafu {
    digits: Vec<i8>,
}

impl FromStr for Snafu {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            digits: s
                .chars()
                .rev()
                .into_iter()
                .map(|c| match c {
                    '0' => Ok(0),
                    '1' => Ok(1),
                    '2' => Ok(2),
                    '-' => Ok(-1),
                    '=' => Ok(-2),
                    c => Err(anyhow!("Unkown SNAFU digit {}", c)),
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl From<i64> for Snafu {
    fn from(x: i64) -> Self {
        let mut x = x;
        let mut digits = Vec::new();

        let mut n = 0;

        loop {
            if digits.len() <= n {
                digits.push(0);
            }
            if digits.len() <= n + 1 {
                digits.push(0);
            }

            let digit = x % 5;

            if digit <= 2 {
                digits[n] += digit as i8;
                if digits[n] > 2 {
                    digits[n] -= 5;
                    digits[n + 1] += 1;
                }
            } else {
                digits[n] += digit as i8 - 5;
                digits[n + 1] += 1;
            }

            x = x.div_euclid(5);
            n += 1;
            if x == 0 {
                break;
            }
        }

        Self {
            digits: digits
                .into_iter()
                .rev()
                .skip_while(|x| *x == 0)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect(),
        }
    }
}

impl From<Snafu> for i64 {
    fn from(snafu: Snafu) -> Self {
        snafu
            .digits
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + (x as i64 * 5_i64.pow(i as u32)))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.digits
                .iter()
                .rev()
                .map(|digit| match digit {
                    -1 => '-',
                    -2 => '=',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => panic!("Invalid Snafu digit {}", digit),
                })
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snafu(decimal: i64, snafu: &str) -> Result<()> {
        let snafu = snafu.parse::<Snafu>()?;
        assert_eq!(decimal, i64::from(snafu.clone()));
        assert_eq!(Snafu::from(decimal), snafu);
        Ok(())
    }

    #[test]
    fn snafu_1x() -> Result<()> {
        snafu(1, "1")
    }

    #[test]
    fn snafu_2x() -> Result<()> {
        snafu(2, "2")
    }

    #[test]
    fn snafu_3x() -> Result<()> {
        snafu(3, "1=")
    }

    #[test]
    fn snafu_4() -> Result<()> {
        snafu(4, "1-")
    }

    #[test]
    fn snafu_5() -> Result<()> {
        snafu(5, "10")
    }

    #[test]
    fn snafu_6() -> Result<()> {
        snafu(6, "11")
    }

    #[test]
    fn snafu_7() -> Result<()> {
        snafu(7, "12")
    }

    #[test]
    fn snafu_8() -> Result<()> {
        snafu(8, "2=")
    }

    #[test]
    fn snafu_9() -> Result<()> {
        snafu(9, "2-")
    }

    #[test]
    fn snafu_10() -> Result<()> {
        snafu(10, "20")
    }

    #[test]
    fn snafu_15() -> Result<()> {
        snafu(15, "1=0")
    }

    #[test]
    fn snafu_20() -> Result<()> {
        snafu(20, "1-0")
    }

    #[test]
    fn snafu_2022() -> Result<()> {
        snafu(2022, "1=11-2")
    }

    #[test]
    fn snafu_12345() -> Result<()> {
        snafu(12345, "1-0---0")
    }

    #[test]
    fn snafu_314159265() -> Result<()> {
        snafu(314159265, "1121-1110-1=0")
    }

    #[test]
    fn snafu_1747() -> Result<()> {
        snafu(1747, "1=-0-2")
    }

    #[test]
    fn snafu_906() -> Result<()> {
        snafu(906, "12111")
    }

    #[test]
    fn snafu_198() -> Result<()> {
        snafu(198, "2=0=")
    }

    #[test]
    fn snafu_11() -> Result<()> {
        snafu(11, "21")
    }

    #[test]
    fn snafu_201() -> Result<()> {
        snafu(201, "2=01")
    }

    #[test]
    fn snafu_31() -> Result<()> {
        snafu(31, "111")
    }

    #[test]
    fn snafu_1257() -> Result<()> {
        snafu(1257, "20012")
    }

    #[test]
    fn snafu_32() -> Result<()> {
        snafu(32, "112")
    }

    #[test]
    fn snafu_353() -> Result<()> {
        snafu(353, "1=-1=")
    }

    #[test]
    fn snafu_107() -> Result<()> {
        snafu(107, "1-12")
    }

    #[test]
    fn snafu_37() -> Result<()> {
        snafu(37, "122")
    }
}
