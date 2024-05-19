use rand::Rng;
use treap::{Element, Treap};
use treap_non_random as treap;

pub struct CountUnique<T: Ord, R: Rng> {
    treap: Treap<T, u32>,
    rng: R,
}

impl<T, R> Default for CountUnique<T, R>
where
    T: Ord,
    R: Rng + Default,
{
    fn default() -> Self {
        CountUnique {
            treap: Treap::new(),
            rng: Default::default(),
        }
    }
}
