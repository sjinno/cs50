use failure::Error;

use std::fs::File;
use std::io::BufReader;
use std::io::{self, Write};
use std::path::Path;

pub fn get_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

pub fn get_number_input(message: &str) -> usize {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(num) => num,
        _ => get_number_input(message),
    }
}

pub fn get_reader(file: &str) -> Result<BufReader<File>, Error> {
    let path = Path::new(file);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}
