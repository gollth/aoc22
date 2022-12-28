mod rb;

use anyhow::{anyhow, Result};
use rb::RingBuffer;
use std::str::FromStr;

#[derive(Debug)]
pub struct Sequence {
    original: Vec<i32>,
    sequence: RingBuffer<(usize, i32)>,
    step: usize,
}

impl FromIterator<i32> for Sequence {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let original = iter.into_iter().collect::<Vec<_>>();
        Self {
            original: original.clone(),
            sequence: RingBuffer::from_iter(original.into_iter().enumerate()),
            step: 0,
        }
    }
}

impl FromStr for Sequence {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let original = s
            .lines()
            .map(|l| l.parse::<i32>().map_err(|e| anyhow!("{}", e)))
            .collect::<Result<Vec<i32>>>()?;
        Ok(original.into_iter().collect())
    }
}

impl Sequence {
    fn mix_step(&mut self) -> bool {
        if self.step >= self.original.len() {
            return true;
        }
        let x = self
            .sequence
            .iter()
            .position(|(i, _)| *i == self.step)
            .unwrap();
        self.sequence.shift(x, self.original[self.step] as isize);
        self.step += 1;

        false
    }

    pub fn values(&self) -> Vec<i32> {
        self.sequence.iter().map(|(_, x)| *x).collect()
    }

    pub fn mix(&mut self) {
        while !self.mix_step() {}
    }

    pub fn coords(&self) -> (i32, i32, i32) {
        let zero = self
            .sequence
            .iter()
            .position(|(_, item)| *item == 0)
            .unwrap();
        println!("Found zero at #{}", zero);
        (
            self.sequence[zero + 1000].1,
            self.sequence[zero + 2000].1,
            self.sequence[zero + 3000].1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_mix_step_n(n: usize, expectation: Vec<i32>) -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let mut sequence = Sequence::from_str(&sample)?;

        for _ in 0..n {
            println!("{:?}", sequence.sequence);
            sequence.mix_step();
        }
        println!("{:?}", sequence.sequence);

        assert_eq!(sequence.values(), expectation);

        Ok(())
    }

    #[test]
    fn sample_mix_step1() -> Result<()> {
        test_mix_step_n(1, vec![2, 1, -3, 3, -2, 0, 4])
    }

    #[test]
    fn sample_mix_step2() -> Result<()> {
        test_mix_step_n(2, vec![1, -3, 2, 3, -2, 0, 4])
    }

    #[test]
    fn sample_mix_step3() -> Result<()> {
        test_mix_step_n(3, vec![1, 2, 3, -2, -3, 0, 4])
    }

    #[test]
    fn sample_mix_step4() -> Result<()> {
        test_mix_step_n(4, vec![1, 2, -2, -3, 0, 3, 4])
    }

    #[test]
    fn sample_mix_step5() -> Result<()> {
        test_mix_step_n(5, vec![-2, 1, 2, -3, 0, 3, 4])
    }

    #[test]
    fn sample_mix_step6() -> Result<()> {
        test_mix_step_n(6, vec![-2, 1, 2, -3, 0, 3, 4])
    }

    #[test]
    fn sample_mix_step7() -> Result<()> {
        test_mix_step_n(7, vec![-2, 1, 2, -3, 4, 0, 3])
    }

    #[test]
    fn sample_coordinates() -> Result<()> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let mut sequence = Sequence::from_str(&sample)?;

        sequence.mix();
        assert_eq!(sequence.coords(), (4, -3, 2));
        Ok(())
    }

    #[test]
    fn large_values() {
        let mut sequence = Sequence::from_iter(vec![1, 5, -2, 0]);
        sequence.mix();
        assert_eq!(sequence.values(), vec![1, 5, -2, 0]);
    }

    #[test]
    fn larger_values() {
        let mut sequence = Sequence::from_iter(vec![12, 1, 2, 3]);
        sequence.mix_step();
        assert_eq!(sequence.values(), vec![12, 1, 2, 3]);
    }

    #[test]
    fn duplicate_values() {
        let mut sequence = Sequence::from_iter(vec![3, 2, 1, 0, 1, 2, 3]);
        sequence.mix_step();
        assert_eq!(sequence.values(), vec![2, 1, 0, 3, 1, 2, 3]);
        sequence.mix_step();
        assert_eq!(sequence.values(), vec![1, 0, 2, 3, 1, 2, 3]);
        sequence.mix_step();
        assert_eq!(sequence.values(), vec![0, 1, 2, 3, 1, 2, 3]);
        sequence.mix_step();
        assert_eq!(sequence.values(), vec![0, 1, 2, 3, 1, 2, 3]);
        sequence.mix_step();
        assert_eq!(sequence.values(), vec![0, 1, 2, 3, 2, 1, 3]);
        sequence.mix_step();
        assert_eq!(sequence.values(), vec![2, 0, 1, 2, 3, 1, 3]);
        sequence.mix_step();
        assert_eq!(sequence.values(), vec![2, 0, 1, 3, 2, 3, 1]);
    }
}
