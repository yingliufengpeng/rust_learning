use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals { // 拿走vals中所有元素的所有权
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });

    for rec in rx {
        println!("Got: {}", rec);
    }
}