use std::fmt;

use regex::Regex;
use utils::{Grid, StaticGrid};

enum Operation {
    AddX(isize),
    Noop,
}

struct Cpu {
    x_register: isize,
    cycles: usize,
    answer: isize,
    display: StaticGrid<Pixel>,
}

#[derive(Clone)]
struct Pixel {
    value: char,
}

impl Default for Pixel {
    fn default() -> Self {
        Self { value: '.' }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>1}", self.value)
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            x_register: 1,
            cycles: 0,
            answer: 0,
            display: StaticGrid::new(6, 40),
        }
    }

    fn calc_answer1(&mut self) {
        if (self.cycles + 20) % 40 == 0 {
            self.answer += self.cycles as isize * self.x_register;
        };
    }

    fn update_display(&mut self) {
        if (self.x_register..self.x_register + 3).contains(&((self.cycles % 40) as isize)) {
            let c = self
                .display
                .get_coord_mut(self.cycles % 40, self.cycles / 40)
                .unwrap();
            c.value = '#';
        }
    }

    pub fn tick(&mut self, num_cycles: usize) {
        for _ in 0..num_cycles {
            self.cycles += 1;
            self.update_display();
            self.calc_answer1();
        }
    }

    pub fn execute_operation(&mut self, operation: &Operation) {
        match operation {
            Operation::AddX(amount) => {
                self.tick(2);
                self.x_register += amount;
            }
            Operation::Noop => {
                self.tick(1);
            }
        }
    }
}

fn parser(s: String) -> Vec<Operation> {
    let re = Regex::new(r"(addx|noop) ?(-?\d+)?").unwrap();
    s.split('\n')
        .into_iter()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            match caps.get(1).unwrap().as_str() {
                "addx" => Operation::AddX(caps.get(2).unwrap().as_str().parse().unwrap()),
                "noop" => Operation::Noop,
                _ => panic!("This should not happen..."),
            }
        })
        .collect()
}

fn solve(operations: &Vec<Operation>) -> isize {
    let mut cpu = Cpu::new();
    for operation in operations {
        cpu.execute_operation(operation);
    }
    cpu.answer
}

fn solve2(operations: &Vec<Operation>) {
    let mut cpu = Cpu::new();
    for operation in operations {
        cpu.execute_operation(operation);
    }
    println!("Solution 2 (8 characters below):");
    println!("{}", cpu.display);
}

fn main() {
    let operations = utils::load_puzzle_data(10, parser);
    let x_register_sum = solve(&operations);
    println!("Solution 1: CPU signal strength sum: {x_register_sum}");

    solve2(&operations);
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(10, parser);
        let solution = solve(&test_data);
        assert_eq!(solution, 13140);
    }
}
