mod data;
mod treap;
use data::Element;
use std::string::String;
use treap::Treap;
fn main() {
    let e0: Element<String> = Element::new("A".into(), 1);
    let e1: Element<String> = Element::new("B".into(), 2);
    let e2: Element<String> = Element::new("C".into(), 3);
    let mut t = Treap::new();
    t.insert(e0);
    t.insert(e1);
    t.insert(e2);
    println!("Treep is {}", t);
    let e0: Element<String> = Element::new("C".into(), 1);
    let e1: Element<String> = Element::new("B".into(), 2);
    let e2: Element<String> = Element::new("A".into(), 3);
    let e3: Element<String> = Element::new("C".into(), 4);
    t.reset();
    t.insert(e0);
    t.insert(e1);
    t.insert(e2);
    println!("Treep is {} max is {}", t, t.get_max().unwrap());
    println!("Get A returns {}", t.get("A".into()).unwrap());
    t.insert(e3);
    println!("Treep is {} max is {}", t, t.get_max().unwrap());
    println!("Get A returns {}", t.get("A".into()).unwrap());

    println!("Tested");
}
