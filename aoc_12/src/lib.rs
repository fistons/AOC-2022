use std::collections::HashMap;

use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

struct Grid {
    start_position: (isize, isize),
    end_position: (isize, isize),
    h_map: HashMap<(isize, isize), char>,
    lowest_position: Vec<(isize, isize)>,
}

impl Grid {
    pub fn new(input_path: &str) -> Self {
        let mut start_position: (isize, isize) = (0, 0);
        let mut end_position: (isize, isize) = (0, 0);
        let mut h_map: HashMap<(isize, isize), char> = HashMap::new();
        let mut lowest_position = vec![];

        let input = std::fs::read_to_string(input_path).unwrap();
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let position = (x as isize, y as isize);
                h_map.insert(position, c);
                match c {
                    'S' => {
                        start_position = position;
                        lowest_position.push(position);
                        h_map.insert(position, 'a');
                    }
                    'E' => {
                        end_position = position;
                        h_map.insert(position, 'z');
                    }
                    'a' => {
                        lowest_position.push(position);
                    }
                    _ => (),
                }
            })
        });

        Grid {
            start_position,
            end_position,
            h_map,
            lowest_position,
        }
    }

    fn shortest_path(&self, start_position: &(isize, isize)) -> Option<u32> {
        let mut visited_node: Vec<(isize, isize)> = vec![];
        let mut walkback_map: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
        let mut to_explore: Vec<(isize, isize)> = vec![];

        let possibles_directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

        /* Build the BSP tree https://en.wikipedia.org/wiki/Breadth-first_search */
        to_explore.push(*start_position);
        visited_node.push(*start_position);
        while !to_explore.is_empty() {
            let node = to_explore.remove(0);
            let node_value = self.h_map.get(&node).unwrap();

            if node == self.end_position {
                // We found the end!
                break;
            }
            for (x, y) in &possibles_directions {
                let neighbor_position = (x + node.0, y + node.1);
                if !visited_node.contains(&neighbor_position) {
                    if let Some(neighbor) = self.h_map.get(&neighbor_position) {
                        if *neighbor as i32 - (*node_value as i32) <= 1 {
                            visited_node.push(neighbor_position);
                            to_explore.push(neighbor_position);

                            // Keep the 'parent' position of the neighbor
                            walkback_map.insert(neighbor_position, node);
                        }
                    }
                }
            }
        }

        // Now let's go backward from the position, if found
        if let Some(mut walkback) = walkback_map.get(&self.end_position) {
            let mut i = 1;
            while walkback != start_position {
                walkback = walkback_map.get(walkback).unwrap();
                i += 1;
            }
            Some(i)
        } else {
            None // We can't find the end_position. No path was doable from the start_position
        }
    }
}

pub fn part1(input_path: &str) -> Option<u32> {
    let grid = Grid::new(input_path);
    grid.shortest_path(&grid.start_position)
}

pub fn part2(input_path: &str) -> Option<u32> {
    let grid = Grid::new(input_path);
    grid.lowest_position
        .par_iter()
        .filter_map(|position| grid.shortest_path(position))
        .min()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test.txt"), Some(31));
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2("input_test.txt"), Some(29))
    }
}
