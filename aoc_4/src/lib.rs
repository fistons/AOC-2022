use std::fmt::Error;
use std::ops::RangeInclusive;

use anyhow::Context;
use itertools::Itertools;

trait InclusiveRange {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn contains_start(&self, other: &Self) -> bool;

    /// If the range contains the start of the other range OR if the other range contains part of
    /// the current range, then we have an overlap
    fn overlaps_or_is_overlapped(&self, other: &Self) -> bool {
        self.contains_start(other) || other.contains_start(self)
    }
}

impl<T> InclusiveRange for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn contains_start(&self, other: &Self) -> bool {
        self.contains(other.start())
    }
}

fn parse_range(range: &str) -> anyhow::Result<RangeInclusive<u32>> {
    let (r1, r2) = range
        .split('-')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect_tuple::<(u32, u32)>()
        .context("Could not collect tuple")?;
    Ok(r1..=r2)
}
fn parse_line(line: &str) -> anyhow::Result<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (first, second) = line.split_once(',').ok_or_else(Error::default)?;
    Ok((parse_range(first)?, parse_range(second)?))
}

pub fn part1(input_path: &str) -> anyhow::Result<u32> {
    Ok(std::fs::read_to_string(input_path)?
        .lines()
        .filter_map(|x| parse_line(x).ok())
        .filter(|(a, b)| a.contains_or_is_contained(b))
        .count() as u32)
}

pub fn part2(input_path: &str) -> anyhow::Result<u32> {
    Ok(std::fs::read_to_string(input_path)?
        .lines()
        .filter_map(|x| parse_line(x).ok())
        .filter(|(a, b)| a.overlaps_or_is_overlapped(b))
        .count() as u32)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    pub fn test_part1() {
        assert!(matches!(part1("input_test.txt"), Ok(2)));
    }

    #[test]
    pub fn test_part2() {
        assert!(matches!(part2("input_test.txt"), Ok(4)));
    }
}
