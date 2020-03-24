#[macro_export]
macro_rules! my_vec { // my_vec! 模仿vec!
    ($($x: expr), *) => {

        {
            let mut vec = Vec::new();

            $(
                vec.push($x);
            )*


            vec
        }

    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
