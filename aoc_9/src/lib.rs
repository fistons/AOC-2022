use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct RopeMove {
    direction: Direction,
    count: i32,
}
#[derive(Debug)]
struct Rope {
    knots: Vec<(i32, i32)>,
    visited_by_tails: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(size: usize) -> Self {
        let mut visited_by_tails = HashSet::default();
        visited_by_tails.insert((0, 0));
        let knots = vec![(0, 0); size]; // We keep the head

        Rope {
            knots,
            visited_by_tails,
        }
    }

    pub fn move_rope(&mut self, direction: &RopeMove) {
        for _ in 0..direction.count {
            let mut head = self.knots.first_mut().unwrap(); // Move the head first
            match direction.direction {
                Direction::Up => head.1 += 1,
                Direction::Down => head.1 -= 1,
                Direction::Right => head.0 += 1,
                Direction::Left => head.0 -= 1,
            }

            for i in 1..self.knots.len() {
                let next_knot = self.knots[i - 1];
                let mut current_knot = self.knots.get_mut(i).unwrap();

                let distance = distance(&next_knot, current_knot);
                if is_detached(&distance) {
                    current_knot.0 += distance.0.signum(); // Move only by at most 1
                    current_knot.1 += distance.1.signum();
                }
            }
            self.visited_by_tails.insert(*self.knots.last().unwrap());
        }
    }
}

/// Compute the distance between two knots
fn distance((head_x, head_y): &(i32, i32), (tail_x, tail_y): &(i32, i32)) -> (i32, i32) {
    ((head_x - tail_x), (head_y - tail_y))
}

/// Taking a distance as an argument, determine if the knot is "detached" from the next one
fn is_detached((hx, hy): &(i32, i32)) -> bool {
    hx.abs() > 1 || hy.abs() > 1
}

pub fn solution(input_path: &str, rope_size: usize) -> Option<usize> {
    let mut rope = Rope::new(rope_size);
    std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .filter_map(|x| x.split_once(' '))
        .map(parse_direction)
        .for_each(|x| rope.move_rope(&x));
    Some(rope.visited_by_tails.len())
}

fn parse_direction((x, y): (&str, &str)) -> RopeMove {
    let count = y.parse::<i32>().unwrap();
    match x {
        "U" => RopeMove {
            direction: Direction::Up,
            count,
        },
        "D" => RopeMove {
            direction: Direction::Down,
            count,
        },
        "R" => RopeMove {
            direction: Direction::Right,
            count,
        },
        "L" => RopeMove {
            direction: Direction::Left,
            count,
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    pub fn test_part1() {
        assert_eq!(solution("input_test_part1.txt", 2), Some(13))
    }

    #[test]
    pub fn test_part2_exemple1() {
        assert_eq!(solution("input_test_part1.txt", 10), Some(1))
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(solution("input_test_part2.txt", 10), Some(36))
    }
}
