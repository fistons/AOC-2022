use aoc_1::{iterator_version, loop_version};

fn main() {
    if let Ok(result) = loop_version("input.txt".to_owned(), 1) {
        println!("Part 1 result is {}", result);
    }

    if let Ok(result) = iterator_version("input.txt".to_owned(), 3) {
        println!("Part 2 result is {}", result);
    }
}
