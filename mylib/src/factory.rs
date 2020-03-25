pub const I: i32 = 20;

pub mod produce_refrigerator {
    pub fn produce_re() {
        println!("produce refrigerator!");
    }
}

pub mod produce_washing_machine {
    // use crate::factory::produce_refrigerator::produce_re;


    use crate::factory::produce_refrigerator::produce_re;

    pub fn produce_washing_machine() {
        produce_re();
        println!("produce washing");

    }
}
