// chapter ownership:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    // if we want to pass `greeting` to a function but continue to use it then  
    // we could return it back as parameter, but for that we can use the keyword
    // `reference`.
    let greeting = String::from("hello");
    let (s2, len) = calculate_length(greeting); // greeting is moved to func
    println!("The length of '{}' is {}.", s2, len);

    // passing variable using reference. 
    // The parameter `s` is a pointer to `saludo`
    // and saludo points to "hola", located in the heap. 
    // Important: `s` does NOT point to the value of `saludo` the heap!
    let saludo = String::from("hola");
    let saludo_len = calculate_length_with_reference(&saludo);
    println!("Spanish greeting: {}, lenght {}", saludo, saludo_len);

    // passing a mutable reference.
    let mut halsning = String::from("Hej");
    change_greeting(&mut halsning);
    println!("Swedish greeting: {}", halsning);

    // returning a string by value
    let gruessen = get_gruessen();
    println!("Austrian greeting: {}", gruessen);
    
    println!("x last call {}", x);
    //println!("s last call {}", s); <- s

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn calculate_length_with_reference(s: &String) -> usize {
    // s.push_str(", world"); cannot be changed, it is unmutable!
    s.len()
}

fn change_greeting(s: &mut String) {
    s.push_str(" Hej");
}

fn get_gruessen() -> String {
    let s = String::from("Habediehre");
    s 
    // returns the value, do not return &s, s is dropped after this function.
}
