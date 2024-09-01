use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    println!("Hello {}", name);
}
