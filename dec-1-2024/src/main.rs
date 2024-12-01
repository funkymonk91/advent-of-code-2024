use common::{read_input, time};

fn main() {
    let (left, right) = parse_input("src/input.txt");

    println!("Historian Hysteria");
    println!("------------------");

    // Pass references directly
    let (duration, result) = time(|| part1(&left, &right));
    println!("Part 1 result: {} (computed in {:?})", result, duration);

    let (duration, result) = time(|| part2(&left, &right));
    println!("Part 2 result: {} (computed in {:?})", result, duration);
    println!();
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();

    read_input(input)
        .lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            left_numbers.push(parts[0].parse::<u32>().unwrap());
            right_numbers.push(parts[1].parse::<u32>().unwrap());
        });

    left_numbers.sort();
    right_numbers.sort();

    (left_numbers, right_numbers)
}

fn part1(left: &[u32], right: &[u32]) -> u32 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn part2(left: &[u32], right: &[u32]) -> u32 {
    left.iter()
        .map(|&l| {
            let count = right.iter()
                .filter(|&&r| l == r)
                .count() as u32;
            
            l * count
        })
        .sum()
}