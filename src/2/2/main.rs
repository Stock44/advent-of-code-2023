use std::cmp::max;
use std::fs;

enum CubeColor {
    BLUE,
    RED,
    GREEN,
}

impl CubeColor {
    fn from_str(value: &str) -> Option<CubeColor> {
        return match value {
            "blue" => Some(CubeColor::BLUE),
            "red" => Some(CubeColor::RED),
            "green" => Some(CubeColor::GREEN),
            _ => None,
        };
    }
}

fn main() {
    let input_string = match fs::read_to_string("src/2/input.txt") {
        Ok(contents) => contents,
        Err(why) => panic!("couldn't read input file: {}", why),
    };

    let sum: i32 = input_string
        .lines()
        .filter_map(|line| {
            line.split_once(':')
                .and_then(|(_, reveals)| {
                    reveals
                        .trim()
                        .split(';')
                        .map(|reveal| reveal.trim().split(','))
                        .flatten()
                        .try_fold((0, 0, 0), |(reds, blues, greens), color_with_count|
                            color_with_count.trim()
                                .split_once(' ')
                                .and_then(|(count, color)| -> Option<(i32, CubeColor)> {
                                    Some((count.parse::<i32>().ok()?, CubeColor::from_str(color.trim())?))
                                })
                                .map(|(count, color)| match color {
                                    CubeColor::RED => (max(count, reds), blues, greens),
                                    CubeColor::BLUE => (reds, max(count, blues), greens),
                                    CubeColor::GREEN => (reds, blues, max(count, greens)),
                                }),
                        ).map(|(reds, blues, greens)| reds * blues * greens)
                })
        }).sum();

    println!("sum of possible games: {}", sum);
}
