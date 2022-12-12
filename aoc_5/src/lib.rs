use std::collections::HashMap;
use std::fmt;

use anyhow::Context;
use itertools::Itertools;

fn parse_initials_stacks(stacks: &str) -> anyhow::Result<HashMap<usize, Vec<char>>> {
    let mut map: HashMap<usize, Vec<char>> = HashMap::new();
    for line in stacks.lines().rev().skip(1) {
        let mut index = 1usize;
        line.chars().collect::<Vec<char>>().chunks(4).for_each(|x| {
            let x = x.iter().collect::<String>();
            let mut m: Vec<&str> = x.matches(char::is_alphabetic).collect();
            if let Some(c) = m.pop() {
                map.entry(index)
                    .or_default()
                    .push(c.chars().next().unwrap());
            }
            index += 1;
        });
    }
    Ok(map)
}

struct Move {
    number: usize,
    from: usize,
    to: usize,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Move {} elements from {} to {}",
            self.number, self.from, self.to
        )
    }
}

/// Reads the "moves" part of the file
fn read_moves(moves: &str) -> anyhow::Result<Vec<Move>> {
    Ok(moves
        .lines()
        .map(|line| {
            let line = line.replace(char::is_alphabetic, "");
            let mut split_line = line.split_whitespace();
            let number = split_line.next().unwrap().parse::<usize>().unwrap();
            let from = split_line.next().unwrap().parse::<usize>().unwrap();
            let to = split_line.next().unwrap().parse::<usize>().unwrap();

            Move { number, from, to }
        })
        .collect())
}

fn moves_crates_9000(stacks: &mut HashMap<usize, Vec<char>>, m: &Move) {
    for _ in 0..m.number {
        //TODO: use drain() instead?
        let c = stacks.get_mut(&(m.from)).unwrap().pop().unwrap();
        stacks.get_mut(&(m.to)).unwrap().push(c);
    }
}

fn moves_crates_9001(stacks: &mut HashMap<usize, Vec<char>>, m: &Move) {
    let mut crates = Vec::with_capacity(m.number);
    for _ in 0..m.number {
        //TODO: use drain() instead?
        let c = stacks.get_mut(&(m.from)).unwrap().pop().unwrap();
        crates.push(c);
    }
    crates.reverse();

    for c in crates {
        stacks.get_mut(&(m.to)).unwrap().push(c);
    }
}

pub fn part1(input_path: &str) -> anyhow::Result<String> {
    let input = std::fs::read_to_string(input_path)?;
    let (initial_stacks, moves) = input.split_once("\n\n").context("Could not split input")?;

    let mut stacks = parse_initials_stacks(initial_stacks)?;
    read_moves(moves)?
        .iter()
        .for_each(|m| moves_crates_9000(&mut stacks, m));

    let result = stacks
        .keys()
        .sorted()
        .map(|key| stacks[key].last().unwrap())
        .collect::<String>();

    Ok(result)
}

pub fn part2(input_path: &str) -> anyhow::Result<String> {
    let input = std::fs::read_to_string(input_path)?;
    let (initial_stacks, moves) = input.split_once("\n\n").context("Could not split input")?;

    let mut stacks = parse_initials_stacks(initial_stacks)?;
    read_moves(moves)?
        .iter()
        .for_each(|m| moves_crates_9001(&mut stacks, m));

    Ok(stacks
        .keys()
        .sorted()
        .map(|key| stacks[key].last().unwrap())
        .collect::<String>())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    pub fn test_part1() {
        assert_eq!(String::from("CMZ"), part1("input_test.txt").unwrap());
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(String::from("MCD"), part2("input_test.txt").unwrap());
    }
}
