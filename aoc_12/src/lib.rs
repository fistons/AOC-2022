use std::collections::HashMap;

pub fn part1(input_path: &str) -> Option<u32> {
    let mut start_position: (isize, isize) = (0, 0);
    let mut end_position: (isize, isize) = (0, 0);
    let mut visited_node: Vec<(isize, isize)> = vec![];
    let mut to_explore: Vec<(isize, isize)> = vec![];
    let mut h_map: HashMap<(isize, isize), char> = HashMap::new();
    let mut walkback_map: HashMap<(isize, isize), (isize, isize)> = HashMap::new();

    let input = std::fs::read_to_string(input_path).ok()?;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            'S' => {
                start_position = (x as isize, y as isize);
                h_map.insert((x as isize, y as isize), 'a');
            }
            'E' => {
                end_position = (x as isize, y as isize);
                h_map.insert((x as isize, y as isize), 'z');
            }
            _ => {
                h_map.insert((x as isize, y as isize), c);
            }
        })
    });

    to_explore.push(start_position);
    visited_node.push(start_position);

    while !to_explore.is_empty() {
        let node = to_explore.remove(0);
        let node_value = h_map.get(&node).unwrap();

        if node == end_position {
            break;
        }

        for x in node.0 - 1..=node.0 + 1 {
            for y in node.1 - 1..=node.1 + 1 {
                // We don't treat the diagonals
                if (node.0 - x).abs() == 1 && (node.1 - y).abs() == 1 {
                    continue;
                }

                if !visited_node.contains(&(x, y)) {
                    if let Some(neighbor) = h_map.get(&(x, y)) {
                        if *neighbor as i32 - (*node_value as i32) <= 1 {
                            visited_node.push((x, y));
                            to_explore.push((x, y));
                            walkback_map.insert((x, y), node);
                        }
                    }
                }
            }
        }
    }

    let mut walkback = walkback_map.get(&end_position).unwrap();
    let mut i = 1;
    while walkback != &start_position {
        walkback = walkback_map.get(walkback).unwrap();
        i += 1;
    }

    Some(i)
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test.txt"), Some(31))
    }
}
