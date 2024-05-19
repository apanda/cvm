use std::cmp::{Ord, Ordering};
use std::fmt::{Display, Formatter, Result};

/// The element type encapsulates the data stored in the
/// treap. It consists of a `value`, which can be used
/// to `get` an element from the Treap, and a `priority`
/// which orders the Treap. The Treap's `get_max()` function
/// can be used to get the element with the largest priority.
///
/// # Example
/// ```
/// use treap::Element;
///
/// let e0 = Element::new("Hello", 22);
/// assert_eq!(*e0.value(), "Hello");
/// assert_eq!(*e0.priority(), 22);
/// ```
pub struct Element<T: Ord, P: Ord = usize> {
    value: T,
    priority: P,
}

impl<T: Ord, P: Ord> Element<T, P> {
    /// Create a new Element.
    pub fn new(value: T, priority: P) -> Self {
        Element { value, priority }
    }

    /// Get the Element's value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get the Element's priority.
    pub fn priority(&self) -> &P {
        &self.priority
    }
}
impl<T, P> PartialEq for Element<T, P>
where
    T: Ord,
    P: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.priority == other.priority
    }
}

impl<T, P> Eq for Element<T, P>
where
    T: Ord,
    P: Ord,
{
}

impl<T: Ord, P: Ord> PartialOrd for Element<T, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord, P: Ord> Ord for Element<T, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.priority.cmp(&other.priority) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.value.cmp(&other.value),
        }
    }
}

impl<T, P> Display for Element<T, P>
where
    T: Ord + Display,
    P: Ord + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<{}, {}>", self.value, self.priority)
    }
}
