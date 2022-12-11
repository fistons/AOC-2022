use aoc_11::part1;

fn main() {
    if let Some(x) = part1("input.txt", 20, 3) {
        println!("Part 1: {x}");
    }

    if let Some(x) = part1("input.txt", 10_000, 1) {
        println!("Part 1: {x}");
    }
}
