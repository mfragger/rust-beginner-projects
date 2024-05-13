use std::io;

fn main() {
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
