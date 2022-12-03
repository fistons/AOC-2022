use std::collections::HashSet;

pub fn part1(input_path: &str) -> anyhow::Result<u32> {
    Ok(std::fs::read_to_string(input_path)?
        .lines()
        .map(|line| line.split_at(line.len() / 2)) // Split in the middle
        .map(|(x, y)| {
            // For each char of the first half, check that is is present in the second
            for a in x.chars() {
                if y.contains(a) {
                    return a;
                }
            }
            unreachable!() // happens if no common char found. Should not happen, right?
        }) // Map to the common char
        .map(score) // Compute the score in a ugly way
        .sum())
}

pub fn part1_hashset(input_path: &str) -> anyhow::Result<u32> {
    Ok(std::fs::read_to_string(input_path)?
        .lines()
        .map(|line| line.split_at(line.len() / 2)) // Split in the middle
        .map(|(x, y)| {
            (
                x.chars().collect::<HashSet<char>>(),
                y.chars().collect::<HashSet<char>>(),
            )
        }) // Map both part as HashSet, to use the intersection method
        .map(|(x, y)| *x.intersection(&y).collect::<Vec<&char>>()[0]) // What can possibly go wrong if there is no intersection? Everything.
        .map(score) // Compute the score in a ugly way
        .sum())
}

pub fn part2(input_path: &str) -> anyhow::Result<u32> {
    Ok(std::fs::read_to_string(input_path)?
        .lines()
        .collect::<Vec<&str>>() // Collect as Vec of &str
        .chunks(3) // Read 3 by 3
        .map(|slice| {
            // For each char of the first line of the chunk, check that it is present in both second
            // and third line.
            for a in slice[0].chars() {
                if slice[1].contains(a) && slice[2].contains(a) {
                    return a;
                }
            }
            unreachable!() // It can't be. Can it?
        }) // Check for each char of line n that it is present it line n + 1 and n + 2
        .map(score) // Compute the score
        .sum())
}

/// #Compute the score of a char.
///
/// I'm pretty sure there is much better way to do it.
/// We copy the char here and could use a reference, but heh.
fn score(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 65 + 27 // 65 the value of A in ASCII + 27 the value priority of A in the exercice
    } else {
        c as u32 - 97 + 1 // 94 the value of a in ASCII + 1 the value priority of a in the exercice
    }
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    pub fn test_part1() {
        assert_eq!(157, part1("input_test.txt").unwrap())
    }

    #[test]
    pub fn test_part1_hashset() {
        assert_eq!(157, part1("input_test.txt").unwrap())
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(70, part2("input_test.txt").unwrap())
    }
}
