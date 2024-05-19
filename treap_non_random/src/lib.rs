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
//! use treap_non_random as treap;
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
    size: usize,
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
        Treap {
            root: None,
            size: 0,
        }
    }

    fn set_root(&mut self, root: Element<T, P>) {
        let _ = mem::replace(&mut self.root, Some(Box::new(root.into())));
    }

    /// Reset the Treap, removing all items.
    pub fn reset(&mut self) {
        mem::take(&mut self.root);
        self.size = 0;
    }

    /// Insert (or update) an item.
    pub fn insert(&mut self, element: Element<T, P>) {
        match &mut self.root {
            None => {
                self.set_root(element);
                self.size = 1;
            }
            Some(e) => {
                if e.insert_or_replace(element.into()) {
                    self.size += 1;
                }
            }
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
                let deleted = if *r.element.value() == e {
                    if r.left.is_none() && r.right.is_none() {
                        self.reset();
                        false
                    } else if r.left.is_none() && r.right.is_some() {
                        r.rotate_left();
                        r.delete(e)
                    } else if r.right.is_none() && r.left.is_some() {
                        r.rotate_right();
                        r.delete(e)
                    } else {
                        let p_left = r.left.as_ref().unwrap().element.priority();
                        let p_right = r.right.as_ref().unwrap().element.priority();
                        if p_left < p_right {
                            r.rotate_left();
                            r.delete(e)
                        } else {
                            r.rotate_right();
                            r.delete(e)
                        }
                    }
                } else {
                    r.delete(e)
                };
                if deleted {
                    self.size -= 1;
                }
            }
        }
    }

    /// Get the number of elements in `self`.
    pub fn size(&self) -> usize {
        self.size
    }

    #[cfg(test)]
    fn maintains_heap(&self) -> bool {
        self.root
            .as_ref()
            .map(|r| r.maintains_heap())
            .unwrap_or(true)
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
        assert!(t.maintains_heap());
        assert!(t.size() == 7);
        let m = t.get_max();
        assert!(m.is_some());
        assert!(m.unwrap().value().eq("hi".into()));
    }

    #[test]
    fn delete_works() {
        let mut t = setup_standard_treap();
        assert!(t.maintains_heap());
        let before = t.get("lo".into());
        assert!(before.is_some());
        assert!(t.size() == 7);
        t.delete("lo".into());
        assert!(t.maintains_heap());
        let after = t.get("lo".into());
        assert!(after.is_none());
        assert!(t.size() == 6);
    }

    #[test]
    fn insert_works() {
        let mut t = setup_standard_treap();
        let prev_max = *t.get_max().unwrap().priority();
        let prev_size = t.size();
        let _ = prev_size;
        t.insert(Element::new("new".into(), prev_max + 1));
        assert!(t.maintains_heap());
        assert!(t.size() == prev_size + 1);
        assert!(*(t.get_max().unwrap().priority()) == prev_max + 1);
        t.insert(Element::new("new2".into(), prev_max - 2));
        assert!(t.maintains_heap());
        assert!(t.size() == prev_size + 2);
        assert!(*(t.get_max().unwrap().priority()) == prev_max + 1);
    }
}
