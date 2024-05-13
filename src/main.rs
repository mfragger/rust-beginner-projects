use std::io;

use rust_beginner_projects::*;

pub mod trianglefromlargestosmallest;
pub mod trianglefromsmallestolargest;
pub mod trianglefromsmallesttolargestskippineven;

fn factorial() {
    println!("Please enter a number:");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");    
    let number : i32 = input.trim().parse().expect("Please type a number");
    println!("Number entered is: {}", number);

    let mut factorials : Vec<i32> = Vec::with_capacity(5);
    let mut product : i32 = number;

    factorials.push(number);
    for i in (1..number).rev() {
        factorials.push(i);
        product *= i;
    }

    println!("Product is: {}", product);
    for factorial in factorials.iter() {
        println!("Factorial {}", factorial);
    }
}

fn currency_converter() {
    const USRATETOUK : f32 = 58.62;
    const USRATETOYEN : f32 = 30.00;
    
    const UKRATETOUS : f32 = 70.23;
    const UKRATETOYEN : f32 = 23.00;

    const YENRATETOUS : f32 = 0.07; 
    const YENRATETOUK : f32 = 0.04;

    println!("Please pick your current currency: \n (1): US \n (2): UK \n (3): YEN");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_choice : i8 = input.trim().parse().expect("Please type a number");

    println!("Please state your amount:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_choice_amount : f32 = input.trim().parse().expect("Please type a number");

    println!("{}{}", "Please pick a curreny to convert to:", 
        match first_choice {
            1 => "\n (2): UK \n (3): YEN",
            2 => "\n (1): US \n (3): YEN",
            3 => "\n (1): US \n (2): UK",
            _ => "",
    });

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let second_choice : i8 = input.trim().parse().expect("Please type a number");

    match first_choice {
        1 => {
            match second_choice {
                2 => {
                    println!("Converted rate from US to UK is: {}", first_choice_amount * USRATETOUK);
                }
                3 => {
                    println!("Converted rate from US to YEN is: {}", first_choice_amount * USRATETOYEN);
                }
                _ => println!("No more"),
            }
        },
        2 => {
            match second_choice {
                1 => {
                    println!("Converted rate from UK to US is: {}", first_choice_amount * UKRATETOUS);
                }
                3 => {
                    println!("Converted rate from UK to YEN is: {}", first_choice_amount * UKRATETOYEN);
                }
                _ => println!("No more"),
            }
        },
        3 => {
            match second_choice {
                1 => {
                    println!("Converted rate from YEN to US is: {}", first_choice_amount * YENRATETOUS);
                }
                2 => {
                    println!("Converted rate from YEN to UK is: {}", first_choice_amount * YENRATETOUK);
                }
                _ => println!("No more"),
            }
        },
        _ => println!("No more"),
    }

}

fn main() {
    println!("Please pick a thing: \n 
            (1): Factorial \n 
            (2): Currency Converter \n
            (3): Triangle from largest to smallest \n
            (4): Triangle from smallest to largest \n
            (5): Trinagle from smallest to largest skipping even rows \n
            (6): Randomness \n
            (7): Guessing game \n
            (8): Fibonacci \n
            (9): Count Vowels and consonants \n
            (10): Fifth root \n
            (11): Combinations \n
            (12): Circle \n
            (13): Piggy Bank \n
            (14): Palindrome \n
            (15): Grades"
    );
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number : i16 = input.trim().parse().expect("Please type a number");

    match number {
        1 => factorial(),
        2 => currency_converter(),
        3 => trianglefromlargestosmallest::execute(),
        4 => trianglefromsmallestolargest::execute(),
        5 => trianglefromsmallesttolargestskippineven::execute(),
        6 => randomfunctions::execute(),
        7 => guessinggame::execute(),
        8 => fibonacci::execute(),
        9 => countvowels::execute(),
        10 => fifthroot::execute(),
        11 => combinations::execute(),
        12 => circle::execute(),
        13 => piggybank::execute(),
        14 => palindrome::execute(),
        15 => grades::execute(),
        _ => println!("No more"),
    }
}
