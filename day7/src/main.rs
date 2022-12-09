use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct File {
    _name: String,
    size: usize,
}

#[derive(Default, Debug)]
struct Directory {
    _name: String,
    directories: HashMap<String, Box<Directory>>,
    files: Vec<File>,
    /// Includes sub dir sizes
    total_dir_size: usize,
}

fn parse_result(current_dir: &mut Directory, lines: &Vec<&str>, mut ndx: usize) -> usize {
    let result_re = Regex::new(r"(?:(dir)|(\d+)) (.+)").unwrap();
    while ndx < lines.len() {
        let line = *lines.get(ndx).unwrap();
        match &line[..4] {
            "$ cd" => {
                // cd command
                if line.len() > 6 && &line[5..7] == ".." {
                    // cd up
                    return ndx;
                } else if &line[5..6] != "/" {
                    // cd into dir
                    ndx = parse_result(
                        current_dir.directories.get_mut(&line[5..]).unwrap(),
                        lines,
                        ndx + 1,
                    );
                }
            }
            "$ ls" => {
                // ls command
                // Nothing to do...
            }
            _ => {
                // Results
                let caps = result_re.captures(line).unwrap();
                match caps.get(1) {
                    Some(_) => {
                        current_dir.directories.insert(
                            caps.get(3).unwrap().as_str().to_string(),
                            Box::new(Directory {
                                _name: caps.get(3).unwrap().as_str().to_string(),
                                ..Default::default()
                            }),
                        );
                    }
                    None => {
                        current_dir.files.push(File {
                            _name: caps.get(3).unwrap().as_str().to_string(),
                            size: caps.get(2).unwrap().as_str().parse().unwrap(),
                        });
                    }
                }
            }
        }
        ndx += 1;
    }
    ndx
}

fn calculate_directory_sizes(dir: &mut Box<Directory>) -> usize {
    let file_sum = dir.files.iter().fold(0, |a, f| a + f.size);
    let total_sum = dir.directories.values_mut().fold(file_sum, |a, sub_dir| {
        a + calculate_directory_sizes(sub_dir)
    });
    dir.total_dir_size = total_sum;
    total_sum
}

fn parser(s: String) -> Box<Directory> {
    let mut root_directory = Box::new(Directory {
        _name: "/".to_string(),
        ..Default::default()
    });
    parse_result(root_directory.as_mut(), &s.split('\n').collect(), 0);
    calculate_directory_sizes(&mut root_directory);
    root_directory
}

fn filter_dirs_max(dir: &Directory, sizes: &mut Vec<usize>, max_size: usize) -> usize {
    for sub_dir in dir.directories.values() {
        filter_dirs_max(sub_dir, sizes, max_size);
    }
    // Do I meet the filter?
    if dir.total_dir_size <= max_size {
        sizes.push(dir.total_dir_size)
    }
    dir.total_dir_size
}

fn filter_dirs_min(dir: &Directory, sizes: &mut Vec<usize>, min_size: usize) -> usize {
    for sub_dir in dir.directories.values() {
        filter_dirs_min(sub_dir, sizes, min_size);
    }
    // Do I meet the filter?
    if dir.total_dir_size >= min_size {
        sizes.push(dir.total_dir_size)
    }
    dir.total_dir_size
}

fn solve(root_dir: &Directory) -> usize {
    let mut filtered_dir_sizes = vec![];
    filter_dirs_max(root_dir, &mut filtered_dir_sizes, 100000);
    filtered_dir_sizes.iter().sum()
}

fn solve2(root_dir: &Directory) -> usize {
    let unused_size = 70000000 - root_dir.total_dir_size;
    let space_needed = 30000000 - unused_size;
    let mut filtered_dir_sizes = vec![];
    filter_dirs_min(root_dir, &mut filtered_dir_sizes, space_needed);
    filtered_dir_sizes.into_iter().min().unwrap()
}

fn main() {
    let data = utils::load_puzzle_data(7, parser);
    let total_size = solve(&data);
    println!("Solution 1: The total size is {total_size} bytes.");

    let total_size = solve2(&data);
    println!("Solution 2: The best dir to delete has a size of {total_size} bytes.");
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(7, parser);
        let solution = solve(&test_data);
        assert_eq!(solution, 95437);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(7, parser);
        let solution = solve2(&test_data);
        assert_eq!(solution, 24933642);
    }
}
