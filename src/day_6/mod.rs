use std::fs::read_to_string;

/**
 * The first thought is to use a sliding window and determine if it contains all unique characters
 * Keeping track of two pointers, i and j for the start and the end of the window respectively
 * Increment j by one, if the chars[j] exists set i to where it previously appears + 1
 * Keep sliding j until the distance between i and j is equal to the required distinct signals, and j + 1 is where the first packet starts
 */
pub fn first_maker_appear_after(required_distinct_signal: usize) -> usize {
    let input = read_to_string("src/day_6/input.txt").unwrap();

    let chars = input.chars().collect::<Vec<char>>();

    let mut i = 0;
    let mut j = i;

    // the current sliding window
    let mut window = Vec::with_capacity(required_distinct_signal - 1);
    for _ in 0..required_distinct_signal - 1 {
        window.push(' ');
    }

    while i < chars.len() && j < chars.len() {
        let mut dup_found = false;
        for (x, c) in window.clone().iter().enumerate() {
            if *c == chars[j] {
                dup_found = true;

                // Duplicate found, update the window to cut off where the duplicate is found
                for y in 0..window.len() {
                    if y + x + 1 >= window.len() {
                        window[y] = ' ';
                    } else {
                        window[y] = window[y + x + 1];
                    }
                }

                // Set the new window start
                i += x + 1;
                break;
            }
        }

        if dup_found == false {
            // j - i + i = window length
            if j - i == required_distinct_signal - 1 {
                return j + 1;
            }
            window[j - i] = chars[j];
            j += 1;
        }
    }

    panic!("Unable to find the market from the given signal input");
}
