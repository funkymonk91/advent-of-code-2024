use common::time;
use std::fs;
use regex::Regex;

fn main() {
    println!("Mull It Over");
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
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            // trim the "mul(" and ")"
            let mul = caps.get(0).unwrap().as_str();
            let mul = &mul[4..mul.len()-1];

            let mut nums = mul.split(",").map(|n| n.parse::<u32>().unwrap());

            let [a, b] = [nums.next().unwrap(), nums.next().unwrap()];
            a * b
        })
        .sum()
}

// fn part2(input: &String) -> u32 {   
//     0
// }