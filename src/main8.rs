use std::ops::Deref;
use std::fmt::{Debug, Display};

fn main() {
    let c = Box::new("kkkk");
    println!("c is  {}", c);


    use List::{Cons, Nil};
    let list = Box::new(Cons {
        head: 3,
        tail: Box::new(Cons {
            head: 4,
            tail: Box::new(Cons {
                head: 5,
                tail: Box::new(Nil),
            }),
        }),
    });
    println!("list is {:#?}", list);

    let a = A {
        v: AA {
            v: 4
        }
    };

    let c2 = *a;

    println!("c is {}", c2);

    let v2 = MyBox::new("4");
    println!("v2 is {}", *v2);
    hello(&v2); // 将MyBox变为&String,再将String的解引用变为字符串slice &str
}

#[derive(Debug)]
enum List {
    Cons { head: i32, tail: Box<List> },
    Nil,
}

#[derive(Debug)]
struct AA {
    v: i32,
}

#[derive(Debug)]
struct A {
    v: AA,
}

impl Deref for AA {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl Deref for A {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello<T: Debug>(name: &T) {
    println!("Hello, {:#?}", name)
}

// 实现Deref trait允许我们重载解引用运算符

