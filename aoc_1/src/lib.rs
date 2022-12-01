pub fn part_1_and_2(input_path: String, top: usize) -> anyhow::Result<u32> {
    let reader = std::fs::read_to_string(input_path)?;

    let mut totals: Vec<u32> = vec![];
    for group in reader.split("\n\n") {
        let mut total = 0u32;
        for line in group.split("\n") {
            total += line.parse::<u32>()?;
        }
        totals.push(total);
    }

    totals.sort_by(|a, b| b.cmp(a));

    Ok(totals[0..top].iter().sum())
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_1() {
        let res = part_1_and_2("input_test.txt".to_owned(), 1);
        assert_eq!(24000, res.unwrap());
    }

    #[test]
    pub fn test_part_2() {
        let res = part_1_and_2("input_test.txt".to_owned(), 3);
        assert_eq!(45000, res.unwrap());
    }
}