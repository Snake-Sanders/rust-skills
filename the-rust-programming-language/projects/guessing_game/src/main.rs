use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret = rand::thread_rng().gen_range(1..=10);

    println!("The secret number is: {secret}.");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // `guess` is shadowing the previous defined variable with the same name
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    println!("You guessed: {guess}");

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
