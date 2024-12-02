use common::time;
use std::fs;

fn main() {
    println!("Red-Nosed Reports");
    println!("------------------");

    let input = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let (duration, result) = time(|| part1(&input));
    println!("Part 1 result: {} (computed in {:?})", result, duration);

    // let (duration, result) = time(|| part2(&left, &right));
    // println!("Part 2 result: {} (computed in {:?})", result, duration);
    // println!();
}

fn part1(input: &String) -> u32 {
    input.lines().map(|report| {
       is_report_safe(report)
    })
    .filter(|&safe| safe)
    .count() as u32
}


fn is_report_safe(report: &str) -> bool {
    // split the line
    let levels: Vec<&str> = report.split_whitespace().collect();

    // check if the line is safe
    let mut asc = true;
    let mut is_safe = true;

    for i in 0..levels.len() - 1 {
        let current = levels[i].parse::<u32>().unwrap();
        let next = levels[i + 1].parse::<u32>().unwrap();
        
        // first, check if all numbers are ascending or descending
        if(i == 0) {
            asc = current < next;
        }
        else if (asc && current > next) || (!asc && current < next) {
            is_safe = false;
        }

        let d = current.abs_diff(next);
        // ensure that the difference between the two numbers is at least one and at most 3
        if(d < 1 || d > 3) {
            is_safe = false;
        }
    }

    is_safe
}