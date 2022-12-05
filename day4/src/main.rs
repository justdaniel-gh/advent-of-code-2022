use std::ops::RangeInclusive;

struct CleaningPair {
    first: RangeInclusive<usize>,
    second: RangeInclusive<usize>,
}

fn parser(s: String) -> Vec<CleaningPair> {
    s.split('\n')
        .map(|g| {
            // Map a pair
            let (first, second) = g.split_once(',').unwrap();
            let (first_start, first_end) = first.split_once('-').unwrap();
            let (second_start, second_end) = second.split_once('-').unwrap();
            CleaningPair {
                first: (first_start.parse::<usize>().unwrap()
                    ..=first_end.parse::<usize>().unwrap()),
                second: (second_start.parse::<usize>().unwrap()
                    ..=second_end.parse::<usize>().unwrap()),
            }
        })
        .collect()
}

fn solve(pairs: Vec<CleaningPair>) -> usize {
    // Fully overlapping ranges
    let count_overlapping_ranges: usize = pairs
        .iter()
        .filter(|&pair| {
            (pair.first.contains(pair.second.start()) && pair.first.contains(pair.second.end()))
                || (pair.second.contains(pair.first.start())
                    && pair.second.contains(pair.first.end()))
        })
        .collect::<Vec<&CleaningPair>>()
        .len();
    count_overlapping_ranges
}

fn solve2(pairs: Vec<CleaningPair>) -> usize {
    // Partial overlapping ranges
    let count_overlapping_ranges: usize = pairs
        .iter()
        .filter(|&pair| {
            pair.first.contains(pair.second.start())
                || pair.first.contains(pair.second.end())
                || pair.second.contains(pair.first.start())
                || pair.second.contains(pair.first.end())
        })
        .collect::<Vec<&CleaningPair>>()
        .len();
    count_overlapping_ranges
}

fn main() {
    let pairs = utils::load_puzzle_data(4, parser);
    let num_overlapping_pairs = solve(pairs);
    println!("Solution 1: Total num overlapping pairs: {num_overlapping_pairs}",);

    let pairs = utils::load_puzzle_data(4, parser);
    let num_partial_overlapping_pairs = solve2(pairs);
    println!("Solution 1: Total num partial overlapping pairs: {num_partial_overlapping_pairs}");
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(4, parser);
        let solution = solve(test_data);
        assert_eq!(solution, 2);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(4, parser);
        let solution = solve2(test_data);
        assert_eq!(solution, 4);
    }
}
