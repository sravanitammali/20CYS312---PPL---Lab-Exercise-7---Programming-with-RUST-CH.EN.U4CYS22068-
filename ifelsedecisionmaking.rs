use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().expect("Please enter a valid number");

    if num > 0 {
        println!("The number is positive.");
    } else if num < 0 {
        println!("The number is negative.");
    } else {
        println!("The number is zero.");
    }
}

