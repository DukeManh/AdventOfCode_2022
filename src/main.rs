use chrono::prelude::*;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    let challenges = [day1, day2, day3, day4];

    let start_date = Utc.with_ymd_and_hms(2022, 12, 1, 0, 0, 0).unwrap();
    let today = Utc::now();

    let day_index = (today.day() - start_date.day()) as usize;
    if day_index > challenges.len() - 1 {
        panic!("Day {} hasn't started yet", day_index + 1)
    }

    let challenge = challenges[day_index as usize];
    challenge();
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
