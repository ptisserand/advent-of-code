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

fn overlap(low_a: i32, high_a: i32, low_b: i32, high_b: i32) -> bool {
    if (low_a == low_b) || (high_a == high_b) {
        return true;
    }
    if low_a < low_b {
        if (high_a >= high_b) || (low_b <= high_a) {
            return true;
        }
    }
    if low_a > low_b {
        if (high_a <= high_b) || (low_a <= high_b) {
            return true;
        }
    }
    false
}

fn check_pairs(re: &regex::Regex, line: &str, check: fn(i32, i32, i32, i32) -> bool) -> u32 {
    let pairs = re.captures(line).unwrap();
    let low_a: i32 = pairs["lowA"].parse().unwrap();
    let high_a: i32 = pairs["highA"].parse().unwrap();
    let low_b: i32 = pairs["lowB"].parse().unwrap();
    let high_b: i32 = pairs["highB"].parse().unwrap();
    let result = check(low_a, high_a, low_b, high_b);
    if result {
        1
    } else {
        0
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut part1_total: u32 = 0;
    let mut part2_total: u32 = 0;
    let re: regex::Regex =
        Regex::new(r"(?P<lowA>\d+)-(?P<highA>\d+),(?P<lowB>\d+)-(?P<highB>\d+)").unwrap();

    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for line in contents.lines() {
        part1_total += check_pairs(&re, &line, contains);
        part2_total += check_pairs(&re, &line, overlap);
    }
    println!("Part 1: {}", part1_total);
    println!("Part 2: {}", part2_total);
}
