use std::collections::HashSet;
use std::ops::RangeInclusive;

use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Ord, PartialOrd)]
struct Position(isize, isize);

#[derive(Debug)]
struct Sensor {
    position: Position,
    distance: isize,
}

impl Sensor {
    pub fn new(position: Position, beacon_position: &Position) -> Self {
        let distance = Sensor::compute_distance(&position, beacon_position);
        Sensor { position, distance }
    }

    /// Compute the Manhattan distance between two point
    fn compute_distance(p1: &Position, p2: &Position) -> isize {
        (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
    }

    /// Return the range scanned by the sensors on a given y. If the area is no scanned at, return
    /// None
    fn get_x_range_for_y(&self, y_coord: &isize) -> Option<RangeInclusive<isize>> {
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
pub fn part1(input_path: &str, y_to_scan: isize) -> Option<usize> {
    let input = std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .map(parse_input)
        .collect::<Vec<(Sensor, Beacon)>>();

    // Find all the beacons on y
    let beacons_position_on_y = input
        .par_iter()
        .map(|(_, b)| b.position)
        .filter(|b| b.1 == y_to_scan)
        .collect::<HashSet<Position>>();

    // Find all the ranges covered by the sensors on y
    let ranges = input
        .par_iter()
        .filter_map(|(s, _)| s.get_x_range_for_y(&y_to_scan))
        .collect::<Vec<RangeInclusive<isize>>>();

    // Min and Max of the x covered by the sensors
    let min_x = *ranges.iter().map(RangeInclusive::start).min().unwrap();
    let max_x = *ranges.iter().map(RangeInclusive::end).max().unwrap();

    // Return each position scanned by any sensors.
    // Note: it would more efficient to build a global coverage range.
    let scanned_positions = (min_x..=max_x)
        .into_par_iter()
        .filter_map(|x| {
            let position = Position(x, y_to_scan);
            ranges
                .par_iter()
                .find_map_any(|r| if r.contains(&x) { Some(position) } else { None })
        })
        .collect::<Vec<Position>>();

    // Number of scanned position, less the kevin beacons
    Some(scanned_positions.len() - beacons_position_on_y.len())
}

pub fn part2(input_path: &str, max_size: isize) -> Option<isize> {
    let input = std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .map(parse_input)
        .map(|(s, _)| s)
        .collect::<Vec<Sensor>>();

    let pos = find_the_spot(&input, max_size)?;
    Some((pos.0 * 4000000) + pos.1)
}

/// Find the "hole" in the sensor coverage
fn find_the_spot(sensors: &[Sensor], max_size: isize) -> Option<Position> {
    (0..=max_size).into_par_iter().find_map_any(|y| {
        // Build the range covered by the sensors on the y line
        let mut ranges = sensors
            .par_iter()
            .flat_map(|x| x.get_x_range_for_y(&y))
            .collect::<Vec<RangeInclusive<isize>>>();

        // Search for hole in all those ranges
        search_for_hole_in_ranges(&mut ranges, max_size).map(|x| Position(x, y))
    })
}

/// Takes a slice of ranges, and check for "holes" in it (ie value not covered by any of the ranges)
/// , stopping to the first one we find. Limits the range to scan from 0 to `max_width` included.
/// Not my proudest code, but does the job
fn search_for_hole_in_ranges(
    ranges: &mut [RangeInclusive<isize>],
    max_width: isize,
) -> Option<isize> {
    ranges.sort_by(|a, b| a.start().cmp(b.start())); // Sort the ranges

    let mut range_coverage = &0isize..=ranges[0].end().max(&1); // Initial range, from 0 to as least 1.
    for r in &ranges[1..] {
        if *range_coverage.end() + 1 < *r.start() {
            // If the end of the global range ends before the begin of the next range, there it is, we have our hole.
            return Some(*range_coverage.end() + 1);
        } else {
            // "Extends the range
            range_coverage =
                *range_coverage.start()..=r.end().clamp(range_coverage.end(), &max_width);
        }
    }
    None
}

/// Parse a line of the input to return a tuple of Set/Beacon.
/// This is so ugly, please forgive me.
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

    let sensor_x = sensor_x.parse::<isize>().unwrap();
    let sensor_y = sensor_y.parse::<isize>().unwrap();

    let kevin_beacon_x = kevin_beacon_x.parse::<isize>().unwrap();
    let kevin_beacon_y = kevin_beacon_y.parse::<isize>().unwrap();

    let kevin_beacon = Beacon {
        position: Position(kevin_beacon_x, kevin_beacon_y),
    };
    let sensors = Sensor::new(Position(sensor_x, sensor_y), &kevin_beacon.position);
    (sensors, kevin_beacon)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Position, Sensor};

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
}
