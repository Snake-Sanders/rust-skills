fn main() {
    let fruits = ["pear", "cherry", "grapes", "mango"];

    // iterating directly over the elements

    println!("direct access: ");
    for f in fruits {
        print!("{} ", f);
    }
    println!("");

    println!("with iter: ");
    for &f in fruits.iter() {
        print!("{} ", f)
    }
    println!("");
}
