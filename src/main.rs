use std::cell::RefCell;
use std::rc::{Weak, Rc};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node{
        value:3,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(Vec::new())
    });

    println!("leaf is {:?}", leaf);
    println!("leaf'parent  is {:?}", leaf.parent.borrow_mut().upgrade());

    let branch = Rc::new(Node{
        value: 4,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![leaf.clone()])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf is {:?}", leaf);
    println!("leaf'parent  is {:?}", leaf.parent.borrow_mut().upgrade());
    // println!("parent is {:#?}", branch);



}