use failure::Error;

use std::fs::File;
use std::io::{self, Lines, Write};
use std::io::{BufRead, BufReader};
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

pub fn load_data(file: &str) -> Result<Lines<BufReader<File>>, Error> {
    let path = Path::new(file);
    let candidates_list = File::open(path)?;
    let reader = BufReader::new(candidates_list);
    Ok(reader.lines())
}
