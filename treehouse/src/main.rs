use std::io::stdin;

fn name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let visitors = ["bert", "steve", "fred"];

    println!("Hello, what is your name?");

    let name = name();
    let mut allow_in = false;

    for visitor in &visitors {
        if visitor == &name {
            allow_in = true;
        }
    }

    if allow_in {
        println!("Welcome, {}", name);
    } else {
        println!("Sorry, you are not on the list.")
    }
}
