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

pub trait Grid {
    type Item;

    fn get_coord(&self, x: usize, y: usize) -> Option<&Self::Item>;
    fn get_coord_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item>;
}

#[derive(Debug, Default, Clone)]
pub struct StaticGrid<T> {
    pub cells: Vec<T>,
    pub num_rows: usize,
    pub num_cols: usize,
}

impl<T> StaticGrid<T>
where
    T: Default + Clone,
{
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        StaticGrid {
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

    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.cells.iter()
    }

    /// Returns an iterator moving in the specified direction, starting at (returning first) the x,y coord
    pub fn direction_iter_at(
        &self,
        x: usize,
        y: usize,
        direction: Direction,
    ) -> DirectionIter<'_, T> {
        DirectionIter {
            grid: self,
            direction,
            next_x: x as isize,
            next_y: y as isize,
        }
    }
}

impl<T> Grid for StaticGrid<T> {
    type Item = T;

    fn get_coord(&self, x: usize, y: usize) -> Option<&Self::Item> {
        if x >= self.num_cols || y >= self.num_rows {
            None
        } else {
            self.cells.get((y * self.num_cols) + x)
        }
    }

    fn get_coord_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item> {
        self.cells.get_mut((y * self.num_cols) + x)
    }
}

impl<T> fmt::Display for StaticGrid<T>
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

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

pub struct DirectionIter<'a, T> {
    grid: &'a dyn Grid<Item = T>,
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
                    Direction::NorthEast => {
                        self.next_x += 1;
                        self.next_y -= 1;
                    },
                    Direction::NorthWest => {
                        self.next_x -= 1;
                        self.next_y -= 1;
                    },
                    Direction::SouthEast => {
                        self.next_x += 1;
                        self.next_y += 1;
                    },
                    Direction::SouthWest =>{
                        self.next_x -= 1;
                        self.next_y += 1;
                    },
                }
                Some(item)
            }
            None => None,
        }
    }
}

pub struct DynamicGrid<T> {
    // [y][x]
    //     -|
    //  -   |    +
    //  ----------
    //      |
    //     +|
    cells: Vec<Vec<T>>,
    center_x: isize,
    center_y: isize,
    num_rows: usize,
    num_cols: usize,
}

/*
Add a new negative column


*/
impl<T> DynamicGrid<T>
where
    T: Default + Clone,
{
    /// Starts as a single cell
    pub fn new() -> Self {
        DynamicGrid {
            cells: vec![vec![Default::default()]],
            center_x: 0,
            center_y: 0,
            num_cols: 1,
            num_rows: 1,
        }
    }
    //fn add_cell

    fn cols(&self) -> usize {
        self.num_cols
    }

    pub fn row(&self, row_ndx: usize) -> &[T] {
        &self.cells[row_ndx]
    }

    fn rel_to_abs(&mut self, rel_x: isize, rel_y: isize) -> (usize, usize) {
        let mut abs_y = self.center_y + rel_y;
        if abs_y < 0 {
            abs_y = abs_y.abs();
            self.center_y += abs_y;
            for _ in 0..abs_y {
                self.cells
                    .insert(0, vec![Default::default(); self.num_cols]);
                self.num_rows += 1;
            }
        } else if abs_y >= self.num_rows as isize {
            let diff_y = self.num_rows as isize - abs_y + 1;
            for _ in 0..diff_y {
                self.cells.push(vec![Default::default(); self.num_cols]);
                self.num_rows += 1;
            }
        };

        let mut abs_x = self.center_x + rel_x;
        if abs_x < 0 {
            abs_x = abs_x.abs();
            self.center_x += abs_x;
            for _ in 0..abs_x {
                for c in self.cells.iter_mut() {
                    c.insert(0, Default::default());
                }
                self.num_cols += 1;
            }
        } else if abs_x >= self.num_cols as isize {
            let diff_x = self.num_cols as isize - abs_x + 1;
            for _ in 0..diff_x {
                for c in self.cells.iter_mut() {
                    c.push(Default::default());
                }
                self.num_cols += 1;
            }
        };
        let abs_y = self.center_y + rel_y;
        let abs_x = self.center_x + rel_x;
        (abs_x as usize, abs_y as usize)
    }

    pub fn get_cell_at(&mut self, rel_x: isize, rel_y: isize) -> &T {
        let (abs_x, abs_y) = self.rel_to_abs(rel_x, rel_y);
        self.cells.get(abs_y).unwrap().get(abs_x).unwrap()
    }

    pub fn get_cell_at_mut(&mut self, rel_x: isize, rel_y: isize) -> &mut T {
        let (abs_x, abs_y) = self.rel_to_abs(rel_x, rel_y);
        self.cells.get_mut(abs_y).unwrap().get_mut(abs_x).unwrap()
    }

    pub fn direction_iter_mut(&mut self, _direction: Direction, _amount: usize) {
        todo!()
    }

    pub fn cell_iter(&mut self) -> std::iter::Flatten<std::slice::Iter<'_, Vec<T>>> {
        self.cells.iter().flatten()
    }
}

impl<T> Default for DynamicGrid<T>
where
    T: Display + Default + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Display for DynamicGrid<T>
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

#[cfg(test)]
mod tests {
    use std::fmt;

    use crate::{DynamicGrid, StaticGrid};

    #[derive(Debug, Default, Clone)]
    struct TestCell {
        value: u32,
    }

    impl fmt::Display for TestCell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:>1}", self.value)
        }
    }

    #[test]
    fn test_grid() {
        let mut g: StaticGrid<TestCell> = StaticGrid::new(2, 2);
        assert_eq!(g.num_cols, 2);
        assert_eq!(g.num_rows, 2);
        assert_eq!(g.cells.len(), 4);

        for c in g.row_mut(0) {
            c.value = 1;
        }

        let mut i = g.direction_iter_at(0, 0, crate::Direction::East);
        assert_eq!(i.next().unwrap().value, 1);
        assert_eq!(i.next().unwrap().value, 1);
    }

    #[test]
    fn test_dynamic_grid() {
        let mut g: DynamicGrid<TestCell> = DynamicGrid::new();
        assert_eq!(g.num_cols, 1);
        assert_eq!(g.num_rows, 1);
        assert_eq!(g.cells.len(), 1);

        let c = g.get_cell_at_mut(1, 1);
        c.value = 1;
        assert_eq!(c.value, 1);

        print!("{g}");
        assert_eq!(g.num_cols, 2);
        assert_eq!(g.num_rows, 2);

        // 2101
        let c = g.get_cell_at_mut(-2, -2);
        c.value = 1;

        assert_eq!(c.value, 1);

        print!("{g}");
        assert_eq!(g.num_cols, 4);
        assert_eq!(g.num_rows, 4);

        let c = g.get_cell_at_mut(-3, -2);
        c.value = 1;

        assert_eq!(c.value, 1);

        print!("{g}");
        assert_eq!(g.num_cols, 5);
        assert_eq!(g.num_rows, 4);
    }
}
