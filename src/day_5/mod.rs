use regex::Regex;
use std::fs::read_to_string;

// brute-force, follow the re-arrangement instructions 1 by 1?
// the hardest part is parsing the input file to stacks and re-arrangement instructions
pub fn crates_end_up_on_top_with_9000() -> String {
    fn crate_mover_9000(stacks: &mut Vec<Vec<char>>, n: &usize, a: &usize, b: &usize) {
        for _ in 0..*n {
            match stacks[*a].pop() {
                Some(c) => {
                    stacks[*b].push(c);
                }
                _ => {
                    println!("stack {} is empty", *a + 1);
                }
            }
        }
    }

    return move_crates(crate_mover_9000);
}

pub fn crates_end_up_on_top_with_9001() -> String {
    fn crate_mover_9001(stacks: &mut Vec<Vec<char>>, n: &usize, a: &usize, b: &usize) {
        let mut auxiliary = Vec::<char>::new();
        for _ in 0..*n {
            match stacks[*a].pop() {
                Some(c) => {
                    auxiliary.push(c);
                }
                _ => {
                    println!("stack {} is empty", a + 1);
                }
            }
        }
        for c in auxiliary.into_iter().rev() {
            stacks[*b].push(c);
        }
    }

    return move_crates(crate_mover_9001);
}

pub fn move_crates(mover: fn(&mut Vec<Vec<char>>, &usize, &usize, &usize)) -> String {
    let input = read_to_string("src/day_5/input.txt").unwrap();

    let mut split = input.split("\n\n");

    let (stacks_str, instructions) = (split.next().unwrap(), split.next().unwrap());

    let mut stacks = Vec::<Vec<char>>::new();

    stacks_str.lines().rev().for_each(|line| {
        let re = Regex::new(r"(\[[(A-Z)]\]|\s\s\s)\s?").unwrap();
        for (i, cap) in re.captures_iter(line).enumerate() {
            if i >= stacks.len() {
                stacks.push(vec![]);
            }
            if &cap[1] != "   " {
                stacks[i].push(cap[1].chars().nth(1).unwrap());
            }
        }
    });

    let re = Regex::new(r"move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();
    for cap in re.captures_iter(instructions) {
        let (n, a, b) = (
            cap[1].parse::<usize>().unwrap(),
            cap[2].parse::<usize>().unwrap() - 1,
            cap[3].parse::<usize>().unwrap() - 1,
        );

        mover(&mut stacks, &n, &a, &b);
    }

    stacks
        .into_iter()
        .fold(Vec::<char>::new(), |mut acc, stack| match stack.last() {
            Some(c) => {
                acc.push(*c);
                acc
            }
            _ => acc,
        })
        .into_iter()
        .collect::<String>()
}
