use std::fs;
use std::time::{Duration, Instant};

pub fn read_input(input: &str) -> String {
    fs::read_to_string(input)
        .expect("Something went wrong reading the file")
}

pub fn time<F, R>(mut f: F) -> (Duration, R)
where
    F: FnMut() -> R,
{
    let start = Instant::now();
    let result = f(); // Execute the closure
    (start.elapsed(), result)
}