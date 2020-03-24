fn main() {

    let a = A {
        name: "kkk",
    };

    println!("{:?}", a);
    println!("{}", a.get_name())
}

trait GetName {
    fn get_name2(&self) -> &str;
}

#[derive(Debug)]
struct A<'a> {
    name: &'a str,
}

impl<'a> A<'a> {
    fn do_something(&self) -> i32 {
        3
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn xx<'c>(&self, b: &'c str) -> &'c str {
        b
    }

    fn xx2<'c>(&self, b: &'c str) -> &'c str {
        b
    }
}

impl<'b> GetName for A<'b> {
    fn get_name2(&self) -> &str {
        self.get_name()
    }
}
/*
** 声明周期的省略的约束
*/

// 第一种省略声明周期的方式
fn get_a_str(s: &str) -> &str {
    s
}

/*
    1, 每个引用的参数都有它自己的生命周期的参数
*/



