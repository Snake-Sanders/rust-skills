// Write a Rust program that prints the numbers from 1 to 20.
// For multiples of 3, print "Fizz" instead of the number,
// and for multiples of 5, print "Buzz".
// For numbers that are multiples of both 3 and 5, print "FizzBuzz".
fn main() {
    for n in 1..=20 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
}
