use conv::*;
use rand::{prelude::*, Rng};
use treap::{Element, Treap};
use treap_non_random as treap;
pub struct CountUnique<T: Ord + Clone, R: Rng> {
    treap: Treap<T, f32>,
    rng: R,
    max_size: usize,
    p: f32,
}

impl<T, R> CountUnique<T, R>
where
    T: Ord + Clone,
    R: Rng,
{
    pub fn new(r: R, sz: usize) -> Self {
        if sz < 1 {
            panic!("Cannot count without state");
        }
        CountUnique {
            treap: Treap::new(),
            rng: r,
            max_size: sz,
            p: 1.0f32,
        }
    }

    pub fn add_token(&mut self, t: T) {
        let u = self.rng.gen::<f32>();
        self.treap.delete(&t);
        if u < self.p {
            if self.treap.size() < self.max_size {
                self.treap.insert(Element::new(t, u));
            } else {
                // This unwrap is safe: we are guaranteed to have one.
                let (m_priority, m_value) = {
                    let m = self.treap.get_max().unwrap();
                    (*m.priority(), m.value().clone())
                };
                if m_priority > u {
                    self.p = u;
                } else {
                    self.treap.delete(&m_value);
                    self.treap.insert(Element::new(t, u));
                    self.p = m_priority;
                }
            }
        }
    }

    pub fn estimate(&self) -> Result<f64, PosOverflow<usize>> {
        let f = f64::value_from(self.treap.size())?;
        let p = self.p as f64;
        Ok(f / p)
    }

    pub fn reset(&mut self) {
        self.treap.reset();
        self.p = 1.0f32;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn sufficient_space_works() {
        let sentence = String::from("This was a triumph I am making a note here huge success");
        let mut ctr = CountUnique::new(thread_rng(), 11);
        let tokens = sentence.split_whitespace();
        for t in tokens {
            ctr.add_token(String::from(t));
        }
        assert!(ctr.estimate().unwrap() == 11.0f64)
    }

    #[test]
    pub fn insufficient_space_works() {
        let sentence = String::from("This was a triumph I am making a note here huge success");
        let mut ctr = CountUnique::new(thread_rng(), 10);
        let mut sum = 0f64;
        const RUNS: u32 = 200;
        for _ in 0..RUNS {
            ctr.reset();
            let tokens = sentence.split_whitespace();
            for t in tokens {
                ctr.add_token(String::from(t));
            }
            let estimate = ctr.estimate().unwrap();
            println!("Estimated {}", estimate);
            sum += estimate;
        }
        let average = sum / (RUNS as f64);
        println!("Average {}", average);
        assert!(0.0 < average);
    }
}
