use std::fs::read_to_string;

pub fn get_total_score() -> Result<i32, std::io::Error> {
    let input = read_to_string("src/day_2/input.txt")?;
    // split input into lines
    // split each line by ' ' to get opponent's move and your move
    // convert str to char
    // determine the point you get for choosing a shape
    // determine if you win, lose or draw and the point you get
    // determine the total point
    // add total point of each round together and return the result

    let x_2_a = 'X' as i8 - 'A' as i8;

    let score: i32 = input.lines().fold(0, |acc, round| {
        let mut line = round.split(' ').into_iter();
        let round_score = if let (Ok(their), Ok(our)) = (
            line.next().unwrap().parse::<char>(),
            line.next().unwrap().parse::<char>(),
        ) {
            let shape_score = match our {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                c => panic!("Unrecognizable char {}", c),
            };

            let score = match (our as i8 - x_2_a) - their as i8 {
                1 | -2 => 6,
                -1 | 2 => 0,
                0 | _ => 3,
            };

            shape_score + score
        } else {
            0
        };
        round_score + acc
    });

    Ok(score)
}

pub fn get_total_score_of_the_right_strategy() -> Result<i32, std::io::Error> {
    // same as the previous strategy
    // however, this time we need to find out which shape to choose for the desired result

    let input = read_to_string("src/day_2/input.txt")?;

    let score: i32 = input.lines().fold(0, |acc, round| {
        let mut line = round.split(' ').into_iter();
        let round_score = if let (Ok(their), Ok(result)) = (
            line.next().unwrap().parse::<char>(),
            line.next().unwrap().parse::<char>(),
        ) {
            let score = match result {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => 0,
            };

            // if we want to win, select the next shape in the circle chain stating from A: A -> B -> C
            // select the previous shape otherwise
            let shape = match result {
                'X' => get_shape(their as u8 - 1),
                'Y' => their,
                'Z' => get_shape(their as u8 + 1),
                c => panic!("Unrecognized char, {}", c),
            };

            let shape_score = match shape {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                c => panic!("Unrecognized char, {}", c),
            };

            shape_score + score
        } else {
            0
        };
        round_score + acc
    });

    Ok(score)
}

fn get_shape(shape_num: u8) -> char {
    // % perform the remainder operation not modulus
    // use rem_euclid() instead
    (('A' as u8) + ((shape_num as i8 - ('A' as i8)).rem_euclid(3)) as u8) as char
}
