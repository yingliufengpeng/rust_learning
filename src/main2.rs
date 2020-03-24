use mac;
use bingbingou::HelloMacro;
use bingbinggouderive::HelloMacro;
use std::fmt::Debug;

macro_rules! unless {
    ($arg:expr, $branch:expr) => (if (!$arg) {$branch;});
}

#[derive(Debug)] // 编译器自动实现了fmt::Display trait的逻辑
struct A {
    a: i32,
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { // 很显然,返回值是需要支持Copy语义才行
    let mut larger = &list[0];
    for item in list.iter() {
        if *item > *larger {
            larger = item;
        }
    }
    *larger
}

#[derive(HelloMacro)]
#[derive(Debug)]
struct Main {}

// 在结构体中使用泛型
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &U {
        &self.y
    }
}

pub trait GetInformataion {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

pub trait SchoolName {
    fn get_school_name(&self) -> String {
        "kk".parse().unwrap() // 数据则是在堆中进行分配的操作
    }
}

trait GetName {
    fn get_name2(&self) -> &String;
}

trait GetAge {
    fn get_age2(&self) -> u32;
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

#[derive(Debug)]
pub struct Student(Person);

impl GetInformataion for Person {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

impl GetInformataion for Student {
    fn get_name(&self) -> &String {
        // self.get_name()
        self.0.get_name()
    }

    fn get_age(&self) -> u32 {
        // unimplemented!()
        self.0.get_age()
    }
}

impl GetName for Student {
    fn get_name2(&self) -> &String {
        self.0.get_name()
    }
}

impl GetAge for Student {
    fn get_age2(&self) -> u32 {
        self.0.get_age()
    }
}

impl SchoolName for Person {}

impl SchoolName for Student {}

fn print_information(item: &impl GetInformataion) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

// 写法一
fn print_information2<T: GetInformataion + GetName + GetAge>(item: &T) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

// 写法二

fn print_information3<T>(item: &T)
    where T: GetInformataion + GetName + GetAge {
    print_information2(item)
}

fn produce_item_with_age() -> impl GetAge + Debug {
    Student {
        0: Person { name: "kk".parse().unwrap(), age: 4 }
    }
}


fn main() {
    // println!("Hello, world!");
    //
    // let (a, b) = (1, 2);
    // cmp(a, b);
    //
    // let c = mac::my_vec!(2, 3, 4);
    //
    // println!("v = {:?}", c)

    // Main::hello_macro()
    let numer_list = vec![1, 2 ,3, 4];
    let n = largest(&numer_list);
    println!("{}", n);

    // let char_list = vec!['3', '4', '5'];
    // let c = largest(&char_list);
    // println!("{}", c)

    // let c = Point{
    //     x: 3,
    //     y: 4.0,
    // };
    // println!("{:#?}", c)

    let s = Student {
        0: Person {
            name: String::from("kkk"),
            age: 44,
        }
    };
    println!("{:#?}", s);
    print_information3(&s);
    println!("{}", s.get_school_name());
    println!("{:#?}", produce_item_with_age())
}


fn cmp(a: i32, b: i32) {
    unless!(a > b, {
        println!("{} < {}", a, b);
    })
}

