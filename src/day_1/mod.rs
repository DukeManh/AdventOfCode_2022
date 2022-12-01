use std::fs;

pub fn find_most_calories() -> Result<i32, std::io::Error> {
    let input = fs::read_to_string("src/day_1/input.txt")?;

    let max: i32 = input.split("\n\n").fold(0i32, |max, each_elf| {
        let total_each = each_elf
            .lines()
            .fold(0, |total, num| match num.parse::<i32>() {
                Ok(calories) => total + calories,
                Err(_) => total,
            });

        if max > total_each {
            max
        } else {
            total_each
        }
    });

    Ok(max)
}

pub fn find_total_calories_of_top_3() -> Result<i32, std::io::Error> {
    let input = fs::read_to_string("src/day_1/input.txt")?;
    let mut elves = Vec::<i32>::new();

    input.split("\n\n").for_each(|elf| {
        elves.push(elf.lines().fold(0, |acc, each| match each.parse::<i32>() {
            Ok(num) => acc + num,
            Err(_) => acc,
        }));
    });

    elves.sort();
    elves.reverse();

    let total = elves.as_slice()[0..3]
        .into_iter()
        .fold(0, |acc, num| acc + num);

    Ok(total)
}
