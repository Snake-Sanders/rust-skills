// compile and run:
// rustc armstrong_numbers.rs
// ./armstrong_numbers

fn main() {
    let number: u32 = 153;
    if is_armstrong_number(number) {
        println!("numer: {} is armstrong ", number);
    } else {
        println!("numer: {} is not armstrong", number);
    }
}

pub fn is_armstrong_number(num: u32) -> bool {
    let str_num = num.to_string();
    let digits_count: u32 = str_num.len() as u32;
    let mut acc: u32 = 0;

    for c in str_num.chars() {
        let digit = c.to_digit(10).unwrap();
        println!("acc {}, digit {}, count {}", acc, digit, digits_count);
        acc += digit.pow(digits_count);
    }

    return acc == num;
}
