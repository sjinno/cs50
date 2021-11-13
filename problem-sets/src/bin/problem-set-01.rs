use problem_sets::prompt;
use std::fmt::{self, Display};

fn main() {
    // mario();
    println!("{}", luhn());
}

#[allow(dead_code)]
fn mario() {
    let height: u8;

    loop {
        let input = prompt::get_input("Height: ");

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

#[allow(dead_code)]
enum CreditCard {
    Amex,
    Master,
    Visa,
    Invalid,
}

impl Display for CreditCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CreditCard::Amex => f.write_str("AMEX"),
            CreditCard::Master => f.write_str("MASTER"),
            CreditCard::Visa => f.write_str("VISA"),
            CreditCard::Invalid => f.write_str("INVALID"),
        }
    }
}

#[allow(dead_code)]
fn luhn() -> CreditCard {
    let mut number: Vec<u32>;
    let mut card_type: CreditCard;

    loop {
        let input = prompt::get_input("Number: ");

        number = input
            .trim()
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        card_type = if number.len() > 1 {
            detect_card_type(&number[0], &number[1])
        } else {
            continue;
        };

        match (&card_type, number.len()) {
            (&CreditCard::Amex, 15) => break,
            (&CreditCard::Master, 16) => break,
            (&CreditCard::Visa, 13 | 16 | 19) => break,
            _ => continue,
        };
    }

    if validate(number) {
        match card_type {
            CreditCard::Amex | CreditCard::Master | CreditCard::Visa => card_type,
            _ => CreditCard::Invalid,
        }
    } else {
        CreditCard::Invalid
    }
}

fn detect_card_type(first: &u32, second: &u32) -> CreditCard {
    match *first {
        4 => CreditCard::Visa,
        5 => match *second {
            (1..=5) => CreditCard::Master,
            _ => CreditCard::Invalid,
        },
        3 => match *second {
            4 | 7 => CreditCard::Amex,
            _ => CreditCard::Invalid,
        },
        _ => CreditCard::Invalid,
    }
}

fn validate(number: Vec<u32>) -> bool {
    let is_odd = number.len() % 2 == 1;
    let rem = if is_odd { 1 } else { 0 };

    number.iter().enumerate().fold(0, |acc, (idx, num)| {
        if idx % 2 == rem {
            let mul = 2 * num;
            if mul > 9 {
                acc + 1 + mul % 10
            } else {
                acc + mul
            }
        } else {
            acc + num
        }
    }) % 10
        == 0
}
