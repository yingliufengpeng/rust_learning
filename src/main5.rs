use std::fmt::Display;

fn function<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("ann is {}", ann);

    if x.len() < y.len() {
        x
    } else {
        y
    }
}

struct Cacher<T>
    where T: Fn(u32) -> u32 {
    calcuation: T,
    value: Option<u32>,
}

impl <T> Cacher<T>
where T: Fn(u32) -> u32 {
    fn new(caclation: T) -> Cacher<T> {
        Cacher{
            calcuation: caclation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v, // 默认使用的是copy语义
            None => {
                let v = (self.calcuation)(arg);
                self.value = Some(v);
                v
            }

        }
    }

}


fn main() {

    // let a = function("aa", "b", 4);
    // println!("a is {}", a)

    // let use_closure = || {
    //   println!("This is a closure!!!")
    // };
    //
    // let use_closure2 = |n: i32| {
    //     println!("n is {}", n);
    //     n
    // };
    //
    // use_closure();
    // use_closure2(3);
    //
    // let add_one_v2 = |x| { x + 1};
    // let add_one_v3 = |x| x + 2;
    // add_one_v2(3);

    // let mut acc = 0;
    // let mut add_one = |x: i32| { acc += x };
    // let mut add_one_v2 = || { acc += 1 };
    // // add_one(3);
    // // add_one(3);
    // // add_one(3);
    //
    fn acc_fun(mut n: i32) {
        while n > 0 {
            n -= 1;
            // add_one_v2();
            add_one_v2();
        }
    }
    //
    acc_fun(1000);
    //
    // println!("acc is {}", acc)

    // let mut c = Cacher::new(|x| x + 1);
    //
    // let m = c.value(3);
    //
    // println!("m is {}", m)

}

