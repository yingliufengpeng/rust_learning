use std::collections::{VecDeque, HashSet};

fn main() {
    let a = 1;
    match a {
        0 => println!("0"),
        1 => println!("1"),
        _ => println!("other"),
    }

    let age: Result<u8, _> = "33".parse();

    println!("{}", age.unwrap());

    let mut stack = vec![];
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(v) = stack.pop() {
        println!("v is {}", v);
    }

    let mut que = VecDeque::new();
    que.push_back(1);
    que.push_back(2);
    que.push_back(3);

    while let Some(v) = que.pop_front() {
        println!("v is {}", v);
    }

    let v = vec![1, 2, 3];
    for (index, v) in v.iter().enumerate() {
        println!("index {} v {}", index, v);
    }

    let mn = (1, 2, 3, 4);

    let (a, .., b) = mn;
    println!("a is {} b is {}", a, b);

    let set: HashSet<i32> = HashSet::new();

    fn print_point(&(x, y): &(i32, i32)) {
        println!("x is {}, y is {}", x, y);
    }

    let x = 5;
    match x {
        1..=5 => println!("1..5"),
        _ => println!("other"),
    }


    let p = Point { x: 1, y: 2 };

    let Point { x: a, y: b } = p;


    let c = 20;

    match p {
        Point { x, y } if y > 0 && x > 0 => println!("ok"),
        _ => println!("ok"),
    }

    let _x = 3;
    let _y = 4;

    println!("_x is {}", _x);

    let s = Some(String::from("hello"));

    if let Some(_c) = s {
        println!("found a string");
    }

    // println!("s is {}", s.unwrap());

    //@运算符允许我们在创建一个存放值的同时,测试这个变量的值是否是匹配的模式

    let msg = Message::Hello(3);

    match msg {
        Message::Hello(a@3..=7) => println!("a is {}", a),
        Message::Hello(a@ 8..=10) => println!("a is {}", a),
        _ => println!("k"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
    Hello(i32),
}

fn foo(_:i32, y: i32) {
    println!("y is {}", y)
}

trait A {
    fn bar(x: i32, y: i32);
}

struct B {
    a: i32,
}

impl A for B {
    fn bar(_: i32, y: i32) {
        println!("y is {}", y);
    }
}