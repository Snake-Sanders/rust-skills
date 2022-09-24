// it is not allowed to mutate a variable's type
// let mut spaces = "   ";
// spaces = spaces.len(); <-- compile error

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");

    x = 6;
    println!("The value of x is {x}");

    // new x shadows old x
    let x = x + 1;

    {
        // this new x will be disposed at the end of this block
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
