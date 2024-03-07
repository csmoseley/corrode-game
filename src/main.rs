// use ferris_says::say;
// use std::io::{stdout, BufWriter};
use std::fs;

fn main() {
    let mut line = String::new();

    let mut new_char = Character::new("dave".to_string());

    println!("{}", new_char.name);

    println!("Enter your name :");
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    new_char.rename(&line);
    println!("test {}", new_char.name);
    fs::write("test.txt",line).expect("Failed to write file");
    let mut static_value =  "item";
    println!("{}", static_value);
    static_value = "test_static";
    println!("{}", static_value);
}

pub struct Character {
    name: String,
}

impl Character {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
    pub fn rename(&mut self, name: &String) {
        self.name = name.to_string();
    }
}