use std::{fs, path::Path, cmp::Ordering};

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

// Thank you Francis Gagné! : https://stackoverflow.com/a/42356713
pub trait SliceExt {
    type Item;

    fn get_two_mut(&mut self, index0: usize, index1: usize) -> (&mut Self::Item, &mut Self::Item);
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn get_two_mut(&mut self, index0: usize, index1: usize) -> (&mut Self::Item, &mut Self::Item) {
        match index0.cmp(&index1) {
            Ordering::Less => {
                let mut iter = self.iter_mut();
                let item0 = iter.nth(index0).unwrap();
                let item1 = iter.nth(index1 - index0 - 1).unwrap();
                (item0, item1)
            }
            Ordering::Equal => panic!("[T]::get_two_mut(): received same index twice ({index0})"),
            Ordering::Greater => {
                let mut iter = self.iter_mut();
                let item1 = iter.nth(index1).unwrap();
                let item0 = iter.nth(index0 - index1 - 1).unwrap();
                (item0, item1)
            }
        }
    }
}