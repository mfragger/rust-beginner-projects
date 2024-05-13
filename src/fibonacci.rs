use std::io;

pub fn fibonacci(number : u32) {
    let mut fib = (0, 1);
    
    println!("Fibonacci sequence up to {} terms:", number);

    if number >= 1 {
        println!("{}", fib.0);
    }
    if number >= 2 {
        println!("{}", fib.1);
    }

    for _ in 3..=number {
        let next_fib = fib.0 + fib.1;
        if next_fib > number {
            break;
        }
        println!("{}", next_fib); 
        fib = (fib.1, next_fib);
    }
}

pub fn execute() {
    println!("Pleas enter a number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number : u32 = input.trim().parse().expect("Please type a number");

    println!("{:?}", fibonacci(number));
}