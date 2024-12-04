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

    let (duration, result) = time(|| part2(&input));
    println!("Part 2 result: {} (computed in {:?})", result, duration);
    println!();
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

fn part2(input: &String) -> u32 {   
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don\'t\(\)").unwrap();

    let mut timeline: Vec<(bool, usize)> = Vec::new();
    timeline.push((true, 0));

    // get the indexes of the dos and donts
    do_re.find_iter(input).for_each(|m| {
        timeline.push((true, m.start()));
    });

    dont_re.find_iter(input).for_each(|m| {
        timeline.push((false, m.start()));
    });

    // sort the timeline
    timeline.sort_by(|a, b| a.1.cmp(&b.1));

    let mut cursor = 0;
    let mut doit = true;

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    re.find_iter(input).map(|m| {
        let mut val = 0;

        while cursor < timeline.len() && m.start() > timeline[cursor].1 {
            doit = timeline[cursor].0;
            cursor += 1;
        }

        if doit {
            let mul = m.as_str();
            let mul = &mul[4..mul.len()-1];

            let mut nums = mul.split(",").map(|n| n.parse::<u32>().unwrap());

            let [a, b] = [nums.next().unwrap(), nums.next().unwrap()];

            val = a * b;
        }

        val
    })
    .sum()
}