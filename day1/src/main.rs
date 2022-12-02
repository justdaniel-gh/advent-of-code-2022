use utils;

struct Elf {
    id: usize,
    calories: u32,
}

fn parser(s: String) -> Vec<Elf> {
    s.split("\n\n")
        .enumerate()
        .map(|(n, g)| Elf {
            id: n + 1,
            calories: g
                .split("\n")
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>(),
        })
        .collect()
}

fn solve(elves: Vec<Elf>) -> Elf {
    elves.into_iter().max_by_key(|e| e.calories).unwrap()
}

fn solve2(mut elves: Vec<Elf>) -> u32 {
    elves.sort_by_key(|e| e.calories);
    elves.reverse();
    elves.drain(..3).collect::<Vec<Elf>>().iter().fold(0,|a: u32, e: &Elf| a + e.calories)
}

fn main() {
    let elves = utils::load_puzzle_data(1, parser);
    let prepared_elf = solve(elves);
    println!(
        "Solution 1: {} is carrying {} calories worth of food.",
        prepared_elf.id, prepared_elf.calories
    );

    let elves = utils::load_puzzle_data(1, parser);
    let top_three_calories = solve2(elves);
    println!(
        "Solution 2: Top three elves are carrying {} calories worth of food.",
        top_three_calories
    );
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(1, parser);
        let solution = solve(test_data);
        assert_eq!(solution.id, 4);
        assert_eq!(solution.calories, 24000);
    }
    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(1, parser);
        let solution = solve2(test_data);
        assert_eq!(solution, 45000);
    }
}
