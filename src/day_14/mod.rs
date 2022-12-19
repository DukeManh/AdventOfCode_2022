use std::fs::read_to_string;

type Cave = Vec<Vec<char>>;

pub fn the_endless_void(input_file: &str) -> Result<usize, std::io::Error> {
    let input = read_to_string(input_file)?;

    // Parse the input fill into a list of coordinates for each path
    let mut coordinates = input.lines().fold(vec![], |mut coords, line| {
        coords.push(line.split(" -> ").fold(vec![], |mut acc, co| {
            let mut split = co.split(",");
            let (x, y) = (
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            );
            acc.push((x, y));
            acc
        }));
        coords
    });

    // Find the min and max coordinates value to create a grid that stores all possible coordinates
    let (min_x, max_x, max_y) = find_min_max_coordinates(&coordinates);

    // Update the coordinate values to fit in the created grid
    coordinates = coordinates
        .into_iter()
        .map(|line| line.into_iter().map(|(x, y)| (x - min_x, y)).collect())
        .collect::<Vec<Vec<_>>>();

    let mut cave = create_cave_grid(max_y, max_x - min_x + 1);
    draw_rock_path(&coordinates, &mut cave);

    let (sand_x, sand_y) = (500 - min_x, 0);

    let mut settled_sands = 0;
    // println!("{:?}", cave);

    while sand_falling(&mut cave, sand_x, sand_y) == true {
        settled_sands += 1;
    }

    // println!("{:?}", cave);

    Ok(settled_sands)
}

fn create_cave_grid(len_y: usize, len_x: usize) -> Vec<Vec<char>> {
    let mut cave = Vec::<Vec<char>>::with_capacity(len_y + 1);
    let row = (0..len_x).into_iter().map(|_| '.').collect::<Vec<char>>();
    for _ in 0..len_y + 1 {
        cave.push(row.clone());
    }
    cave
}

fn draw_rock_path(coordinates: &Vec<Vec<(usize, usize)>>, cave: &mut Vec<Vec<char>>) {
    for line in coordinates {
        let mut starting = true;
        let (mut p_x, mut p_y) = (0, 0);
        for (x, y) in line {
            let (x, y) = (*x, *y);
            if starting {
                starting = false;
            } else {
                if p_x == x {
                    let range = if y > p_y { p_y..y + 1 } else { y..p_y + 1 };
                    for i in range {
                        cave[i][x] = '#';
                    }
                } else if p_y == y {
                    let range = if x > p_x { p_x..x + 1 } else { x..p_x + 1 };
                    for i in range {
                        cave[y][i] = '#';
                    }
                }
            }

            p_x = x;
            p_y = y;
        }
    }
}

enum Dir {
    D,
    L,
    R,
}

pub fn sand_falling(cave: &mut Cave, s_x: usize, s_y: usize) -> bool {
    let (len_x, len_y) = (cave[0].len(), cave.len());
    let mut x = s_x;
    let mut y = s_y;

    let mut is_settled = false;

    while !is_settled {
        let dirs = [Dir::D, Dir::L, Dir::R];
        let mut found_move = false;

        for dir in dirs.iter() {
            if let Some((i, j)) = get_next_cell(dir, x, y, len_x, len_y) {
                if cave[j][i] == '.' {
                    found_move = true;
                    x = i;
                    y = j;
                    break;
                }
            } else {
                // the sand has fallen outside of the grid
                return false;
            }
        }

        if found_move {
            is_settled = false;
        } else {
            is_settled = true;
        }
    }

    if is_settled {
        cave[y][x] = 'o';
    }

    is_settled
}

fn get_next_cell(
    d: &Dir,
    x: usize,
    y: usize,
    len_x: usize,
    len_y: usize,
) -> Option<(usize, usize)> {
    match d {
        Dir::D => (y != len_y - 1).then(|| (x, y + 1)),
        Dir::L => (y != len_y - 1 && x != 0).then(|| (x - 1, y + 1)),
        Dir::R => (y != len_y - 1 && x != len_x - 1).then(|| (x + 1, y + 1)),
    }
}

fn find_min_max_coordinates(coordinates: &Vec<Vec<(usize, usize)>>) -> (usize, usize, usize) {
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    for line in coordinates {
        for (x, y) in line {
            min_x = std::cmp::min(min_x, *x);
            max_x = std::cmp::max(max_x, *x);
            max_y = std::cmp::max(max_y, *y);
        }
    }

    (min_x, max_x, max_y)
}
