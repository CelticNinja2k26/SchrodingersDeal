// Schrodinger's Deal
use std::io::{self, Write};

fn pop_cases_nums(mut cases: Vec<i32>) -> Vec<i32> {
    for i in 1..27 {
        cases.push(i);
    }
    cases
}
fn main() {
    //Intro, number of spies,take use Info (name and case number)
    let mut name:String = String::new();
    println!("Schrodinger's Deal");
    print!("Hello, What is your name? ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(&mut name).expect("Could not read line");
    let name = name.trim().to_string();
    println!("Welcome, {}", name);
    let mut bag_o_values: Vec<f64> = vec![0.01, 1.0, 5.0, 10.0, 25.0, 50.0, 75.0, 100.0, 200.0, 300.0, 400.0, 500.0, 750.0, 1000.0, 5000.0, 10000.0,25000.0,50000.0, 75000.0, 100000.0, 200000.0, 300000.0, 400000.0, 500000.0, 750000.0, 1000000.0];
    let mut cases: Vec<i32> = Vec::new();
    let mut case_nums = pop_cases_nums(cases);
    println!("{:?}", case_nums);
    }