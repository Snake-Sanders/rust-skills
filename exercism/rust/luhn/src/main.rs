/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut is_valid = true;
    let numbers = code
        .replace(" ", "")
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| match c.to_digit(10) {
            Some(d) if (i % 2 != 0) => double_with_overflow(d),
            Some(d) => d,
            None => {
                // not a digit
                is_valid = false;
                0
            }
        })
        .collect::<Vec<u32>>();

    is_valid = is_valid && numbers.len() > 1;

    if is_valid {
        let sum: u32 = numbers.into_iter().sum();
        is_valid = sum % 10 == 0;
    }

    is_valid
}

fn double_with_overflow(digit: u32) -> u32 {
    let double: u32 = digit * 2;

    if double > 9 {
        double - 9
    } else {
        double
    }
}

fn main() {
    println!("059 {}", is_valid("059"));
    //println!("empty {}", is_valid(""));
    //println!("one {}", is_valid("1"));
    println!("code valid {}", is_valid("4539 3195 0343 6467"));
    println!("code invalid {}", is_valid("8273 1232 7352 0569"));
}
#[test]
fn range_test() {
    let digits = ['1', '2', '3'];
    assert!(digits.contains(&'1'));
    assert!(!digits.contains(&'4'));
}

#[test]
fn enumerate_test() {
    let mut iter = "789".chars().enumerate();

    assert_eq!(iter.next(), Some((0, '7')));
    assert_eq!(iter.next(), Some((1, '8')));
    assert_eq!(iter.next(), Some((2, '9')));
    assert_eq!(iter.next(), None);
}

#[test]
fn chars_iterator_test() {
    let mut chars = "123".chars();

    assert_eq!(chars.next(), Some('1'));
    assert_eq!(chars.next(), Some('2'));
    assert_eq!(chars.next(), Some('3'));
    assert_eq!(chars.last(), None);
}

#[test]
fn double_with_overflow_test() {
    assert_eq!(double_with_overflow(0), 0);
    assert_eq!(double_with_overflow(1), 2);
    assert_eq!(double_with_overflow(4), 8);
    assert_eq!(double_with_overflow(5), 1);
    assert_eq!(double_with_overflow(6), 3);
    assert_eq!(double_with_overflow(9), 9);
}

#[test]
fn single_digit_strings_can_not_be_valid() {
    assert!(!is_valid("1"));
}

#[test]
//#[ignore]
fn a_single_zero_is_invalid() {
    assert!(!is_valid("0"));
}

#[test]
//#[ignore]
fn a_simple_valid_sin_that_remains_valid_if_reversed() {
    assert!(is_valid("059"));
}

#[test]
//#[ignore]
fn a_simple_valid_sin_that_becomes_invalid_if_reversed() {
    assert!(is_valid("59"));
}

#[test]
//#[ignore]
fn a_valid_canadian_sin() {
    assert!(is_valid("055 444 285"));
}
