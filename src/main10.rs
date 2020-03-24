use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {

    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Rc::new(Cons(4, Rc::clone(&a)));
    // let c = Rc::new(Cons(33, Rc::clone(&a)));

    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::new(Cons(4, a.clone()));
    let c = Rc::new(Cons(33, a.clone()));

    println!("count after creating a = {}", Rc::strong_count(&a));


}