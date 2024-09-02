use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn get_visitors_list() -> [Visitor; 3] {
    return [
        Visitor::new("sam", "Hi Sam, welcom back"),
        Visitor::new("dan", "Hey hey Dam, long time not see you"),
        Visitor::new("fran", "Hello sir, welcome."),
    ];
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();

    //let visitors_list = ["sam", "dan", "fran"];
    let visitors_list = get_visitors_list();
    let known_visitor = visitors_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("Sorry, you are not in the list, {}", name),
    }
}
