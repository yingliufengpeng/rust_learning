use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {

        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
use List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(3, RefCell::new(Weak::new())));
    println!("1, a strong count = {}, a weak count {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("1, a tail = {:?}", a.tail());
    println!();
    let b = Rc::new(Cons(33, RefCell::new(Weak::new())));

    if let Some(link) = b.tail() {
        *link.borrow_mut() = Rc::downgrade(&a);
    }
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }

    println!("2, a strong count = {}, a weak count {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("2, a tail = {:?}", a.tail());
    println!();

    println!("2, b strong count = {}, b weak count {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("2, b tail = {:?}", b.tail());

}
