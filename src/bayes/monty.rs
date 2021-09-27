use crate::bayes::Pmf;
use std::fmt;

pub struct Monty {
    hypos: Vec<String>,
    pmf: Pmf,
}

impl Monty {
    pub fn new(hypos: &Vec<String>) -> Monty {
        let mut pmf = Pmf::new();
        // それぞれの扉を開く確率は等確率であることを表している
        for hypo in hypos {
            pmf.set(hypo, 1.0);
        }
        pmf.normalize();
        Monty {
            hypos: hypos.to_vec(),
            pmf,
        }
    }

    fn likelihood(&self, data: &str, hypo: &str) -> f64 {
        if hypo == data {
            return 0.0;
        } else if hypo == "A" {
            return 0.5;
        }
        return 1.0;
    }

    pub fn update(&mut self, data: &str) {
        for hypo in &self.hypos {
            let like = self.likelihood(data, hypo.as_str());
            self.pmf.multi(hypo, like);
        }
        self.pmf.normalize();
    }
}

impl fmt::Display for Monty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        // `x`と`y`のみが明示的になるようにカスタマイズ
        let mut ret = String::new();
        for hypo in &self.hypos {
            let t = format!("{}: {}\n", hypo, self.pmf.prob(hypo));
            ret.push_str(&t);
        }
        ret.remove(ret.len() - 1);
        write!(f, "{}", ret)
    }
}
