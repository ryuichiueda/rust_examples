struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
}

fn main() {
    //let ore = Person{ name: "上田".to_string(), age: 100046 };
    let ore = Person::new("上田", 100046);
    println!("おっすおら{}、{}歳。", &ore.name, &ore.age);
}
