use std::collections::HashSet;
use std::env::current_exe;

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
#[derive(Debug, Clone)]
struct Rope {
    head: (i32, i32), // useful?
    knots: Vec<(i32, i32)>,
    visited_by_tails: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(size: usize) -> Self {
        let mut visited_by_tails = HashSet::new();
        let knots = vec![(0, 0); size]; // We keep the head
        visited_by_tails.insert((0, 0));

        Rope {
            head: (0, 0),
            knots,
            visited_by_tails,
        }
    }

    pub fn move_rope2(&mut self, direction: &RopeMove) {
        println!("{direction:?}");

        let len = self.knots.len() as i32;

        // Move the head one by one
        for _ in 0..direction.count {
            match direction.direction {
                Direction::Up => self.head.1 += 1,
                Direction::Down => self.head.1 -= 1,
                Direction::Right => self.head.0 += 1,
                Direction::Left => self.head.0 -= 1,
            }
            println!("New head {:?}", self.head);

            for i in 0..len as usize {
                let precedent_knot_or_head = if i == 0 { self.head } else { self.knots[i - 1] };
                let mut current_knot = self.knots.get_mut(i).unwrap();

                let distance = distance(&precedent_knot_or_head, current_knot);
                if !neighbors(&distance) {
                    current_knot.0 += distance.0.signum();
                    current_knot.1 += distance.1.signum();
                }
            }
            self.print_tail();
            println!("---")
        }
        println!("{self:?}");
        self.print_tail();
    }

    fn print_tail(&self) {
        let max_x = self.visited_by_tails.iter().map(|x| x.0).max().unwrap();
        let min_x = self.visited_by_tails.iter().map(|x| x.0).min().unwrap();
        let max_y = self.visited_by_tails.iter().map(|x| x.1).max().unwrap();
        let min_y = self.visited_by_tails.iter().map(|x| x.1).min().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.visited_by_tails.contains(&(x, y)) {
                    print!("X ");
                } else {
                    print!(". ");
                }
            }
            println!()
        }
    }
}

fn distance((head_x, head_y): &(i32, i32), (tail_x, tail_y): &(i32, i32)) -> (i32, i32) {
    ((head_x - tail_x).abs(), (head_y - tail_y).abs())
}

fn neighbors((hx, hy): &(i32, i32)) -> bool {
    *hx > 1 || *hy > 1
}

fn move_knot((knot_x, knot_y): &mut (i32, i32), (hx, hy): &(i32, i32)) {}

pub fn solution(input_path: &str, rope_size: i32) -> Option<usize> {
    let mut rope = Rope::new(rope_size as usize);
    std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .filter_map(|x| x.split_once(' '))
        .map(parse_direction)
        .for_each(|x| rope.move_rope2(&x));

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
        assert_eq!(solution("input_test_part1.txt", 1), Some(13))
    }

    #[test]
    pub fn test_part2_exemple1() {
        assert_eq!(solution("input_test_part1.txt", 9), Some(1))
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(solution("input_test_part2.txt", 9), Some(36))
    }
}
