use std::fs;

fn main() {
    // part1();
    part2();
}

fn part1() {
    // let file_path = "src/exampleInput.txt";
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let file_lines: Vec<_> = contents.lines().collect();

    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();
    let mut deltas: Vec<u32> = Vec::new();
    
    for line in &file_lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        left_numbers.push(parts[0].parse::<u32>().unwrap());
        right_numbers.push(parts[1].parse::<u32>().unwrap());
    }

    // sort the left and right numbers from low to high
    left_numbers.sort();
    right_numbers.sort();

    for i in 0..left_numbers.len() {
        if left_numbers[i] > right_numbers[i] {
            deltas.push(left_numbers[i] - right_numbers[i]);
        } else {
            deltas.push(right_numbers[i] - left_numbers[i]);
        }

        println!("Left: {}, Right: {}, Delta: {}", left_numbers[i], right_numbers[i], deltas[i]);
    }

    let deltas_sum: u32 = deltas.iter().sum();
    println!("Sum of deltas: {}", deltas_sum);
}

fn part2() {
    // let file_path = "src/exampleInput.txt";
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let file_lines: Vec<_> = contents.lines().collect();

    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();
    let mut similarity_scores: Vec<u32> = Vec::new();

    for line in &file_lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        left_numbers.push(parts[0].parse::<u32>().unwrap());
        right_numbers.push(parts[1].parse::<u32>().unwrap());
    }

    for (i, &left) in left_numbers.iter().enumerate() {
        let mut count: u32 = 0;

        for (j, &right) in right_numbers.iter().enumerate() {
            if left == right {
                count += 1;
            }
        }

        let score: u32 = left * count;

        similarity_scores.push(score);

        println!("Left: {}, Count: {},  Score: {}", left, count, score);
        
    }
    println!("Sum of similarity scores: {}", similarity_scores.iter().sum::<u32>());
}