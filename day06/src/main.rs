use std::env;
use std::fs;

fn find_marker_part1(stream: String) -> usize {
    let bytes = stream.as_bytes();
    let mut position: usize = 3;
    for _ in 0..bytes.len() - 4 {
        if !bytes[position-2..position+1].contains(&bytes[position-3]) && 
        !bytes[position-1..position+1].contains(&bytes[position-2]) &&
        bytes[position-1] != bytes[position] {
            break;
        }
        position += 1;
    }
    position + 1
}

fn find_marker_part2(stream: String) -> usize {
    let bytes = stream.as_bytes();
    let mut position: usize = 13;
    for _ in 0..bytes.len() - 14 {
        let mut found = 0;
        for i in 0..14 {
            if bytes[position-i+1..position+1].contains(&bytes[position-i]) {
                found += 1;
                break;
            }
        }
        if bytes[position-1] == bytes[position] {
            found = 1;
        }
        if found == 0{
            break;
        }
        position += 1;
    }
    position + 1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for line in contents.lines() {
        let position_part1 = find_marker_part1(String::from(line));
        println!("Part1: {}", position_part1);
        let position_part2 = find_marker_part2(String::from(line));
        println!("Part2: {}", position_part2);

    }
}
