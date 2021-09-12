use std::collections::HashMap;

pub struct Pmf {
    map: HashMap<String, f64>,
}

impl Pmf {
    pub fn new() -> Pmf {
        Pmf {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, hypo: String, prob: f64) {
        self.map.insert(hypo, prob);
    }

    pub fn multi(&mut self, hypo: String, factor: f64) {
        match self.map.get_mut(&hypo) {
            Some(x) => *x *= factor,
            None => eprintln!("undefined hypos: {}", hypo),
        }
    }

    pub fn normalize(&mut self) {
        let mut total = 0.0;
        for (_, value) in &self.map {
            total += value;
        }
        if total == 0.0 {
            eprintln!("total is zero.");
        }
        for (_, value) in &mut self.map {
            *value = *value / total;
        }
    }

    pub fn prob(&self, hypo: String) -> f64 {
        match self.map.get(&hypo) {
            Some(x) => *x,
            None => {
                eprintln!("undefined hypos: {}", hypo);
                0.0
            }
        }
    }
}
