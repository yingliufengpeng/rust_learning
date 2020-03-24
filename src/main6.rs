/*
    闭包可以通过三种方式获取其环境,它们对应函数的三种获取参数的方式,分别是获取所有权,可变借用,不可变借用.
    FnOnce 消费从周围作用域获取获取的变量,闭包周围的作用域

*/

use std::iter::Iterator;

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

fn main() {

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x; // 所有权进行转移的操作
    // println!("x is {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    let mut v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next() {
        *v = 33;
    }

    let i = 0 ;

    // 消费适配器
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let s: i32 = v1_iter.sum();
    println!("s is {}", s);

    // ---迭代适配器
    let v3 = vec![1, 2, 3];
    let v4: Vec<_> = v3.iter().map(|i| *i + 2).filter(|e| *e > 3).collect() ;
    println!("v4 is {:?}", v4);



}