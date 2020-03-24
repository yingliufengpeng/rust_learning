
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter{
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.count;
        self.count += 1;
        Some(c)
    }
}

fn main() {
    let mut c = Counter::new();

    for i in 0..1000 {
        if let Some(v) = c.next() {
            println!("i = {}, v = {}", i, v);
        }
    }
}