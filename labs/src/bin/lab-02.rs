use labs::input;
use std::cmp::Ordering;

fn main() {
    scrabble();
}

fn scrabble() {
    let input0 = input::get_input("Player 1: ");
    let input1 = input::get_input("Player 2: ");

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
