use std::env;
use std::fs;

use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse_subgame(game_result: &str) -> Bag {
    let mut max_guest = Bag {
        red: 0,
        green: 0,
        blue: 0
    };
    let subgame_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    for subgame in game_result.split(';') {
        for result in subgame_re.captures_iter(subgame) {
            let count: usize = result[1].parse().unwrap();
            match &result[2] {
                "red" => if count > max_guest.red {max_guest.red = count},
                "green" => if count > max_guest.green {max_guest.green = count},
                "blue" => if count > max_guest.blue {max_guest.blue = count},
                _ => (),
            }
        }
    }
    max_guest
}

fn evaluate_part1(line: &str, bag: &Bag) -> (usize, bool) {
    let splitted = Regex::new(r"Game (?P<game_id>\d+):(?P<game_result>.*)").unwrap().captures(line).unwrap();
    let game_id: usize = splitted["game_id"].parse().unwrap();
    let game_result: String = splitted["game_result"].parse().unwrap();
    let max_guest = parse_subgame(&game_result);
    let valid = max_guest.red <= bag.red && max_guest.blue <= bag.blue && max_guest.green <= bag.green;
    (game_id, valid)
}

fn evaluate_part2(line: &str) -> usize {
    let splitted = Regex::new(r"Game (?P<game_id>\d+):(?P<game_result>.*)").unwrap().captures(line).unwrap();
    let game_result: String = splitted["game_result"].parse().unwrap();
    let max_guest = parse_subgame(&game_result);
    max_guest.blue * max_guest.green * max_guest.red
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let bag_part1 = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let value_part1: usize = contents.lines().map(|l| evaluate_part1(l, &bag_part1)).filter(|r| r.1).map(|r| r.0).sum();
    println!("Cube part1: {}", value_part1);
    let value_part2: usize = contents.lines().map(evaluate_part2).sum();
    println!("Cube part2: {}", value_part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_part1() {
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert_eq!((1, true), evaluate_part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", &bag));
        assert_eq!((2, true), evaluate_part1("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", &bag));
        assert_eq!((3, false), evaluate_part1("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", &bag));
        assert_eq!((4, false), evaluate_part1("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", &bag));
        assert_eq!((5, true), evaluate_part1("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", &bag));
    }

    #[test]
    fn test_evaluate_part2() {
        assert_eq!(48, evaluate_part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
        assert_eq!(12, evaluate_part2("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"));
        assert_eq!(1560, evaluate_part2("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"));
        assert_eq!(630, evaluate_part2("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"));
        assert_eq!(36, evaluate_part2("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"));
    }
}