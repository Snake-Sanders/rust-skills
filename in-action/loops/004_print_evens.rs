fn main() {
    print!("even numbers: ");
    for i in (1..=10).step_by(2) {
        print!("{} ", i);
    }
    println!("");
}
