pub mod monty;
use crate::bayes::monty::Monty;
use std::collections::hash_map::Entry;
use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use std::option::Option;

pub struct Pmf {
    map: HashMap<String, f64>,
}

impl Pmf {
    pub fn new() -> Pmf {
        Pmf {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, hypo: &str, prob: f64) {
        match self.map.entry(hypo.to_string()) {
            Occupied(_) => panic!("already set: {}", hypo),
            Vacant => self.map.insert(hypo.to_string(), prob),
        };
    }

    pub fn multi(&mut self, hypo: &str, factor: f64) {
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

    pub fn prob(&self, hypo: &str) -> f64 {
        match self.map.get(hypo) {
            Some(x) => *x,
            None => {
                panic!("undefined hypos: {}", hypo);
            }
        }
    }
}
