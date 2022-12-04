use crate::select_part;

pub(crate)

fn main() {
    let part = select_part::select_part();

    let input = include_str!("./inputs/1");
    
    let mut elves: Vec<i32> = Vec::new();

    let mut current_elf = 0;   

    // Creates a subtotal of the number of calories for each elf, then adds it to elves
    for line in input.lines() {
        let calories = line.parse::<i32>();
        match calories {
            Ok(calories) => {
                current_elf += calories;
            }
            Err(_) => {
                // If the line is empty, the current elf has no more items, so add their toal to elves
                elves.push(current_elf);
                current_elf = 0;
            }
        }
    }

    if part == 1 {
        // Find the elf with the highest calories and returns it
        let mut max = 0;
        for elf in elves {
            if elf.max(max) == elf {
                max = elf;
            }
        }
        println!("Max: {}", max);
    } else {
        let mut top_3 = [0; 3];
        
        // Compares each elf to top 3 and replaces the first one it finds that's smaller
        for elf in elves {
            for i in 0..top_3.len() {
                if elf > top_3[i] {
                    top_3[i] = elf;
                    break;
                }
            }
        }

        let sum: i32 = top_3.iter().sum();

        println!("Sum of top 3: {:?}", sum);
    }
}
