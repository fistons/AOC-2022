use std::cmp::Ordering;

use nom::branch::alt;
use nom::character::complete::{char, u8};
use nom::combinator::{cut, map};
use nom::multi::separated_list0;
use nom::sequence::{preceded, terminated};
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub enum Packet {
    Value(u8),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_elements(self, other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Parse a list of Packets
fn parse_list(i: &str) -> IResult<&str, Vec<Packet>> {
    preceded(
        char('['),
        cut(terminated(
            separated_list0(char(','), parse_value),
            char(']'),
        )),
    )(i)
}

/// Parse a Packet
fn parse_value(i: &str) -> IResult<&str, Packet> {
    alt((map(u8, Packet::Value), map(parse_list, Packet::List)))(i)
}

/// Read a string and create a Packet from it
fn parse_line(i: &str) -> Packet {
    let result = map(parse_list, Packet::List)(i);
    result.unwrap().1
}

fn read_pair(line_pair: &str) -> (Packet, Packet) {
    let mut pairs = line_pair.split('\n').map(parse_line);
    (pairs.next().unwrap(), pairs.next().unwrap())
}

pub fn part1(input_path: &str) -> Option<usize> {
    Some(
        std::fs::read_to_string(input_path)
            .ok()?
            .split("\n\n") // Separate the input on empty lines
            .map(read_pair) // Build a pair of Packet
            .enumerate()
            .map(|(index, (left, right))| (index + 1, compare_elements(&left, &right))) // Compare the pair
            .filter(|(_, value)| *value == Ordering::Less) // Count the number of pair in the right order
            .map(|(index, _)| index)
            .sum::<usize>(),
    )
}

pub fn part2(input_path: &str) -> Option<usize> {
    let mut items = std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .filter(|x| !x.is_empty()) // Discard empty lines
        .map(parse_line) // Parse the packet one by one
        .collect::<Vec<Packet>>();

    // Adding the dividers
    items.push(Packet::List(vec![Packet::Value(6)]));
    items.push(Packet::List(vec![Packet::Value(2)]));

    // Sorting
    items.sort();

    items
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            // Check if we are on a divider
            *item == &Packet::List(vec![Packet::Value(6)])
                || *item == &Packet::List(vec![Packet::Value(2)])
        })
        .map(|(index, _)| index + 1)
        .reduce(|x, y| x * y)
}

/// Compare two elements. If they are different, wrap the value one to a list and compare both of them
fn compare_elements(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::List(left), Packet::Value(right)) => {
            // Left is a list, Right is a value: Wrap Right in a list and compare them
            let right = vec![Packet::Value(*right)];
            compare_list(left, &right)
        }
        (Packet::Value(left), Packet::List(right)) => {
            // Left is a value, Right is a list: Wrap Left in a list and compare them
            let left = vec![Packet::Value(*left)];
            compare_list(&left, right)
        }
        (Packet::List(left), Packet::List(right)) => compare_list(left, right), // Both are list, lest compare them
        (Packet::Value(left), Packet::Value(right)) => left.cmp(right), // Both a are element, compare them!
    }
}

/// Compare each Packet::List element to each other
fn compare_list(left: &[Packet], right: &[Packet]) -> Ordering {
    let mut left = left.iter();
    let mut right = right.iter();

    loop {
        let left = left.next();
        let right = right.next();

        let result = match (left, right) {
            (None, None) => return Ordering::Equal, // Both are empty, we return equals and we are finish
            (Some(_), None) => Ordering::Greater,   // Left have some, Right is empty, L > R
            (None, Some(_)) => Ordering::Less,      // Left is empty, Right have some, R < L
            (Some(x), Some(y)) => compare_elements(x, y), // Both a some, let's compare the element
        };

        if result != Ordering::Equal {
            // If we have a non Equal result, yay, we have finish! Else we continue by comparing the next two elements
            return result;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test.txt"), Some(13));
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2("input_test.txt"), Some(140));
    }
}
