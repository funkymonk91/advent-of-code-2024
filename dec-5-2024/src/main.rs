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

    let (duration, result) = time(|| part2(&rules, &updates));
    println!("Part 2 result: {} (computed in {:?})", result, duration);
    println!();
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

fn part2(rules: &Vec<(u32, u32)>, updates: &Vec<Vec<u32>>) -> u32 {
    updates
        .iter()
        .filter_map(|update| {
            let mut temp = update.clone();
            if !is_valid(&temp, rules) {
                fix_pages(&mut temp, rules);
                Some(temp[temp.len() / 2]) // Use the middle value after fixing
            } else {
                None // Ignore valid updates
            }
        })
        .sum() // Sum up the middle values of fixed updates
}

fn is_valid(pages: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    rules.iter().all(|&(before, after)| {
        let before_index = pages.iter().position(|&x| x == before);
        let after_index = pages.iter().position(|&x| x == after);

        // If both elements exist, they must satisfy the rule
        before_index.is_none() || after_index.is_none() || before_index < after_index
    })
}

fn fix_pages(pages: &mut Vec<u32>, rules: &Vec<(u32, u32)>) {
    let mut changed = true;

    // Iteratively fix until all rules are satisfied
    while changed {
        changed = false;

        for &(before, after) in rules {
            if let (Some(before_idx), Some(after_idx)) = (
                pages.iter().position(|&x| x == before),
                pages.iter().position(|&x| x == after),
            ) {
                if before_idx > after_idx {
                    pages.swap(before_idx, after_idx);
                    changed = true; // Mark as changed to repeat the loop
                }
            }
        }
    }
}

