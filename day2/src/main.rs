
use utils;

struct RPSRound {
    me: char,
    them: char,
}

enum Result {
    Win(u32),
    Lose(u32),
    Tie(u32),
}

fn parser(s: String) -> Vec<RPSRound> {
    s.split("\n")
        .map(|g| {
            let plays: Vec<&str> = g.splitn(2, ' ').collect();
            RPSRound {
                them: plays[0].chars().nth(0).unwrap(),
                me: plays[1].chars().nth(0).unwrap(),
            }
        })
        .collect()
}

fn solve(plays: Vec<RPSRound>) -> u32 {
    // A/X = Rock, B/Y = Paper, C/Z = Scissors
    // 1 for Rock, 2 for Paper, 3 for Scissors
    // 0 for lose, 3 for tie, 6 for win
    let scores: Vec<u32> = plays
        .iter()
        .map(|p| {
            let result = match p.me {
                'X' => match p.them {
                    'A' => Result::Tie(1),
                    'B' => Result::Lose(1),
                    'C' => Result::Win(1),
                    _ => panic!("Invalid Input"),
                },
                'Y' => match p.them {
                    'A' => Result::Win(2),
                    'B' => Result::Tie(2),
                    'C' => Result::Lose(2),
                    _ => panic!("Invalid Input"),
                },
                'Z' => match p.them {
                    'A' => Result::Lose(3),
                    'B' => Result::Win(3),
                    'C' => Result::Tie(3),
                    _ => panic!("Invalid Input"),
                },
                _ => panic!("Invalid Input"),
            };
            match result {
                Result::Win(e) => 6 + e,
                Result::Lose(e) => 0 + e,
                Result::Tie(e) => 3 + e,
            }
        })
        .collect();
    scores.iter().sum()
}

fn solve2(plays: Vec<RPSRound>) -> u32 {
    // A = Rock, B = Paper, C = Scissors
    // 1 for Rock, 2 for Paper, 3 for Scissors
    // X for lose, Y for tie, Z for win
    let scores: Vec<u32> = plays
        .iter()
        .map(|p| {
            let result = match p.them {
                'A' => match p.me {
                    'X' => Result::Lose(3),
                    'Y' => Result::Tie(1),
                    'Z' => Result::Win(2),
                    _ => panic!("Invalid Input"),
                },
                'B' => match p.me {
                    'X' => Result::Lose(1),
                    'Y' => Result::Tie(2),
                    'Z' => Result::Win(3),
                    _ => panic!("Invalid Input"),
                },
                'C' => match p.me {
                    'X' => Result::Lose(2),
                    'Y' => Result::Tie(3),
                    'Z' => Result::Win(1),
                    _ => panic!("Invalid Input"),
                },
                _ => panic!("Invalid Input"),
            };
            match result {
                Result::Win(e) => 6 + e,
                Result::Lose(e) => 0 + e,
                Result::Tie(e) => 3 + e,
            }
        })
        .collect();
    scores.iter().sum()
}

fn main() {
    let plays = utils::load_puzzle_data(2, parser);
    let score = solve(plays);
    println!("Solution 1: Total score: {}", score);
    
    let plays = utils::load_puzzle_data(2, parser);
    let score = solve2(plays);
    println!("Solution 2: Total score: {}", score);
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(2, parser);
        let solution = solve(test_data);
        assert_eq!(solution, 15);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(2, parser);
        let solution = solve2(test_data);
        assert_eq!(solution, 12);
    }
}
