/// A treap module.
mod treap_node;
use crate::data::Element;
use treap_node::TreapNode;

use std::{
    fmt::{Display, Formatter, Result},
    mem,
};

/// A `Treap` is a binary search tree and a heap
/// combined. This version was created for implementing
/// the CMV algorithm.
pub struct Treap<T, P>
where
    T: Ord,
    P: Ord,
{
    root: Option<Box<TreapNode<T, P>>>,
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

    /// Get the element with the maximum element.
    pub fn get_max(&self) -> Option<&Element<T, P>> {
        self.root.as_ref().map(|n| &n.element)
    }

    pub fn get(&self, e: T) -> Option<&Element<T, P>> {
        self.root.as_ref().and_then(|n| n.get(e))
    }

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
