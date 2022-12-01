use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1_and_2(input_path: String, top: usize) -> anyhow::Result<u32> {
    let reader = BufReader::new(File::open(input_path)?);

    let mut current_elf_total = 0u32;
    let mut totals: Vec<u32> = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            totals.push(current_elf_total);
            current_elf_total = 0;
        } else {
            current_elf_total += line.parse::<u32>()?;
        }
    }
    totals.push(current_elf_total); // Ugly.

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