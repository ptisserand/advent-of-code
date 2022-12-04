use std::env;
use std::fs;

fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

fn check_backpack(line: &str) -> u32 {
    let length = line.chars().count();
    let first_part = line.get(0..length/2).unwrap();
    let second_part = line.get(length/2..).unwrap();
    let mut common: Vec<char> = Vec::new();
    let mut total = 0;
    // println!("{}", line);
    // println!("{:#?}", first_part);
    // println!("{:#?}", second_part);
    for c in first_part.chars() {
        if common.contains(&c) {
            break;
        }
        if second_part.contains(c) {
            common.push(c);
        }
    }
    for c in common {
        total += get_priority(c);
    }
    total
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut total: u32 = 0;
    for line in contents.lines() {
        total += check_backpack(line);
    }
    println!("Total: {}", total);
}