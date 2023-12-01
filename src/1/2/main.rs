use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use crate::Digit::One;

enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    fn as_u32(&self) -> u32 {
        match self {
            One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            Digit::One => "one",
            Digit::Two => "two",
            Digit::Three => "three",
            Digit::Four => "four",
            Digit::Five => "five",
            Digit::Six => "six",
            Digit::Seven => "seven",
            Digit::Eight => "eight",
            Digit::Nine => "nine",
        }
    }
}

fn get_first_and_last_digits(input: &str) -> Option<(u32, u32)> {
    const DIGITS: [Digit; 9] = [
        One,
        Digit::Two,
        Digit::Three,
        Digit::Four,
        Digit::Five,
        Digit::Six,
        Digit::Seven,
        Digit::Eight,
        Digit::Nine
    ];

    let digits: Vec<u32> = input
        .char_indices()
        .filter_map(|(idx, char)|
            char
                .to_digit(10)
                .or(DIGITS
                    .iter()
                    .find(|digit| (&input[idx..]).starts_with(digit.as_str()))
                    .map(Digit::as_u32)))
        .collect();

    if digits.len() == 0 {
        return None;
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
