fn main() {
    let word: &str = "continental";
    for c in word.chars() {
        print!("[{c}]");
    }
    println!("");
}
