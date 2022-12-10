use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

pub fn part1(input_path: &str) -> Option<i32> {
    let mut register = 1;
    let mut cycles = 1;
    let marks = vec![20, 60, 100, 140, 180, 220];
    let mut position = HashMap::<i32, i32>::new();

    std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .map(parse_instruction)
        .for_each(|i| {
            cycles += 1;
            match i {
                Instruction::Noop => {
                    if marks.contains(&cycles) {
                        position.insert(cycles, register);
                    }
                }
                Instruction::Addx(v) => {
                    if marks.contains(&cycles) {
                        position.insert(cycles, register);
                    }

                    cycles += 1;
                    register += v;
                    if marks.contains(&cycles) {
                        position.insert(cycles, register);
                    }
                }
            };
        });

    Some(position.iter().map(|(k, v)| k * v).sum())
}

fn parse_instruction(line: &str) -> Instruction {
    match line {
        l if l.starts_with("addx") => {
            let value = l.split_once(' ').unwrap().1.parse::<i32>().unwrap();
            Instruction::Addx(value)
        }
        l if l.starts_with("noop") => Instruction::Noop,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test.txt"), Some(13140));
    }
}
