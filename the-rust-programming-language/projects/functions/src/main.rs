// Statements do not return a value
// example: let y = 6
// Expressions return a value
// example: 6 + 3
// A code block is an expressions
// {
//     let x = 3;
//     x + 1
// }
// Expressions do not include ending semicolons. 
// If you add a semicolon to the end of an expression, 
// you turn it into a statement, and it will then not return a value.

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    print_labeled_measurement(5, 'h');
    let x = five();
    let y = plus_one(5);
    println!("the value of x is: {x}");
    println!("the value of y is: {y}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}.");
}
