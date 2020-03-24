use gui::{Screen, Button, SelectBox};

trait Clone2 {
    fn clone(&self) -> Self;
}

fn main() {

    let s = Screen {
        components: vec![
            Box::new(Button{
                width: 3,
                height: 4,
                label: String::from("kk"),
            }),
            Box::new(SelectBox{
                width: 3,
                height: 4,
                option: Some(vec![
                    String::from("Yes"),
                    String::from("No"),
                ]),
            })
       ]
    };
    s.run();

}