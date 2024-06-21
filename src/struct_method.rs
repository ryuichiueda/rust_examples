struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn aging(&mut self, inc: u32) { //&mut selfはself: &mut Personの略
        self.age += inc;
    }
}

fn main() {
    let mut ore = Person{ name: "上田".to_string(), age: 100046 };
    ore.aging(100000);
    println!("おっすおら{}、{}歳。", &ore.name, &ore.age);
}
