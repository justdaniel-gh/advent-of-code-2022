#![feature(iter_array_chunks)]

use regex::Regex;
use utils::SliceExt;

struct MovementInstruction {
    count: usize,
    from_ndx: usize,
    to_ndx: usize,
}

struct PuzzleInput {
    stacks: Vec<Vec<char>>,
    instructions: Vec<MovementInstruction>,
}

fn parser(s: String) -> PuzzleInput {
    let (cargo_map, instructions) = s.split_once("\n\n").unwrap();
    let lines: Vec<Vec<char>> = cargo_map
        .split_inclusive('\n')
        .map(|line| {
            line.chars()
                .array_chunks::<4>()
                .map(|chunk| *chunk.get(1).unwrap())
                .collect()
        })
        .collect();
    // [[' ', 'D', ' '],
    //  ['N', 'C', ' '],
    //  ['Z', 'M', 'P'],
    let mut stacks = vec![vec![]; lines[0].len()];
    for line in lines {
        for (ndx, item) in line.into_iter().enumerate() {
            if item.is_ascii_uppercase() {
                stacks[ndx].insert(0, item);
            }
        }
    }
    // [['N', 'Z'], ['D', 'C', 'M'], ['P']]
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    // Now instructions...
    PuzzleInput {
        stacks,
        instructions: instructions
            .split('\n')
            .map(|s| {
                let caps = re.captures(s).unwrap();
                MovementInstruction {
                    count: caps.get(1).unwrap().as_str().parse().unwrap(),
                    from_ndx: caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                    to_ndx: caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
                }
            })
            .collect(),
    }
}

fn solve(mut cargo: PuzzleInput) -> String {
    for instruction in cargo.instructions {
        let (from, to) = cargo
            .stacks
            .get_two_mut(instruction.from_ndx, instruction.to_ndx);
        to.extend(from.drain((from.len() - instruction.count)..).rev());
    }
    cargo
        .stacks
        .into_iter()
        .map(|vc| vc.last().unwrap().to_owned())
        .collect()
}

fn solve2(mut cargo: PuzzleInput) -> String {
    for instruction in cargo.instructions {
        let (from, to) = cargo
            .stacks
            .get_two_mut(instruction.from_ndx, instruction.to_ndx);
        to.extend(from.drain((from.len() - instruction.count)..));
    }
    cargo
        .stacks
        .into_iter()
        .map(|vc| vc.last().unwrap().to_owned())
        .collect()
}

fn main() {
    let cargo = utils::load_puzzle_data(5, parser);
    let top_of_cargo = solve(cargo);
    println!("Solution 1: Items on top of the stacks: {top_of_cargo}",);

    let cargo = utils::load_puzzle_data(5, parser);
    let top_of_cargo = solve2(cargo);
    println!("Solution 2: Items on top of the stacks: {top_of_cargo}",);
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(5, parser);
        let solution = solve(test_data);
        assert_eq!(solution, "CMZ".to_string());
    }
    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(5, parser);
        let solution = solve2(test_data);
        assert_eq!(solution, "MCD".to_string());
    }
}