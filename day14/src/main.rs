use std::fmt::Display;

use utils::{Direction, DynamicGrid, Grid, Point, Growable};

struct Cave<CellType> {
    grid: DynamicGrid<CellType>,
}

#[derive(Clone, Default)]
struct CaveSpace {
    is_rock: bool,
    is_sand: bool,
}

impl CaveSpace {
    pub fn is_blocked(&self) -> bool {
        self.is_rock || self.is_sand
    }
}

impl<CellType> Cave<CellType> {
    pub fn new(grid: DynamicGrid<CellType>) -> Self {
        Cave { grid }
    }
}

impl Display for CaveSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>1}",
            if self.is_rock {
                '#'
            } else if self.is_sand {
                'o'
            } else {
                '.'
            }
        )
    }
}

fn parser(s: String) -> Cave<CaveSpace> {
    let start = Point { x: 500, y: 0 };
    let grid = DynamicGrid::<CaveSpace>::new(start.x, start.y);
    let mut cave = Cave::<CaveSpace>::new(grid);

    for path in s.split('\n') {
        let lines: Vec<_> = path.split(" -> ").collect();
        for line in lines.as_slice().windows(2) {
            // 498,4 -> 498,6 -> 496,6
            // [  Horizontal ]
            //          [ Vertical    ]
            let (x0_str, y0_str) = line.first().unwrap().split_once(',').unwrap();
            let (x_start, y_start): (isize, isize) =
                (x0_str.parse().unwrap(), y0_str.parse().unwrap());
            let (x1_str, y1_str) = line.get(1).unwrap().split_once(',').unwrap();
            let (x_end, y_end): (isize, isize) = (x1_str.parse().unwrap(), y1_str.parse().unwrap());

            let mut vals = if x_start == x_end {
                [y_start, y_end]
            } else {
                [x_start, x_end]
            };
            vals.sort();
            let val_start = *vals.first().unwrap();
            let val_end = *vals.get(1).unwrap();
            for val in val_start..=val_end {
                let (x, y) = if x_start == x_end {
                    (x_start, val)
                } else {
                    (val, y_start)
                };
                let c = cave.grid.get_cell_or_add_mut(x, y);
                c.is_rock = true;
            }
        }
    }
    cave
}

fn solve(cave: &mut Cave<CaveSpace>, has_abyss: bool) -> usize {
    let mut count = 0;
    let mut new_sand = true;
    let mut x: isize = 500;
    let mut y: isize = 0;
    loop {
        if new_sand {
            count += 1;
        }
        // New Piece of sand drops
        (x, y) = match new_sand {
            true => (500, 0),
            false => (x, y),
        };
        let mut sand_lost_to_abyss = false;
        loop {
            let start_x = x;
            let start_y = y;
            let mut iter = cave
                .grid
                .direction_iter(start_x, start_y, Direction::South)
                .enumerate()
                .peekable();
            while let Some((ndx, _)) = iter.next() {
                // Sand tries to fall straight down
                y = start_y + ndx as isize + 1;
                if let Some((_, c)) = iter.peek() {
                    // If the tile immediately below is blocked (by rock or sand),
                    if c.is_blocked() {
                        // the unit of sand attempts to instead move diagonally one step down and to the left
                        x -= 1;
                        let Some(c) = cave.grid.get_cell(x, y) else {
                            // Into the abyss!
                            sand_lost_to_abyss = true;
                            break;
                        };
                        // If that tile is blocked
                        if c.is_blocked() {
                            // the unit of sand attempts to instead move diagonally one step down and to the right
                            x += 2;
                            let Some(c) = cave.grid.get_cell(x, y) else {
                                // Into the abyss!
                                sand_lost_to_abyss = true;
                                break;
                            };
                            // If that tile is blocked
                            if c.is_blocked() {
                                // Stays where it is
                                x -= 1;
                                y -= 1;
                            }
                        }
                    }
                } else {
                    // Fell off the edge of the world (no more known spaces below)
                    sand_lost_to_abyss = true;
                }
                if x != start_x || (y == start_y + ndx as isize) {
                    // Moved columns or sand did not move, restart iterator
                    break;
                }
            }
            if sand_lost_to_abyss {
                break;
            }
            if x == start_x {
                // Did not move, put it to rest
                let b = cave.grid.get_cell_mut(x, y).unwrap();
                b.is_sand = true;
                break;
            }
        }
        new_sand = true;
        if !has_abyss && sand_lost_to_abyss {
            // Add column and continue falling
            let last = cave.grid.last_cell_coord();
            let b = cave.grid.get_cell_or_add_mut(x, last.1);
            b.is_rock = true;
            new_sand = false;
            if y > last.1 {
                y = last.1 - 1;
                if x > 500 {
                    x -= 1;
                } else {
                    x += 1;
                }
            }
        } else if sand_lost_to_abyss {
            // Floated off -- lost one, and done
            count -= 1;
            break;
        } else if y == 0 {
            // Filled them all!
            break;
        }
    }
    count
}

fn solve2(cave: &mut Cave<CaveSpace>) -> usize {
    // There's actually a floor!
    // Add it...
    let bottom = cave.grid.last_cell_coord();
    let y = bottom.1 + 2;
    for x in cave.grid.first_cell_coord().0..=bottom.0 {
        let c = cave.grid.get_cell_or_add_mut(x, y);
        c.is_rock = true;
    }

    solve(cave, false)
}

fn main() {
    let mut cave = utils::load_puzzle_data(14, parser);
    let mut pieces_of_sand = solve(&mut cave, true);
    println!("Solution 1: There were {pieces_of_sand} pieces of sand that came to rest.");

    pieces_of_sand += solve2(&mut cave);
    print!("{}",cave.grid);
    println!("Solution 2: There were {pieces_of_sand} pieces of sand that came to rest.");
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let mut test_data = utils::load_puzzle_test(14, parser);
        let solution = solve(&mut test_data, true);
        assert_eq!(solution, 24);
    }

    #[test]
    fn test_puzzle2() {
        let mut test_data = utils::load_puzzle_test(14, parser);
        let mut solution = solve(&mut test_data, true);
        assert_eq!(solution, 24);
        print!("{}",test_data.grid);

        solution += solve2(&mut test_data);
        assert_eq!(solution, 93);
        print!("{}",test_data.grid);
    }
}
