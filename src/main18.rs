use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx1.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }

    });

    thread::spawn(move || {
        let vals = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ];
        for v in vals {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }

    });


    thread::spawn(move || {
        let vals = vec![
            String::from("aa"),
            String::from("bb"),
            String::from("cc"),
            String::from("dd"),
        ];
        for v in vals {
            tx2.send(v).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }

    });
    for r in rx {
        println!("r is {}", r);
    }
}