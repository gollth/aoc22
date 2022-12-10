use std::str::FromStr;

use tenth::{cpu::Cpu, instruction::Instruction, TenthError};

fn main() -> Result<(), TenthError> {
    let mut cpu = Cpu::default();
    for instruction in std::fs::read_to_string("input.txt")?
        .lines()
        .map(Instruction::from_str)
    {
        cpu.execute(&instruction?);
    }

    println!("Solution 10a: {}", cpu.signal_strength());
    Ok(())
}
