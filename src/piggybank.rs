use std::io;

pub fn execute() {
    const TWO_EURO: f64 = 2f64;
    const ONE_EURO: f64 = 1f64;
    const FIFTY_PENNIES : f64 = 0.50f64;
    const TWENTY_PENNIES : f64 = 0.20f64;
    const TEN_PENNIES : f64 = 0.10f64;
    const FIVE_PENNIES : f64 = 0.05f64;

    println!("How many 2 euros:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to readline");
    let two_euro_count : f64 = input.trim().parse().expect("Please type a number");

    println!("How many 1 euro:");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to readline");
    let one_euro_count : f64 = input.trim().parse().expect("Please type a number");

    println!("How many 50 pennies:");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to readline");
    let fifty_pennies_count : f64 = input.trim().parse().expect("Please type a number");

    println!("How many 20 pennies:");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to readline");
    let twenty_pennies_count : f64 = input.trim().parse().expect("Please type a number");

    println!("How many 10 pennies:");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to readline");
    let ten_pennies_count : f64 = input.trim().parse().expect("Please type a number");

    println!("How many 5 pennies:");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to readline");
    let five_pennies_count : f64 = input.trim().parse().expect("Please type a number");

    println!("Total is:{}", (
        (two_euro_count * TWO_EURO) + 
        (one_euro_count * ONE_EURO) + 
        (fifty_pennies_count * FIFTY_PENNIES) + 
        (twenty_pennies_count * TWENTY_PENNIES) + 
        (ten_pennies_count * TEN_PENNIES)+ 
        (five_pennies_count * FIVE_PENNIES)
    ));
}