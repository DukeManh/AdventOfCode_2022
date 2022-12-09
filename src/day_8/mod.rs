use std::{fs::read_to_string, str::FromStr};

enum Direction {
    U,
    R,
    D,
    L,
}

struct Grid(Vec<Vec<usize>>);

// convert the input into a 2d-array grid
impl FromStr for Grid {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::<Vec<usize>>::new();

        s.lines().for_each(|line| {
            let row = line
                .chars()
                .map(|c| c as usize - '0' as usize)
                .collect::<Vec<usize>>();

            grid.push(row);
        });

        Ok(Grid(grid))
    }

    type Err = std::io::Error;
}

pub fn total_visible_trees(input_file: &str) -> Result<usize, std::io::Error> {
    let input = read_to_string(input_file)?;

    let trees = Grid::from_str(&input).unwrap();
    let grid = &trees.0;

    let mut total = 0usize;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let visible = is_tree_visible(&trees, i, j);

            if visible {
                total += 1;
            }
        }
    }

    Ok(total)
}

pub fn hightest_scenic_score(input_file: &str) -> Result<usize, std::io::Error> {
    let input = read_to_string(input_file)?;

    let trees = Grid::from_str(&input).unwrap();
    let grid = &trees.0;

    let mut highest = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let score = cal_scenic_score(&trees, i, j);
            if score > highest {
                highest = score;
            }
        }
    }

    Ok(highest)
}

fn is_tree_visible(trees: &Grid, i: usize, j: usize) -> bool {
    let grid = &trees.0;
    if is_edge_tree(trees, i, j) {
        return true;
    }

    for d in [Direction::U, Direction::R, Direction::D, Direction::L] {
        let (a, b) = get_adjacent_tree(&d, i, j);
        if is_blocked(trees, a, b, d, grid[i][j]) {
            return true;
        }
    }

    false
}

fn is_blocked(trees: &Grid, i: usize, j: usize, d: Direction, current: usize) -> bool {
    let grid = &trees.0;
    if is_edge_tree(trees, i, j) {
        return current > grid[i][j];
    }
    if current > grid[i][j] {
        let (a, b) = get_adjacent_tree(&d, i, j);
        return is_blocked(trees, a, b, d, current);
    }

    false
}

fn cal_scenic_score(trees: &Grid, i: usize, j: usize) -> usize {
    if is_edge_tree(trees, i, j) {
        return 0;
    }

    let grid = &trees.0;
    let mut score = 1;
    for d in [Direction::U, Direction::R, Direction::D, Direction::L] {
        let (a, b) = get_adjacent_tree(&d, i, j);
        score *= number_of_tree_within_sight(trees, a, b, d, grid[i][j]);
    }

    score
}

fn number_of_tree_within_sight(
    trees: &Grid,
    i: usize,
    j: usize,
    d: Direction,
    current: usize,
) -> usize {
    if is_edge_tree(trees, i, j) {
        return 1;
    }
    let grid = &trees.0;

    if current > grid[i][j] {
        let (a, b) = get_adjacent_tree(&d, i, j);
        return 1 + number_of_tree_within_sight(trees, a, b, d, current);
    }

    1
}

fn is_edge_tree(trees: &Grid, i: usize, j: usize) -> bool {
    let grid = &trees.0;
    return i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() - 1;
}

fn get_adjacent_tree(d: &Direction, i: usize, j: usize) -> (usize, usize) {
    match d {
        Direction::U => (i - 1, j),
        Direction::R => (i, j + 1),
        Direction::D => (i + 1, j),
        Direction::L => (i, j - 1),
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_total_visible_tree() {
        assert_eq!(
            total_visible_trees("src/day_8/example_input.txt").unwrap(),
            21
        );
    }

    #[test]
    fn test_hightest_scenic_score() {
        assert_eq!(
            hightest_scenic_score("src/day_8/example_input.txt").unwrap(),
            8
        );
    }
}
