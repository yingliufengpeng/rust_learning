// 4,解引用裸指针
// 不可变和可变的,分别写作*const T, *mut T
// (1) 允许忽略借用规则,可以同时拥有不可变和可变的指针
// (2) 不保证指向的内存是有效的
// (3) 允许为空
// (4) 不允许实现任何自动清理的功能

// 5 调用不安全的函数或者方法


unsafe fn dangerous() {
    println!("do something dangerous");
}

// 调用C语言的函数
extern "C" {
    fn abs(input: i32) -> i32;
}

// 6 访问或者修改可变静态变量

static HELLO_WORLD: &str = "Hello, World";

const HELLO_WORLD2: &str = "Hello, World";

static mut COUNTER: u32 = 0;

fn add_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// 7 实现不安全的trait,
// (1)当至少有一个方法中包含编译器不能验证不变量时,该trait是不安全的.
// (2) 在trait之前增加unsafe声明其为不安全时,同时trait的实现也必须用unsafe标记.

unsafe trait Foo {
    fn foo(&self);
}

struct Bar;

unsafe impl Foo for Bar {
    fn foo(&self) {
        println!("unsafe foo ...");
    }
}
// 关联类型在trait定义中指定占位符类型
// 关联类型 是一个将类型占位符和trait相关联的方式

pub trait Iter<T> {
    fn next(&mut self) -> Option<T>;
}

struct A {
    value: i32,
}

impl Iter<i32> for A {
    fn next(&mut self) -> Option<i32> {
        println!("in I32");
        if self.value > 3 {
            self.value += 1;
            Some(self.value)
        } else {
            None
        }
    }
}
// 默认泛型参数和运算法的重载
// (1) 使用泛型类型参数使,可以为泛型指定一个默认的具体类型
// (2) 运算符重载是指在特定情况下自定义运算符行为的操作.
// Rust并不允许创建自定义的运算符或者运算符重载,
// 不过杜宇std::ops中列出的运算符和相应的trait,我们可以实现运算符相关的trait类重载

use std::ops::Add;
use std::rc::Rc;

#[derive(Debug,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Meters;

    fn add(self, rhs: Meters) -> Self::Output {
        Meters(self.0 + rhs.0)
    }
}

// 完全限定语法
trait AA {
    fn print(&self);
}
trait BB {
    fn print(&self);
}

struct MyType;

impl AA for MyType {
    fn print(&self) {
        println!("AA")
    }
}

impl BB for MyType {
    fn print(&self) {
        println!("BB")
    }
}

impl MyType {
    fn print(&self) {
        println!("MyType")
    }
}


trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Dog")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Animal")
    }
}


fn main() {
    let mut num = 10;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let add = 0x111111usize;
    let _r = add as *const i32;

    unsafe {
        *r2 = 20;
        println!("r1 is {}", *r1);
        println!("r1 is {}", *r2);
        // println!("r3 is {}", *_r);
        dangerous();
        println!("abs(-3) is {}", abs(-3));
    }
    add_counter(3);
    let b = Bar {};
    b.foo();

    let p = Point {
        x: 3,
        y: 4
    };

    let p2 = Point {
        x: 3,
        y: 4
    };

    assert_eq!(p + p2, Point {x: 6, y: 8});

    // println!("{:?}", p);

    // let my = Box::new(MyType{});
    let my = Rc::new(MyType{});
    my.print();

    // (my as Box<dyn AA>).print();
    (my.clone() as Rc<dyn AA>).print();
    (my.clone() as Rc<dyn BB>).print();
    my.print();
    // (my as BB).print();
    // AA::print(my);

    Dog::baby_name();

    <Dog as Animal>::baby_name();

}
