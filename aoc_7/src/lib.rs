use std::collections::HashMap;

#[derive(Debug)]
enum Entry {
    Cd(String),
    Ls,
    Dir(String),
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

    println!("{map:?}");
    let total_size = 70000000;
    let needed_space = 30000000;
    let unused_space = map.get("/").unwrap();

    println!("Unused space {}", unused_space);

    None
}

fn my_own_du(input_path: &str) -> Option<HashMap<String, u32>> {
    let instructions: Vec<Entry> = std::fs::read_to_string(input_path)
        .ok()?
        .lines()
        .map(parse_line)
        .collect();

    let mut current_dir = String::default();
    let mut map = HashMap::<String, u32>::new();
    let mut directory_path: Vec<String> = vec![];

    for entry in instructions {
        match entry {
            Entry::Cd(change_dir) => {
                if change_dir == ".." {
                    let total_size_precedent_dir = map.get(&current_dir).unwrap();
                    directory_path.pop().unwrap();
                    current_dir = directory_path.join("/");
                    //*(map.entry("toto".to_owned()).or_default()) += *total_size_precedent_dir;
                    *map.get_mut(&current_dir).unwrap() += *total_size_precedent_dir;
                } else {
                    directory_path.push(change_dir.clone());
                    current_dir = directory_path.join("/");
                    map.insert(current_dir.clone(), 0);
                }
            }
            Entry::File(file_size, _file_name) => {
                // Add the size of the file to the current directory
                *map.get_mut(&current_dir).unwrap() += file_size;
            }
            _ => (),
        }
    }

    Some(map)
}

fn parse_line(line: &str) -> Entry {
    match line {
        x if x.starts_with("$ cd ") => {
            let directory_name = x.split_whitespace().collect::<Vec<&str>>()[2];
            Entry::Cd(directory_name.to_owned())
        }
        x if x.starts_with("$ ls") => Entry::Ls,
        x if x.starts_with("dir") => {
            let directory_name = x.split_whitespace().collect::<Vec<&str>>()[1];
            Entry::Dir(directory_name.to_owned())
        }
        x => {
            let input = x.split_whitespace().collect::<Vec<&str>>();
            Entry::File(input[0].parse().unwrap(), input[1].to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("input_test.txt"), Some(95437))
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2("input_test.txt"), Some(24933642))
    }
}
