extern crate rand;
use rand::Rng;

fn first_function() -> String {
    return "this is the first function".to_string();
}

fn second_function() -> String {
    "this is the second function".to_string()
}

fn third_function() -> String {
    "this is the third function".to_string()
}

pub fn execute() {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(1..=3);

    println!("{}", match number {
        1 => first_function(),
        2 => second_function(),
        3 => third_function(),
        _ => "".to_string(), 
    });
}