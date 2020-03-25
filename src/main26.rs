use mylib::factory;
use crate::mode_a::mode_b::mode_c::print_c;
extern crate rand;

mod mode_a {
    #[derive(Debug)]
    pub struct A {
        pub number: i32,
        name: String,
    }

    impl A {
        pub fn new_a() -> A {
            A {
                number: 1,
                name: String::from("kk")
            }
        }

        pub fn print_a(&self) {
            println!("number: {}, name {}", self.number, self.name);
        }
    }

    pub fn print_a() {
        println!("a");
    }

    pub mod mode_b {
        pub fn print_b() {
            println!("B");
        }

        pub mod mode_c {
            pub fn print_c() {
                println!("C");
                super::print_b();
                super::super::print_a();

            }
        }
    }
}

fn main() {
    println!("{:?}", factory::I);
    factory::produce_washing_machine::produce_washing_machine();

    let a = mode_a::A::new_a();
    println!("{:?}", a);

    print_c();

    let r: i32 = rand::random();
    println!("{:?}", r);
    // rand::seq::index::sample()
    mylib::aa::index::show();
}