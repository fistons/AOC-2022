extern crate core;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;

struct Valve {
    name: String,
    flow: i32,
    neighbors: Vec<String>,
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Hash for Valve {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Eq for Valve {}

impl Debug for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.flow)
    }
}

#[derive(Debug)]
struct ToVisit<'a> {
    valve: &'a Valve,
    distance: usize,
}

impl<'a> Ord for ToVisit<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<'a> PartialEq<Self> for ToVisit<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<'a> PartialOrd for ToVisit<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Eq for ToVisit<'a> {}

#[derive(Debug, Default)]
struct Path<'a> {
    flow: i32,
    path: Vec<&'a Valve>,
}

impl<'a> Eq for Path<'a> {}

impl<'a> PartialEq<Self> for Path<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

impl<'a> PartialOrd<Self> for Path<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Path<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.flow.cmp(&other.flow)
    }
}

type DistancesTo = HashMap<String, HashMap<String, usize>>; // source -> [destination -> distance]

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$").unwrap()
});

pub fn part1(input_path: &str) -> Option<i32> {
    let valves = std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .map(parse)
        .collect::<Vec<Valve>>();

    let non_zero_valves = valves
        .iter()
        .filter(|v| v.flow > 0)
        .collect::<Vec<&Valve>>();

    let valves_map = valves
        .iter()
        .map(|x| (x.name.clone(), x))
        .collect::<HashMap<String, &Valve>>();

    let distance_to: DistancesTo = build_distance_matrix(&valves, &valves_map);
    let Path { path, flow } = find_max_flow(
        valves_map["AA"],
        &distance_to,
        &non_zero_valves,
        &[valves_map["AA"]],
        30,
    );
    println!("Flow: {flow} - path {path:?}");

    Some(flow)
}

pub fn part2(input_path: &str) -> Option<i32> {
    let valves = std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .map(parse)
        .collect::<Vec<Valve>>();

    let non_zero_valves = valves
        .iter()
        .filter(|v| v.flow > 0)
        .collect::<Vec<&Valve>>();

    let valves_map = valves
        .iter()
        .map(|x| (x.name.clone(), x))
        .collect::<HashMap<String, &Valve>>();

    let distance_to: DistancesTo = build_distance_matrix(&valves, &valves_map);
    let start_valve = valves_map["AA"];

    let Path {
        path: human_path,
        flow: human_flow,
    } = find_max_flow(
        start_valve,
        &distance_to,
        &non_zero_valves,
        &[start_valve],
        26,
    );

    let remaining_valves = non_zero_valves
        .into_iter()
        .filter(|v| human_path.contains(v))
        .collect();

    println!("Human flow: {human_flow} - path {human_path:?}");
    println!("Remaining valves: {remaining_valves:?}");

    let Path {
        path: elephant_path,
        flow: elephant_flow,
    } = find_max_flow(
        start_valve,
        &distance_to,
        &remaining_valves,
        &[start_valve],
        26,
    );

    println!("Elephant flow: {elephant_flow} - path {elephant_path:?}");

    Some(elephant_flow + human_flow)
}

fn find_max_flow<'a>(
    start_valve: &'a Valve,
    distances: &DistancesTo,
    to_open: &Vec<&'a Valve>,
    current_path: &[&'a Valve],
    minutes: i32,
) -> Path<'a> {
    to_open
        .par_iter()
        .filter_map(|valve| {
            let distance = distances[&start_valve.name][&valve.name] as i32;
            if distance >= minutes {
                None
            } else {
                let minutes_left = minutes - distance - 1; // Time left - time to go to the valve - 1 minute to open the valve.
                let flow = valve.flow * minutes_left; // Total flow of the valve until the end
                let to_open = to_open
                    .iter()
                    .filter_map(|v| if v.name == valve.name { None } else { Some(*v) })
                    .collect(); // remaining valves to open

                let mut next_path = current_path.to_owned();
                next_path.push(valve);
                let path = find_max_flow(valve, distances, &to_open, &next_path, minutes_left);

                let mut new_complete_path = current_path.to_owned();
                new_complete_path.extend(path.path);

                Some(Path {
                    path: new_complete_path,
                    flow: path.flow + flow,
                })
            }
        })
        .max()
        .unwrap_or_default()
}

fn build_distance_matrix(valves: &Vec<Valve>, valve_map: &HashMap<String, &Valve>) -> DistancesTo {
    let mut distances = HashMap::new();
    for start_valve in valves {
        // For each valve, we will compute the distance to all other valves
        let distances_from: &mut HashMap<String, usize> =
            distances.entry(start_valve.name.clone()).or_default(); // Build or get the map of distance for the current start_valve.
        let mut visited: HashSet<&str> = HashSet::new(); // Node already visited, stating by the start_valve
        let mut to_visit: BinaryHeap<ToVisit> = BinaryHeap::new(); // Binary heap to store the next valves to visit, from the nearest to the farthest

        to_visit.push(ToVisit {
            valve: start_valve,
            distance: 0,
        });

        while let Some(ToVisit { valve, distance }) = to_visit.pop() {
            // If the node has already been visited, let's continue to the next node
            if !visited.insert(&valve.name) {
                continue;
            }
            for neighbour in &valve.neighbors {
                if let Some(neighbour) = valve_map.get(neighbour) {
                    let new_distance = distance + 1; // We are one "cave" further in the gallery
                    let use_distance = distances_from
                        .get(&neighbour.name)
                        .map_or(true, |&current_distance| current_distance > new_distance); // if there is no precedent link, use the new computed distance. If there is one, use the new distance if it's shorter than the current one
                    if use_distance {
                        distances_from.insert(neighbour.name.clone(), new_distance);
                        to_visit.push(ToVisit {
                            valve: neighbour,
                            distance: new_distance,
                        });
                    }
                }
            }
        }
    }

    distances
}

fn parse(line: &str) -> Valve {
    let captures = REGEX.captures(line).unwrap();
    let name = captures[1].to_owned();
    let rate = captures[2].parse::<i32>().unwrap();
    let neighbors = captures[3]
        .split(',')
        .map(|s| s.trim().to_owned())
        .collect::<Vec<String>>();

    Valve {
        name,
        flow: rate,
        neighbors,
    }
}
#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test.txt"), Some(1651));
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2("input_test.txt"), Some(1707));
    }
}
