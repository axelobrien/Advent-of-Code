use std::io;

mod select_part;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn get_input(input: &mut String) -> usize {
    let io_in = io::stdin()
    .read_line(input);
    // if io_in is 2, then input was blank
    if io_in.as_ref().unwrap_or(&2) == &2 {
        println!("Please enter a number");
        get_input(input);
    }

    io_in.unwrap_or(1 as usize)
}

fn main() {
    let days = [day_1::main as fn(), day_2::main as fn(), day_3::main as fn(), day_4::main as fn()];
    println!("Welcome Sirbananathe6th's answers to Advent of Code 2022!");
    loop {
        println!("Which day do you want to run? (1-{})", days.len());
        let mut user_input = String::new();

        get_input(&mut user_input);

        // Turn user input into a number
        let user_input: usize = user_input.trim().parse().unwrap_or(1);
        
        // Run selected function, -1 is so user doesn't have to 0-index in their head
        let selected_day = days[user_input - 1];
        selected_day();
    }
}
