use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

pub struct Pmf<T> {
    map: HashMap<T, f64>,
}

impl<T> Pmf<T>
where
    T: Display + Eq + Hash + Copy,
{
    pub fn new() -> Pmf<T> {
        Pmf {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, hypo: T, prob: f64) {
        match self.map.entry(hypo) {
            Occupied(_) => panic!("already set: {}", hypo),
            _vacant => self.map.insert(hypo, prob),
        };
    }

    pub fn multi(&mut self, hypo: &T, factor: f64) {
        match self.map.get_mut(hypo) {
            Some(x) => *x *= factor,
            None => panic!("undefined hypos: {}", hypo),
        }
    }

    pub fn normalize(&mut self) {
        let mut total = 0.0;
        for (_, value) in &self.map {
            total += value;
        }
        if total == 0.0 {
            panic!("total is zero.");
        }
        for (_, value) in &mut self.map {
            *value = *value / total;
        }
    }

    pub fn prob(&self, hypo: &T) -> f64 {
        match self.map.get(hypo) {
            Some(x) => *x,
            None => {
                panic!("undefined hypos: {}", hypo);
            }
        }
    }
}
