mod rb;

use anyhow::{anyhow, Result};
use rb::RingBuffer;
use std::str::FromStr;

type Number = i64;

#[derive(Debug)]
pub struct Sequence {
    original: Vec<Number>,
    sequence: RingBuffer<(usize, Number)>,
    step: usize,
}

impl FromIterator<Number> for Sequence {
    fn from_iter<I: IntoIterator<Item = Number>>(iter: I) -> Self {
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
            .map(|l| l.parse::<Number>().map_err(|e| anyhow!("{}", e)))
            .collect::<Result<Vec<_>>>()?;
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

    pub fn values(&self) -> Vec<Number> {
        self.sequence.iter().map(|(_, x)| *x).collect()
    }

    pub fn mix(&mut self) {
        while !self.mix_step() {}
        self.step = 0;
    }

    pub fn coords(&self) -> (Number, Number, Number) {
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

    fn test_mix_step_n(n: usize, expectation: Vec<Number>) -> Result<()> {
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

    fn test_mix_n(key: i64, rounds: usize, expectation: Vec<Number>) -> Result<()> {
        let numbers = std::fs::read_to_string("sample.txt")?
            .lines()
            .map(|x| x.parse::<i64>().map_err(|e| anyhow!("{}", e)))
            .map(|x| Ok(x? * key))
            .collect::<Result<Vec<_>>>()?;
        let mut sequence = Sequence::from_iter(numbers);

        for _ in 0..rounds {
            println!("{:?}", sequence.sequence);
            sequence.mix();
        }
        println!("{:?}", sequence.sequence);
        assert_eq!(sequence.values(), expectation);
        Ok(())
    }

    const KEY: i64 = 811589153;

    #[test]
    fn sample_with_key1() -> Result<()> {
        test_mix_n(
            KEY,
            1,
            vec![
                0,
                -2434767459,
                3246356612,
                -1623178306,
                2434767459,
                1623178306,
                811589153,
            ],
        )
    }

    #[test]
    fn sample_with_key2() -> Result<()> {
        test_mix_n(
            KEY,
            2,
            vec![
                0,
                2434767459,
                1623178306,
                3246356612,
                -2434767459,
                -1623178306,
                811589153,
            ],
        )
    }

    #[test]
    fn sample_with_key3() -> Result<()> {
        test_mix_n(
            KEY,
            3,
            vec![
                2434767459,
                3246356612,
                1623178306,
                -1623178306,
                -2434767459,
                0,
                811589153,
            ],
        )
    }

    #[test]
    fn sample_with_key4() -> Result<()> {
        test_mix_n(
            KEY,
            4,
            vec![
                1623178306,
                -2434767459,
                811589153,
                2434767459,
                3246356612,
                -1623178306,
                0,
            ],
        )
    }

    #[test]
    fn sample_with_key5() -> Result<()> {
        test_mix_n(
            KEY,
            5,
            vec![
                811589153,
                -1623178306,
                1623178306,
                -2434767459,
                3246356612,
                2434767459,
                0,
            ],
        )
    }

    #[test]
    fn sample_with_keys6() -> Result<()> {
        test_mix_n(
            KEY,
            6,
            vec![
                811589153,
                -1623178306,
                3246356612,
                -2434767459,
                1623178306,
                2434767459,
                0,
            ],
        )
    }
    #[test]
    fn sample_with_keys7() -> Result<()> {
        test_mix_n(
            KEY,
            7,
            vec![
                2434767459,
                1623178306,
                -1623178306,
                811589153,
                3246356612,
                0,
                -2434767459,
            ],
        )
    }

    #[test]
    fn sample_with_keys8() -> Result<()> {
        test_mix_n(
            KEY,
            8,
            vec![
                3246356612,
                811589153,
                -2434767459,
                2434767459,
                -1623178306,
                0,
                1623178306,
            ],
        )
    }

    #[test]
    fn sample_with_keys9() -> Result<()> {
        test_mix_n(
            KEY,
            9,
            vec![
                811589153,
                1623178306,
                -2434767459,
                3246356612,
                2434767459,
                -1623178306,
                0,
            ],
        )
    }

    #[test]
    fn sample_with_keys10() -> Result<()> {
        test_mix_n(
            KEY,
            10,
            vec![
                -2434767459,
                1623178306,
                3246356612,
                -1623178306,
                2434767459,
                811589153,
                0,
            ],
        )
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
    fn sample_coordinates_with_key() -> Result<()> {
        let numbers = std::fs::read_to_string("sample.txt")?
            .lines()
            .map(|x| x.parse::<i64>().map_err(|e| anyhow!("{}", e)))
            .map(|x| Ok(x? * KEY))
            .collect::<Result<Vec<_>>>()?;
        let mut sequence = Sequence::from_iter(numbers);

        for _ in 0..10 {
            sequence.mix();
        }
        assert_eq!(sequence.coords(), (811589153, 2434767459, -1623178306));

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
