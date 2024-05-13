use std::io;

pub fn countvowels(input : &String) -> (i32, i32) {
    let mut articulation_count = (0, 0);
    for c in input.chars(){
        match c {
            c if!c.is_alphanumeric() => break,
            'A' | 'E' | 'I' | 'O' | 'U' => articulation_count.0 += 1,
            _ => articulation_count.1 += 1,
        }
    }
    articulation_count
}

pub fn execute() {
    println!("Please enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.to_uppercase();
    let count = countvowels(&input);
    print!("vowel: {}, consonant: {}", count.0, count.1);
}