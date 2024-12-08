use common::time;
use std::{collections::HashSet, fs};

fn main() {
    println!("Guard Gallivant");
    println!("------------------");

    let input = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let mut initial_position: (usize, usize) = (0, 0);
    let mut map: Vec<Vec<char>> = parse_input(&input, &mut initial_position);

    let (duration, result) = time(|| part1(&mut map, &initial_position) as u32);
    println!("Part 1 result: {} (computed in {:?})", result, duration);

    // let (duration, result) = time(|| part2(&rules, &updates));
    // println!("Part 2 result: {} (computed in {:?})", result, duration);
    // println!();
}

fn parse_input(input: &String, initial_position: &mut (usize, usize)) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => c, // Keep '#' as it is
                    '^' | 'v' | '<' | '>' => {
                        *initial_position = (i, j);
                        c
                    }, // Keep '^' as it is
                    _ => '.', // Replace other characters with a default value (e.g., '.')
                })
                .collect()
        })
        .collect();

    grid
}

fn rotate_90_cw(current_direction: char) -> char {
    match current_direction {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Invalid direction"),
    }
}

fn part1(map: &mut Vec<Vec<char>>, initial_position: &(usize, usize)) -> u32 {
    let mut current_position = *initial_position;
    let mut path_taken: HashSet<(usize, usize)> = HashSet::new();

    loop {
        // Record the current position before making a move
        path_taken.insert(current_position);

        let current_direction = map[current_position.0][current_position.1];
        let next_position = match current_direction {
            '^' if current_position.0 > 0 => (current_position.0 - 1, current_position.1),
            '>' if current_position.1 < map[0].len() - 1 => (current_position.0, current_position.1 + 1),
            'v' if current_position.0 < map.len() - 1 => (current_position.0 + 1, current_position.1),
            '<' if current_position.1 > 0 => (current_position.0, current_position.1 - 1),
            _ => {
                // If out of bounds, stop the loop
                break;
            }
        };

        if map[next_position.0][next_position.1] == '#' {
            // Rotate clockwise if the next position is a wall
            map[current_position.0][current_position.1] = rotate_90_cw(current_direction);
        } else {
            // Mark the current position as visited and move to the next position
            map[current_position.0][current_position.1] = 'X';
            map[next_position.0][next_position.1] = current_direction;
            current_position = next_position;
        }
    }

    // Print the map after the guard's traversal
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }

    path_taken.len() as u32
}
