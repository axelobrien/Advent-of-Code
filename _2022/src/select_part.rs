use std::io;

pub(crate)


// Gets user input for the part number
// Part numbers can be 1 or 2
fn select_part() -> u8 {
    println!("Which part would you like to solve? (1/2)");
    
    // Gets part
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Please input a number");

    let part: u8 = user_input.trim().parse().unwrap();

    // Loop until the user enters a valid number
    if !(part == 1 || part == 2) {
        println!("Invalid input");
        select_part();
    }

    part
}