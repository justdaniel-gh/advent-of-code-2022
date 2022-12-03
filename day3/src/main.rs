#![feature(iter_array_chunks)]

use utils;

struct Sack {
    first: Vec<char>,
    second: Vec<char>,
}

struct Group {
    first: Vec<char>,
    second: Vec<char>,
    third: Vec<char>,
}

fn parser(s: String) -> Vec<Sack> {
    s.split("\n")
        .map(|g| {
            let (first, second) = g.split_at(g.len() / 2);
            Sack {
                first: first.chars().collect(),
                second: second.chars().collect(),
            }
        })
        .collect()
}

fn parser2(s: String) -> Vec<Group> {
    s.split("\n")
        .array_chunks::<3>()
        .map(|g| Group {
            first: g.get(0).unwrap().chars().collect(),
            second: g.get(1).unwrap().chars().collect(),
            third: g.get(2).unwrap().chars().collect(),
        })
        .collect()
}

fn solve(sacks: Vec<Sack>) -> u32 {
    // a-z = 1-26
    // A-Z = 27-52
    let matching_items: Vec<u32> = sacks
        .iter()
        .map(|sack| {
            let shared = sack
                .first
                .iter()
                .find(|&fc| sack.second.contains(fc))
                .unwrap();
            let priority = match shared.is_ascii_lowercase() {
                true => *shared as u32 - 96,
                false => *shared as u32 - 64 + 26,
            };
            priority
        })
        .collect();
    matching_items.iter().sum()
}

fn solve2(groups: Vec<Group>) -> u32 {
    // a-z = 1-26
    // A-Z = 27-52
    let matching_items: Vec<u32> = groups
        .iter()
        .map(|group| {
            let mut shared_first_second: Vec<&char> = group
                .first
                .iter()
                .filter(|&fc| group.second.contains(fc))
                .collect();
            shared_first_second.sort();
            shared_first_second.dedup();
            let shared_third = shared_first_second
                .into_iter()
                .find(|&&fc| group.third.contains(&fc))
                .unwrap();
            let priority = match shared_third.is_ascii_lowercase() {
                true => *shared_third as u32 - 96,
                false => *shared_third as u32 - 64 + 26,
            };
            priority
        })
        .collect();
    matching_items.iter().sum()
}

fn main() {
    let sacks = utils::load_puzzle_data(3, parser);
    let priority_sum = solve(sacks);
    println!("Solution 1: Total priority: {}", priority_sum);

    let groups = utils::load_puzzle_data(3, parser2);
    let priority_sum = solve2(groups);
    println!("Solution 1: Total priorities: {}", priority_sum);
}

#[cfg(test)]
mod tests {
    use crate::{parser, parser2, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(3, parser);
        let solution = solve(test_data);
        assert_eq!(solution, 157);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(3, parser2);
        let solution = solve2(test_data);
        assert_eq!(solution, 70);
    }
}
