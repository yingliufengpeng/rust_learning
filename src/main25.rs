
fn main() {

    let x = 1;
    {
        let y = 2;
        println!("{:?}", y);
    }
    println!("{:?}", x);

    let a = String::from("kk2");
    {
        let aa = a.clone() ;
        println!("{:?}", aa);
    }
    println!("{:?}", a);

    let b = "kkk";
    {
        let bb = b;
        println!("{}", bb);
    }
    println!("{}", b);

    let c = 1;
    let cc = c; // 编译器默认是整型的Copy Trait的语法 编译器默认实现
    // Copy trait, Clone trait
    let m = String::from("kk");
    takes_ownership(m.clone());
    println!("{:?}", m);
    let n = gives_ownership();
    let n2 = takes_and_gives_back(n);
    println!("{:?}", n2);
    let n_length = calculate_length(&n2);
    println!("{:?}", n2);
    let mut n3 = String::from("kk");
    modify_s(&mut n3);
    println!("{:?}", n3);
    let n4 = &mut n3;
    println!("{:?}", n4);
    let n5 = &mut n3;
    // let n6 = &mut n5;
    println!("{:?}", n5);
    // modify_s(n4);
    // modify_s(n5);
    let mut k1 = String::from("kk");
    let k2 = &mut k1;
    let k3 = &mut k1;
    let k4 = &mut k1;
    let k5 = &mut k1;
    k1.push('k');
    let k6 = &mut k1;
    let k7 = &mut k1;

    let s = String::from("hello");
    let h = &s[0..4];
    println!("{:?}", s);
    println!("{:?}", h);
    let h2 = &h[0..1];
    println!("{:?}", h2);
}

fn modify_s(s: &mut String) {
     s.push('k');
     s.push('k');
     s.push('k');
     s.push('k');
     s.push('k');
}

fn dangle() -> &'static str {
    let s = String::from("kk");
    "kk"
}

fn calculate_length(s: &String) -> u32 {
    s.len() as u32
}

fn gives_ownership() -> String {
    String::from("kk")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn takes_ownership(some_string: String) {
    println!("copy {:?}", some_string);
}