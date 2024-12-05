use common::time;
use std::fs;

fn main() {
    println!("Print Queue");
    println!("------------------");

    let input = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    parse_input(&input, &mut rules, &mut updates);

    // println!("updates: {:?}", updates);

    let (duration, result) = time(|| part1(&rules, &updates));
    println!("Part 1 result: {} (computed in {:?})", result, duration);


    // let (duration, result) = time(|| part2(&input));
    // println!("Part 2 result: {} (computed in {:?})", result, duration);
    // println!();
}

fn parse_input(input: &String, rules: &mut Vec<(u32, u32)>, updates: &mut Vec<Vec<u32>>) {
    // split on empty newline!
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    sections[0].lines().for_each(|rule| {
        let parts = rule.split("|").collect::<Vec<&str>>();
        let before = parts[0].parse::<u32>().unwrap();
        let after = parts[1].parse::<u32>().unwrap();

        rules.push((before, after));
    });

    sections[1].lines().for_each(|line| {
        updates.push(line.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>());
    });
}

fn part1(rules: &Vec<(u32, u32)>, updates: &Vec<Vec<u32>>) -> u32 {
    updates
        .iter()
        .map(|update| {
            let middle: u32 = update[update.len() / 2];

            let mut value = middle;
            let mut valid = true;

            // Check rules
            for (before, after) in rules {
                let before_index = update.iter().position(|&x| x == *before);
                let after_index = update.iter().position(|&x| x == *after);

                // If both elements exist in the update, enforce the rule
                if let (Some(before_idx), Some(after_idx)) = (before_index, after_index) {
                    if before_idx > after_idx {
                        valid = false;
                        break; // Exit the loop early if the rule is violated
                    }
                }
            }

            // Invalidate value if rules are not satisfied
            if !valid {
                value = 0;
            }

            value
        })
        .sum() // Sum up all the values
}