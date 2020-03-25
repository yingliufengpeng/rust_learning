// 4 父 trait 用于在另一个trait中使用某trait的功能,
// 有时我们可能需要某个trait 使用另一个trait的功能.
// 在这种情况下,需要能够依赖先关的trait也被实现.
// 这个所需trait是我们实现的trait的父trait

// 5 newtype 模式用以在外部类型上实现 trait
// 孤儿规则(orphan rule) : 只要trait或类型对于当前create是本地的话,就可以在
// 此类型上实现trait.
// 一个绕开这个限制的方法的是使用newtype模式(newtype pattern).



use std::fmt;
use std::fmt::{Formatter, Error};
use std::rc::Rc;

trait OutPrint: fmt::Display {
    fn out_print(&self) {
        let output = self.to_string();
        println!("output: {}", output);
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutPrint for Point {}


fn main() {
    let point = Point { x: 3, y: 4 };
    point.out_print();

    let w = Wrapper(vec![
        String::from("kkk"),
        String::from("kkk"),
        String::from("kkk"),
        String::from("kkk"),
    ]);
    println!("{}", w);

    let x: Kilometers = 3;
    let y = 3;

    assert_eq!(x, y);

    let t = Box::new(|| { println!("hhh") });
    t();

    // bar();

    let s1 = "kkk";

    let mut c = do_twice(add_one, 2);

    println!("{:?}", c);

    let d = wrapper_func(add_one, 3);
    println!("{:?}", d);

    let e = wrapper_func(|e| e + 2, 4);
    println!("{:?}", e);

    let clourse = |e| e + d;
    let f = wrapper_func(clourse, 4);
    println!("{:?}", f);

    let clourse2= |e: i32| {
        c = 33;
        e + d
    };
    let g = wrapper_func2(clourse2, 4);
    println!("{:?}", g);

    let h = return_colurse();
    let h2 = h(4);
    println!("{:?}", h2);

}

fn return_colurse() -> Rc<dyn Fn(i32) -> i32>{

    Rc::new(|e| e)
}

fn wrapper_func<T>(t: T, v: i32) -> i32
    where T: Fn(i32) -> i32{
    t(v)
}

fn wrapper_func2<T>(mut t: T, v: i32) -> i32
    where T:FnMut(i32) -> i32{
    t(v)
}

// Fn, FnMut, FnOnce
// 函数指针实现了这三个闭包,

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, a: i32) -> i32 {
    f(a) + f(a)
}

fn generic<T>(t: T) {}

// 等价于
fn generic2<T: Sized>(t: T) {}

fn generic3<T: ?Sized>(t: &T) {}

fn bar() -> ! {
    fn loop2() -> ! {
        loop2()
    }
    loop2()
}

type Kilometers = i32;

type T = Box<dyn Fn() + Send + 'static>;


struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Wrapper ({})", self.0.join(" 0 "))
    }
}