use std::fs;

enum CubeColor {
    BLUE,
    RED,
    GREEN,
}

impl CubeColor {
    fn from_str(value: &str) -> CubeColor {
        return match value {
            "blue" => CubeColor::BLUE,
            "red" => CubeColor::RED,
            "green" => CubeColor::GREEN,
            _ => panic!("unknown cube color {}", value)
        };
    }
}

fn main() {
    let input_string = match fs::read_to_string("src/2/input.txt") {
        Ok(contents) => contents,
        Err(why) => panic!("couldn't read input file: {}", why),
    };

    const RED_CUBES: i32 = 12;
    const GREEN_CUBES: i32 = 13;
    const BLUE_CUBES: i32 = 14;

    let sum: i32 = input_string
        .lines()
        .filter_map(|line| {
            line.split_once(':')
                .and_then(|(game_id, reveals)| {
                    if reveals
                        .trim()
                        .split(';')
                        .any(|reveal|
                            reveal.trim().split(',').any(|cube| {
                                let values: Vec<&str> = cube.trim().split(' ').collect();
                                let count = values[0].parse::<i32>();
                                let color = CubeColor::from_str(values[1]);
                                return count.map_or(true, |count| match color {
                                    CubeColor::BLUE => count > BLUE_CUBES,
                                    CubeColor::RED => count > RED_CUBES,
                                    CubeColor::GREEN => count > GREEN_CUBES,
                                });
                            })
                        ) {
                        None
                    } else {
                        match game_id.chars().skip(5).collect::<String>().parse::<i32>() {
                            Ok(id) => Some(id),
                            Err(_) => None,
                        }
                    }
                })
        }).sum();

    println!("sum of powers of minimum sets: {}", sum);
}
