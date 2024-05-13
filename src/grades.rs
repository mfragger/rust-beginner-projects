use std::io;

pub fn execute() {
    println!("Please enter midterm grade:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let midterm_grade: f32 = input.trim().parse().expect("Please enter a number:");

    println!("Please enter final grade:");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let final_grade : f32 = input.trim().parse().expect("Please enter a number:");

    println!("Please enter homework grade:");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let homework_grade : f32 = input.trim().parse().expect("Please enter a number:");

    println!("Mid term grade: {}, Final term grade: {}, Homework grade: {}", midterm_grade * 0.20, final_grade * 0.40, homework_grade * 0.40);
}