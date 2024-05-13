use std::io;
use rand::Rng;

pub fn execute() {
    let mut rng = rand::thread_rng();
    let true_number: u32 = rng.gen_range(1..=100);

    loop {
        println!("Please enter a number between 1 to 100");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let number : u32 = input.trim().parse().expect("Please enter a number");
        
        if number > 100 {
            println!("Please enter a number between 1 to 100");
        }
        
        else if number < true_number {
            println!("Number is below RNG Number");
        }
        
        else if number > true_number {
            println!("Number is above RNG Number");
        }

        else if number == true_number {
            println!("You got it!");
            break;
        }
    }

}