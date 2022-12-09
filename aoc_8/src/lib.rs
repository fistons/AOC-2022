use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

use take_until::TakeUntilExt;

#[derive(Eq, PartialEq)]
struct Tree {
    pub x: usize,
    pub y: usize,
    pub value: u32,
}

impl Debug for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}[{}]", self.x, self.y, self.value)
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

pub fn part1(input_path: &str) -> Option<u32> {
    let patch = parse_input(input_path)?;
    let width = patch[0].len() - 1;
    let height = patch.len() - 1;
    let mut visibles = (width + height) * 2;

    for x_tree in 1..patch[0].len() - 1 {
        for y_tree in 1..patch.len() - 1 {
            let current_tree = &patch[x_tree][y_tree];

            // Get the line of the current tree
            let line = &patch[x_tree];
            // Get the column of the current tree
            let col = &patch
                .iter()
                .map(|value| &value[y_tree])
                .collect::<Vec<&Tree>>();

            /* Split the line and column on the current tree */
            let mut split_line = line.split(|other_tree| other_tree == current_tree);
            let mut split_column = col.split(|other_tree| other_tree == &current_tree);

            /* For each side, we retrieve the optional max height of all the tree */
            let left = split_line.next()?.iter().max();
            let right = split_line.next()?.iter().max();
            let top = split_column.next()?.iter().max();
            let bottom = split_column.next()?.iter().max();

            // If there is a taller tree of all 4 sides, then the tree is NOT visible, else, well,
            // it is.
            match (right, left, top, bottom) {
                (Some(r), Some(l), Some(t), Some(b))
                    if r >= current_tree
                        && l >= current_tree
                        && t >= &current_tree
                        && b >= &current_tree => {}
                _ => {
                    visibles += 1;
                }
            }
        }
    }
    Some(visibles as u32)
}

pub fn part2(input_path: &str) -> Option<usize> {
    let patch = parse_input(input_path)?;
    let mut scenic_scores = 0;
    for x_tree in 1..patch[0].len() - 1 {
        for y_tree in 1..patch.len() - 1 {
            let current_tree = &patch[x_tree][y_tree];

            // Get the line of the current tree
            let line = &patch[x_tree];
            // Get the column of the current tree
            let col = &patch
                .iter()
                .map(|value| &value[y_tree])
                .collect::<Vec<&Tree>>();

            /* Split the line and column on the current tree */
            let mut split_line = line.split(|other_tree| other_tree == current_tree);
            let mut split_column = col.split(|other_tree| other_tree == &current_tree);

            // For each side, count the number of tree until we encounter a higher tree
            let left = split_line
                .next()?
                .iter()
                .rev() // Trees are on the left, we need to read the list from right to left
                .take_until(|x| current_tree <= x)
                .count();
            let right = split_line
                .next()?
                .iter()
                .take_until(|x| current_tree <= x)
                .count();
            let top = split_column
                .next()?
                .iter()
                .rev() // Trees are on the top, we need to read the list from bottom to top
                .take_until(|x| current_tree <= x)
                .count();
            let bottom = split_column
                .next()?
                .iter()
                .take_until(|x| current_tree <= x)
                .count();

            scenic_scores = scenic_scores.max(right * left * top * bottom);
        }
    }
    Some(scenic_scores)
}

fn parse_input(input_path: &str) -> Option<Vec<Vec<Tree>>> {
    let mut patch: Vec<Vec<Tree>> = vec![];

    std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .enumerate()
        .for_each(|(index_x, line)| {
            patch.push(
                line.split_whitespace()
                    .flat_map(|line| line.chars().enumerate())
                    .filter_map(|(index_y, value)| {
                        let value = value.to_digit(10)?;
                        Some(Tree {
                            x: index_x,
                            y: index_y,
                            value,
                        })
                    })
                    .collect(),
            );
        });
    Some(patch)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test_part1.txt"), Some(21))
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2("input_test_part1.txt"), Some(8))
    }
}
