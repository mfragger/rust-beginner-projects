use std::io;

pub fn execute() {
    println!("Please enter a number");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let count : i32 = input.trim().parse().expect("Please type a number");

    let mut entry_count = count;

    for _row in (0..count).rev() {
        for _entry in 0..entry_count {
            print!("*");
        }
        entry_count -= 1;
        print!("\n");
    }
}