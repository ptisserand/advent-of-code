use std::{collections::HashMap, env, fs};

use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn parse(contents: &str) -> Self {
        let re = Regex::new(
            r"\s*(?P<name>[A-Z]{3})\s*=\s*\((?P<left>[A-Z]{3})\s*,\s*(?P<right>[A-Z]{3})\)",
        )
        .unwrap();
        let mut nodes = HashMap::new();
        let mut directions = Vec::new();
        let mut it = contents.lines();
        for c in it.next().unwrap().chars() {
            directions.push(if c == 'L' {
                Direction::Left
            } else {
                Direction::Right
            });
        }
        for line in it {
            if line.is_empty() {
                continue;
            };
            let parsed = re.captures(line).unwrap();
            nodes.insert(
                parsed["name"].parse().unwrap(),
                Node {
                    left: parsed["left"].parse().unwrap(),
                    right: parsed["right"].parse().unwrap(),
                },
            );
        }
        Map { directions, nodes }
    }

    fn nb_steps(&self) -> usize {
        let mut target = "AAA".to_string();
        let mut count = 0;
        let nb_directions = self.directions.len();
        let final_target = "ZZZ".to_string();
        while target != final_target {
            let direction = self.directions[count % nb_directions];
            let node = self.nodes.get(&target).unwrap();
            target = if direction == Direction::Left { node.left.clone()} else {node.right.clone()};
            count += 1;
        }
        count
    }
}

fn part1(contents: &str) -> usize {
    Map::parse(contents).nb_steps()
}

fn part2(contents: &str) -> usize {
    0
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let value_part1 = part1(&contents);
    println!("Almanac part1: {}", value_part1);
    let value_part2 = part2(&contents);
    println!("Almanac part2: {}", value_part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_parse() {
        let contents = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let map = Map::parse(contents);
        assert_eq!(map.directions.len(), 3);
        assert_eq!(map.directions[0], Direction::Left);
        assert_eq!(map.directions[1], Direction::Left);
        assert_eq!(map.directions[2], Direction::Right);
        assert!(map.nodes.contains_key("AAA"));
        assert!(map.nodes.contains_key("BBB"));
        assert!(map.nodes.contains_key("ZZZ"));
        assert_eq!(
            *map.nodes.get("BBB").unwrap(),
            Node {
                left: "AAA".to_string(),
                right: "ZZZ".to_string()
            }
        );
    }

    #[test]
    fn test_map_nb_steps() {
        let contents = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let map = Map::parse(contents);
        assert_eq!(map.nb_steps(), 6);

        let contents = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let map = Map::parse(contents);
        assert_eq!(map.nb_steps(), 2);
    }
}
