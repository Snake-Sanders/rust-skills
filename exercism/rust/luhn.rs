// compile and run:
// rustc luhn.rs
// ./luhn

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut is_valid: bool = false;
    let number = code.replace(" ", "");

    println!("{} <- evaluating ", code);

    if number.len() > 1 {
        let res: u32 = number
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let d = match c.to_digit(10) {
                    Some(d) if (i % 2 == 0) => double_with_overflow(d),
                    Some(d) => d,
                    None => panic!(),
                };

                print!("{}", d);
                d
            })
            .collect::<Vec<u32>>()
            .into_iter()
            .sum();

        is_valid = res % 10 == 0;
        println!("\nresult: {}, valid: {}", res, is_valid);
    }

    return is_valid;
}

fn double_with_overflow(digit: u32) -> u32 {
    let double: u32 = digit * 2;

    let result = if double > 9 { double - 9 } else { double };
    return result;
}

fn main() {
    println!("empty {}", is_valid(""));
    println!("one {}", is_valid("1"));
    println!("code valid {}", is_valid("4539 3195 0343 6467"));
    println!("code invalid {}", is_valid("8273 1232 7352 0569"));
}
