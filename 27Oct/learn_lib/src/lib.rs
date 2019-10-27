#[derive(Debug)]
pub struct patient {
    name: String,
    age: u8,
    salary: u32
}

impl patient {
    pub fn status(&self) {
        if self.salary > 30000 {
            println!("rich")
        }
        else {
            println!("poor")
        }
    }
    pub fn hello(name: String,age:u8,salary:u32)-> patient {
        patient {
            name,
            age,
            salary
        }
    }
}
