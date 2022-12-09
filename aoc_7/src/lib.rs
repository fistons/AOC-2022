use std::collections::HashMap;

#[derive(Debug)]
enum Entry {
    Cd(String),
    GoUp,
    File(u32, String),
}

pub fn part1(input_path: &str) -> Option<u32> {
    let map = my_own_du(input_path)?;

    Some(
        map.iter()
            .filter(|(_, y)| **y <= 100000)
            .map(|(_, y)| y)
            .sum(),
    )
}

pub fn part2(input_path: &str) -> Option<u32> {
    let map = my_own_du(input_path)?;

    let total_size = 70000000;
    let needed_space = 30000000;
    let unused_space = total_size - map.get("/").unwrap();
    let space_to_clean = needed_space - unused_space;

    map.iter()
        .filter_map(|(_, y)| if *y >= space_to_clean { Some(*y) } else { None })
        .min()
}

fn my_own_du(input_path: &str) -> Option<HashMap<String, u32>> {
    let mut map = HashMap::<String, u32>::new();
    let mut directory_path: Vec<String> = vec![];

    std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .filter_map(parse_line)
        .for_each(|entry| match entry {
            Entry::File(file_size, _file_name) => {
                for directory in &directory_path {
                    *map.entry(directory.clone()).or_default() += file_size;
                }
            }
            Entry::Cd(change_dir) => {
                let mut complete_path = directory_path.join("/");
                complete_path.push_str(&change_dir);
                directory_path.push(complete_path);
            }
            Entry::GoUp => {
                directory_path.pop().unwrap();
            }
        });

    Some(map)
}

fn parse_line(line: &str) -> Option<Entry> {
    match line {
        x if x.starts_with("$ cd ..") => Some(Entry::GoUp),
        x if x.starts_with("$ cd ") => {
            let directory_name = x.split_whitespace().collect::<Vec<&str>>()[2];
            Some(Entry::Cd(directory_name.to_owned()))
        }
        x if x.starts_with(char::is_numeric) => {
            let input = x.split_whitespace().collect::<Vec<&str>>();
            Some(Entry::File(input[0].parse().unwrap(), input[1].to_owned()))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test_part1.txt"), Some(95437))
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2("input_test_part1.txt"), Some(24933642))
    }
}
