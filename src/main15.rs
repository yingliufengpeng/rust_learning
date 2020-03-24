use std::thread;
use std::time::Duration;

// 通道介绍, mpsc:多个生产者,一个消费者, spmc一个生产者,多个消费者


fn main() {


    // thread::spawn(|| {
    //     for i in 1..100 {
    //         println!("1 number {} ", i);
    //         thread::sleep(Duration::from_millis(10));
    //     }
    // }).join();
    //
    // thread::spawn(|| {
    //     for i in 1..200 {
    //         println!("2 number {} ", i);
    //         thread::sleep(Duration::from_millis(10));
    //     }
    // }).join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
       println!("v is {:?}", v);
    });
    handle.join();
    // println!("{:?}", rv.clone());


    println!("Hello, World!");

}