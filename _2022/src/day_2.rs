use crate::select_part::select_part;

pub(crate)

fn main() {
    let part = select_part();
    let input = include_str!("./inputs/2");
    let mut score = 0;

    for line in input.lines() {
        let opponent_shape_char = line.chars().nth(0).unwrap();
        let player_shape_char = line.chars().nth(2).unwrap();

        let opponent_shape = match opponent_shape_char {
            'A' => 1, // Rock
            'B' => 2, // Paper
            'C' => 3, // Scissors
            _ => 1, // Default to Rock
        };
        
        let player_shape: i32;
        
        if part == 1 {
            player_shape = match player_shape_char {
                'X' => 1, // Rock
                'Y' => 2, // Paper
                'Z' => 3, // Scissors
                _ => 0, // Default to Rock
            };
            score += player_shape;
        } else {
            player_shape = match player_shape_char {
                'X' => {
                    let mut tmp_score = opponent_shape - 1;
                    if tmp_score < 1 {
                        tmp_score = 3;
                    }
                    tmp_score
                }, // Lose
                'Y' => {
                    opponent_shape
                }, // Tie
                'Z' => {
                    let mut tmp_score = (opponent_shape + 1) % 4;
                    if tmp_score == 0 {
                        tmp_score += 1;
                    }
                    tmp_score
                }, // Win
                _ => 0, // Default to Rock
            };
            score += player_shape;
        }
        
        if player_shape == opponent_shape {
            score += 3; // Tie
        } else if player_shape - 1 == opponent_shape % 3 {
            score += 6; // Win
        }
    }

    println!("Score: {}", score);
}