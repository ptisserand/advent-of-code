use std::{env, fs};

use regex::Regex;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Mapping {
    dest: usize,
    src: usize,
    length: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Map {
    mappings: Vec<Mapping>,
}

#[derive(Debug, Eq, PartialEq)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Mapping {
    fn convert(&self, value: usize) -> Option<usize> {
        if value < self.src || (value > self.src + self.length) {
            None
        } else {
            Some(self.dest + value - self.src)
        }
    }

    fn parse(line: &str) -> Self {
        let parsed = Regex::new(r"\s*(?P<dest>\d+)\s+(?P<src>\d+)\s+(?P<length>\d+)\s*")
            .unwrap()
            .captures(line)
            .unwrap();
        Self {
            dest: parsed["dest"].parse().unwrap(),
            src: parsed["src"].parse().unwrap(),
            length: parsed["length"].parse().unwrap(),
        }
    }
}

impl Map {
    fn convert(&self, value: usize) -> usize {
        for mapping in &self.mappings {
            match mapping.convert(value) {
                Some(x) => return x,
                None => continue,
            }
        }
        value
    }
}

impl Almanac {
    fn convert(&self, value: usize) -> usize {
        let mut ret = value;
        for map in &self.maps {
            ret = map.convert(ret);
        }
        ret
    }

    fn parse(contents: &str) -> Self {
        let map_name_re = Regex::new(r"^\s*\w+-to-\w+\s*map:$").unwrap();
        let mut maps: Vec<Map> = Vec::new();
        let mut seeds: Vec<usize> = Vec::new();
        let mut mappings: Vec<Mapping> = Vec::new();
        let mut it = contents.lines();
        let mut in_map = false;
        for seed in Regex::new(r"(\d+)")
            .unwrap()
            .captures_iter(it.next().unwrap())
        {
            seeds.push(seed[1].parse().unwrap());
        }
        for line in it {
            // println!("-- {} --", line);
            if line.is_empty() {
                if in_map {
                    maps.push(Map {
                        mappings: mappings.clone(),
                    });
                    mappings.clear();
                }
                in_map = false;
                continue;
            };
            if map_name_re.is_match(line) {
                // println!("NEW MAP!!! {} {}", line, maps.len());
                in_map = true;
                continue;
            }
            mappings.push(Mapping::parse(line));
            // println!("{}", line);
        }
        if in_map {
            maps.push(Map {
                mappings: mappings.clone(),
            });
        }
        // println!("{}", maps.len());
        Almanac { seeds, maps }
    }
}

fn part1(contents: &str) -> usize {
    let almanac = Almanac::parse(contents);
    let mut location = almanac.convert(almanac.seeds[0]);
    for seed in almanac.seeds.clone() {
        let new_location = almanac.convert(seed);
        location = if new_location < location {
            new_location
        } else {
            location
        }
    }
    location
}

fn part2(contents: &str) -> usize {
    let almanac = Almanac::parse(contents);
    let mut location = almanac.convert(almanac.seeds[0]);
    for i in (0..almanac.seeds.len()).step_by(2) {
        let start = almanac.seeds[i];
        let len = almanac.seeds[i + 1];
        for seed in start..(start+len) {
            let new_location = almanac.convert(seed);
            location = if new_location < location {
                new_location
            } else {
                location
            }    
        }
    }
    location
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
    fn test_mapping_convert() {
        let mapping = Mapping {
            dest: 50,
            src: 98,
            length: 2,
        };
        assert_eq!(Some(50), mapping.convert(98));
        assert_eq!(Some(51), mapping.convert(99));
        assert_eq!(None, mapping.convert(33));
        let mapping = Mapping {
            dest: 52,
            src: 50,
            length: 48,
        };
        assert_eq!(Some(52), mapping.convert(50));
        assert_eq!(Some(98), mapping.convert(96));
        assert_eq!(None, mapping.convert(1));
    }

    #[test]
    fn test_mapping_parse() {
        assert_eq!(
            Mapping {
                dest: 50,
                src: 98,
                length: 2
            },
            Mapping::parse("50 98 2")
        );
    }

    #[test]
    fn test_map_convert() {
        let map = Map {
            mappings: vec![
                Mapping {
                    dest: 50,
                    src: 98,
                    length: 2,
                },
                Mapping {
                    dest: 52,
                    src: 50,
                    length: 48,
                },
            ],
        };
        assert_eq!(81, map.convert(79));
        assert_eq!(14, map.convert(14));
        assert_eq!(50, map.convert(98));
    }

    #[test]
    fn test_almanac_convert() {
        let almanac = Almanac {
            seeds: vec![79, 14, 55, 13],
            maps: vec![
                Map {
                    mappings: vec![Mapping::parse("50 98 2"), Mapping::parse("52 50 48")],
                },
                Map {
                    mappings: vec![
                        Mapping::parse("0 15 37"),
                        Mapping::parse("37 52 2"),
                        Mapping::parse("39 0 15"),
                    ],
                },
            ],
        };
        assert_eq!(81, almanac.convert(79));
        assert_eq!(53, almanac.convert(14));
        assert_eq!(57, almanac.convert(55));
    }

    #[test]
    fn test_almanac_parse() {
        let almanac = Almanac::parse(
            r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15
",
        );
        assert_eq!(vec![79, 14, 55, 13], almanac.seeds);
        assert_eq!(
            Map {
                mappings: vec![Mapping::parse("50 98 2"), Mapping::parse("52 50 48")],
            },
            almanac.maps[0]
        );
        assert_eq!(
            Map {
                mappings: vec![
                    Mapping::parse("0 15 37"),
                    Mapping::parse("37 52 2"),
                    Mapping::parse("39 0 15"),
                ],
            },
            almanac.maps[1]
        );
    }

    #[test]
    fn test_part1() {
        let contents = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(35, part1(contents));
    }

    #[test]
    fn test_part2() {
        let contents = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(46, part2(contents));
    }

}
