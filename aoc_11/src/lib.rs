use regex::Regex;
use std::collections::HashMap;

enum Operation {
    Add,
    Mul,
}

struct Formula {
    operand_1: i32,
    operation: Operation,
    operand_2: i32,
}

enum Operand {
    SelfOperand,
    Other(u32),
}

struct Monkey {
    id: u32,
    operation: Operation,
    items: Vec<u32>,
    modulo: u32,
    outcome_if_true: u32,
    outcome_if_false: u32,
    item_inspected: u32,
}

impl Monkey {
    pub fn new(line: &str) -> Self {
        let regex_id = Regex::new(r"").unwrap();
        let regex_items = Regex::new(r"").unwrap();
        let regex_operation = Regex::new(r"").unwrap();
        let regex_test = Regex::new(r"").unwrap();
        let regex_outcome = Regex::new(r"").unwrap();

        println!("{line}");

        let mut lines = line.lines();
        lines.next().unwrap();

        println!("-----");
        Monkey {
            id: 1,
            operation: Operation::Add,
            items: vec![],
            modulo: 12,
            outcome_if_true: 1,
            outcome_if_false: 1,
            item_inspected: 0,
        }
    }

    pub fn compute(&self, item: u32, other: Operand) -> u32 {
        let value = match other {
            Operand::SelfOperand => item,
            Operand::Other(x) => x,
        };
        match self.operation {
            Operation::Add => item + value,
            Operation::Mul => item * value,
        }
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

pub fn part1(input_path: &str) -> Option<u32> {
    let monkeys = std::fs::read_to_string(input_path)
        .ok()?
        .split("\n\n")
        .map(Monkey::new)
        .map(|monkey| (monkey.id, monkey))
        .collect::<HashMap<u32, Monkey>>();
    None
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test.txt"), None);
    }
}
