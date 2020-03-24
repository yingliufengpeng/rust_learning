use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Nil, Cons};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    //
    // let value = Rc::new(
    //     Cons(RefCell::new(4), Rc::new(Cons(RefCell::new(5), Rc::new(Nil)))));
    let value = Rc::new(RefCell::new(4));
    let a = Cons(value.clone(), Rc::new(Nil));
    let b = Cons(value.clone(), Rc::new(Nil));
    let c = Cons(value.clone(), Rc::new(Nil));
    *value.borrow_mut() += 20;

    println!("v is {:?}", value);



}