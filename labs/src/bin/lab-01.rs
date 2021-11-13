use labs::prompt;

enum Input {
    Start,
    End,
}

fn main() {
    let start_size: u32 = take_user_input(9, Input::Start);

    let min_end_size = start_size + (start_size / 3) - (start_size / 4);
    let end_size: u32 = take_user_input(min_end_size, Input::End);

    let years = calculate_years(start_size, end_size);
    println!("Years: {}", years);
}

fn take_user_input(min: u32, in_ty: Input) -> u32 {
    let input = if matches!(in_ty, Input::Start) {
        prompt::get_input("Start size: ")
    } else {
        prompt::get_input("End size: ")
    };

    match input.trim().parse::<u32>() {
        Ok(n) if n > (min - 1) => n,
        _ => {
            if matches!(in_ty, Input::Start) {
                println!("Please enter a positive number greater than or equal to 9");
            } else {
                println!("Please enter a number greater than or equal to {}", min);
            }
            take_user_input(min, in_ty)
        }
    }
}

fn calculate_years(current_size: u32, end_size: u32) -> u32 {
    (1..)
        .scan(current_size, |state, x| {
            if *state < end_size {
                *state += (*state / 3) - (*state / 4);
                Some(x)
            } else {
                None
            }
        })
        .last()
        .unwrap()
}

// 1. Given `n` llamas, `n / 3` will be born and `n / 4` will die every year.
// 2. The minimum starting population size must not be less than 9.
// 3. Validate the end size input according to the start size input.
// 4. Calculate the number of years it takes to reach the goal.
