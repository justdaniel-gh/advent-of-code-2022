use std::fmt;

use take_until::TakeUntilExt;

#[derive(Default)]
struct GridCell {
    value: i32,
    is_visible: bool,
    viewing_score: usize,
}

impl fmt::Display for GridCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>1}{} ",
            self.value,
            if self.is_visible { '*' } else { ' ' }
        )
    }
}
struct Grid {
    cells: Vec<GridCell>,
    num_rows: usize,
    num_cols: usize,
}

impl Grid {
    fn row(&self, row_num: usize) -> &[GridCell] {
        &self.cells[row_num * self.num_cols..(row_num * self.num_cols) + self.num_cols]
    }

    fn row_mut(&mut self, row_num: usize) -> &mut [GridCell] {
        &mut self.cells[row_num * self.num_cols..(row_num * self.num_cols) + self.num_cols]
    }

    #[allow(dead_code)]
    fn col(&self, col_num: usize) -> Vec<&GridCell> {
        let mut ret_cells: Vec<&GridCell> = Vec::new();
        for cell in self.cells.iter().skip(col_num).step_by(self.num_cols) {
            ret_cells.push(cell);
        }
        ret_cells
    }
    fn col_mut(&mut self, col_num: usize) -> Vec<&mut GridCell> {
        let mut ret_cells: Vec<&mut GridCell> = Vec::new();
        for cell in self.cells.iter_mut().skip(col_num).step_by(self.num_cols) {
            ret_cells.push(cell);
        }
        ret_cells
    }

    fn get_coord(&self, x: usize, y: usize) -> Option<&GridCell> {
        if x >= self.num_cols || y >= self.num_rows {
            None
        } else {
            self.cells.get((y * self.num_cols) + x)
        }
    }

    fn get_coord_mut(&mut self, x: usize, y: usize) -> Option<&mut GridCell> {
        self.cells.get_mut((y * self.num_cols) + x)
    }

    #[allow(dead_code)]
    fn row_iter(&self, direction: Direction) -> RowIter<'_> {
        RowIter {
            grid: self,
            direction,
            next_x: 0,
            next_y: 0,
        }
    }

    fn row_iter_at(&self, x: usize, y: usize, direction: Direction) -> RowIter<'_> {
        RowIter {
            grid: self,
            direction,
            next_x: x as isize,
            next_y: y as isize,
        }
    }
}

impl fmt::Display for Grid {
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

enum Direction {
    North,
    South,
    East,
    West,
}

struct RowIter<'a> {
    grid: &'a Grid,
    direction: Direction,
    next_x: isize,
    next_y: isize,
}

impl<'a> Iterator for RowIter<'a> {
    type Item = &'a GridCell;

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

fn parser(s: String) -> Grid {
    let mut num_rows = 0;
    let mut num_cols = 0;
    let mut cells: Vec<GridCell> = vec![];
    for row in s.split('\n') {
        cells.extend(row.chars().map(|v| GridCell {
            value: v.to_digit(10).unwrap() as i32,
            is_visible: false,
            ..Default::default()
        }));
        if num_cols == 0 {
            num_cols = cells.len();
        }
        num_rows += 1;
    }
    Grid {
        cells,
        num_rows,
        num_cols,
    }
}

fn solve(grid: &mut Grid) -> usize {
    // Visible: iff all trees between it and an edge are < it
    for row_ndx in 1..grid.num_rows - 1 {
        let row = grid.row_mut(row_ndx);
        let mut left_max_height = -1;
        let mut right_max_height = -1;
        for cell in row.iter_mut() {
            if cell.value > left_max_height {
                cell.is_visible = true;
                left_max_height = cell.value;
            }
        }
        row.reverse();
        for cell in row.iter_mut() {
            if cell.value > right_max_height {
                cell.is_visible = true;
                right_max_height = cell.value;
            }
        }
        row.reverse(); // :eyes:
    }
    for col_ndx in 0..grid.num_cols {
        let col = grid.col_mut(col_ndx);
        let mut left_max_height = -1;
        let mut right_max_height = -1;
        for cell in col.into_iter() {
            if cell.value > left_max_height {
                cell.is_visible = true;
                left_max_height = cell.value;
            }
        }
        let mut col = grid.col_mut(col_ndx);
        col.reverse();
        for cell in col.into_iter() {
            if cell.value > right_max_height {
                cell.is_visible = true;
                right_max_height = cell.value;
            }
        }
    }
    println!("{grid}");
    grid.cells.iter().filter(|c| c.is_visible).count()
}

fn solve2(grid: &mut Grid) -> usize {
    // Edge trees have a 0 viewing score (0*X = 0) -- skip those
    // For each tree, calculate its viewing distance
    //  A(T) = N(T) * E(T) * W(T) * S(T)
    let mut max_value = 0;
    for y in 1..grid.num_rows - 1 {
        for x in 1..grid.num_cols - 1 {
            let cell = grid.get_coord(x, y).unwrap();
            if cell.is_visible {
                let north_value = grid
                    .row_iter_at(x, y, Direction::North)
                    .skip(1)
                    .take_until(|&c| c.value >= cell.value)
                    .count();
                let south_value = grid
                    .row_iter_at(x, y, Direction::South)
                    .skip(1)
                    .take_until(|&c| c.value >= cell.value)
                    .count();
                let east_value = grid
                    .row_iter_at(x, y, Direction::East)
                    .skip(1)
                    .take_until(|&c| c.value >= cell.value)
                    .count();
                let west_value = grid
                    .row_iter_at(x, y, Direction::West)
                    .skip(1)
                    .take_until(|&c| c.value >= cell.value)
                    .count();
                let cell = grid.get_coord_mut(y, x).unwrap();
                cell.viewing_score = north_value * south_value * east_value * west_value;
                if cell.viewing_score > max_value {
                    max_value = cell.viewing_score;
                }
            }
        }
    }
    max_value
}

fn main() {
    let mut data = utils::load_puzzle_data(8, parser);
    let total_size = solve(&mut data);
    println!("Solution 1: The total number of visible trees is {total_size} trees.");

    // Tallest trees have been marked now
    let visible_score = solve2(&mut data);
    println!("Solution 2: The tree with the best view has a score of {visible_score}.");
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let mut test_data = utils::load_puzzle_test(8, parser);
        let solution = solve(&mut test_data);
        assert_eq!(solution, 21);
    }

    #[test]
    fn test_puzzle2() {
        let mut test_data = utils::load_puzzle_test(8, parser);
        // To get the is_visible set
        solve(&mut test_data);
        let solution = solve2(&mut test_data);
        assert_eq!(solution, 8);
    }
}
