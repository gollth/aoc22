use std::{
    collections::{BinaryHeap, HashMap},
    fmt::{Debug, Display},
    hash::Hash,
};

pub trait Cost {
    fn cost(&self) -> i32;
}

pub struct A<N> {
    done: HashMap<N, N>,
    todo: BinaryHeap<N>,
}

impl<N> A<N>
where
    N: Ord + Hash + Clone + Cost + Debug + Display,
{
    pub fn new() -> Self {
        Self {
            done: HashMap::new(),
            todo: BinaryHeap::new(),
        }
    }

    pub fn solve<F, G>(&mut self, candidates: F, start: &N, goal: G) -> Vec<N>
    where
        F: Fn(&N) -> Vec<N>,
        G: Fn(&N) -> bool,
    {
        // dist[n]: Currently shortest distance from start .. n
        let mut dist = HashMap::from([(start.clone(), 0)]);

        self.todo.push(start.clone());

        while let Some(n) = self.todo.pop() {
            if goal(&n) {
                let mut path = vec![n.clone()];
                let mut x = n.clone();
                while let Some(previous) = self.done.get(&x) {
                    path.push(previous.clone());
                    x = previous.clone();
                }
                return path.into_iter().rev().collect();
            }

            let cands = candidates(&n);
            // Inspect all next candidates regarding their costs
            for next in cands {
                let cost = next.cost();
                let d = dist.entry(next.clone()).or_insert(i32::MAX);
                if cost < *d {
                    *d = cost;
                    let entry = self.done.entry(next.clone()).or_insert(n.clone());
                    *entry = n.clone();
                    self.todo.push(next);
                }
            }
        }

        unreachable!("Path")
    }
}
