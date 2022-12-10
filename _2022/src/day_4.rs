use crate::select_part::select_part;

pub(crate)

fn main() {
    let part = select_part();
    let input = include_str!("./inputs/4");

    let mut superset_count = 0;

    for line in input.lines() {
        // convert "a-b,x-y" to [[a, b], [x, y]] 
        let group_sections: Vec<&str> = line.split(",").collect();
        let mut sections: Vec<Vec<i32>> = Vec::new();

        for i in 0..2 {
            let section_strings: Vec<&str> = group_sections[i].split("-").collect();
            let section: Vec<i32> = section_strings.iter().map(|x| {
                x.parse().unwrap()
            }).collect();
            sections.push(section);
        }

        if part == 1 {
            // Calculates if one of the sections is a superset of the other

            // In pseudocode
            // [x1, y1], [x2, y2]
            // if x1 <= x2 && y1 >= y2
            // else if x2 <= x1 && y2 >= y1

            // In English:
            // If array A's x is lower than B's x, and if A's y is higher than B's y

            if sections[0][0] <= sections[1][0] && sections[0][1] >= sections[1][1] {
                superset_count += 1;
            } else if sections[1][0] <= sections[0][0] && sections[1][1] >= sections[0][1] {
                superset_count += 1;
            }
        } else {
            // Calculates if the sections overlap at all

            // In pseudocode
            // [x1, y1], [x2, y2]
            // if y1 >= x2 && (y1 <= y2 || x1 <= y2)

            // In English:
            // If y1 is bigger than the lower bound of the 2nd array, and if either x1 || y1 is smaller than the upper bound of the 2nd array

            if sections[0][1] >= sections[1][0] && (sections[0][1] <= sections[1][1] || sections[0][0] <= sections[1][1]) {
                superset_count += 1;
            }
        }
    }
    println!("{}", superset_count)
}