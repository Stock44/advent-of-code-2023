use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn get_first_and_last_digits(input: &str) -> Option<(u32, u32)> {
    let digits: Vec<u32> = input
        .chars()
        .filter_map(|x| char::to_digit(x, 10))
        .collect();

    if digits.len() == 0 {
        return None
    }

    return Some((digits[0], digits[digits.len() - 1]));
}
fn main() {
    let input_path = Path::new("src/1/input.txt");
    let display = input_path.display();
    let mut file = match File::open(&input_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {}
        Err(why) => panic!("couldn't read {}: {}", display, why),
    }

    let lines = s.split("\n");

    let total: u32 = lines
        .filter_map(|line| get_first_and_last_digits(line))
        .map(|(a, b)| 10 * a + b)
        .sum();

    println!("Sum of all calibration values: {}", total);
}
