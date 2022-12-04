use std::fs::read_to_string;

pub fn number_of_containing_pairs() -> i32 {
    // The goal is to find how many pairs where one fully contains the other
    // One fully contains the other when min1 <= min2 <= max2 <= max1
    // Split lines, then split each line by comma, then by dash to get min1, max1, min2, and max2
    // One contains the other when:
    // if min1 < min2: return max2 <= max1
    //      r1: ----------
    //      r2:   ------
    // if min1 = min2: return true because one range must cover the other
    //      r1: ----------
    //      r2: -------
    // if min1 > min2: return max1 <= max2
    //      r1:   ------
    //      r2: ----------

    let input = read_to_string("src/day_4/input.txt").unwrap();

    let total = input.lines().fold(0i32, |acc, line| {
        let (min1, max1, min2, max2) = read_line(line);

        let is_contained = match min1 - min2 {
            0 => true,
            n if n < 0 => max2 <= max1,
            _ => max1 <= max2,
        };

        match is_contained {
            true => acc + 1,
            false => acc,
        }
    });

    total
}

pub fn number_of_overlapping_pairs() -> i32 {
    // Similar to the previous solution, but different in how a pair is counted as overlapping
    // A pair is overlapping when either the min of max of one range is between the other range or min1 > min2 and max1 < max2
    //    r1: ----------
    //    r2:    ----------

    //    r1:    ----------
    //    r2: ----------

    //    r1:    ----------
    //    r2: ---------------

    let input = read_to_string("src/day_4/input.txt").unwrap();

    let total = input.lines().fold(0i32, |acc, line| {
        let (min1, max1, min2, max2) = read_line(line);

        let is_overlapping = (min1 <= min2 && min2 <= max1)
            || (min1 <= max2 && max2 <= max1)
            || (min1 > min2 && max1 < max2);

        match is_overlapping {
            true => acc + 1,
            false => acc,
        }
    });

    total
}

fn read_line(line: &str) -> (i32, i32, i32, i32) {
    let mut ranges = line.split(",");
    let (range1, range2) = (ranges.next().unwrap(), ranges.next().unwrap());

    let mut split = range1.split("-");
    let (min1, max1) = (
        split.next().unwrap().parse::<i32>().unwrap(),
        split.next().unwrap().parse::<i32>().unwrap(),
    );

    let mut split = range2.split("-");
    let (min2, max2) = (
        split.next().unwrap().parse::<i32>().unwrap(),
        split.next().unwrap().parse::<i32>().unwrap(),
    );

    (min1, max1, min2, max2)
}
