use common::time;
use std::fs;

fn main() {
    println!("Ceres Search");
    println!("------------------");

    let input = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let (duration, result) = time(|| part1(&input));
    println!("Part 1 result: {} (computed in {:?})", result, duration);

    // let (duration, result) = time(|| part2(&input));
    // println!("Part 2 result: {} (computed in {:?})", result, duration);
    // println!();
}

fn find_xmas(lines: &Vec<&str>, x: &usize, y: &usize, reverse: bool) -> u32 {
    let mut xmas = String::from("XMAS");
    let mut count = 0;
    if reverse {
        xmas = xmas.chars().rev().collect();
    }

    // check horizontally
    if x + 3 < lines[*y].len() {
        let check = &lines[*y][*x..*x+4];
        if check == xmas {
            count += 1;
        }
    }

    // check vertically
    if y + 3 < lines.len() {
        let check = &lines[*y..*y+4].iter().map(|line| line.chars().nth(*x).unwrap()).collect::<String>();
        
        if check.to_string() == xmas {
            count += 1;
        }
    }

    // check diagonally (down-right)
    if x + 3 < lines[*y].len() && y + 3 < lines.len() {
        let check = (0..4).map(|i| lines[*y+i].chars().nth(*x+i).unwrap()).collect::<String>();
        if check == xmas {
            count += 1;
        }
    }

    // check diagonally (down-left)
    if x >= &3 && y + 3 < lines.len() {
        let check = (0..4).map(|i| lines[*y+i].chars().nth(*x-i).unwrap()).collect::<String>();
        if check == xmas {
            count += 1;
        }
    }

    count
}

fn part1(input: &String) -> u32 {
    let mut count = 0;
    let lines: Vec<&str> = input.lines().collect();

    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, character)| {
            count += match character {
                'X' => find_xmas(&lines, &x, &y, false),
                'S' => find_xmas(&lines, &x, &y, true),
                _ => 0
            }
        });
    });

    count
}

// fn part2(input: &String) -> u32 {   
    
// }