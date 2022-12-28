use std::{collections::VecDeque, fmt::Debug, ops::Index};

#[derive(Debug)]
pub struct RingBuffer<T> {
    freq: usize,
    buf: VecDeque<T>,
}

impl<T> FromIterator<T> for RingBuffer<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let buf = iter.into_iter().collect::<VecDeque<_>>();
        Self {
            freq: buf.len(),
            buf,
        }
    }
}
impl<T> Index<usize> for RingBuffer<T>
where
    T: Debug,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.buf.index(self.idx(index as isize))
    }
}
impl<T> IntoIterator for RingBuffer<T> {
    type Item = T;

    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buf.into_iter()
    }
}
impl<T> RingBuffer<T>
where
    T: Debug,
{
    fn idx(&self, x: isize) -> usize {
        let modulo = x % self.freq as isize;
        if x >= 0 {
            return modulo as usize;
        }
        (self.freq as isize + modulo) as usize
    }

    fn idxmod(&self, x: isize) -> usize {
        let f = self.freq as isize - 1;
        let modulo = x % f;
        if x >= 0 {
            return modulo as usize;
        }
        (f + modulo) as usize
    }
    pub fn iter(&self) -> std::collections::vec_deque::Iter<T> {
        self.buf.iter()
    }

    pub fn shift(&mut self, index: usize, amount: isize) {
        if amount == 0 {
            // Already in place
            return;
        }
        let ii = index as isize;
        let x = self.buf.remove(self.idx(ii)).unwrap();
        let new_idx = self.idxmod(ii + amount);
        if new_idx > self.buf.len() {
            self.buf.push_back(x);
        } else {
            self.buf.insert(new_idx, x);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexing_over_freq_wraps_around() {
        let rb = RingBuffer::from_iter([1, 2, 3, 4]);
        assert_eq!(rb[4], 1);
        assert_eq!(rb[21], 2);
    }

    #[test]
    fn shifting_one_place_right() {
        let mut rb = RingBuffer::from_iter([1, 2, 3]);
        rb.shift(0, 1);
        assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), vec![2, 1, 3]);
    }

    #[test]
    fn shifting_one_place_left() {
        let mut rb = RingBuffer::from_iter([1, 2, 3]);
        rb.shift(2, -1);
        assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), vec![1, 3, 2]);
    }

    #[test]
    fn shifting_multiple_places_right() {
        let mut rb = RingBuffer::from_iter(1..=5);
        rb.shift(1, 3);
        assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), vec![2, 1, 3, 4, 5])
    }

    #[test]
    fn shifting_multiple_places_left() {
        let mut rb = RingBuffer::from_iter(1..=5);
        rb.shift(3, -2);
        assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), vec![1, 4, 2, 3, 5])
    }

    #[test]
    fn shifting_right_with_overflow() {
        let mut rb = RingBuffer::from_iter(1..=5);
        rb.shift(2, 3);
        assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), vec![1, 3, 2, 4, 5])
    }

    #[test]
    fn shifting_right_with_underflow() {
        let mut rb = RingBuffer::from_iter(1..=5);
        rb.shift(2, -3);
        assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), vec![1, 2, 4, 3, 5]);
    }

    #[test]
    fn shifting_at_boudary() {
        let mut rb = RingBuffer::from_iter(1..=5);
        rb.shift(2, -6);
        assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), vec![1, 2, 4, 5, 3]);
    }
}
