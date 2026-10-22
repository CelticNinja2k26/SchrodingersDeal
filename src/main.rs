// Schrodinger's Deal
use std::io::{self, Write};
fn main() {
    //Intro, number of spies,take use Info (name and case number)
    let mut name:String = String::new();
    println!("Schrodinger's Deal");
    print!("Hello, What is your name? ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(&mut name).expect("Could not read line");
    let name = name.trim().to_string();
    println!("Welcome, {}", name);

}
