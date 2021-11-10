use std::io::{self, Write};

fn main() {
    let height: u8;

    loop {
        print!("Height: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Faileed to read line");

        match input.trim().parse::<u8>() {
            Ok(num) if num > 0 && num < 9 => height = num,
            _ => continue,
        }

        break;
    }

    build_pyramid(height as usize);
}

fn build_pyramid(height: usize) {
    for i in 1..height + 1 {
        let mut level = vec![' '; 2 + 2 * i + height - i];
        for j in 0..i {
            level[height - 1 - j] = '#';
            level[height + 2 + j] = '#';
        }
        println!("{}", level.iter().collect::<String>());
    }
}
