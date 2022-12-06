use aoc_6::{solution, Version};

fn main() {
    println!("Loop version");
    if let Some(result) = solution("input.txt", Version::LoopVersion, 4) {
        println!("{result}");
    }

    if let Some(result) = solution("input.txt", Version::LoopVersion, 14) {
        println!("{result}");
    }

    println!("Iterator version");
    if let Some(result) = solution("input.txt", Version::IteratorVersion, 4) {
        println!("{result}");
    }

    if let Some(result) = solution("input.txt", Version::IteratorVersion, 14) {
        println!("{result}");
    }
}
