use std::fs::read_to_string;

#[derive(Debug)]
enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

pub fn sum_of_pair_indices(input_file: &str) -> Result<u32, std::io::Error> {
    let input = read_to_string(input_file)?;

    let sum = input.split("\n\n").enumerate().fold(0, |acc, (i, pair)| {
        let mut split = pair.split("\n");
        let (p1, p2) = (split.next().unwrap(), split.next().unwrap());

        let packet_1 = parse_packet(p1);
        let packet_2 = parse_packet(p2);

        match is_valid(&packet_1, &packet_2).unwrap() {
            true => acc + i as u32 + 1,
            false => acc,
        }
    });

    Ok(sum)
}

fn parse_packet(s: &str) -> Vec<Packet> {
    fn parse(s: &str, i: usize) -> (usize, Vec<Packet>) {
        let chars = s.chars().collect::<Vec<char>>();
        let mut packets: Vec<Packet> = vec![];
        let mut j = i;

        while j < s.len() {
            let c = chars[j];
            if c == '[' {
                let (end, packet) = parse(s, j + 1);
                packets.push(Packet::List(packet));
                j = end + 1;
            } else if c == ']' {
                break;
            } else if c == ',' {
                j += 1;
            } else {
                let int = c.to_digit(10).unwrap() as u8;
                packets.push(Packet::Int(int));
                j += 1;
            }
        }

        (j, packets)
    }

    let (_, packets) = parse(s, 1);
    packets
}

fn is_valid(packet_1: &Vec<Packet>, packet_2: &Vec<Packet>) -> Option<bool> {
    let mut current = 0;

    while current < packet_1.len() && current < packet_2.len() {
        let valid = match (&packet_1[current], &packet_2[current]) {
            (Packet::Int(int1), Packet::Int(int2)) => {
                if int1 < int2 {
                    Some(true)
                } else if int1 == int2 {
                    None
                } else {
                    Some(false)
                }
            }
            (Packet::Int(int1), Packet::List(list2)) => is_valid(&vec![Packet::Int(*int1)], list2),
            (Packet::List(list1), Packet::Int(int2)) => is_valid(list1, &vec![Packet::Int(*int2)]),
            (Packet::List(list1), Packet::List(list2)) => is_valid(list1, list2),
        };

        if let Some(result) = valid {
            return Some(result);
        } else {
            current += 1;
        }
    }

    if current < packet_1.len() {
        Some(false)
    } else if current < packet_2.len() {
        Some(true)
    } else {
        None
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_sum_of_pair_indices() {
        assert_eq!(
            sum_of_pair_indices("src/day_13/example_input.txt").unwrap(),
            13
        );
    }
}
