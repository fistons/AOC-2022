struct Area {
    start: u32,
    end: u32,
}

impl Area {
    fn contains(&self, other: &Area) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

impl From<&str> for Area {
    fn from(input: &str) -> Self {
        let (low, high) = input.split_once('-').unwrap();
        let (low, high): (u32, u32) = (low.parse().unwrap(), high.parse().unwrap());
        Area {
            start: low,
            end: high,
        }
    }
}

fn parse(line: &str) -> (Area, Area) {
    let (first, second) = line.split_once(',').unwrap();
    (first.into(), second.into())
}

pub fn part1(input_path: &str) -> anyhow::Result<u32> {
    Ok(std::fs::read_to_string(input_path)?
        .lines()
        .map(parse)
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count() as u32)
}

#[cfg(test)]
mod test {
    use crate::part1;

    #[test]
    pub fn test_part1() {
        assert_eq!(2, part1("input_test.txt").unwrap());
    }
}
