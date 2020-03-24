pub trait HelloMacro {
    fn hello_macro();  // 定义特征宏
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
