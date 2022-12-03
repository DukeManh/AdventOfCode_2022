use std::fs::read_to_string;

/**
 * Split each input line into 2 halves, each contains items for each compartment
 * Find out the only item that appear in both compartment:
 *  Loop through both compartment one by one
 *  Use an array of size 52 or a bit vector to record what characters have appeared in the first half
 *  Stop in anywhere in the second half if found a character that already appear
 *  add its priority to the total
 */
pub fn priorities_sum_of_same_items() -> i32 {
    let input = read_to_string("src/day_3/input.txt").unwrap();

    let sum = input.lines().fold(0, |mut acc, line| {
        let mut bit_vector = 0i64;

        line[0..line.len() / 2].chars().for_each(|c| {
            bit_vector |= 1 << char_to_index(c);
        });

        for c in line[line.len() / 2..].chars() {
            let char_index = char_to_index(c);
            if bit_vector & (1 << char_index) > 0 {
                acc += char_index as i32 + 1;
                break;
            }
        }
        acc
    });

    sum
}

pub fn priorities_sum_of_group_item() -> i32 {
    let input = read_to_string("src/day_3/input.txt").unwrap();

    let mut iter = input.lines();
    let mut sum = 0;
    while let Some(line) = iter.next() {
        let mut bit_vector_1 = 0i64;

        line.chars().for_each(|c| {
            bit_vector_1 |= 1 << char_to_index(c);
        });

        let mut bit_vector_2 = 0i64;
        if let Some(line) = iter.next() {
            line.chars().for_each(|c| {
                bit_vector_2 |= 1 << char_to_index(c);
            });
        }

        let bit_vector = bit_vector_1 & bit_vector_2;

        if let Some(line) = iter.next() {
            for c in line.chars() {
                if bit_vector & (1 << char_to_index(c)) > 0 {
                    sum += char_to_index(c) as i32 + 1;
                    break;
                }
            }
        }
    }

    sum
}

fn char_to_index(c: char) -> u8 {
    if c >= 'a' && c <= 'z' {
        c as u8 - 'a' as u8
    } else {
        c as u8 - ('A' as u8) + 26
    }
}
