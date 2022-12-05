use regex::Regex;
use std::env;
use std::fs;

fn contains(low_a: i32, high_a: i32, low_b: i32, high_b: i32) -> bool {
    if (low_a == low_b) || (high_a == high_b) {
        return true;
    }
    if low_a < low_b {
        if high_a >= high_b {
            return true;
        }
    }
    if low_a > low_b {
        if high_a <= high_b {
            return true;
        }
    }
    return false;
}

fn parse_pairs(re: &regex::Regex, line: &str) -> u32 {
    let pairs = re.captures(line).unwrap();
    let low_a: i32 = pairs["lowA"].parse().unwrap();
    let high_a: i32 = pairs["highA"].parse().unwrap();
    let low_b: i32 = pairs["lowB"].parse().unwrap();
    let high_b: i32 = pairs["highB"].parse().unwrap();
    let result = contains(low_a, high_a, low_b, high_b);
    if result {
        1
    } else {
        0
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut total: u32 = 0;
    let re: regex::Regex =
        Regex::new(r"(?P<lowA>\d+)-(?P<highA>\d+),(?P<lowB>\d+)-(?P<highB>\d+)").unwrap();

    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for line in contents.lines() {
        total += parse_pairs(&re, line);
    }
    println!("Part 1: {}", total);
}
