use regex::Regex;
use std::fs::read_to_string;

struct Monkey {
    number: usize,
    operator: String,
    change: String,
    divisor: u32,
    if_divisible: usize,
    if_not_divisible: usize,
    num_inspection: usize,
}

impl Monkey {
    fn operate(&self, old: u32) -> (u32, usize) {
        let num = match self.change.as_str() {
            "old" => old,
            num => num.parse::<u32>().unwrap(),
        };

        let item = match self.operator.as_str() {
            "*" => old * num,
            _ => old + num,
        } / 3;

        match item.rem_euclid(self.divisor) {
            0 => (item, self.if_divisible),
            _ => (item, self.if_not_divisible),
        }
    }
}

pub fn monkey_business(input_file: &str) -> Result<usize, std::io::Error> {
    let input = read_to_string(input_file)?;

    let mut monkeys = Vec::<Monkey>::new();

    let mut monkey_items: Vec<Vec<u32>> = vec![];
    let pattern = [
        r"\s*Monkey (\d+):\n",
        r"\s*Starting items: (.*)\n",
        r"\s*Operation: new = old ([\+\*]) (\d+|old)\n",
        r"\s*Test: divisible by (\d+)\n",
        r"\s*If true: throw to monkey (\d+)\n",
        r"\s*If false: throw to monkey (\d+)\n",
    ]
    .concat();

    let re = Regex::new(&pattern).unwrap();
    for turn in re.captures_iter(&input) {
        let number = turn[1].parse::<usize>().unwrap();

        let items = turn[2]
            .split(", ")
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let operator = String::from(&turn[3]);
        let change = String::from(&turn[4]);
        let divisor = turn[5].parse::<u32>().unwrap();
        let (if_divisible, if_not_divisible) = (
            turn[6].parse::<usize>().unwrap(),
            turn[7].parse::<usize>().unwrap(),
        );

        monkeys.push(Monkey {
            number,
            operator,
            change,
            divisor,
            if_divisible,
            if_not_divisible,
            num_inspection: 0,
        });
        monkey_items.push(items);
    }

    for _ in 0..20usize {
        for monkey in monkeys.iter_mut() {
            while let Some(item) = monkey_items[monkey.number].pop() {
                let (worry, next_monkey) = monkey.operate(item);

                monkey_items[next_monkey].push(worry);
                monkey.num_inspection += 1;
            }
        }
    }

    monkeys.sort_by(|a, b| a.num_inspection.cmp(&b.num_inspection));

    Ok(monkeys.pop().unwrap().num_inspection * monkeys.pop().unwrap().num_inspection)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_monkey_business() {
        assert_eq!(
            monkey_business("src/day_11/example_input.txt").unwrap(),
            10605
        );
    }
}
