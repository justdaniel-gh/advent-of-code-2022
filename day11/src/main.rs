use std::collections::VecDeque;

enum OperationValue {
    Constant(usize),
    Old
}

/// Operation is (first OP second)
enum Operation {
    Add((OperationValue, OperationValue)),
    Mul((OperationValue, OperationValue)),
}

struct Monkey {
    /// List of items, where usize is the current worry level
    items: VecDeque<usize>,
    operation: Operation,
    /// Divisible by test_value
    test_value: usize,
    /// If test true: throw to monkey
    truth_monkey: usize,
    /// If test false: throw to monkey
    falsity_monkey: usize,
    num_inspected_items: usize
}

struct Game {
    monkeys: Vec<Monkey>,
    current_round: usize,
    divide_by_three: bool,
    divisor: usize
}

impl Game {
    fn play(&mut self, num_rounds: usize) {
        self.divisor = self.monkeys.iter().fold(1, |a,m| a * m.test_value);
        for _ in 0..num_rounds {
            for monkey_ndx in 0..self.monkeys.len() {
                self.run_turn(monkey_ndx);
            }
            self.current_round += 1;
        }
    }

    fn run_turn(&mut self, monkey_ndx: usize) {
        // 0. To avoid borrowing issues, save passes for later
        let mut passes: Vec<(usize, usize)> = vec![];
        // 1. Inspect item
        let monkey = self.monkeys.get_mut(monkey_ndx).unwrap();
        while let Some(mut item) = monkey.items.pop_front() {
            monkey.num_inspected_items += 1;
            // 2. Perform monkey's operation, modifying the worry level
            item = match &monkey.operation {
                Operation::Add(op_values) => {
                    let val1 = match op_values.0 {
                        OperationValue::Constant(val) => val,
                        OperationValue::Old => item,
                    };
                    let val2 = match op_values.1 {
                        OperationValue::Constant(val) => val,
                        OperationValue::Old => item,
                    };
                    val1 + val2
                },
                Operation::Mul(op_values) => {
                    let val1 = match op_values.0 {
                        OperationValue::Constant(val) => val,
                        OperationValue::Old => item,
                    };
                    let val2 = match op_values.1 {
                        OperationValue::Constant(val) => val,
                        OperationValue::Old => item,
                    };
                    val1 * val2
                }
            };
            // 3. Monkey bored...
            if self.divide_by_three {
                // 3a. Divide worry by 3
                item /= 3;
            } else {
                // 3b. Mod worry by the product of the tests
                item %= self.divisor;
            }
            // 4. Perform monkey's test
            let pass_monkey_ndx = match (item % monkey.test_value) == 0 {
                true => {
                    monkey.truth_monkey
                },
                false => {
                    monkey.falsity_monkey
                },
            };
            // 5. Pass the item (Save)
            passes.push((pass_monkey_ndx, item));
            //pass_monkey.items.push_back(item);
        }
        for pass in passes {
            self.monkeys.get_mut(pass.0).unwrap().items.push_back(pass.1);
        }
    }
}

fn parser(s: String) -> Vec<Monkey> {
    /*
    Monkey 0:
      Starting items: 79, 98
      Operation: new = old * 19
      Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3
    */
    //let items_re = Regex::new(r"Starting items: (?:(\d+),?)+").unwrap();
    s.split("\n\n")
        .into_iter()
        .map(|monkey| {
            let mut line = monkey.split('\n');
            // Skip Monkey X: line
            line.next();
            let items = line.next().unwrap().split_once(": ").unwrap().1.split(", ").map(|n| n.parse().unwrap()).collect();
            let operation_str_vec: Vec<_> = line.next().unwrap().split_once(": ").unwrap().1.split_once("= ").unwrap().1.split(' ').collect();
            let op_val_1 = match operation_str_vec.first() {
                Some(&"old") => OperationValue::Old,
                Some(&n) => OperationValue::Constant(n.parse().unwrap()),
                _ => panic!("This should not happen")
            };
            let op_val_2 = match operation_str_vec.get(2) {
                Some(&"old") => OperationValue::Old,
                Some(&n) => OperationValue::Constant(n.parse().unwrap()),
                _ => panic!("This should not happen")
            };
            let operation = match *operation_str_vec.get(1).unwrap() {
                "*" => Operation::Mul((op_val_1, op_val_2)),
                "+" => Operation::Add((op_val_1, op_val_2)),
                _ => panic!("This should not happen")
            };
            let test_value = line.next().unwrap().split(' ').last().unwrap().parse().unwrap();
            let truth_monkey = line.next().unwrap().split(' ').last().unwrap().parse().unwrap();
            let falsity_monkey = line.next().unwrap().split(' ').last().unwrap().parse().unwrap();
            Monkey {
                items,
                operation,
                test_value,
                truth_monkey,
                falsity_monkey,
                num_inspected_items: 0
            }
        })
        .collect()
}

fn solve(monkeys: Vec<Monkey>, num_rounds: usize, divide_by_three: bool) -> usize {
    let mut game = Game {
        monkeys,
        current_round: 1,
        divide_by_three,
        divisor: 1
    };
    game.play(num_rounds);
    let mut ultimate = 0;
    let mut penultimate = 0;
    for monkey in game.monkeys {
        if monkey.num_inspected_items > ultimate {
            penultimate = ultimate;
            ultimate = monkey.num_inspected_items;
        } else if monkey.num_inspected_items > penultimate {
            penultimate = monkey.num_inspected_items;
        }
    }
    ultimate * penultimate
}

fn main() {
    let monkeys = utils::load_puzzle_data(11, parser);
    let officer_farva = solve(monkeys, 20, true);
    println!("Solution 1: There is a monkey shenanigan level of: {officer_farva}");

    let monkeys = utils::load_puzzle_data(11, parser);
    let the_business = solve(monkeys, 10000, false);
    println!("Solution 2: After 10,000 rounds, there is a monkey business level of: {the_business}");
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(11, parser);
        let solution = solve(test_data, 20, true);
        assert_eq!(solution, 10605);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(11, parser);
        let solution = solve(test_data, 10000, false);
        assert_eq!(solution, 2713310158);
    }
}
