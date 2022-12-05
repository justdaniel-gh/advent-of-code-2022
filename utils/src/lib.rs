use std::{fs, path::Path};

fn load_puzzle<T, F: FnOnce(String) -> T>(puzzle_path: &Path, parser: F) -> T {
    parser(String::from_utf8(fs::read(puzzle_path).expect("Unable to open input!")).unwrap())
}

pub fn load_puzzle_data<T, F: FnOnce(String) -> T>(day: u32, parser: F) -> T {
    let puzzle_filename = format!("puzzles/day{day}.txt");
    let puzzle_path = Path::new(&puzzle_filename);
    load_puzzle(puzzle_path, parser)
}

pub fn load_puzzle_test<T, F: FnOnce(String) -> T>(day: u32, parser: F) -> T {
    let puzzle_filename = format!("../puzzles/day{day}_test.txt");
    let puzzle_path = Path::new(&puzzle_filename);
    load_puzzle(puzzle_path, parser)
}
