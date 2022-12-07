#![feature(iter_array_chunks)]

fn parser(s: String) -> String {
    // Nothing to parse
    s
}

fn solve(data_stream: String, marker_len: usize) -> usize {
    data_stream
        .as_bytes()
        .windows(marker_len)
        .position(|s| !(1..s.len()).any(|n| s[n..].contains(&s[n - 1])))
        .unwrap() + marker_len
}

fn main() {
    let data = utils::load_puzzle_data(6, parser);
    let marker_ndx = solve(data, 4);
    println!("Solution 1: First packet marker comes after {marker_ndx} chars received.",);

    let data = utils::load_puzzle_data(6, parser);
    let marker_ndx = solve(data, 14);
    println!("Solution 2: First message marker comes after {marker_ndx} chars received.",);
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(6, parser);
        let solution = solve(test_data, 4);
        assert_eq!(solution, 10);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(6, parser);
        let solution = solve(test_data, 14);
        assert_eq!(solution, 29);
    }
}
