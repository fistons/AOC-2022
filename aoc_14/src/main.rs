use aoc_14::solution;

fn main() {
    if let Some(x) = solution("input.txt", true) {
        println!("Part 1: {x}");
    }

    if let Some(x) = solution("input.txt", false) {
        println!("Part 2: {x}");
    }
}
