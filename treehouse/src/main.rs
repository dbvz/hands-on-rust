use std::io::stdin;

fn name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNot { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            // note that here we create new string and copy name into it
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house, {}", self.name),
            VisitorAction::AcceptWithNot { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Refuse => println!("Do no allow {} in!", self.name),
            VisitorAction::Probation => println!("{} is now a probation member", self.name),
        }
    }
}

fn main() {
    let mut visitors = vec![
        Visitor::new("bert", VisitorAction::Accept, 45),
        Visitor::new(
            "steve",
            VisitorAction::AcceptWithNot {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            15,
        ),
        Visitor::new("fred", VisitorAction::Refuse, 30),
    ];

    loop {
        println!("Hello, what is your name?");

        let name = name();

        let visitor = visitors.iter().find(|visitor| visitor.name == name);

        match visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitors.push(Visitor::new(&name, VisitorAction::Probation, 0));
                    // println!("Sorry, you are not on the list. Please leave.")
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitors)
}
