#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let my_list = ["One", "Two", "Three"];
    for word in &my_list {
        println!("{}", word)
    }
}