use std::collections::HashMap;

#[derive(PartialEq)]
enum UnitType {
    Rock,
    Sand,
}

enum SandStatus {
    Flowing(u32, u32),
    Blocked(u32, u32),
    IntoTheDepthOfEternityOrSomething,
}

struct Unit {
    unit_type: UnitType,
    x: u32,
    y: u32,
}

pub struct Cave {
    map: HashMap<(u32, u32), Unit>,
    sand: (u32, u32),
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
            sand: (500, 0),
            max_height,
            infinite,
        }
    }

    /// Compute the max depth of the cave
    fn max_depth(map: &HashMap<(u32, u32), Unit>) -> u32 {
        *map.keys().map(|(_, y)| y).max().unwrap()
    }

    /// Simulate a turn, moving the sand downward
    fn simulate_turn(&mut self) -> SandStatus {
        // Sand will try to flow below, left below and right below, in this order
        let possible_positions = vec![
            (self.sand.0, self.sand.1 + 1),
            (self.sand.0 - 1, self.sand.1 + 1),
            (self.sand.0 + 1, self.sand.1 + 1),
        ];

        for (x, y) in possible_positions {
            if !self.map.contains_key(&(x, y)) {
                return if y > self.max_height + 1 {
                    if self.infinite {
                        // Part 1, the Infinite Darkness of the Void below us will feed on the Sand
                        // we send to It
                        SandStatus::IntoTheDepthOfEternityOrSomething
                    } else {
                        // Part 2, we consider we hit the ground and are blocked
                        SandStatus::Blocked(self.sand.0, self.sand.1)
                    }
                } else {
                    // We keep flowing
                    SandStatus::Flowing(x, y)
                };
            }
        }

        // If we are here, that mean that the sand is blocked. Sending coordinate.
        SandStatus::Blocked(self.sand.0, self.sand.1)
    }

    pub fn run_simulation(&mut self) {
        loop {
            let status = self.simulate_turn();
            match status {
                SandStatus::Blocked(x, y) => {
                    // The sand is blocked, let's insert it in the map
                    self.map.insert(
                        (x, y),
                        Unit {
                            unit_type: UnitType::Sand,
                            x,
                            y,
                        },
                    );

                    if (x, y) == (500, 0) {
                        // Blocked at the starting point, Part 2 is done
                        break;
                    }

                    // New sand!
                    self.sand = (500, 0);
                }
                SandStatus::Flowing(x, y) => {
                    // Sand is flowing, set it to new position
                    self.sand = (x, y);
                }
                SandStatus::IntoTheDepthOfEternityOrSomething => {
                    // We have reach the eternal depth of damnation or I don't know. Part 1 id done
                    break;
                }
            }
        }
    }
    // Count the number of sand unit currently on the map
    pub fn count_sands(&self) -> usize {
        self.map
            .values()
            .filter(|x| x.unit_type == UnitType::Sand)
            .count()
    }
}

/// Parse a line, building a list of Unit
fn parse_line(line: &str) -> Vec<Unit> {
    line.split("->") // Split the line on ->
        .map(|x| x.trim().split_once(',').unwrap()) // Split the coordinate on ,
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())) // Build a tuple of x,y
        .map(|(x, y)| Unit {
            unit_type: UnitType::Rock,
            x,
            y,
        }) // Build a Rock
        .collect::<Vec<Unit>>() // Here comes a list of "edge" Unit, we need to fill the gapes
        .windows(2) // For this, we take the Unit by pair
        .flat_map(build_between_edge) // And we build a new list with the gape filled
        .collect::<Vec<Unit>>()
}

/// Build a list of all Unit between two edges
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
