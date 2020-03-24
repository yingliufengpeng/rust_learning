// 1, Drop trait类似于其他语言中的析构函数,

struct Dog {
    name: String,
}

impl Drop for Dog {
    // Drop这样的参数必须是可变参数
    fn drop(&mut self) {
        println!("Dog leave!!!")
    }
}

fn main() {


    let  a = Dog {
        name: "44".parse().unwrap(),
    };

    std::mem::drop(a);

    println!("ok")

}