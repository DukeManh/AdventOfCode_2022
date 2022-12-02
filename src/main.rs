use chrono::prelude::*;

mod day_1;
mod day_2;

fn main() {
    let challenges = [day1, day2];

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
