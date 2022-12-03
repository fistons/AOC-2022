pub fn part1(input_path: &str) -> anyhow::Result<u32> {
    Ok(std::fs::read_to_string(input_path)?
        .lines()
        .map(|line| line.split_at(line.len() / 2)) // Split in the middle
        .map(|(x, y)| {
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

pub fn part2(input_path: &str) -> anyhow::Result<u32> {
    Ok(std::fs::read_to_string(input_path)?
        .lines()
        .collect::<Vec<&str>>() // Collect as Vec of &str
        .chunks(3) // Read 3 by 3
        .map(|slice| {
            for a in slice[0].chars() {
                if slice[1].contains(a) && slice[2].contains(a) {
                    return a;
                }
            }
            unreachable!()
        }) // Check for each char of first line that it is present it line 2 and 3
        .map(score) // Compute the score
        .sum())
}

/// #Compute the score of a char
///
/// I'm pretty sure there is much better way to do it.
fn score(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 65 + 27
    } else {
        c as u32 - 97 + 1
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
    pub fn test_part2() {
        assert_eq!(70, part2("input_test.txt").unwrap())
    }
}
