use std::{env, fs};

use regex::Regex;

fn apply_procedures_part1(stacks: &mut Vec<Vec<u8>>, procedures: &Vec<String>) {
    let re: regex::Regex =
        Regex::new(r"move (?P<nb>\d+) from (?P<source>\d+) to (?P<target>\d+)").unwrap();
    for procedure in procedures {
        // println!("{}", procedure);
        let motion = re.captures(&procedure).unwrap();
        let nb: usize = motion["nb"].parse().unwrap();
        let source: usize = motion["source"].parse::<usize>().unwrap() - 1;
        let target: usize = motion["target"].parse::<usize>().unwrap() - 1;
        for _ in 0..nb {
            let elem = stacks[source].pop().unwrap();
            stacks[target].push(elem);
        }
    }
}

fn apply_procedures_part2(stacks: &mut Vec<Vec<u8>>, procedures: &Vec<String>) {
    let re: regex::Regex =
        Regex::new(r"move (?P<nb>\d+) from (?P<source>\d+) to (?P<target>\d+)").unwrap();
    for procedure in procedures {
        // println!("{}", procedure);
        let motion = re.captures(&procedure).unwrap();
        let nb: usize = motion["nb"].parse().unwrap();
        let source: usize = motion["source"].parse::<usize>().unwrap() - 1;
        let target: usize = motion["target"].parse::<usize>().unwrap() - 1;
        let mut tmp_vec: Vec<u8> = Vec::new();
        for _ in 0..nb {
            let elem = stacks[source].pop().unwrap();
            tmp_vec.push(elem);
        }
        for _ in 0..nb {
            let elem: u8 = tmp_vec.pop().unwrap();
            stacks[target].push(elem);
        }
    }
}

fn get_result(stacks: &mut Vec<Vec<u8>>) -> String {
    let mut result: Vec<u8> = Vec::new();
    for stack in stacks {
        let letter = stack.pop().unwrap();
        result.push(letter);
    }
    String::from_utf8(result).unwrap()   
}

fn init_stacks(drawings: &Vec<String>, nb_stacks: usize) -> Vec<Vec<u8>> {
    let mut stacks: Vec<Vec<u8>> = Vec::new();
    for _ in 0..nb_stacks {
        stacks.push(Vec::new());
    }
    for dd in drawings.iter().rev() {
        let mut pos: usize = 0;
        let mut idx: usize = 0;
        while pos < dd.len() {
            if dd.as_bytes()[pos] == b'[' {
                stacks[idx].push(dd.as_bytes()[pos + 1]);
            }
            pos += 4;
            idx += 1;
        }
    }
    stacks
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let stack_index: regex::Regex = Regex::new(r"^ (\d+\s*)*(?P<last>\d+\s*)$").unwrap();
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut step = 0;
    let mut drawings: Vec<String> = Vec::new();
    let mut procedures: Vec<String> = Vec::new();
    let mut nb_stacks: usize = 0;
    for line in contents.lines() {
        if stack_index.is_match(line) {
            // println!("Found stack index line {}", line);
            nb_stacks = stack_index.captures(line).unwrap()["last"]
                .trim()
                .parse()
                .unwrap();
            step = 1;
            continue;
        }
        if step == 0 {
            drawings.push(line.to_owned());
        }
        if step == 1 {
            if line != "" {
                procedures.push(line.to_owned());
            }
        }
    }
    println!("Nb stacks: {}", nb_stacks);

    let mut stacks: Vec<Vec<u8>> = init_stacks(&drawings, nb_stacks);

    apply_procedures_part1(&mut stacks, &procedures);
    let part1_result = get_result(&mut stacks);
    println!("Part 1: {}", part1_result);
    stacks.clear();
    stacks = init_stacks(&drawings, nb_stacks);
    apply_procedures_part2(&mut stacks, &procedures);
    let part2_result = get_result(&mut stacks);
    println!("Part 2: {}", part2_result);

}
