// 通道介绍, mpsc:多个生产者,一个消费者, spmc一个生产者,多个消费者

use std::thread;
use std::sync::mpsc;

fn get_str() -> &'static str {
    "kkk"
}

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("kk");
        tx.send(val).unwrap(); // 调用Send的时候会发生move这样的动作
        // println!("val is {}", val);
    }) ;

    println!("Hello, World!");

    let received = rx.recv().unwrap();

    println!("received is {}", received);



}