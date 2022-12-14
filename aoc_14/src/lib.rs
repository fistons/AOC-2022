use crate::cave::Cave;

mod cave;

pub fn solution(input_path: &str, infinite: bool) -> Option<usize> {
    let map = std::fs::read_to_string(input_path).ok()?;

    let mut cave = Cave::new(&map, infinite);
    cave.run_simulation();

    Some(cave.count_sands())
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    pub fn test_part1() {
        assert_eq!(solution("input_test.txt", true), Some(24));
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(solution("input_test.txt", false), Some(93));
    }
}
