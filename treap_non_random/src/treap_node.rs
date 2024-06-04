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
    P: PartialOrd,
{
    pub element: Element<T, P>,
    pub left: Option<Box<TreapNode<T, P>>>,
    pub right: Option<Box<TreapNode<T, P>>>,
}

enum TreapChild {
    Left,
    Right,
}

impl<T, P> TreapNode<T, P>
where
    T: Ord,
    P: PartialOrd,
{
    fn left_insert(&mut self, node: Self) -> bool {
        match &mut self.left {
            None => {
                let _ = mem::replace(&mut self.left, Some(Box::new(node)));
                true
            }
            Some(e) => e.insert_or_replace(node),
        }
    }

    fn right_insert(&mut self, node: Self) -> bool {
        match &mut self.right {
            None => {
                let _ = mem::replace(&mut self.right, Some(Box::new(node)));
                true
            }
            Some(e) => e.insert_or_replace(node),
        }
    }

    /// Rotate the tree right.
    #[no_alloc]
    pub fn rotate_right(&mut self) {
        // Get left subtree.
        let l = mem::take(&mut self.left);
        if let Some(mut p) = l {
            // self now has the contents of the left subtree
            mem::swap(self, &mut *p);
            // Make right child of left subtree, the old self's right subtree.
            mem::swap(&mut self.right, &mut p.left);
            // Make old self the right subtree.
            let _ = mem::replace(&mut self.right, Some(p));
        }
    }

    /// Rotate the tree left.
    #[no_alloc]
    pub fn rotate_left(&mut self) {
        let r = mem::take(&mut self.right);
        if let Some(mut q) = r {
            // Self now points to the right subtree
            mem::swap(self, &mut *q);
            // Move the right subtrees left branch to the old self's right subtree.
            mem::swap(&mut self.left, &mut q.right);
            let _ = mem::replace(&mut self.left, Some(q));
        }
    }

    /// Check heap property holds. The goal here is to make sure that
    /// the root is always the largest value, and larger values propagate
    /// up the tree.
    #[no_alloc]
    pub fn heap_check(&self, n: &Option<Box<TreapNode<T, P>>>) -> bool {
        if let Some(node) = n {
            node.element.priority() <= self.element.priority()
        } else {
            true
        }
    }

    /// Insert a new node or modify an existing one.
    /// Return true if a new node is inserted.
    pub fn insert_or_replace(&mut self, node: Self) -> bool {
        match self.element.value().cmp(node.element.value()) {
            Ordering::Equal => {
                let _ = mem::replace(self, node);
                if !self.heap_check(&self.left) {
                    self.rotate_right()
                } else if !self.heap_check(&self.right) {
                    self.rotate_left()
                };
                false
            }
            Ordering::Greater => {
                let r = self.left_insert(node);
                if !self.heap_check(&self.left) {
                    self.rotate_right()
                };
                r
            }
            Ordering::Less => {
                let r = self.right_insert(node);
                if !self.heap_check(&self.right) {
                    self.rotate_left()
                };
                r
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_mut_node(&mut self, e: T) -> Option<&mut Self> {
        match &self.element.value().cmp(&e) {
            Ordering::Equal => Some(self),
            Ordering::Greater => {
                if let Some(l) = self.left.as_mut() {
                    l.get_mut_node(e)
                } else {
                    None
                }
            }
            Ordering::Less => {
                if let Some(r) = self.right.as_mut() {
                    r.get_mut_node(e)
                } else {
                    None
                }
            }
        }
    }

    /// Get the node with value `e`. Note, we do not provide a
    /// get with priorities, the tree is not set up to make that
    /// lookup efficient.
    #[no_alloc]
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

    /// Delete a node with element `e`. Note, we cannot delete the root itself,
    /// for one we might have nothing to replace it with. The Treap itself takes
    /// care of this problem.
    #[no_alloc]
    pub fn delete(&mut self, e: &T) -> bool {
        match &self.element.value().cmp(e) {
            Ordering::Equal => {
                panic!("You don't want to do this, it is bad idea.")
            }
            Ordering::Greater => {
                if self.left.is_some() {
                    if self.left.as_ref().unwrap().element.value() == e {
                        self.delete_child(TreapChild::Left);
                        true
                    } else {
                        self.left.as_deref_mut().unwrap().delete(e)
                    }
                } else {
                    false
                }
            }
            Ordering::Less => {
                if self.right.is_some() {
                    if self.right.as_ref().unwrap().element.value() == e {
                        self.delete_child(TreapChild::Right);
                        true
                    } else {
                        self.right.as_deref_mut().unwrap().delete(e)
                    }
                } else {
                    false
                }
            }
        }
    }

    #[no_alloc]
    fn delete_child(&mut self, child: TreapChild) {
        let done = {
            let which = match child {
                TreapChild::Left => self.left.as_deref_mut().unwrap(),
                TreapChild::Right => self.right.as_deref_mut().unwrap(),
            };
            if which.left.is_none() && which.right.is_none() {
                true
            } else if which.left.is_none() {
                which.rotate_left();
                which.delete_child(TreapChild::Left);
                false
            } else if which.right.is_none() {
                which.rotate_right();
                which.delete_child(TreapChild::Right);
                false
            } else {
                let p_left = which.left.as_ref().unwrap().element.priority();
                let p_right = which.right.as_ref().unwrap().element.priority();
                if p_left < p_right {
                    which.rotate_left();
                    which.delete_child(TreapChild::Left);
                } else {
                    which.rotate_right();
                    which.delete_child(TreapChild::Right);
                }
                false
            }
        };
        if done {
            match child {
                TreapChild::Left => mem::take(&mut self.left),
                TreapChild::Right => mem::take(&mut self.right),
            };
        }
    }

    #[cfg(test)]
    pub fn maintains_heap(&self) -> bool {
        let left_test = self
            .left
            .as_ref()
            .map(|n| (n.element.priority() <= self.element.priority() && n.maintains_heap()))
            .unwrap_or(true);
        let right_test = self
            .right
            .as_ref()
            .map(|n| (n.element.priority() <= self.element.priority() && n.maintains_heap()))
            .unwrap_or(true);
        left_test && right_test
    }
}

impl<T, P> Display for TreapNode<T, P>
where
    T: Ord + Display,
    P: PartialOrd + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match (&self.left, &self.right) {
            (Some(l), Some(r)) => write!(f, "( {} {} {} )", l, self.element, r),
            (None, Some(r)) => write!(f, "( _ {} {} )", self.element, r),
            (Some(l), None) => write!(f, "( {} {} _ )", l, self.element),
            (None, None) => write!(f, "( _ {} _ )", self.element),
        }
    }
}

impl<T, P> From<Element<T, P>> for TreapNode<T, P>
where
    T: Ord,
    P: PartialOrd,
{
    fn from(element: Element<T, P>) -> Self {
        TreapNode {
            element,
            left: None,
            right: None,
        }
    }
}
