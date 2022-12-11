use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_ID: Regex = Regex::new(r"(\d+)").unwrap();
    static ref REGEX_ITEMS: Regex = Regex::new(r"Starting items: (.*)").unwrap();
    static ref REGEX_OPERATION: Regex = Regex::new(r"Operation: new = (.+) (\D) (.+)").unwrap();
    static ref REGEX_TEST: Regex = Regex::new(r"(\d+)").unwrap();
    static ref REGEX_OUTCOME: Regex = Regex::new(r"(\d+)").unwrap();
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}
#[derive(Debug)]
struct Formula {
    operand_1: Operand,
    operation: Operation,
    operand_2: Operand,
}

#[derive(Debug)]
enum Operand {
    SelfOperand,
    Other(u32),
}

#[derive(Debug)]
struct Monkey {
    id: u32,
    formula: Formula,
    items: Vec<u32>,
    modulo: u32,
    outcome_if_true: u32,
    outcome_if_false: u32,
    item_inspected: u32,
    divider: u32,
}

impl Monkey {
    pub fn new(line: &str, divider: u32) -> Self {
        let mut lines = line.split('\n');
        let id: u32 = REGEX_ID
            .captures_iter(lines.next().unwrap())
            .next()
            .unwrap()[1]
            .parse()
            .unwrap();
        let items: &str = &REGEX_ITEMS
            .captures_iter(lines.next().unwrap())
            .next()
            .unwrap()[1];
        let items = items
            .split(',')
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let capture_operation = REGEX_OPERATION
            .captures_iter(lines.next().unwrap())
            .next()
            .unwrap();

        let operand_1 = if &capture_operation[1] == "old" {
            Operand::SelfOperand
        } else {
            Operand::Other(capture_operation[1].parse::<u32>().unwrap())
        };
        let operation = if &capture_operation[2] == "+" {
            Operation::Add
        } else {
            Operation::Mul
        };
        let operand_2 = if &capture_operation[3] == "old" {
            Operand::SelfOperand
        } else {
            Operand::Other(capture_operation[3].parse::<u32>().unwrap())
        };

        let formula = Formula {
            operand_1,
            operation,
            operand_2,
        };

        let modulo: u32 = REGEX_TEST
            .captures_iter(lines.next().unwrap())
            .next()
            .unwrap()[1]
            .parse()
            .unwrap();
        let outcome_if_true: u32 = REGEX_OUTCOME
            .captures_iter(lines.next().unwrap())
            .next()
            .unwrap()[1]
            .parse()
            .unwrap();
        let outcome_if_false: u32 = REGEX_OUTCOME
            .captures_iter(lines.next().unwrap())
            .next()
            .unwrap()[1]
            .parse()
            .unwrap();

        let mut lines = line.lines();
        lines.next().unwrap();

        Monkey {
            id,
            formula,
            items,
            modulo,
            outcome_if_true,
            outcome_if_false,
            item_inspected: 0,
            divider,
        }
    }

    pub fn compute(&self, item: u32) -> u32 {
        let operand_1 = match self.formula.operand_1 {
            Operand::SelfOperand => item,
            Operand::Other(x) => x,
        };
        let operand_2 = match self.formula.operand_2 {
            Operand::SelfOperand => item,
            Operand::Other(x) => x,
        };
        (match self.formula.operation {
            Operation::Add => operand_1 + operand_2,
            Operation::Mul => operand_1 * operand_2,
        }) / self.divider
    }

    pub fn inspect(&mut self, item: u32) -> u32 {
        self.item_inspected += 1;

        if item % self.modulo == 0 {
            self.outcome_if_true
        } else {
            self.outcome_if_false
        }
    }
}

pub fn part1(input_path: &str, iteration: u32, divider: u32) -> Option<u32> {
    let monkeys = std::fs::read_to_string(input_path)
        .ok()?
        .split("\n\n")
        .map(|line| Monkey::new(line, divider))
        .map(|monkey| (monkey.id, RefCell::new(monkey)))
        .collect::<HashMap<u32, RefCell<Monkey>>>();
    for i in 1..=iteration {
        for monkey_id in 0..monkeys.len() {
            let mut monkey = monkeys.get(&(monkey_id as u32)).unwrap().borrow_mut();
            while !monkey.items.is_empty() {
                let item = monkey.items.remove(0);
                let res = monkey.compute(item);
                let next_monkey_id = monkey.inspect(res);

                let mut next_monkey = monkeys.get(&(next_monkey_id as u32)).unwrap().borrow_mut();
                next_monkey.items.push(res);
            }
        }
        println!("Round {i}");
        for k in monkeys.keys().sorted() {
            println!("{:?}", monkeys[k]);
        }
        println!();
    }

    let top_2 = monkeys
        .values()
        .map(|m| m.borrow())
        .sorted_by(|a, b| b.item_inspected.cmp(&a.item_inspected))
        .take(2)
        .map(|m| m.item_inspected)
        .collect::<Vec<u32>>();

    Some(top_2[0] * top_2[1])
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test.txt", 20, 3), Some(10605));
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part1("input_test.txt", 10_000, 1), Some(2713310158));
    }
}
