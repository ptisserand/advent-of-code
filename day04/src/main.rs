use std::env;
use std::fs;

use regex::Regex;

fn parse_input(line: &str) -> (Vec<u32>, Vec<u32>) {
    let splitted = Regex::new(r"Card\s*\d+:\s*(?P<winnings>(\d+\s*)*)\|\s*(?P<choices>(\d+\s*)*)")
        .unwrap()
        .captures(line)
        .unwrap();
    let number_re = Regex::new(r"\d+").unwrap();
    let winnings: Vec<u32> = number_re
        .captures_iter(&splitted["winnings"])
        .map(|x| x[0].parse().unwrap())
        .collect();
    let choices: Vec<u32> = number_re
        .captures_iter(&splitted["choices"])
        .map(|x| x[0].parse().unwrap())
        .collect();
    (winnings, choices)
}

fn matching_numbers(line: &str) -> usize {
    let (winnings, choices) = parse_input(line);
    let found = choices.iter().filter(|x| winnings.contains(x)).count();
    if found > 0 {
        1 << (found - 1)
    } else {
        0
    }
}

fn part1(contents: &str) -> usize {
    contents.lines().map(matching_numbers).sum()
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let value_part1 = part1(&contents);
    println!("Part1: {}", value_part1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
            parse_input("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
        );

        assert_eq!(
            (
                vec![13, 32, 20, 16, 61],
                vec![61, 30, 68, 82, 17, 32, 24, 19]
            ),
            parse_input("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")
        );
        assert_eq!(
            (vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1]),
            parse_input("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")
        );
        assert_eq!(
            (
                vec![41, 92, 73, 84, 69],
                vec![59, 84, 76, 51, 58, 5, 54, 83]
            ),
            parse_input("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83")
        );
        assert_eq!(
            (
                vec![87, 83, 26, 28, 32],
                vec![88, 30, 70, 12, 93, 22, 82, 36]
            ),
            parse_input("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")
        );
        assert_eq!(
            (
                vec![31, 18, 13, 56, 72],
                vec![74, 77, 10, 23, 35, 67, 36, 11]
            ),
            parse_input("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
        );
    }

    #[test]
    fn test_matching_numbers() {
        assert_eq!(8, matching_numbers("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"));
        assert_eq!(2, matching_numbers("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"));
        assert_eq!(2, matching_numbers("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"));
        assert_eq!(1, matching_numbers("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"));
        assert_eq!(0, matching_numbers("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"));
        assert_eq!(0, matching_numbers("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"));
    }

    #[test]
    fn test_part1() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, part1(input));
    }
}
