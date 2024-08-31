// compile with: rustc file_name.rs
// execute as: ./file_name
fn main() {
    for i in 1..=10 {
        print!("{}", i)
    }

    // Adds a newline after the loop to avoid extra %
    // println!(""); 
}

// notes:
//
// the output is:
// 12345678910%
// the last % is added by the terminal becasue the print does not have a new
// line at the end

