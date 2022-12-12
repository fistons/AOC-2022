use std::cell::RefCell;
use std::collections::HashMap;

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

static REGEX_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
static REGEX_ITEMS: Lazy<Regex> = Lazy::new(|| Regex::new(r"Starting items: (.*)").unwrap());
static REGEX_OPERATION: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Operation: new = (.+) (\D) (.+)").unwrap());
static REGEX_TEST: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
static REGEX_OUTCOME: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());

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
    Other(usize),
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    formula: Formula,
    items: Vec<usize>,
    modulo: usize,
    outcome_if_true: usize,
    outcome_if_false: usize,
    item_inspected: usize,
    divider: usize,
}

impl Monkey {
    pub fn new(line: &str, divider: usize) -> Self {
        let mut lines = line.split('\n');
        let id: usize = REGEX_ID
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
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let capture_operation = REGEX_OPERATION
            .captures_iter(lines.next().unwrap())
            .next()
            .unwrap();

        let operand_1 = if &capture_operation[1] == "old" {
            Operand::SelfOperand
        } else {
            Operand::Other(capture_operation[1].parse::<usize>().unwrap())
        };
        let operation = if &capture_operation[2] == "+" {
            Operation::Add
        } else {
            Operation::Mul
        };
        let operand_2 = if &capture_operation[3] == "old" {
            Operand::SelfOperand
        } else {
            Operand::Other(capture_operation[3].parse::<usize>().unwrap())
        };

        let formula = Formula {
            operand_1,
            operation,
            operand_2,
        };

        let modulo: usize = REGEX_TEST
            .captures_iter(lines.next().unwrap())
            .next()
            .unwrap()[1]
            .parse()
            .unwrap();
        let outcome_if_true: usize = REGEX_OUTCOME
            .captures_iter(lines.next().unwrap())
            .next()
            .unwrap()[1]
            .parse()
            .unwrap();
        let outcome_if_false: usize = REGEX_OUTCOME
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

    pub fn compute(&self, item: usize, common_divider: usize) -> usize {
        let operand_1 = match self.formula.operand_1 {
            Operand::SelfOperand => item,
            Operand::Other(x) => x,
        };
        let operand_2 = match self.formula.operand_2 {
            Operand::SelfOperand => item,
            Operand::Other(x) => x,
        };

        let res = match self.formula.operation {
            Operation::Add => operand_1 + operand_2,
            Operation::Mul => operand_1 * operand_2,
        };

        let res = res.rem_euclid(common_divider);
        res / self.divider
    }

    pub fn inspect(&mut self, item: usize) -> usize {
        self.item_inspected += 1;

        if item % self.modulo == 0 {
            self.outcome_if_true
        } else {
            self.outcome_if_false
        }
    }
}

pub fn part1(input_path: &str, iteration: usize, divider: usize) -> Option<usize> {
    let monkeys = std::fs::read_to_string(input_path)
        .ok()?
        .split("\n\n")
        .map(|line| Monkey::new(line, divider))
        .map(|monkey| (monkey.id, RefCell::new(monkey)))
        .collect::<HashMap<usize, RefCell<Monkey>>>();

    let common_divider = monkeys
        .values()
        .map(|monkey| monkey.borrow().modulo)
        .reduce(|a, b| a * b)?;

    for _ in 1..=iteration {
        for monkey_id in 0..monkeys.len() {
            let mut monkey = monkeys.get(&(monkey_id as usize)).unwrap().borrow_mut();
            while !monkey.items.is_empty() {
                let item = monkey.items.remove(0);
                let res = monkey.compute(item, common_divider);
                let next_monkey_id = monkey.inspect(res);

                let mut next_monkey = monkeys
                    .get(&(next_monkey_id as usize))
                    .unwrap()
                    .borrow_mut();
                next_monkey.items.push(res);
            }
        }
    }

    let top_2 = monkeys
        .values()
        .map(|m| m.borrow())
        .sorted_by(|a, b| b.item_inspected.cmp(&a.item_inspected))
        .take(2)
        .map(|m| m.item_inspected)
        .reduce(|m, n| m * n);

    top_2
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
