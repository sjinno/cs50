use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

type Key = HashMap<char, char>;

fn main() {
    let len = env::args().skip(1).count();
    if len != 1 {
        println!("Expected only 1 argument, but given {} arguments.", len);
        println!("Usage: cargo run <key>");
        std::process::exit(1);
    }

    if let Some(key) = env::args().skip(1).next() {
        match key.len() {
            26 => {
                println!("{}", key);
                let key = ('A'..='Z')
                    .zip(key.to_ascii_uppercase().chars())
                    .collect::<Key>();

                let mut plaintext = get_input();
                plaintext.pop(); // Pops the tailing new line
                let ciphertext = cipher(key, plaintext);
                println!("ciphertext: {}", ciphertext);
            }
            _ => {
                println!("Key must contain 26 characters.");
                std::process::exit(1);
            }
        }
    };
}

fn get_input() -> String {
    print!("plaintext:  ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn is_cap(c: char, c_cap: char) -> bool {
    c == c_cap
}

fn cipher(key: Key, text: String) -> String {
    text.chars()
        .map(|c| {
            let c_cap = c.to_ascii_uppercase();
            match c_cap {
                ('A'..='Z') => {
                    let val = key.get(&c_cap);
                    if is_cap(c, c_cap) {
                        *val.unwrap()
                    } else {
                        val.unwrap().to_ascii_lowercase()
                    }
                }
                _ => c,
            }
        })
        .collect()
}
