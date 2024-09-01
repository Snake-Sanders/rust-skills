fn main() {
    let mut acc: u8 = 0;

    for i in 1..=10 {
        acc = acc + i;
    }

    println!("sum is {}", acc);
}
