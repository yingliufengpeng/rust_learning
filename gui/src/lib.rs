use std::fmt::{Display, Formatter, Error};

pub trait Draw {
    fn draw(&self);
}

// #[derive(Debug)]
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
    // pub components: Vec<Box<dyn Clone>>,
}

impl Screen {
    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

// impl Display for Screen {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         formatter.write_str("Foo")
//     }
// }

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button draw")
    }
}
#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub option: Option<Vec<String>>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox draw")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
