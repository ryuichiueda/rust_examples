struct Person {
    pub name: String,
    pub age: u32,
}

fn main() {
    let ore = Person{ name: "上田".to_string(), age: 100046 };
    println!("おっすおら{}、{}歳。", &ore.name, &ore.age);
}
