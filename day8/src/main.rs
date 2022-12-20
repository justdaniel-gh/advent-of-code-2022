use std::fmt;

use take_until::TakeUntilExt;
use utils::{StaticGrid, Direction, Grid};

#[derive(Default, Clone)]
struct TreeCell {
    height: i32,
    is_visible: bool,
    viewing_score: usize,
}

impl fmt::Display for TreeCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>1}{} ",
            self.height,
            if self.is_visible { '*' } else { ' ' }
        )
    }
}

fn parser(s: String) -> StaticGrid<TreeCell> {
    let mut num_rows = 0;
    let mut num_cols = 0;
    let mut cells: Vec<TreeCell> = vec![];
    for row in s.split('\n') {
        cells.extend(row.chars().map(|v| TreeCell {
            height: v.to_digit(10).unwrap() as i32,
            is_visible: false,
            ..Default::default()
        }));
        if num_cols == 0 {
            num_cols = cells.len();
        }
        num_rows += 1;
    }
    StaticGrid {
        cells,
        num_rows,
        num_cols,
    }
}

fn solve(grid: &mut StaticGrid<TreeCell>) -> usize {
    // Visible: iff all trees between it and an edge are < it
    for row_ndx in 1..grid.num_rows - 1 {
        let row = grid.row_mut(row_ndx);
        let mut left_max_height = -1;
        let mut right_max_height = -1;
        for cell in row.iter_mut() {
            if cell.height > left_max_height {
                cell.is_visible = true;
                left_max_height = cell.height;
            }
        }
        row.reverse();
        for cell in row.iter_mut() {
            if cell.height > right_max_height {
                cell.is_visible = true;
                right_max_height = cell.height;
            }
        }
        row.reverse(); // :eyes:
    }
    for col_ndx in 0..grid.num_cols {
        let col = grid.col_mut(col_ndx);
        let mut left_max_height = -1;
        let mut right_max_height = -1;
        for cell in col.into_iter() {
            if cell.height > left_max_height {
                cell.is_visible = true;
                left_max_height = cell.height;
            }
        }
        let mut col = grid.col_mut(col_ndx);
        col.reverse();
        for cell in col.into_iter() {
            if cell.height > right_max_height {
                cell.is_visible = true;
                right_max_height = cell.height;
            }
        }
    }
    println!("{grid}");
    grid.cell_iter().filter(|c| c.is_visible).count()
}

fn solve2(grid: &mut StaticGrid<TreeCell>) -> usize {
    // Edge trees have a 0 viewing score (0*X = 0) -- skip those
    // For each tree, calculate its viewing distance
    //  A(T) = N(T) * E(T) * W(T) * S(T)
    let mut max_value = 0;
    for y in 1..grid.num_rows as isize - 1 {
        for x in 1..grid.num_cols as isize - 1 {
            let cell = grid.get_cell(x, y).unwrap();
            if cell.is_visible {
                let north_value = grid
                    .direction_iter_at(x, y, Direction::North)
                    .skip(1)
                    .take_until(|&c| c.height >= cell.height)
                    .count();
                let south_value = grid
                    .direction_iter_at(x, y, Direction::South)
                    .skip(1)
                    .take_until(|&c| c.height >= cell.height)
                    .count();
                let east_value = grid
                    .direction_iter_at(x, y, Direction::East)
                    .skip(1)
                    .take_until(|&c| c.height >= cell.height)
                    .count();
                let west_value = grid
                    .direction_iter_at(x, y, Direction::West)
                    .skip(1)
                    .take_until(|&c| c.height >= cell.height)
                    .count();
                let cell = grid.get_cell_mut(y, x).unwrap();
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
