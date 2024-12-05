use common::time;
use std::fs;

fn main() {
    println!("Print Queue");
    println!("------------------");

    let input = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let (duration, result) = time(|| part1(&input));
    println!("Part 1 result: {} (computed in {:?})", result, duration);

    // let (duration, result) = time(|| part2(&input));
    // println!("Part 2 result: {} (computed in {:?})", result, duration);
    // println!();
}

fn part1(input: &String) -> u32 {
    0
}