use std::io::stdin;

fn name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            // note that here we create new string and copy name into it
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn main() {
    let visitors = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi, Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    println!("Hello, what is your name?");

    let name = name();

    let visitor = visitors
        .iter()
        .find(|visitor| visitor.name == name);

    match visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("Sorry, you are not on the list. Please leave.")
    }
}
