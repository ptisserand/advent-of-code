use std::env;
use std::fs;

fn parse_contents(contents: String) -> Vec<i32>{
    let mut elves: Vec<i32> = vec![0];
    let mut idx = 0;
    for line in contents.lines() {
        // println!("{}", line);
        if line.eq("") {
            elves.push(0);
            idx += 1;
        } else {
            let value: i32 = line.parse().unwrap();
            elves[idx] += value;
        }
    }
    elves
}

fn get_max_calories(elves: &Vec<i32>) -> i32 {
    let mut value = 0;
    for v in elves {
        if *v > value {
            value = *v;
        }
    }
    value
}

fn get_total_calories(mut elves: Vec<i32>) -> i32 {
    elves.sort_unstable_by(|a, b| b.cmp(a));
    elves[0] + elves[1] + elves[2]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let elves = parse_contents(contents);
    println!("Number of elves: {}", elves.len());
    println!("Part1: Max calories: {}", get_max_calories(&elves));
    println!("Part2: Total max calories: {}", get_total_calories(elves));

}
