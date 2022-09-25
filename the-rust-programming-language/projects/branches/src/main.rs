fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let counter = 6;

    if counter % 4 == 0 {
        println!("counter is divisible by 4");
    } else if counter % 3 == 0 {
        println!("counter is divisible by 3");
    } else if counter % 2 == 0 {
        println!("counter is divisible by 2");
    } else {
        println!("counter is not divisible by 4, 3, or 2");
    }
}
