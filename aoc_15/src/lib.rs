use std::cmp;
use std::collections::HashSet;
use std::ops::RangeInclusive;

use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Ord, PartialOrd)]
struct Position(i32, i32);

#[derive(Debug)]
struct Sensor {
    position: Position,
    distance: i32,
}

impl Sensor {
    pub fn new(position: Position, beacon_position: &Position) -> Self {
        let distance = Sensor::compute_distance(&position, beacon_position);
        Sensor { position, distance }
    }

    pub fn in_area(&self, position: &Position) -> bool {
        let distance = Sensor::compute_distance(&self.position, position);
        distance <= self.distance
    }

    fn compute_distance(p1: &Position, p2: &Position) -> i32 {
        (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
    }

    fn get_x_range_for_y(&self, y_coord: &i32) -> Option<RangeInclusive<i32>> {
        let x_diff = self.distance - (self.position.1 - y_coord).abs();
        if x_diff < 0 {
            None
        } else {
            Some((self.position.0 - x_diff)..=(self.position.0 + x_diff))
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Beacon {
    position: Position,
}
pub fn part1(input_path: &str, y_to_scan: i32) -> Option<usize> {
    let input = std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .map(parse_input)
        .collect::<Vec<(Sensor, Beacon)>>();

    let beacons_position_on_y = input
        .iter()
        .map(|(_, b)| b.position)
        .filter(|b| b.1 == y_to_scan)
        .collect::<HashSet<Position>>();

    let mut scanned_positions = HashSet::new();

    let ranges = input
        .iter()
        .filter_map(|(s, _)| s.get_x_range_for_y(&y_to_scan))
        .collect::<Vec<RangeInclusive<i32>>>();

    let min_x = *ranges.iter().map(RangeInclusive::start).min().unwrap();
    let max_x = *ranges.iter().map(RangeInclusive::end).max().unwrap();
    // Determine beacons on y_to_scan by brute forcing like a champ
    for x in min_x..=max_x {
        let position = Position(x, y_to_scan);

        if ranges.iter().any(|r| r.contains(&x)) {
            scanned_positions.insert(position);
        }
    }
    let mut scanned_positions = scanned_positions.iter().collect::<Vec<&Position>>();
    scanned_positions.sort();
    Some(scanned_positions.len() - beacons_position_on_y.len())
}

pub fn part2(input_path: &str, max_size: i32) -> Option<i32> {
    let input = std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .map(parse_input)
        .map(|(s, _)| s)
        .collect::<Vec<Sensor>>();

    let pos = find_the_spot(&input, max_size)?;
    Some((pos.0 * 4000000) + pos.1)
}
fn find_the_spot(sensors: &[Sensor], max_size: i32) -> Option<Position> {
    for y in 0..=max_size {
        let ranges = sensors
            .par_iter()
            .flat_map(|x| x.get_x_range_for_y(&y))
            .collect::<Vec<RangeInclusive<i32>>>();

        // let mut sad = HashSet::new();
        // ranges.into_iter().for_each(|x| {
        //     for i in x {
        //         sad.insert(i);
        //     }
        // });
        // let mut sad = sad.into_iter().collect::<Vec<i32>>();
        // sad.sort();
        // println!("{ranges:?}");
        let min_x = *ranges.iter().map(|x| x.start()).min().unwrap().max(&0);
        let max_x = *ranges.iter().map(|x| x.end()).max().unwrap().min(&max_size);

        println!("{y}");

        // let mut i = min_x;
        // let mut s = sad.into_iter().filter(|x| *x >= 0 && *x <= max_size);
        // loop {
        //     if let Some(next) = s.next() {
        //         // println!("{i} {next}");
        //         if next != i {
        //             return Some(Position(i, y));
        //         }
        //     } else {
        //         println!("break");
        //         break;
        //     }
        //     i += 1;
        // }

        for x in 0.max(min_x)..=max_size.min(max_x) {
            if !ranges.iter().any(|r| r.contains(&x)) {
                return Some(Position(x, y));
            }
        }
    }

    None
}
fn parse_input(line: &str) -> (Sensor, Beacon) {
    let split = line.split_once(':').unwrap();

    let sensor = split.0;
    let kevin_beacon = split.1;

    let sensor = sensor.split_once(',').unwrap();
    let kevin_beacon = kevin_beacon.split_once(',').unwrap();

    let sensors_x = sensor.0;
    let sensors_y = sensor.1;

    let kevin_beacon_x = kevin_beacon.0;
    let kevin_beacon_y = kevin_beacon.1;

    let sensor_x = sensors_x.split_once('=').unwrap().1;
    let sensor_y = sensors_y.split_once('=').unwrap().1;

    let kevin_beacon_x = kevin_beacon_x.split_once('=').unwrap().1;
    let kevin_beacon_y = kevin_beacon_y.split_once('=').unwrap().1;

    let sensor_x = sensor_x.parse::<i32>().unwrap();
    let sensor_y = sensor_y.parse::<i32>().unwrap();

    let kevin_beacon_x = kevin_beacon_x.parse::<i32>().unwrap();
    let kevin_beacon_y = kevin_beacon_y.parse::<i32>().unwrap();

    let kevin_beacon = Beacon {
        position: Position(kevin_beacon_x, kevin_beacon_y),
    };
    let sensors = Sensor::new(Position(sensor_x, sensor_y), &kevin_beacon.position);
    (sensors, kevin_beacon)
}

fn overlapping(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
    // println!("Comparing {a:?} and {b:?}");
    if !(b.start() <= a.end() || b.end() >= a.start()) {
        println!("ho shit");
        return None; //Not overlapping
    }

    let min = *a.start().min(b.start());
    let max = *b.end().max(a.end());

    Some(min..=max)
}

fn overlaps(one: &RangeInclusive<i32>, other: &RangeInclusive<i32>) -> bool {
    (other.start() >= one.start() && other.start() <= one.end())
        || (other.end() >= one.start() && other.end() <= one.end())
}

fn merge(one: &RangeInclusive<i32>, other: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    cmp::min(*one.start(), *other.start())..=cmp::max(*one.end(), *other.end())
}

#[cfg(test)]
mod tests {
    use crate::{overlapping, part1, part2, Position, Sensor};

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test.txt", 10), Some(26));
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2("input_test.txt", 20), Some(56000011));
    }

    #[test]
    pub fn test_x_range() {
        let sensor = Sensor::new(Position(8, 7), &Position(2, 10));

        assert_eq!(sensor.distance, 9);
        assert_eq!(sensor.get_x_range_for_y(&7), Some(-1..=17));
        assert_eq!(sensor.get_x_range_for_y(&17), None);
        assert_eq!(sensor.get_x_range_for_y(&16), Some(8..=8));
        assert_eq!(sensor.get_x_range_for_y(&15), Some(7..=9));

        assert_eq!(sensor.get_x_range_for_y(&-3), None);
        assert_eq!(sensor.get_x_range_for_y(&-2), Some(8..=8));
        assert_eq!(sensor.get_x_range_for_y(&-1), Some(7..=9));
    }

    #[test]
    pub fn test_intersection() {
        let a = 0..=5;
        let b = 3..=8;

        let r = overlapping(&a, &b);
        assert_eq!(r, Some(0..=8));
        let r = overlapping(&b, &a);
        assert_eq!(r, Some(0..=8));
    }
}
