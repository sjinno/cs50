use std::io::{self, Write};

enum Input {
    Start,
    End,
}

fn main() {
    let mut start_size: u32 = take_user_input(9, Input::Start);

    let min_end_size = start_size + (start_size / 3) - (start_size / 4);
    let end_size: u32 = take_user_input(min_end_size, Input::End);

    let years = calculate_years(&mut start_size, end_size);
    println!("Years: {}", years);
}

fn take_user_input(min: u32, in_ty: Input) -> u32 {
    match in_ty {
        Input::Start => {
            print!("Start size: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<u32>() {
                Ok(n) if n > (min - 1) => return n,
                _ => {
                    println!("Please enter a positive number greater than or equal to 9");
                    return take_user_input(min, in_ty);
                }
            };
        }
        Input::End => {
            print!("End size: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<u32>() {
                Ok(n) if n > (min - 1) => return n,
                _ => {
                    println!("Please enter a number greater than or equal to {}", min);
                    return take_user_input(min, in_ty);
                }
            };
        }
    }
}

fn calculate_years(current_size: &mut u32, end_size: u32) -> u32 {
    let mut counter = 0;
    while *current_size < end_size {
        *current_size += (*current_size / 3) - (*current_size / 4);
        counter += 1;
    }
    counter
}

// 1. Given `n` llamas, `n / 3` will be born and `n / 4` will die every year.
// 2. The minimum starting population size must not be less than 9.
// 3. Validate the end size input according to the start size input.
// 4. Calculate the number of years it takes to reach the goal.
