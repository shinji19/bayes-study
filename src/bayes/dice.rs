use crate::bayes::pmf::Pmf;
use std::collections::HashMap;
use std::fmt;

pub struct Dice {
    hypos: Vec<i64>,
    pmf: Pmf<i64>,
}

impl Dice {
    pub fn new(hypos: &Vec<i64>) -> Dice {
        let mut pmf = Pmf::new();
        // 事前確率：それぞれのダイスが選ばれる確率は同じ
        for hypo in hypos {
            pmf.set(*hypo, 1.0 / hypos.len() as f64);
        }
        Dice {
            hypos: hypos.to_vec(),
            pmf,
        }
    }

    fn likelihood(&self, data: i64, hypo: i64) -> f64 {
        if hypo < data {
            return 0.0;
        }
        return 1.0 / hypo as f64;
    }

    pub fn update(&mut self, data: i64) {
        for hypo in &self.hypos {
            let like = self.likelihood(data, *hypo);
            self.pmf.multi(hypo, like);
        }
        self.pmf.normalize();
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        for hypo in &self.hypos {
            let t = format!("{}: {}\n", hypo, self.pmf.prob(hypo));
            ret.push_str(&t);
        }
        ret.remove(ret.len() - 1);
        write!(f, "{}", ret)
    }
}
