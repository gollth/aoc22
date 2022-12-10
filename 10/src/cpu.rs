use crate::instruction::Instruction;

#[derive(Debug, PartialEq, Eq)]
pub struct Cpu {
    cycles: i32,
    x: i32,

    signal_strength: i32,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            cycles: 0,
            x: 1,
            signal_strength: 0,
        }
    }
}

impl Cpu {
    fn tick(&mut self) {
        self.cycles += 1;
        if (self.cycles - 20) % 40 == 0 {
            self.signal_strength += self.cycles as i32 * self.x;
            println!(
                "#{:05}: Signal Strength = {}",
                self.cycles, self.signal_strength
            );
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            &Instruction::Noop => self.tick(),
            &Instruction::AddX(n) => {
                self.tick();
                self.tick();
                self.x += n;
            }
        }
    }

    pub fn signal_strength(&self) -> i32 {
        self.signal_strength
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::TenthError;

    use super::*;

    #[test]
    fn a_little_sample() {
        let mut cpu = Cpu::default();
        cpu.execute(&Instruction::Noop);
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.cycles, 1);

        cpu.execute(&Instruction::AddX(3));
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.cycles, 3);

        cpu.execute(&Instruction::AddX(-5));
        assert_eq!(cpu.x, -1);
        assert_eq!(cpu.cycles, 5);
    }

    #[test]
    fn a_big_sample() -> Result<(), TenthError> {
        let content = std::fs::read_to_string("sample.txt")?;
        let mut cpu = Cpu::default();
        for instruction in content.lines().map(Instruction::from_str) {
            cpu.execute(&instruction?);
        }
        assert_eq!(cpu.signal_strength, 13140);
        Ok(())
    }
}
