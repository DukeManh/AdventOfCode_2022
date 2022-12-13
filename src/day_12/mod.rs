use std::fs::read_to_string;

use crate::day_8::Direction;

type HeightMap = Vec<Vec<char>>;
type Cache = Vec<Vec<bool>>;

pub fn fewest_steps(input_file: &str) -> Result<usize, std::io::Error> {
    let input = read_to_string(input_file)?;

    let (mut row, mut col) = (0, 0);
    let mut grid: HeightMap = vec![];
    let mut visited: Cache = vec![];

    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(i, line)| {
            grid.push(vec![]);
            visited.push(vec![]);
            line.chars().enumerate().for_each(|(j, c)| {
                grid[i].push(c);
                visited[i].push(false);
                if c == 'S' {
                    row = i;
                    col = j;
                }
            });
        });

    let minimum_steps = climb(&mut grid, &mut visited, row, col);

    Ok(minimum_steps.unwrap_or_default())
}

pub fn climb(grid: &mut HeightMap, visited: &mut Cache, i: usize, j: usize) -> Option<usize> {
    if grid[i][j] == 'E' {
        return Some(0);
    }

    visited[i][j] = true;
    let mut min: Option<usize> = None;
    for d in [Direction::U, Direction::R, Direction::D, Direction::L] {
        let next = match d {
            Direction::U => {
                if i == 0 {
                    None
                } else {
                    Some((i - 1, j))
                }
            }
            Direction::R => {
                if j == grid[0].len() - 1 {
                    None
                } else {
                    Some((i, j + 1))
                }
            }
            Direction::D => {
                if i == grid.len() - 1 {
                    None
                } else {
                    Some((i + 1, j))
                }
            }
            Direction::L => {
                if j == 0 {
                    None
                } else {
                    Some((i, j - 1))
                }
            }
        };

        if let Some((row, col)) = next {
            if visited[row][col] == true {
                continue;
            }
            if is_reachable(grid, row, col, grid[i][j]) {
                if let Some(climb) = climb(grid, visited, row, col) {
                    min = match min {
                        Some(val) => {
                            if climb < val {}
                            Some(std::cmp::min(val, climb))
                        }
                        None => Some(climb),
                    }
                }
            }
        }
    }

    min.map(|m| m + 1)
}

pub fn is_reachable(grid: &HeightMap, i: usize, j: usize, current_height: char) -> bool {
    let height = match current_height {
        'S' => 'a',
        _ => current_height,
    } as u8;

    let next_height = match grid[i][j] {
        'E' => 'z',
        _ => grid[i][j],
    } as u8;

    (next_height) < height || next_height == height || (height + 1) == next_height
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_the_fewest_step() {
        assert_eq!(fewest_steps("src/day_12/example_input.txt").unwrap(), 31);
    }
}
