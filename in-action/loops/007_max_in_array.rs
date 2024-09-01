// Write a Rust program that finds and prints the maximum value
// in an array of integers.
//
fn main() {
    let numbers = [34, 50, 25, 100, 65];
    let mut max = numbers[0];

    for n in numbers {
        if n > max {
            max = n;
        }
    }

    println!("Max is {}", max);
}
