#[derive(Debug)]
enum Packet {
    Value(u32),
    List(Vec<Box<Packet>>),
}

pub fn part1(input_path: &str) -> Option<u32> {
    let packets_pair = std::fs::read_to_string(input_path)
        .ok()?
        .split("\n\n")
        .map(parse_pair)
        .collect::<Vec<(Packet, Packet)>>();
    None
}
fn parse_pair(line_pair: &str) -> (Packet, Packet) {
    let mut pairs = line_pair.split('\n').map(parse_packet);
    (pairs.next().unwrap(), pairs.next().unwrap())
}
fn parse_packet(line: &str) -> Packet {
    Packet::Value(1)
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test.txt"), Some(13));
    }
}
