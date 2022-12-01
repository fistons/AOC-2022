use aoc_1::part_1_and_2;

fn main() {
    if let Ok(result) = part_1_and_2("input.txt".to_owned(), 1) {
        println!("Part 1 result is {}", result);
    }

    if let Ok(result) = part_1_and_2("input.txt".to_owned(), 3) {
        println!("Part 2 result is {}", result);
    }
}
