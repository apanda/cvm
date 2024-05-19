//! A Treap library
//!
//! A treap (Aragon and Siedel '89, Randomized Search Tree) is a binary search
//! tree. Each element in the binary search tree contains a value and a priority,
//! and the algorithm guarantees two things:
//! (a) The binary search tree is arranged according to values, and thus in (good
//!     cases), you can find a value (or check for its existence) in O(lg n) time.
//! (b) The root of the binary tree is always the element with the largest "priority".
//!
//! Traditionally, random priorities are used and thus in expectation the tree is balanced.
//! However, Treaps are not a particularly interesting way to build sets or hashmaps, you are
//! better served using the standard Rust BTree instead.
//!
//! This implementation exists instead to be used in cases where accessing elements with max
//! priorities and checking existence are both necessary, as is the case with the CVM algorithm
//! (https://cs.stanford.edu/~knuth/papers/cvm-note.pdf).
//!
//! # Example
//! ```
//! use treap::{Element, Treap};
//!
//! let mut t: Treap<String, i32> = Treap::new();
//! t.insert(Element::new("A".into(), 0));
//! t.insert(Element::new("lo".into(), -22));
//! t.insert(Element::new("hi".into(), 65536));
//! let max = t.get_max();
//! assert!(max.is_some() && max.unwrap().value().eq("hi".into()));
//! let lo = t.get("lo".into());
//! assert!(lo.is_some());
//! let no = t.get("missing".into());
//! assert!(no.is_none());
//! ```

#![deny(missing_docs)]
mod data;
mod treap_node;
pub use data::Element;
use treap_node::TreapNode;

use std::{
    fmt::{Display, Formatter, Result},
    mem,
};

/// The Treap structure.
pub struct Treap<T, P>
where
    T: Ord,
    P: Ord,
{
    root: Option<Box<TreapNode<T, P>>>,
}

impl<T, P> Default for Treap<T, P>
where
    T: Ord,
    P: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, P> Treap<T, P>
where
    T: Ord,
    P: Ord,
{
    /// Create a new Treap.
    pub fn new() -> Treap<T, P> {
        Treap { root: None }
    }

    fn set_root(&mut self, root: Element<T, P>) {
        let _ = mem::replace(&mut self.root, Some(Box::new(root.into())));
    }

    /// Reset the Treap, removing all items.
    pub fn reset(&mut self) {
        mem::take(&mut self.root);
    }

    /// Insert (or update) an item.
    pub fn insert(&mut self, element: Element<T, P>) {
        match &mut self.root {
            None => self.set_root(element),
            Some(e) => e.insert(element.into()),
        }
    }

    /// Get the element with the highest priority, otherwise return `None`.
    pub fn get_max(&self) -> Option<&Element<T, P>> {
        self.root.as_ref().map(|n| &n.element)
    }

    /// Get an element whose value is `e` if it exists, otherwise return `None`.
    pub fn get(&self, e: T) -> Option<&Element<T, P>> {
        self.root.as_ref().and_then(|n| n.get(e))
    }

    /// Delete element whose value is `e`.
    pub fn delete(&mut self, e: T) {
        match &mut self.root {
            None => {}
            Some(r) => {
                if *r.element.value() == e {
                    if r.left.is_none() && r.right.is_none() {
                        self.reset()
                    } else if r.left.is_none() && r.right.is_some() {
                        r.rotate_left();
                        r.delete(e);
                    } else if r.right.is_none() && r.left.is_some() {
                        r.rotate_right();
                        r.delete(e);
                    } else {
                        let p_left = r.left.as_ref().unwrap().element.priority();
                        let p_right = r.right.as_ref().unwrap().element.priority();
                        if p_left < p_right {
                            r.rotate_left();
                            r.delete(e);
                        } else {
                            r.rotate_right();
                            r.delete(e);
                        }
                    }
                } else {
                    r.delete(e)
                }
            }
        }
    }
}

impl<T, P> Display for Treap<T, P>
where
    T: Ord + Display,
    P: Ord + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.root {
            None => write!(f, "nil"),
            Some(r) => r.fmt(f),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_standard_treap() -> Treap<String, i32> {
        let mut t = Treap::new();
        t.insert(Element::new("A".into(), 0));
        t.insert(Element::new("lo".into(), -22));
        t.insert(Element::new("hi".into(), 65536));
        t.insert(Element::new("xx".into(), 2));
        t.insert(Element::new("y".into(), 4));
        t.insert(Element::new("z".into(), 6));
        t.insert(Element::new("cc".into(), 8));
        t
    }

    #[test]
    fn max_is_correct() {
        let t = setup_standard_treap();
        let m = t.get_max();
        assert!(m.is_some());
        assert!(m.unwrap().value().eq("hi".into()));
    }

    #[test]
    fn delete_works() {
        let mut t = setup_standard_treap();
        let before = t.get("lo".into());
        assert!(before.is_some());
        t.delete("lo".into());
        let after = t.get("lo".into());
        assert!(after.is_none());
    }
}
