use aoc_15::{part1, part2};

fn main() {
    if let Some(x) = part1("input.txt", 2000000) {
        println!("Part 1: {x}");
    }

    if let Some(x) = part2("input.txt", 4000000) {
        println!("Part 2: {x}");
    }
}
