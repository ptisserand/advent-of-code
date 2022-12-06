use std::env;
use std::fs;

fn find_marker_part1(stream: String) -> usize {
    let bytes = stream.as_bytes();
    let mut position: usize = 3;
    for _ in 0..bytes.len() - 4 {
        if (bytes[position - 3] != bytes[position - 2])
            && (bytes[position - 3] != bytes[position - 1])
            && (bytes[position - 3] != bytes[position])
        {
            if (bytes[position - 2] != bytes[position - 1])
                && (bytes[position - 2] != bytes[position])
            {
                if bytes[position - 1] != bytes[position] {
                    break;
                }
            }
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
        let position = find_marker_part1(String::from(line));
        println!("Part1: {}", position);
    }
}
