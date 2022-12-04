use std::fmt::Error;

struct Area {
    start: u32,
    end: u32,
}

impl Area {
    fn contains(&self, other: &Area) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

impl TryFrom<&str> for Area {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (low, high) = value.split_once('-').ok_or_else(Error::default)?;
        let (low, high): (u32, u32) = (low.parse()?, high.parse()?);
        Ok(Area {
            start: low,
            end: high,
        })
    }
}

fn parse(line: &str) -> anyhow::Result<(Area, Area)> {
    let (first, second) = line.split_once(',').ok_or_else(Error::default)?;
    Ok((first.try_into()?, second.try_into()?))
}

pub fn part1(input_path: &str) -> anyhow::Result<u32> {
    Ok(std::fs::read_to_string(input_path)?
        .lines()
        .filter_map(|x| parse(x).ok())
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count() as u32)
}

#[cfg(test)]
mod test {
    use crate::part1;

    #[test]
    pub fn test_part1() {
        assert!(matches!(part1("input_test.txt"), Ok(2)));
    }
}
