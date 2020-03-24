fn main2() {

    let m = PeopleMatchInformation{
        master: Person{name: "p".parse().unwrap(), age: 40},
        student: Student{
            0: Person{name: "w".parse().unwrap(), age: 30},
        }
    };

    m.print_all_information()
}

trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}

#[derive(Debug)]
struct PeopleMatchInformation<T, U> {
    master: T,
    student: U,
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

#[derive(Debug)]
pub struct Student(Person);

impl GetName for Person {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Person {
    fn get_age(&self) -> u32 {
        self.age
    }
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        self.0.get_name()
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.0.get_age()
    }
}



// impl<T: GetName + GetAge, U: GetName + GetAge>  PeopleMatchInformation<T, U> {
impl<T, U>  PeopleMatchInformation<T, U>
where T: GetName + GetAge, U: GetName + GetAge {
    fn print_all_information(&self) {
        println!("print_all_information...");
        println!("master name is {}", self.master.get_name());
        println!("master age is {}", self.master.get_age());
        println!("student name is {}", self.master.get_name());
        println!("student age is {}", self.master.get_age());
    }
}
