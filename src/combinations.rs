extern crate itertools;

use itertools::Itertools;

pub fn execute() {
    println!("Please enter a word");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_owned();
    if input.chars().count() != 4 {
        println!("Please enter only a 4 letter world");
        return;
    }

    combinations(&input);
}

pub fn combinations(input: &String) {
    let chars: Vec<char> = input.chars().collect();
    let combinations : Vec<_> = chars.into_iter().permutations(4).collect();

    println!("All combinations:");
    for combination in &combinations {
        let word : String = combination.into_iter().collect();
        println!("{:?}", word);
    }
}