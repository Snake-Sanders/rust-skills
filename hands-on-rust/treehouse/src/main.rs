use std::io::stdin;

// derivign Debug allows to print the whole structure with {:?}
#[derive(Debug)]
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

fn get_visitors_list() -> Vec<Visitor> {
    return vec![
        Visitor::new("sam", "Hi Sam, welcom back"),
        Visitor::new("dan", "Hey hey Dam, long time no see you"),
        Visitor::new("fran", "Hello sir, welcome."),
    ];
}

fn main() {
    let mut visitors_list = get_visitors_list();
    loop {
        print!("Hello, what's your name?");
        println!(" (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        if name.is_empty() {
            println!("Exit.");
            break;
        }

        let known_visitor = visitors_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                println!("{} is not in the visitor list", name);
                visitors_list.push(Visitor::new(&name, "Hello, New friend"));
            }
        }
    }
}
