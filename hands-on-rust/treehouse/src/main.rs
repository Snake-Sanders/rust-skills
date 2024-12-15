use std::io::stdin;

// derivign Debug allows to print the whole structure with {:?}
#[derive(Debug)]
struct Visitor {
    action: VisitorAction,
    age: i8,
    name: String,
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            age,
            action,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree hose, {}", self.name),
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the tree hose, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alacohol to {}", self.name);
                }
            }
        }
    }
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn get_visitors_list() -> Vec<Visitor> {
    return vec![
        Visitor::new("sam", VisitorAction::Accept, 45),
        Visitor::new(
            "dan",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            15,
        ),
        Visitor::new("fran", VisitorAction::Refuse, 30),
    ];
}

fn main() {
    let mut visitors_list = get_visitors_list();
    loop {
        print!("Hello, what's your name?");
        println!(" (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        if name.is_empty() {
            break;
        }

        let known_visitor = visitors_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                println!("{} is not in the visitor list", name);
                visitors_list.push(Visitor::new(&name, VisitorAction::Probation, 18));
            }
        }
    }

    println!("The final visitors list:");
    println!("{:#?}", visitors_list);
}
