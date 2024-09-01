// Write a Rust program that takes a string and prints it in reverse.

fn main() {
    let word = "fantastic";
    for c in word.chars().rev() {
        print!("{}", c);
    }
    println!("");
}
