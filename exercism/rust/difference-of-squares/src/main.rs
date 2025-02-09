pub fn square_of_sum(n: u32) -> u32 {
    let mut sum: u32 = 0;

    for x in 1..=n {
        sum += x;
    }

    println!("square of sum result = {}", sum * sum);
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sqr: u32 = 0;

    for x in 1..=n {
        sqr += x * x;
    }

    sqr
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

fn main() {
    let mut res;
    let mut val;

    val = 1;
    res = sum_of_squares(val);
    println!("sum of squares. val: {} res: {}", val, res);

    val = 5;
    res = square_of_sum(val);
    println!("square of sum. val: {} res: {}", val, res);

    val = 6;
    res = difference(val);
    println!("diff. val: {} res: {}", val, res);
}
