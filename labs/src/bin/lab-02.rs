use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    scrabble();
}

fn scrabble() {
    print!("Player 1: ");
    io::stdout().flush().unwrap();
    let mut input0 = String::new();

    io::stdin()
        .read_line(&mut input0)
        .expect("Failed to read line");

    print!("Player 2: ");
    io::stdout().flush().unwrap();
    let mut input1 = String::new();

    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    match calculate_score(input0).cmp(&calculate_score(input1)) {
        Ordering::Equal => println!("Tie!"),
        Ordering::Greater => println!("Player 1 wins!"),
        Ordering::Less => println!("Player 2 wins!"),
    }
}

fn calculate_score(input: String) -> u32 {
    input
        .to_ascii_lowercase()
        .chars()
        .map(|c| match c {
            'a' | 'e' | 'i' | 'l' | 'n' | 'o' | 'r' | 's' | 't' | 'u' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0,
        })
        .sum()
}
