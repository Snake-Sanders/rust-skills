use std::io::stdin;

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();

    let visitors_list = ["sam", "dan", "fran"];
    let mut allow_them_in = false;
    for visitor in &visitors_list {
        if name == *visitor {
            allow_them_in = true;
        }
    }
    if allow_them_in {
        println!("Welcome {}", name);
    } else {
        println!("Sorry, you are not in the list, {}", name);
    }
}
