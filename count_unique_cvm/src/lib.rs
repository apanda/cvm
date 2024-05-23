//! An implementation of the CVM algorithm
//! The [CVM]((https://cs.stanford.edu/~knuth/papers/cvm-note.pdf) algorithm provides
//! a probabilistic data structure for counting the number of unique elements in a stream.
//! This is an implementation of the simple version of the algorithm.
//!
//! # Example
//! ```
//! use count_unique_cvm::*;
//! use rand::prelude::*;
//!
//! // Create a CountUnique CVM which stores at most 4 elements.
//! let mut c = CountUnique::new(rand::thread_rng(), 4);
//! let v = vec![0..100];
//! for i in v {
//!     c.add_token(*v);
//! }
//! // The actual value depends on the random number generator,
//! // but should be close.
//! println!("Estimated number of tokens are {}", c.estimate());
//! ```
#![deny(missing_docs)]

use conv::*;
use rand::Rng;
use treap::{Element, Treap};
use treap_non_random as treap;

/// The CVM algorithm state. `T` is the type of tokens that are
/// being counted, and `R` is the random number generator that should
/// be used.
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
    /// Create a new `CountUnique` structure. `sz` is the number
    /// of elements stored by CVM, and must be non-zero.
    ///
    /// # Panics
    /// Function will panic if called with `sz < 1`.
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

    /// Add a token to the CVM.
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

    /// Return the current estimated number of tokens. Note that
    /// CVM is a probabilistic algorithm and the returned value can
    /// be far from the actual value, the algorithm merely guarantees
    /// that it will return the correct value in expectation.
    pub fn estimate(&self) -> Result<f64, PosOverflow<usize>> {
        let f = f64::value_from(self.treap.size())?;
        let p = self.p as f64;
        Ok(f / p)
    }

    /// Reset the CVM state allowing the structure to be reused.
    pub fn reset(&mut self) {
        self.treap.reset();
        self.p = 1.0f32;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    use rand::rngs::StdRng;
    #[test]
    pub fn sufficient_space_works() {
        let sentence = String::from("This was a triumph I am making a note here huge success");
        let mut ctr = CountUnique::new(StdRng::from_entropy(), 11);
        let tokens = sentence.split_whitespace();
        for t in tokens {
            ctr.add_token(String::from(t));
        }
        assert!(ctr.estimate().unwrap() == 11.0f64)
    }

    #[test]
    pub fn insufficient_space_works() {
        let sentence = String::from("This was a triumph I am making a note here huge success");
        let mut ctr = CountUnique::new(StdRng::from_entropy(), 10);
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
