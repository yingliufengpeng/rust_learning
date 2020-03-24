use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    // 运行时改变其中的值
    Nil,
}

use List::{Cons, Nil};
use std::borrow::BorrowMut;

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            _ => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Cons(4, RefCell::new(Rc::new(Nil)))))));
    println!("a is {:?}", a);
    let b = Rc::new(Cons(10, RefCell::new(a.clone())));
    println!("b is {:?}", b);
    if let Some(t) = a.tail() {
        *t.borrow_mut() = b.clone();
    }
    println!("b is {:?}", b);
}