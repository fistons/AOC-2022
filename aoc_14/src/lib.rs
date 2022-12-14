use std::collections::HashMap;

#[derive(Debug, PartialOrd, PartialEq)]
enum UnitType {
    Rock,
    Sand,
}

#[derive(Debug)]
enum SandStatus {
    Flowing(u32, u32),
    Blocked(u32, u32),
    IntoTheDepthOfEternityOrSomething,
}

#[derive(Debug)]
struct Unit {
    unit_type: UnitType,
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Cave {
    map: HashMap<(u32, u32), Unit>,
    max_height: u32,
    infinite: bool,
}

impl Cave {
    pub fn new(input: &str, infinite: bool) -> Self {
        let map = input
            .lines()
            .flat_map(parse_line)
            .map(|u| ((u.x, u.y), u))
            .collect::<HashMap<(u32, u32), Unit>>();
        let max_height = Cave::max_depth(&map);

        Cave {
            map,
            max_height,
            infinite,
        }
    }

    pub fn max_depth(map: &HashMap<(u32, u32), Unit>) -> u32 {
        *map.keys().map(|(_, y)| y).max().unwrap()
    }

    pub fn count_sands(&self) -> u32 {
        self.map
            .values()
            .filter(|x| x.unit_type == UnitType::Sand)
            .count() as u32
    }
}
fn parse_line(line: &str) -> Vec<Unit> {
    line.split("->")
        .map(|x| x.trim().split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .map(|(x, y)| Unit {
            unit_type: UnitType::Rock,
            x,
            y,
        })
        .collect::<Vec<Unit>>()
        .windows(2)
        .flat_map(build_between_edge)
        .collect::<Vec<Unit>>()
}

fn build_between_edge(edges: &[Unit]) -> Vec<Unit> {
    let u1 = &edges[0];
    let u2 = &edges[1];
    let mut in_between: Vec<Unit> = vec![];

    for x in u1.x.min(u2.x)..=u2.x.max(u1.x) {
        for y in u1.y.min(u2.y)..=u2.y.max(u1.y) {
            let u = Unit {
                unit_type: UnitType::Rock,
                x,
                y,
            };
            in_between.push(u);
        }
    }

    in_between
}

fn simulate(cave: &Cave, sand: &(u32, u32)) -> SandStatus {
    let possible_position = vec![
        (sand.0, sand.1 + 1),
        (sand.0 - 1, sand.1 + 1),
        (sand.0 + 1, sand.1 + 1),
    ];

    for (x, y) in possible_position {
        if !cave.map.contains_key(&(x, y)) {
            return if y > cave.max_height + 1 {
                if cave.infinite {
                    SandStatus::IntoTheDepthOfEternityOrSomething
                } else {
                    SandStatus::Blocked(sand.0, sand.1)
                }
            } else {
                SandStatus::Flowing(x, y)
            };
        }
    }
    SandStatus::Blocked(sand.0, sand.1)
}

pub fn solution(input_path: &str, infinite: bool) -> Option<u32> {
    let map = std::fs::read_to_string(input_path).ok()?;

    let mut cave = Cave::new(&map, infinite);
    let mut current_sand = (500, 0);

    loop {
        let status = simulate(&cave, &current_sand);
        match status {
            SandStatus::Blocked(x, y) => {
                cave.map.insert(
                    (x, y),
                    Unit {
                        unit_type: UnitType::Sand,
                        x,
                        y,
                    },
                );

                if (x, y) == (500, 0) {
                    // Blocked at the start point
                    break;
                }

                current_sand = (500, 0);
            }
            SandStatus::Flowing(x, y) => {
                current_sand = (x, y);
            }
            SandStatus::IntoTheDepthOfEternityOrSomething => {
                break;
            }
        }
    }

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
