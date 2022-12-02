#[derive(Eq, PartialEq, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!(),
        }
    }
}

pub fn part1(input_path: &str) -> anyhow::Result<u32> {
    Ok(std::fs::read_to_string(input_path)?
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(opponent, player)| (opponent.into(), player.into()))
        .map(|(opponent, player)| {
            let outcome = match (opponent, player) {
                (opponent, player) if opponent == player => Outcome::Draw,
                (Hand::Paper, Hand::Scissors) => Outcome::Win,
                (Hand::Scissors, Hand::Rock) => Outcome::Win,
                (Hand::Rock, Hand::Paper) => Outcome::Win,
                _ => Outcome::Lose,
            };
            outcome as u32 + player as u32
        })
        .sum())
}

pub fn part2(input_path: &str) -> anyhow::Result<u32> {
    Ok(std::fs::read_to_string(input_path)?
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(opponent, outcome)| (opponent.into(), outcome.into()))
        .map(|(opponent, outcome)| {
            let mine = match (opponent, outcome) {
                (other, Outcome::Draw) => other,
                (Hand::Paper, Outcome::Lose) => Hand::Rock,
                (Hand::Paper, Outcome::Win) => Hand::Paper,
                (Hand::Rock, Outcome::Lose) => Hand::Scissors,
                (Hand::Rock, Outcome::Win) => Hand::Paper,
                (Hand::Scissors, Outcome::Lose) => Hand::Paper,
                (Hand::Scissors, Outcome::Win) => Hand::Rock,
            };
            outcome as u32 + mine as u32
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    pub fn test1() {
        assert_eq!(15, part1("input_test.txt").unwrap());
    }

    #[test]
    pub fn test2() {
        assert_eq!(12, part2("input_test.txt").unwrap());
    }
}
