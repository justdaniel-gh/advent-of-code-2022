#![allow(dead_code)]

use std::{
    cmp::Ordering,
    fmt::{self, Display},
    fs,
    path::Path,
};

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

pub struct Grid<T> {
    pub cells: Vec<T>,
    pub num_rows: usize,
    pub num_cols: usize,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        Grid {
            cells: vec![Default::default(); num_rows * num_cols],
            num_rows,
            num_cols,
        }
    }

    pub fn row(&self, row_ndx: usize) -> &[T] {
        &self.cells[row_ndx * self.num_cols..(row_ndx * self.num_cols) + self.num_cols]
    }

    pub fn row_mut(&mut self, row_ndx: usize) -> &mut [T] {
        &mut self.cells[row_ndx * self.num_cols..(row_ndx * self.num_cols) + self.num_cols]
    }

    pub fn col(&self, col_ndx: usize) -> Vec<&T> {
        let mut ret_cells: Vec<&T> = Vec::new();
        for cell in self.cells.iter().skip(col_ndx).step_by(self.num_cols) {
            ret_cells.push(cell);
        }
        ret_cells
    }

    pub fn col_mut(&mut self, col_ndx: usize) -> Vec<&mut T> {
        let mut ret_cells: Vec<&mut T> = Vec::new();
        for cell in self.cells.iter_mut().skip(col_ndx).step_by(self.num_cols) {
            ret_cells.push(cell);
        }
        ret_cells
    }

    pub fn get_coord(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.num_cols || y >= self.num_rows {
            None
        } else {
            self.cells.get((y * self.num_cols) + x)
        }
    }

    pub fn get_coord_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.cells.get_mut((y * self.num_cols) + x)
    }

    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.cells.iter()
    }

    /// Returns an iterator moving in the specified direction, starting at (returning first) the x,y coord
    pub fn direction_iter_at(&self, x: usize, y: usize, direction: Direction) -> DirectionIter<'_, T> {
        DirectionIter {
            grid: self,
            direction,
            next_x: x as isize,
            next_y: y as isize,
        }
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: Display + Default + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = String::new();

        for row_ndx in 0..self.num_rows {
            let row_str: String = self.row(row_ndx).iter().map(ToString::to_string).collect();
            rows.push_str(&row_str);
            rows.push('\n');
        }

        write!(f, "{rows}")
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct DirectionIter<'a, T> {
    grid: &'a Grid<T>,
    direction: Direction,
    next_x: isize,
    next_y: isize,
}

impl<'a, T> Iterator for DirectionIter<'a, T>
where
    T: Default + Clone,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self
            .grid
            .get_coord(self.next_x as usize, self.next_y as usize)
        {
            Some(item) => {
                match self.direction {
                    Direction::North => {
                        self.next_y -= 1;
                    }
                    Direction::South => {
                        self.next_y += 1;
                    }
                    Direction::East => {
                        self.next_x += 1;
                    }
                    Direction::West => {
                        self.next_x -= 1;
                    }
                }
                Some(item)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[derive(Debug, Default, Clone)]
    struct TestCell {
        value: u32,
    }

    #[test]
    fn test_grid() {
        let mut g: Grid<TestCell> = Grid::new(2, 2);
        assert_eq!(g.num_cols, 2);
        assert_eq!(g.num_rows, 2);
        assert_eq!(g.cells.len(), 4);

        for c in g.row_mut(0) {
            c.value = 1;
        }

        let mut i = g.direction_iter_at(0,0,crate::Direction::East);
        assert_eq!(i.next().unwrap().value, 1);
        assert_eq!(i.next().unwrap().value, 1);
    }
}
