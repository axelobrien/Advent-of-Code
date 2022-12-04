use std::io;

mod day_1;
mod select_part;
mod day_2;

fn main() {
    let days = vec![day_1::main as fn(), day_2::main as fn()];
    println!("Welcome to the answers to Advent of Code 2022 by Sirbananathe6th!");
    loop {
        println!("Which day do you want to run? (1-{})", days.len());
        let mut user_input = String::new();

        // Get user input
        io::stdin()
            .read_line(&mut user_input)
            .expect("Please input a number");

        // Turn user input into a number
        let user_input: usize = user_input.trim().parse().unwrap();

        // Run selected function, -1 is so user doesn't have to 0-index in their head
        let selected_day = days[user_input - 1];
        selected_day();
    }
}
