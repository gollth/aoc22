pub mod cpu;
pub mod crt;
pub mod instruction;

#[derive(Debug, PartialEq, Eq)]
pub enum TenthError {
    FileProblem(String),
    InputInvalid(String),
}

impl From<nom::error::Error<&str>> for TenthError {
    fn from(e: nom::error::Error<&str>) -> Self {
        Self::InputInvalid(format!("{}", e))
    }
}

impl From<std::io::Error> for TenthError {
    fn from(e: std::io::Error) -> Self {
        Self::FileProblem(format!("{}", e))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{cpu::Cpu, crt::Screen, instruction::Instruction};

    use super::*;

    #[test]
    fn sample_b() -> Result<(), TenthError> {
        let content = std::fs::read_to_string("sample.txt")?;

        let mut cpu = Cpu::default();
        let mut screen = Screen::default();

        for instruction in content.lines().map(Instruction::from_str) {
            cpu.execute(&instruction?, &mut screen);
        }

        println!("{}", screen);
        Ok(())
    }
}
