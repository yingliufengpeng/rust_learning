use std::collections::HashMap;
use maplit::hashmap;
use std::hash::Hash;

// 有条件的实现trait 对任何实现的类型,有条件的实现trait
trait GetName {
    fn get_name(&self) -> &String;
}

trait PrintName {
    fn print_name(&self);
}

// 条件trait的使用
impl <T: GetName> PrintName for T {
    fn print_name(&self) {
        println!("name = {}", self.get_name())
    }
}

struct Student {
    name: String,
    age: u32,
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}

fn longest<'a>(x: &'a str, y:&'a str) ->  &'a str {

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn a_str<'a>(x: &'a str, y: &'a str) -> &'a str {
//
//     // let r: &String = "abc".parse().unwrap();
//     let r = String::from("kkk"); // 内存回收,数据返回了回去,就是感觉是有问题的例子
//     r.as_str()
// }

fn main() {

    let s = Student {
        name: "w".parse().unwrap(),
        age: 33
    };

    let h = vec![
        vec![3, 4],
        vec![5, 6],
        vec![5, 6],
        vec![5, 6],
        vec![5, 6],
    ];
    println!("{:#?}", h);

    let m = hashmap! {
       "a" => 1,
       "b" => 2,
    };

    let mut v: Vec<Vec<Vec<i32>>>;
    let mut h: HashMap<String, i32> = HashMap::new();

    let mut arr:[[i32; 10]; 10] = [[0; 10]; 10];
    for row in arr.iter() {
        for col in row.iter() {
            println!("{}", *col);
        }
    }

    println!("m is {:?}", m);
    s.print_name()
}