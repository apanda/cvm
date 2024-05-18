use std::{
    fmt::{Display, Formatter, Result},
    mem,
};

use super::treap_node::TreapNode;
use crate::data::Element;

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
    pub fn new() -> Treap<T, P> {
        Treap { root: None }
    }

    fn set_root(&mut self, root: Element<T, P>) {
        let _ = mem::replace(&mut self.root, Some(Box::new(root.into())));
    }

    pub fn reset(&mut self) {
        mem::take(&mut self.root);
    }

    pub fn insert(&mut self, element: Element<T, P>) {
        match &mut self.root {
            None => self.set_root(element),
            Some(e) => e.insert(element.into()),
        }
    }

    pub fn get_max(&self) -> Option<&Element<T, P>> {
        self.root.as_ref().map(|n| &n.element)
    }

    pub fn get(&self, e: T) -> Option<&Element<T, P>> {
        self.root.as_ref().map_or(None, |n| n.get(e))
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
