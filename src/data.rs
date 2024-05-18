use std::cmp::{Ord, Ordering};
use std::fmt::{Display, Formatter, Result};

/// This is fundamentally what we are storing.
/// # Examples
/// ```
/// let e0 = Element::new("Hello", 22);
/// let e1 = Element::new("Bye", 12);
/// let e3 = Element::new("Hello", 21);
/// assert!(e0 > e1);
/// assert!(e3 < e0);
/// println!("Tested");
/// ```
pub struct Element<T: Ord, P: Ord = usize> {
    pub value: T,
    pub priority: P,
}

impl<T: Ord, P: Ord> Element<T, P> {
    pub fn new(value: T, priority: P) -> Self {
        Element { value, priority }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

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
