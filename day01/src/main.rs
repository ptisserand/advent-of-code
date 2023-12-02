use std::env;
use std::fs;

use itertools::rev;

fn calibration_part1(line: &str) -> usize {
    let mut calibration: usize = 0;
    for c in line.chars() {
        if "0123456789".contains(c) {
            calibration = (c as usize - '0' as usize) * 10;
            break;
        }
    }
    for c in rev(line.chars()) {
        if "0123456789".contains(c) {
            calibration += c as usize - '0' as usize;
            break;
        }
    }
    calibration
}

fn calibration_part2(line: &str) -> usize {
    let mut decimal: usize = 0;
    let mut unit: usize = 0;
    let mut decimal_idx = line.len();
    let mut unit_idx: usize = 0;
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    (0..digits.len()).for_each(|ii| {
        for ee in [digits[ii], ii.to_string().as_str()] {
            let mut idx = line.find(ee);
            if idx.is_some() && idx.unwrap() < decimal_idx {
                decimal_idx = idx.unwrap();
                decimal = ii;
            }
            idx = line.rfind(ee);
            if idx.is_some() && idx.unwrap() >= unit_idx {
                unit_idx = idx.unwrap();
                unit = ii;
            }
        }
    });
    decimal * 10 + unit
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let calibration1: usize = contents.lines().map(calibration_part1).sum();
    println!("Calibration part1: {}", calibration1);
    let calibration2: usize = contents.lines().map(calibration_part2).sum();
    println!("Calibration part2: {}", calibration2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_part1() {
        assert_eq!(12, calibration_part1("1abc2"));
        assert_eq!(38, calibration_part1("pqr3stu8vwx"));
        assert_eq!(15, calibration_part1("a1b2c3d4e5f"));
        assert_eq!(77, calibration_part1("treb7uchet"));
    }

    #[test]
    fn test_calibration_part2() {
        assert_eq!(29, calibration_part2("two1nine"));
        assert_eq!(83, calibration_part2("eightwothree"));
        assert_eq!(13, calibration_part2("abcone2threexyz"));
        assert_eq!(24, calibration_part2("xtwone3four"));
        assert_eq!(42, calibration_part2("4nineeightseven2"));
        assert_eq!(14, calibration_part2("zoneight234"));
        assert_eq!(76, calibration_part2("7pqrstsixteen"));
    }
}
