use std::{fs::read_to_string, num::ParseIntError, str::FromStr};

use crate::day_8::Direction;

struct Movement {
    direction: Direction,
    positions: usize,
}

impl FromStr for Movement {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.split(" ");

        Ok(Movement {
            direction: match input.next().unwrap() {
                "U" => Direction::U,
                "R" => Direction::R,
                "D" => Direction::D,
                "L" => Direction::L,
                c => panic!("{} is not a valid direction", c),
            },

            positions: input.next().unwrap().parse::<usize>()?,
        })
    }

    type Err = ParseIntError;
}

struct Grid<T>(Vec<Vec<T>>);

impl<T: Copy> Grid<T> {
    fn new() -> Self {
        Grid(vec![vec![]])
    }

    fn expand(&mut self, d: &Direction, by: usize, value: T) {
        match d {
            Direction::U => {
                for _ in 0..by {
                    self.0.insert(
                        0,
                        (0..self.0[0].len())
                            .into_iter()
                            .map(|_| value)
                            .collect::<Vec<T>>(),
                    )
                }
            }
            Direction::R => {
                for i in 0..self.0.len() {
                    for _ in 0..by {
                        self.0[i].push(value);
                    }
                }
            }
            Direction::D => {
                for _ in 0..by {
                    self.0.push(
                        (0..self.0[0].len())
                            .into_iter()
                            .map(|_| value)
                            .collect::<Vec<T>>(),
                    )
                }
            }
            Direction::L => {
                for i in 0..self.0.len() {
                    for _ in 0..by {
                        self.0[i].insert(0, value);
                    }
                }
            }
        }
    }
}

pub fn unique_visited_positions(input_file: &str) -> Result<usize, std::io::Error> {
    // We are not given with the position where head and tails starts and what is the size of the 2-d grid
    let mut grid = Grid::<bool>::new();
    grid.0[0].push(true);

    // let i, j be the starting point for head
    let (mut i, mut j) = (0, 0);

    // let r, c be the starting point for tail
    let (mut r, mut c) = (0isize, 0isize);

    let input = read_to_string(input_file)?;

    // follow the movement of head 1 by 1
    input.lines().for_each(|line| {
        let movement = Movement::from_str(line).unwrap();

        // update the position of head, then tail
        // if i + delX > grid bound, expand the grid by (i + delX - bound).abs(), update i = bound
        // if j+ delX > grid bound, expand the grid by (j + delX - bound).abs(), update j = bound
        // mark the position of tail as visited

        match movement.direction {
            Direction::U => {
                if i < movement.positions {
                    grid.expand(&movement.direction, movement.positions - i, false);
                    let del = movement.positions - i;
                    i += del;
                    r += del as isize;
                }

                for _ in 0..movement.positions {
                    i -= 1;
                    update_tail_position(&mut i, &mut j, &mut r, &mut c);
                    mark_as_visited(&mut grid, r, c);
                }
            }
            Direction::R => {
                if j + movement.positions >= grid.0[0].len() {
                    grid.expand(
                        &movement.direction,
                        movement.positions + j - grid.0[0].len() + 1,
                        false,
                    );
                }

                for _ in 0..movement.positions {
                    j += 1;
                    update_tail_position(&mut i, &mut j, &mut r, &mut c);
                    mark_as_visited(&mut grid, r, c);
                }
            }
            Direction::D => {
                if i + movement.positions >= grid.0.len() {
                    grid.expand(
                        &movement.direction,
                        movement.positions + i - grid.0.len() + 1,
                        false,
                    );
                }
                for _ in 0..movement.positions {
                    i += 1;
                    update_tail_position(&mut i, &mut j, &mut r, &mut c);
                    mark_as_visited(&mut grid, r, c);
                }
            }
            Direction::L => {
                if j < movement.positions {
                    grid.expand(&movement.direction, movement.positions - j, false);
                    let del = movement.positions - j;
                    j += del;
                    c += del as isize;
                }
                for _ in 0..movement.positions {
                    j -= 1;
                    update_tail_position(&mut i, &mut j, &mut r, &mut c);
                    mark_as_visited(&mut grid, r, c);
                }
            }
        }
    });
    let total = grid.0.into_iter().fold(0usize, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, cell| acc + if *cell { 1 } else { 0 })
    });

    Ok(total)
}

fn mark_as_visited(grid: &mut Grid<bool>, r: isize, c: isize) {
    grid.0[r as usize][c as usize] = true
}

fn update_tail_position(i: &usize, j: &usize, r: &mut isize, c: &mut isize) {
    let delta_y = (*i as isize - *r) / 2;
    if delta_y != 0 {
        *c += *j as isize - *c;
    }
    *r += delta_y;

    let delta_x = (*j as isize - *c) / 2;
    if delta_x != 0 {
        *r += *i as isize - *r;
    }
    *c += delta_x;
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_unique_positions() {
        assert_eq!(
            unique_visited_positions("src/day_9/example_input.txt").unwrap(),
            13
        );
    }
}
