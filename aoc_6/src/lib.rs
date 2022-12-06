use std::collections::HashSet;

pub enum Version {
    LoopVersion,
    IteratorVersion,
}

pub fn solution(input_path: &str, version: Version, packet_length: usize) -> Option<usize> {
    match version {
        Version::LoopVersion => {
            find_header_index(&std::fs::read_to_string(input_path).ok()?, packet_length)
        }
        Version::IteratorVersion => {
            find_header_index_iter(&std::fs::read_to_string(input_path).ok()?, packet_length)
        }
    }
}

fn find_header_index(input: &str, packet_length: usize) -> Option<usize> {
    for i in 0..input.len() - packet_length {
        let s = &input[i..i + packet_length];
        let set = s.chars().collect::<HashSet<char>>();
        if set.len() == packet_length {
            return Some(i + packet_length);
        }
    }
    None
}

fn find_header_index_iter(input: &str, packet_length: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(packet_length)
        .position(|window| window.iter().collect::<HashSet<&u8>>().len() == packet_length)
        .map(|x| x + packet_length)
}

#[cfg(test)]
mod tests {
    use crate::{find_header_index, find_header_index_iter};

    #[test]
    pub fn test_part1() {
        assert_eq!(
            find_header_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
            Some(7)
        );
        assert_eq!(
            find_header_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4),
            Some(5)
        );
        assert_eq!(
            find_header_index("nppdvjthqldpwncqszvftbrmjlhg", 4),
            Some(6)
        );
        assert_eq!(
            find_header_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            Some(10)
        );
        assert_eq!(
            find_header_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            Some(11)
        );
    }

    #[test]
    pub fn test_part1_iter() {
        assert_eq!(
            find_header_index_iter("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
            Some(7)
        );
        assert_eq!(
            find_header_index_iter("bvwbjplbgvbhsrlpgdmjqwftvncz", 4),
            Some(5)
        );
        assert_eq!(
            find_header_index_iter("nppdvjthqldpwncqszvftbrmjlhg", 4),
            Some(6)
        );
        assert_eq!(
            find_header_index_iter("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            Some(10)
        );
        assert_eq!(
            find_header_index_iter("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            Some(11)
        );
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(
            find_header_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            Some(19)
        );
        assert_eq!(
            find_header_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 14),
            Some(23)
        );
        assert_eq!(
            find_header_index("nppdvjthqldpwncqszvftbrmjlhg", 14),
            Some(23)
        );
        assert_eq!(
            find_header_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            Some(29)
        );
        assert_eq!(
            find_header_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            Some(26)
        );
    }

    #[test]
    pub fn test_part2_iter() {
        assert_eq!(
            find_header_index_iter("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            Some(19)
        );
        assert_eq!(
            find_header_index_iter("bvwbjplbgvbhsrlpgdmjqwftvncz", 14),
            Some(23)
        );
        assert_eq!(
            find_header_index_iter("nppdvjthqldpwncqszvftbrmjlhg", 14),
            Some(23)
        );
        assert_eq!(
            find_header_index_iter("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            Some(29)
        );
        assert_eq!(
            find_header_index_iter("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            Some(26)
        );
    }
}
