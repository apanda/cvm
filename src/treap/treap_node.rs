use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
    mem,
};

use crate::data::Element;
use std::convert::From;

pub struct TreapNode<T, P>
where
    T: Ord,
    P: Ord,
{
    pub element: Element<T, P>,
    pub left: Option<Box<TreapNode<T, P>>>,
    pub right: Option<Box<TreapNode<T, P>>>,
}

impl<T, P> TreapNode<T, P>
where
    T: Ord,
    P: Ord,
{
    fn left_insert(&mut self, node: Self) {
        match &mut self.left {
            None => {
                let _ = mem::replace(&mut self.left, Some(Box::new(node)));
            }
            Some(e) => e.insert(node),
        }
    }

    fn right_insert(&mut self, node: Self) {
        match &mut self.right {
            None => {
                let _ = mem::replace(&mut self.right, Some(Box::new(node)));
            }
            Some(e) => e.insert(node),
        }
    }

    //       q               p
    //      / \             / \
    //     p  C   --->     A  q
    //    / \                / \
    //   A  B               B  C
    pub fn right_rotate(&mut self) {
        // Get left subtree.
        let l = mem::replace(&mut self.left, None);
        if let Some(mut p) = l {
            // self now has the contents of the left subtree
            mem::swap(self, &mut *p);
            // Make right child of left subtree, the old self's right subtree.
            mem::swap(&mut self.right, &mut p.left);
            // Make old self the right subtree.
            let _ = mem::replace(&mut self.right, Some(p));
        }
    }

    //     p               q
    //    / \             / \
    //   A  q   --->     p  C
    //     / \          / \
    //    B  C         A  B
    pub fn left_rotate(&mut self) {
        let r = mem::replace(&mut self.right, None);
        if let Some(mut q) = r {
            // Self now points to the right subtree
            mem::swap(self, &mut *q);
            // Move the right subtrees left branch to the old self's right subtree.
            mem::swap(&mut self.left, &mut q.right);
            let _ = mem::replace(&mut self.left, Some(q));
        }
    }
    pub fn heap_check(&self, n: &Option<Box<TreapNode<T, P>>>) -> bool {
        if let Some(node) = n {
            node.element.priority() <= self.element.priority()
        } else {
            true
        }
    }
    pub fn insert(&mut self, node: Self) {
        match self.element.value().cmp(node.element.value()) {
            Ordering::Equal => {
                let _ = mem::replace(self, node);
                if !self.heap_check(&self.left) {
                    self.right_rotate()
                } else if !self.heap_check(&self.right) {
                    self.left_rotate()
                }
            }
            Ordering::Greater => {
                self.left_insert(node);
                if !self.heap_check(&self.left) {
                    self.right_rotate()
                }
            }
            Ordering::Less => {
                self.right_insert(node);
                if !self.heap_check(&self.right) {
                    self.left_rotate()
                }
            }
        }
    }

    pub fn get(&self, e: T) -> Option<&Element<T, P>> {
        match &self.element.value().cmp(&e) {
            Ordering::Equal => Some(&self.element),
            Ordering::Greater => {
                if let Some(l) = self.left.as_ref() {
                    l.get(e)
                } else {
                    None
                }
            }
            Ordering::Less => {
                if let Some(r) = self.right.as_ref() {
                    r.get(e)
                } else {
                    None
                }
            }
        }
    }
}

impl<T, P> Display for TreapNode<T, P>
where
    T: Ord + Display,
    P: Ord + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match (&self.left, &self.right) {
            (Some(l), Some(r)) => write!(
                f,
                "( {} {} {} )",
                l.to_string(),
                self.element.to_string(),
                r.to_string()
            ),
            (None, Some(r)) => write!(f, "( _ {} {} )", self.element.to_string(), r.to_string()),
            (Some(l), None) => write!(f, "( {} {} _ )", l.to_string(), self.element.to_string()),
            (None, None) => write!(f, "( _ {} _ )", self.element.to_string()),
        }
    }
}

impl<T, P> From<Element<T, P>> for TreapNode<T, P>
where
    T: Ord,
    P: Ord,
{
    fn from(element: Element<T, P>) -> Self {
        TreapNode {
            element,
            left: None,
            right: None,
        }
    }
}
