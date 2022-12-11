use std::fs::read_to_string;

enum Instruction {
    AddX(i32),
    Noop,
}

struct CPU {
    num_cycles: usize,
    register_x: i32,
}

type Screen = Vec<Vec<char>>;

pub fn total_signal_strength(input_file: &str) -> Result<i32, std::io::Error> {
    let input = read_to_string(input_file)?;

    let mut cpu = CPU {
        num_cycles: 0,
        register_x: 1,
    };

    let signal_strength = input.lines().fold(0i32, |mut acc, line| {
        let instruction = parse_instruction(line);

        // if the instruction is addX, add x to the register x, if noop, increment the cycles count only
        match instruction {
            Instruction::AddX(n) => {
                for _ in 0..2 {
                    acc = add_signal_strength(&mut cpu, acc);
                }
                // add x after the 2 cycle has completed
                cpu.register_x += n;
            }
            Instruction::Noop => {
                acc = add_signal_strength(&mut cpu, acc);
            }
        };

        acc
    });

    Ok(signal_strength)
}

pub fn render_image(input_file: &str) -> Result<String, std::io::Error> {
    let input = read_to_string(input_file)?;

    let mut screen = create_screen(40, 6);

    let mut crt = CPU {
        num_cycles: 0,
        register_x: 1,
    };

    let (mut i, mut j) = (0usize, 0usize);

    input.lines().for_each(|line| {
        let instruction = parse_instruction(line);

        match instruction {
            Instruction::AddX(n) => {
                for _ in 0..2 {
                    update_position(&mut i, &mut j, &screen);
                    draw(&mut crt, &mut screen, &mut i, &mut j);
                    crt.num_cycles += 1;
                    j += 1;
                }
                crt.register_x += n;
            }
            Instruction::Noop => {
                update_position(&mut i, &mut j, &screen);
                draw(&mut crt, &mut screen, &mut i, &mut j);
                crt.num_cycles += 1;
                j += 1;
            }
        };
    });

    Ok(print_screen(screen))
}

fn print_screen(screen: Vec<Vec<char>>) -> String {
    let mut render = String::from("");
    for row in screen {
        for pixel in row {
            render.push(pixel);
        }
        render.push('\n');
    }
    render
}

// the current pixel is marked as 'lit' when it is within the sprint
fn draw(crt: &mut CPU, screen: &mut Vec<Vec<char>>, i: &mut usize, j: &mut usize) {
    if (crt.register_x - *j as i32).abs() < 2 {
        screen[*i][*j] = '#';
    }
}

// If the current horizontal position is greater than screen width, go the the next row
fn update_position(i: &mut usize, j: &mut usize, screen: &Vec<Vec<char>>) {
    if *j >= screen[0].len() {
        *i += 1;
        *j = 0;
    }
}

fn create_screen(width: usize, height: usize) -> Screen {
    let row = (0..width).into_iter().map(|_| '.').collect::<Vec<char>>();
    (0..height).into_iter().map(|_| row.clone()).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    match line {
        "noop" => Instruction::Noop,
        inst => Instruction::AddX(
            inst.split(" ")
                .skip(1)
                .next()
                .expect("Invalid instruction")
                .parse::<i32>()
                .expect("Invalid increment"),
        ),
    }
}

fn add_signal_strength(cpu: &mut CPU, strength: i32) -> i32 {
    let mut total = strength;
    if [20, 60, 100, 140, 180, 220].contains(&(cpu.num_cycles + 1)) {
        total += (cpu.num_cycles + 1) as i32 * cpu.register_x;
    }
    cpu.num_cycles += 1;
    return total;
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_total_signal_strength() {
        assert_eq!(
            total_signal_strength("src/day_10/example_input.txt").unwrap(),
            13140
        );
    }

    #[test]
    fn test_image_rendering() {
        assert_eq!(
            render_image("src/day_10/example_input.txt").unwrap(),
            [
                "##..##..##..##..##..##..##..##..##..##..\n",
                "###...###...###...###...###...###...###.\n",
                "####....####....####....####....####....\n",
                "#####.....#####.....#####.....#####.....\n",
                "######......######......######......####\n",
                "#######.......#######.......#######.....\n",
            ]
            .concat()
        );
    }
}
