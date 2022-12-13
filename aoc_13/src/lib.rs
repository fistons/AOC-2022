use std::cmp::Ordering;

#[derive(Debug)]
enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

impl Packet {
    pub fn new(_input: &str) -> Self {
        // Please magically parse.
        Packet::List(vec![Packet::Value(0)])
    }
}

pub fn part1(input_path: &str) -> Option<usize> {
    Some(
        std::fs::read_to_string(input_path)
            .ok()?
            .split("\n\n")
            .map(parse_pair)
            .enumerate()
            .map(|(index, (left, right))| (index + 1, are_elements_in_right_order(&left, &right)))
            .filter(|(_, value)| *value == Ordering::Less)
            .map(|(index, _)| index)
            .sum::<usize>(),
    )
}
fn parse_pair(line_pair: &str) -> (Packet, Packet) {
    let mut pairs = line_pair.split('\n').map(parse_packet);
    (pairs.next().unwrap(), pairs.next().unwrap())
}
fn parse_packet(line: &str) -> Packet {
    Packet::new(line)
}

fn are_elements_in_right_order(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::List(left), Packet::Value(right)) => {
            let right = vec![Packet::Value(*right)];
            are_list_in_right_order(left, &right)
        }
        (Packet::Value(left), Packet::List(right)) => {
            let left = vec![Packet::Value(*left)];
            are_list_in_right_order(&left, right)
        }
        (Packet::List(left), Packet::List(right)) => are_list_in_right_order(left, right),
        (Packet::Value(left), Packet::Value(right)) => left.cmp(right),
    }
}

fn are_list_in_right_order(left: &[Packet], right: &[Packet]) -> Ordering {
    let mut left = left.iter();
    let mut right = right.iter();

    loop {
        let left = left.next();
        let right = right.next();

        let result = match (left, right) {
            (None, None) => return Ordering::Equal,
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (Some(x), Some(y)) => are_elements_in_right_order(x, y),
        };

        if result != Ordering::Equal {
            return result;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::vec;

    use crate::{are_elements_in_right_order, part1, Packet};

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test.txt"), Some(13));
    }

    #[test]
    pub fn first_case() {
        let first_list = Packet::List(vec![
            Packet::Value(1),
            Packet::Value(1),
            Packet::Value(3),
            Packet::Value(1),
            Packet::Value(1),
        ]);

        let second_list = Packet::List(vec![
            Packet::Value(1),
            Packet::Value(1),
            Packet::Value(5),
            Packet::Value(1),
            Packet::Value(1),
        ]);

        assert_eq!(
            are_elements_in_right_order(&first_list, &second_list),
            Ordering::Less
        );
    }

    #[test]
    pub fn second_case() {
        let first_list = Packet::List(vec![
            Packet::List(vec![Packet::Value(1)]),
            Packet::List(vec![Packet::Value(2), Packet::Value(3), Packet::Value(4)]),
        ]);

        let second_list =
            Packet::List(vec![Packet::List(vec![Packet::Value(1)]), Packet::Value(4)]);

        assert_eq!(
            are_elements_in_right_order(&first_list, &second_list),
            Ordering::Less
        );
    }

    #[test]
    pub fn third_case() {
        let first_list = Packet::List(vec![]);
        let second_list = Packet::List(vec![Packet::Value(3)]);

        assert_eq!(
            are_elements_in_right_order(&first_list, &second_list),
            Ordering::Less
        );
    }

    #[test]
    pub fn fourth_case() {
        let first_list = Packet::List(vec![Packet::List(vec![Packet::List(vec![])])]);
        let second_list = Packet::List(vec![Packet::List(vec![])]);

        assert_eq!(
            are_elements_in_right_order(&first_list, &second_list),
            Ordering::Greater
        );
    }

    #[test]
    pub fn fifth_case() {
        let first_list = Packet::List(vec![
            Packet::Value(7),
            Packet::Value(7),
            Packet::Value(7),
            Packet::Value(7),
        ]);

        let second_list = Packet::List(vec![Packet::Value(7), Packet::Value(7), Packet::Value(7)]);

        assert_eq!(
            are_elements_in_right_order(&first_list, &second_list),
            Ordering::Greater
        );
    }

    #[test]
    pub fn sixth_case() {
        let first_list = Packet::List(vec![Packet::Value(9)]);
        let second_list = Packet::List(vec![Packet::List(vec![
            Packet::Value(8),
            Packet::Value(7),
            Packet::Value(6),
        ])]);

        assert_eq!(
            are_elements_in_right_order(&first_list, &second_list),
            Ordering::Greater
        );
    }
}
