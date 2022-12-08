// Let's be fancy today and use struct

use std::{
    fs::read_to_string,
    num::ParseIntError,
    str::{FromStr, Lines},
};

struct File {
    size: u32,
}

impl FromStr for File {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        Ok(File {
            size: split.next().unwrap().parse::<u32>()?,
        })
    }

    type Err = ParseIntError;
}

struct Dir {
    files: Vec<File>,
    dirs: Vec<Box<Dir>>,
    path: String,
}

impl Dir {
    fn new(path: String) -> Self {
        Dir {
            files: vec![],
            dirs: vec![],
            path,
        }
    }
}

pub fn total_size_of_directory_under_threshold(threshold: u32) -> u32 {
    let input = read_to_string("src/day_7/input.txt").unwrap();

    let mut root = Dir::new(String::from("/"));

    let mut x = input.lines().into_iter();
    x.next().unwrap();

    read_terminal(&mut x, &mut root);

    let mut total_size_below_threshold = 0u32;
    calculate_size(&root, &threshold, &mut total_size_below_threshold);

    total_size_below_threshold
}

pub fn smallest_dir_to_be_deleted(total_disk_size: u32, space_needed: u32) -> u32 {
    let input = read_to_string("src/day_7/input.txt").unwrap();

    let mut root = Dir::new(String::from("/"));

    let mut x = input.lines().into_iter();
    x.next().unwrap();
    read_terminal(&mut x, &mut root);

    let total_size = calculate_size(&root, &0, &mut 0);
    let mut smallest = total_size;

    smallest_dir(
        &root,
        &(total_disk_size - total_size),
        &space_needed,
        &mut smallest,
    );

    return smallest;
}

fn read_terminal(lines: &mut Lines, current_dir: &mut Dir) {
    if let Some(line) = lines.next() {
        let mut cd = line.clone();
        if line.starts_with("$ ls") {
            while let Some(line) = lines.next() {
                if line.starts_with("$ cd") {
                    cd = line;
                    break;
                }

                if line.starts_with("dir") {
                    current_dir.dirs.push(Box::new(Dir::new(
                        current_dir.path.clone() + &line[4..] + "/",
                    )))
                } else {
                    current_dir.files.push(File::from_str(line).unwrap());
                }
            }
        }

        if let Some(x) = cd.split(" ").skip(2).next() {
            if x == ".." {
                return;
            }

            for mut dir in current_dir.dirs.iter_mut() {
                if dir.as_mut().path.ends_with(&(x.to_string() + "/")) {
                    read_terminal(lines, &mut dir);
                    break;
                }
            }
            read_terminal(lines, current_dir);
        }
    }
}

fn calculate_size(root: &Dir, threshold: &u32, total: &mut u32) -> u32 {
    let mut size = root.files.iter().fold(0u32, |acc, f| acc + f.size);

    root.dirs.iter().for_each(|dir| {
        size += calculate_size(dir.as_ref(), threshold, total);
    });

    if size <= *threshold {
        *total += size;
    }

    size
}

fn smallest_dir(root: &Dir, unused_space: &u32, space_needed: &u32, smallest: &mut u32) -> u32 {
    let mut size = root.files.iter().fold(0u32, |acc, f| acc + f.size);

    root.dirs.iter().for_each(|dir| {
        size += smallest_dir(dir.as_ref(), unused_space, space_needed, smallest);
    });

    if unused_space + size >= *space_needed && size < *smallest {
        *smallest = size;
    }

    size
}
