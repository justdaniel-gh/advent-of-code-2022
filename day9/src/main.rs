use regex::Regex;
use std::{fmt, ops::Sub};

use vector2d::Vector2D;

use utils::{Direction, DynamicGrid, SliceExt};

struct Instruction {
    direction: Direction,
    distance: usize,
}

struct Rope {
    knots: Vec<Vector2D<isize>>,
    grid: DynamicGrid<RopeCell>,
}

type Point = Vector2D<isize>;

impl Rope {
    pub fn new(num_knots: usize) -> Self {
        let mut g = DynamicGrid::<RopeCell>::new();
        let c = g.get_cell_at_mut(0, 0);
        c.tail_visited = true;
        Rope {
            knots: vec![Vector2D { x: 0, y: 0 }; num_knots],
            grid: g
        }
    }

    /// return direction the head is in from tail
    fn get_direction(knot_a: &Point, knot_b: &Point) -> Option<Direction> {
        let dir = match knot_a.y.cmp(&knot_b.y) {
            std::cmp::Ordering::Less => Some(Direction::North),
            std::cmp::Ordering::Greater => Some(Direction::South),
            std::cmp::Ordering::Equal => None,
        };
        match knot_a.x.cmp(&knot_b.x) {
            std::cmp::Ordering::Less => {
                match dir {
                    Some(Direction::North) => Some(Direction::NorthWest),
                    Some(Direction::South) => Some(Direction::SouthWest),
                    _ => Some(Direction::West)
                }
            },
            std::cmp::Ordering::Greater => {
                match dir {
                    Some(Direction::North) => Some(Direction::NorthEast),
                    Some(Direction::South) => Some(Direction::SouthEast),
                    _ => Some(Direction::East)
                }
            },
            std::cmp::Ordering::Equal => dir,
        }
    }

    pub fn move_head(&mut self, direction: &Direction, amount: usize) {
        let amount = amount as isize;
        for _ in 0..amount {
            let head = self.knots.get_mut(0).unwrap();
            match direction {
                Direction::North => {
                    head.y -= 1;
                },
                Direction::South => {
                    head.y += 1;
                },
                Direction::East => {
                    head.x += 1;
                },
                Direction::West => {
                    head.x -= 1;
                },
                Direction::NorthEast => {
                    head.y -= 1;
                    head.x += 1;
                },
                Direction::NorthWest => {
                    head.y -= 1;
                    head.x -= 1;
                },
                Direction::SouthEast => {
                    head.y += 1;
                    head.x += 1;
                },
                Direction::SouthWest => {
                    head.y += 1;
                    head.x -= 1;
                }
            }
            let c = self.grid.get_cell_at_mut(head.x, head.y);
            c.head_visited = true;
            for ndx in 0..self.knots.len()-1 {
                self.move_knot(ndx+1, ndx);
            }
            let c = self.grid.get_cell_at_mut(self.knots.last().unwrap().x, self.knots.last().unwrap().y);
            c.tail_visited = true;
        }
    }

    /// Returns if knot traveled
    pub fn move_knot(&mut self, knot_ndx: usize, parent_knot_ndx: usize) -> bool {
        // Follow the head, like so...
        /*
            If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough:

            .....    .....    .....
            .TH.. -> .T.H. -> ..TH.
            .....    .....    .....

            ...    ...    ...
            .T.    .T.    ...
            .H. -> ... -> .T.
            ...    .H.    .H.
            ...    ...    ...

            Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:

            .....    .....    .....
            .....    ..H..    ..H..
            ..H.. -> ..... -> ..T..
            .T...    .T...    .....
            .....    .....    .....

            .....    .....    .....
            .....    .....    .....
            ..H.. -> ...H. -> ..TH.
            .T...    .T...    .....
            .....    .....    .....
        */
        let (knot, parent_knot) = self.knots.get_two_mut(knot_ndx, parent_knot_ndx);
        if parent_knot.sub(*knot).length_squared() < 4 { // 4 = 2**2
            return false;
        }
        match Rope::get_direction(&*parent_knot, &*knot) {
            Some(Direction::North) => {
                knot.y -= 1;
                true
            },
            Some(Direction::South) => {
                knot.y += 1;
                true
            },
            Some(Direction::East) => {
                knot.x += 1;
                true
            },
            Some(Direction::West) => {
                knot.x -= 1;
                true
            },
            Some(Direction::NorthEast) => {
                knot.y -= 1;
                knot.x += 1;
                true
            },
            Some(Direction::NorthWest) => {
                knot.y -= 1;
                knot.x -= 1;
                true
            },
            Some(Direction::SouthEast) => {
                knot.y += 1;
                knot.x += 1;
                true
            },
            Some(Direction::SouthWest) => {
                knot.y += 1;
                knot.x -= 1;
                true
            },
            None => false,
        }
    }
}

#[derive(Default, Clone)]
struct RopeCell {
    tail_visited: bool,
    head_visited: bool,
}

impl fmt::Display for RopeCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>1} ", if self.tail_visited { '#' } else { '.' })
    }
}

fn parser(s: String) -> Vec<Instruction> {
    let re = Regex::new(r"(U|D|L|R) (\d+)").unwrap();
    s.split('\n')
        .into_iter()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let direction = match caps.get(1).unwrap().as_str() {
                "U" => Direction::North,
                "D" => Direction::South,
                "L" => Direction::West,
                "R" => Direction::East,
                _ => panic!("This should not happen..."),
            };
            Instruction {
                direction,
                distance: caps.get(2).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

fn solve(instructions: &Vec<Instruction>, num_knots: usize) -> usize {
    let mut rope = Rope::new(num_knots);
    for instruction in instructions {
        rope.move_head(&instruction.direction, instruction.distance);
    }
    println!("{}", rope.grid);
    rope.grid.cell_iter().filter(|&c| c.tail_visited).count()
}

fn main() {
    let instructions = utils::load_puzzle_data(9, parser);
    let total_visited = solve(&instructions, 2);
    println!("Solution 1: Visited {total_visited} spaces.");

    let total_visited = solve(&instructions, 10);
    println!("Solution 2: Visited {total_visited} spaces.");
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(9, parser);
        let solution = solve(&test_data, 2);
        assert_eq!(solution, 13);
    }

    #[test]
    fn test_puzzle2() {
        let test_input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20".to_string();
        let test_data = parser(test_input);
        let solution = solve(&test_data, 10);
        assert_eq!(solution, 36);
    }
}
