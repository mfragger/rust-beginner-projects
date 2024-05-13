use std::io;

pub fn line(num : i32) -> String {
    let mut output = String::with_capacity(num as usize);

    for _ in 0..num {
        output.push('*');
    }

    output
}

pub fn execute() {
    println!("Please enter a  number");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let count : i32 = input.trim().parse().expect("Please type a number");

    for row in 1..=count {
        if row % 2 != 0 {
            println!("{}", line(row));
        }
    }
}