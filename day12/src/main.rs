extern crate pathfinding;

use utils::StaticGrid;

use pathfinding::prelude::{astar, astar_bag};

struct Board {
    grid: StaticGrid<Pos>,
    start_pos: Pos,
    end_pos: Pos,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Default, Copy)]
struct Pos {
    x: usize,
    y: usize,
    height: usize,
}

impl Pos {
    fn distance(&self, other: &Pos) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn parser(s: String) -> Board {
    let mut start_pos = None;
    let mut end_pos = None;
    let mut num_rows = 0;
    let mut num_cols = 0;
    let mut cells: Vec<Pos> = vec![];
    for (row_ndx, row) in s.split('\n').enumerate() {
        cells.extend(row.bytes().enumerate().map(|(c_ndx, v)| {
            let height = match v {
                83 => 1_usize,
                69 => 26_usize,
                e => (e - 96) as usize, // 1-26
            };
            let p = Pos {
                x: c_ndx,
                y: row_ndx,
                height,
            };
            match v {
                83 => start_pos = Some(p),
                69 => end_pos = Some(p),
                _ => (),
            };
            p
        }));
        if num_cols == 0 {
            num_cols = cells.len();
        }
        num_rows += 1;
    }
    Board {
        grid: StaticGrid {
            cells,
            num_rows,
            num_cols,
        },
        start_pos: start_pos.unwrap(),
        end_pos: end_pos.unwrap(),
    }
}

fn solve(board: &Board) -> usize {
    let goal: Pos = board.end_pos;
    let result = astar(
        &board.start_pos,
        |p| {
            let north: Vec<&Pos> = board
                .grid
                .direction_iter_at(p.x, p.y, utils::Direction::North)
                .skip(1)
                .take(1)
                .filter(|&c| c.height <= p.height + 1)
                .collect();
            let south: Vec<&Pos> = board
                .grid
                .direction_iter_at(p.x, p.y, utils::Direction::South)
                .skip(1)
                .take(1)
                .filter(|&c| c.height <= p.height + 1)
                .collect();
            let east: Vec<&Pos> = board
                .grid
                .direction_iter_at(p.x, p.y, utils::Direction::East)
                .skip(1)
                .take(1)
                .filter(|&c| c.height <= p.height + 1)
                .collect();
            let west: Vec<&Pos> = board
                .grid
                .direction_iter_at(p.x, p.y, utils::Direction::West)
                .skip(1)
                .take(1)
                .filter(|&c| c.height <= p.height + 1)
                .collect();
            let v = vec![north.iter(), south.iter(), east.iter(), west.iter()];
            let b: Vec<(Pos, usize)> = v
                .iter()
                .flat_map(|it| it.clone())
                .map(|&f| (*f, 1_usize))
                .collect();
            b
        },
        |p| p.distance(&goal) / 3,
        |p| *p == goal,
    );
    result.expect("No path found!").1
}

fn solve2(board: &Board) -> usize {
    // This time, start at the End, and find all paths to 'a', use shortest path
    let goal: Pos = board.end_pos;
    let results = astar_bag(
        &board.end_pos,
        |p| {
            let north: Vec<&Pos> = board
                .grid
                .direction_iter_at(p.x, p.y, utils::Direction::North)
                .skip(1)
                .take(1)
                .filter(|&c| p.height <= c.height + 1)
                .collect();
            let south: Vec<&Pos> = board
                .grid
                .direction_iter_at(p.x, p.y, utils::Direction::South)
                .skip(1)
                .take(1)
                .filter(|&c| p.height <= c.height + 1)
                .collect();
            let east: Vec<&Pos> = board
                .grid
                .direction_iter_at(p.x, p.y, utils::Direction::East)
                .skip(1)
                .take(1)
                .filter(|&c| p.height <= c.height + 1)
                .collect();
            let west: Vec<&Pos> = board
                .grid
                .direction_iter_at(p.x, p.y, utils::Direction::West)
                .skip(1)
                .take(1)
                .filter(|&c| p.height <= c.height + 1)
                .collect();
            let v = vec![north.iter(), south.iter(), east.iter(), west.iter()];
            let b: Vec<(Pos, usize)> = v
                .iter()
                .flat_map(|it| it.clone())
                .map(|&f| (*f, 1_usize))
                .collect();
            b
        },
        |p| p.distance(&goal) / 3,
        |p| p.height == 1,
    );
    results.expect("No shortest path").1
}

fn main() {
    let board = utils::load_puzzle_data(12, parser);
    let moves = solve(&board);
    println!("Solution 1: It took {moves} moves to get to the end!");

    let moves = solve2(&board);
    println!(
        "Solution 2: It took {moves} moves to get to the end, when starting from any lowest point!"
    );
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(12, parser);
        let solution = solve(&test_data);
        assert_eq!(solution, 31);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(12, parser);
        let solution = solve2(&test_data);
        assert_eq!(solution, 29);
    }
}
