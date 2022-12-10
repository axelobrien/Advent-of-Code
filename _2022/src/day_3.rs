use crate::select_part::select_part;

fn char_to_priority(c: char) -> i32 {
    let c_ascii = c as u8;
    if c_ascii < 91 {
        // ASCII 65 - 90 is A-Z
        // In the challenge, A-Z has a priority of 27 - 52
        (c_ascii - 38).into()
    } else {
        // ASCII 97 - 122 is a-z
        // In the challenge, a-z has a priority of 1 - 26
        (c_ascii - 96).into()
    }
}

pub(crate)

fn main() {
    let part = select_part();
    let input = include_str!("./inputs/3");

    let mut sum_of_priorities = 0;

    if part == 1 {
        for rucksack in input.lines() {
            // Each half of the rucksack is a different compartment
            let compartment_1 = rucksack
                .split_at(rucksack.len() / 2)
                .0;
            let compartment_2 = rucksack
                .split_at(rucksack.len() / 2)
                .1;

            for item in compartment_1.chars() {
                if compartment_2.contains(item) {
                    sum_of_priorities += char_to_priority(item);
                    break;
                }
            }
        }
    } else {
        let mut group = [""; 3];

        'outer: for line in input.lines() {
            // This for loop is to fill the group array before searching the group
            // 3 is group.len()
            for i in 0..3 { 
                if group[i] == "" {
                    group[i] = line;
                    // If i == 2, then the array has just been filled, no need to go back and restart
                    if i != 2 {
                        continue 'outer;
                    }
                }
            }
            
            // By here, group is filled
            for i in group[0].chars() {
                if group[1].contains(i) {
                    if group[2].contains(i) {
                        sum_of_priorities += char_to_priority(i);
                        break;
                    }
                }
            }
            // Reset group
            group = [""; 3];
        }
    }

    println!("Sum of the priorities: {}", sum_of_priorities);
}