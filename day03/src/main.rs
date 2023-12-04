use std::env;
use std::fs;

#[derive(Debug, Copy, Clone)]
struct EnginePart {
    part_number: u32,
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
}

#[derive(Debug)]
struct GearSymbol {
    row_idx: usize,
    col_idx: usize,
}

#[derive(Debug)]
struct Symbol {
    row_idx: usize,
    col_idx: usize,
}

fn is_adjacent(part: &EnginePart, symbol: &Symbol) -> bool {
    let row_offset: usize = if part.row_start != 0 { 1 } else { 0 };
    let col_offset: usize = if part.col_start != 0 { 1 } else { 0 };
    (symbol.row_idx >= part.row_start - row_offset)
        && (symbol.row_idx <= part.row_end + 1)
        && (symbol.col_idx >= part.col_start - col_offset)
        && (symbol.col_idx <= part.col_end + 1)
}

fn is_part_number(part: &EnginePart, symbols: &Vec<Symbol>) -> bool {
    let mut ret = false;
    for symbol in symbols {
        ret = is_adjacent(part, symbol);
        if ret {
            // println!("{:#?}", part);
            // println!("{:#?}", symbol);
            break;
        };
    }
    ret
}

fn parsing(input: &str) -> (Vec<EnginePart>, Vec<Symbol>, Vec<GearSymbol>) {
    let mut parts = Vec::<EnginePart>::new();
    let mut symbols = Vec::<Symbol>::new();
    let mut gears = Vec::<GearSymbol>::new();
    let mut is_number = false;
    let mut number = 0_u32;

    let mut row_start: usize = 0;
    let mut col_start: usize = 0;
    let mut col_idx: usize;
    for (row_idx, line) in input.lines().enumerate() {
        col_idx = 0;
        for c in line.chars() {
            if "0123456789".contains(c) {
                if !is_number {
                    row_start = row_idx;
                    col_start = col_idx;
                }
                number = number * 10 + (c as u32 - '0' as u32);
                is_number = true;
            } else {
                if is_number {
                    parts.push(EnginePart {
                        part_number: number,
                        row_start,
                        row_end: row_start,
                        col_start,
                        col_end: if col_idx > 0 { col_idx - 1 } else { col_idx },
                    });
                    is_number = false;
                    number = 0;
                }
                if c != '.' {
                    if c == '*' {
                        gears.push(GearSymbol { row_idx, col_idx });
                    }
                    symbols.push(Symbol { row_idx, col_idx });
                }
            }
            col_idx += 1;
        }
        if is_number {
            parts.push(EnginePart {
                part_number: number,
                row_start,
                row_end: row_start,
                col_start,
                col_end: if col_idx > 0 { col_idx - 1 } else { col_idx },
            });
            is_number = false;
            number = 0;
        }
    }
    (parts, symbols, gears)
}

fn part1(input: &str) -> u32 {
    let (parts, symbols, _) = parsing(input);
    parts
        .iter()
        .filter(|p| is_part_number(p, &symbols))
        .map(|p| p.part_number)
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    let (parts, symbols, gears) = parsing(input);
    let parts: Vec<&EnginePart> = parts
        .iter()
        .filter(|p| is_part_number(p, &symbols)).collect();
    for gear in gears {
        let mut gear_parts = Vec::<EnginePart>::new();
        for part in &parts {
            if is_adjacent(part, &Symbol{row_idx: gear.row_idx, col_idx: gear.col_idx}) {
                gear_parts.push(**part);
            }
        }
        if gear_parts.len() == 2 {
            result += gear_parts[0].part_number * gear_parts[1].part_number;
        }
    }        
    result
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let value_part1 = part1(&contents);
    println!("Engine part1: {}", value_part1);
    let value_part2 = part2(&contents);
    println!("Engine part1: {}", value_part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, part1(input));
    }

    #[test]
    fn test_part_2() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835, part2(input));
    }

    #[test]
    fn test_is_part_number() {
        assert!(is_part_number(
            &EnginePart {
                part_number: 446,
                row_start: 139,
                row_end: 139,
                col_start: 137,
                col_end: 139
            },
            &vec![Symbol {
                row_idx: 138,
                col_idx: 136
            }]
        ))
    }
}
