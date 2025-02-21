use std::io;

fn main() {
    
    println!("Enter an integer:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().expect("Please enter a valid number");

    if num % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}

