pub struct AverCollect {
    list: Vec<i32>,
    aver: f64,
}

impl AverCollect {
    pub fn new() -> Self {
        AverCollect {
            list: vec![],
            aver: 0.0,
        }
    }

    pub fn get_aver(&self) -> f64 {
        self.aver
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(v) => {
                self.update_average();
                Some(v)
            },
            None => None,
        }
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.aver = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
