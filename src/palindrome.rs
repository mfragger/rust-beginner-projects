use std::io;

pub fn execute() {
    println!("Please enter a text:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to readline");
    let real_input = input.to_uppercase().trim().chars().filter(|c| c.is_alphabetic()).collect::<String>();

    let rev_input = real_input.chars().rev().collect::<String>();

    if real_input == rev_input {
        println!("Palindrome");
    }
    else {
        println!("Not a palindrome");
    }
}