#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name;
    if your_name == "bert" || your_name == "steve" {
        println!("Welcome.");
    }   else {
        println!("Sorry, you are not on the list.")
    }
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    println!("Hello, {}", name);
}