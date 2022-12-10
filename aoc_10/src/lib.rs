use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn print_pixel(register: &i32, cycles: &i32) {
    let tick = (cycles - 1) % 40;
    if (register - 1..=register + 1).contains(&tick) {
        print!("#");
    } else {
        print!(" ");
    }

    if tick == 39 {
        println!();
    }
}

pub fn part1(input_path: &str) -> Option<i32> {
    let mut register = 1;
    let mut cycles = 1;
    let marks = vec![20, 60, 100, 140, 180, 220];
    let mut positions = HashMap::<i32, i32>::new();

    std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .map(parse_instruction)
        .for_each(|instruction| {
            print_pixel(&register, &cycles);

            cycles += 1;
            positions.insert(cycles, register);

            match instruction {
                Instruction::Noop => (), // Nope.
                Instruction::Addx(v) => {
                    print_pixel(&register, &cycles);

                    cycles += 1;
                    register += v;
                    positions.insert(cycles, register);
                }
            };
        });

    let sum = positions
        .iter()
        .filter(|(k, _)| marks.contains(*k))
        .map(|(k, v)| *k as i32 * v)
        .sum();
    Some(sum)
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
