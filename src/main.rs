use chrono::prelude::*;
use std::env;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
    let challenges = [
        day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14,
    ];
    let mut day_index = 0usize;

    // run a specific challenge from the cli arguments, 1 -> 30
    if env::args().len() >= 2 {
        let day = env::args().skip(1).next().unwrap();

        match day.parse::<usize>() {
            Ok(index) => day_index = index - 1,
            Err(_) => panic!("'{}' is not a valid day number", day),
        }
    }

    // if no argument provided, run today's
    day_index = if day_index > 0 {
        day_index
    } else {
        let start_date = Local.with_ymd_and_hms(2022, 12, 1, 0, 0, 0).unwrap();
        let today = Local::now();
        (today.day() - start_date.day()) as usize
    };

    if day_index > challenges.len() - 1 {
        println!(
            "Day {} hasn't started yet, running previous challenges\n",
            day_index + 1
        );
        challenges
            .into_iter()
            .enumerate()
            .for_each(|(i, challenge)| {
                println!("Day {}:", i + 1);
                challenge();
            })
    } else {
        let today_challenge = challenges[day_index as usize];
        today_challenge();
    }
}

fn day1() {
    println!(
        "Most calories carried by a single elf: {}",
        day_1::find_most_calories().unwrap()
    );

    println!(
        "Total calories carried by the top 3 elf: {}",
        day_1::find_total_calories_of_top_3().unwrap()
    );
}

fn day2() {
    println!(
        "Total score if I follow the assumed strategy: {}",
        day_2::get_total_score().unwrap()
    );

    println!(
        "Total score if I follow the right strategy: {}",
        day_2::get_total_score_of_the_right_strategy().unwrap()
    );
}

fn day3() {
    println!(
        "Sum of the priorities of item that appears twice: {}",
        day_3::priorities_sum_of_same_items()
    );

    println!(
        "Sum of the priorities of item that appears twice: {}",
        day_3::priorities_sum_of_group_item()
    );
}

fn day4() {
    println!(
        "Total of number of pairs where one fully contains the other: {}",
        day_4::number_of_containing_pairs()
    );

    println!(
        "Total of number of pairs where one overlaps with the other: {}",
        day_4::number_of_overlapping_pairs()
    );
}

fn day5() {
    println!(
        "Crate that end up sitting on top of each stack using CreateMover 9000: {}",
        day_5::crates_end_up_on_top_with_9000()
    );
    println!(
        "Crate that end up sitting on top of each stack using CreateMover 9001: {}",
        day_5::crates_end_up_on_top_with_9001()
    );
}

fn day6() {
    println!(
        "The first maker appears after {} signals",
        day_6::first_marker_appear_after(4)
    );

    println!(
        "The first message appears after {} signals",
        day_6::first_marker_appear_after(14)
    );
}

fn day7() {
    println!(
        "Total size of directors with size below 100000: {}",
        day_7::total_size_of_directory_under_threshold(100000)
    );
    println!(
        "The smallest directory that needs to be deleted to reserve enough space for update: {}",
        day_7::smallest_dir_to_be_deleted(70000000, 30000000)
    );
}

fn day8() {
    println!(
        "Total number of visible tree: {}",
        day_8::total_visible_trees("src/day_8/input.txt").unwrap()
    );

    println!(
        "The highest scenic score for a tree house: {}",
        day_8::hightest_scenic_score("src/day_8/input.txt").unwrap()
    );
}

fn day9() {
    println!(
        "Total number of positions that the tail visited: {}",
        day_9::unique_visited_positions("src/day_9/input.txt").unwrap()
    );
}

fn day10() {
    println!(
        "Total signal strength: {}",
        day_10::total_signal_strength("src/day_10/input.txt").unwrap()
    );

    println!(
        "Rendered image:\n{}",
        day_10::render_image("src/day_10/input.txt", Some(1)).unwrap()
    );
}

fn day11() {
    println!(
        "Monkey business score: {}",
        day_11::monkey_business("src/day_11/input.txt").unwrap()
    );
}

fn day12() {
    println!("{}", day_12::fewest_steps("src/day_12/input.txt").unwrap());
}

fn day13() {
    println!(
        "Sum of indices of pairs of packets that are in the right order: {}",
        day_13::sum_of_pair_indices("src/day_13/input.txt").unwrap()
    );
}

fn day14() {
    println!(
        "Day 14: {}",
        day_14::the_endless_void("src/day_14/input.txt").unwrap()
    );
}
