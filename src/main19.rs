

fn main() {
    use std::sync::{Mutex, Arc}; // 这两者则是线程安全的语言
    use std::thread;
    use std::rc::Rc; // Rc不是线程安全的共享指针

    // let m = Mutex::new(5);
    //
    // {
    //     let mut num = m.lock().unwrap();
    //     *num += 3;
    //     println!("num is {}", num);
    // } // 离开作用域的时候,自动释放锁
    //
    // let mut num = m.lock().unwrap();
    // println!("num is {}", num);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let c_rc = counter.clone();
        let h = thread::spawn(move || {
            let mut n = c_rc.lock().unwrap();
            *n += 1;
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("res is {}", *counter.lock().unwrap());
}