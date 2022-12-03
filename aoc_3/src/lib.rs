pub fn part1(input_path: &str) -> anyhow::Result<u32> {
    Ok(std::fs::read_to_string(input_path)?
        .lines()
        .map(|line| line.split_at(line.len() / 2)) // Split in the middle
        .map(|(x, y)| {
            (
                x.chars().collect::<Vec<char>>(),
                y.chars().collect::<Vec<char>>(),
            )
        }) // map to two list of chars
        .map(|(x, y)| {
            for a in x {
                if y.contains(&a) {
                    return a;
                }
            }
            unreachable!() // happens if no common char found. Should not happen, right?
        }) // Map to the common char
        .map(score) // I'm pretty sure there is much better way to do it
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

pub fn part2() -> anyhow::Result<u32> {
    Ok(0)
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
        assert_eq!(0, part2().unwrap())
    }
}
