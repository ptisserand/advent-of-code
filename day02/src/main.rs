use std::env;
use std::fs;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Rock => write!(f, "rock"),
            Shape::Paper => write!(f, "paper"),
            Shape::Scissors => write!(f, "scissors"),
        }
    }
}

fn win(opponent: &Shape, me: &Shape) -> i32 {
    let mut winner = 0;
    if *opponent == *me {
        winner = 0;
        return winner;
    }
    if *opponent == Shape::Rock {
        if *me == Shape::Paper {
            winner = 1;
        } else {
            winner = -1;
        }
    }
    if *opponent == Shape::Paper {
        if *me == Shape::Scissors {
            winner = 1;
        } else {
            winner = -1;
        }
    }
    if *opponent == Shape::Scissors {
        if *me == Shape::Rock {
            winner = 1;
        } else {
            winner = -1;
        }
    }
    return winner;
}

fn strategy_part1(line: &str) -> (Shape, Shape) {
    let opponent = match line.as_bytes()[0] {
        b'A' => Shape::Rock,
        b'B' => Shape::Paper,
        b'C' => Shape::Scissors,
        _ => Shape::Rock,
    };
    let me = match line.as_bytes()[2] {
        b'X' => Shape::Rock,
        b'Y' => Shape::Paper,
        b'Z' => Shape::Scissors,
        _ => Shape::Rock,
    };
    (opponent, me)
}

fn loser(opponent: &Shape) -> Shape {
    match *opponent {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

fn winner(opponent: &Shape) -> Shape {
    match *opponent {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}


fn strategy_part2(line: &str) -> (Shape, Shape) {
    let opponent = match line.as_bytes()[0] {
        b'A' => Shape::Rock,
        b'B' => Shape::Paper,
        b'C' => Shape::Scissors,
        _ => Shape::Rock,
    };
    let me = match line.as_bytes()[2] {
        b'X' => loser(&opponent),
        b'Y' => opponent, // draw
        b'Z' => winner(&opponent),
        _ => Shape::Rock,
    };
    (opponent, me)
}

fn apply_strategy(contents: &String, strategy: fn(&str) -> (Shape, Shape)) -> i32 {
    let mut total = 0;
    for line in (*contents).lines() {
        let round = strategy(line);
        let score = win(&round.0, &round.1);
        total += match round.1 {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };
        total += match score {
            -1 => 0,
            0 => 3,
            1 => 6,
            _ => 0,
        };
    }
    total
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut score =  apply_strategy(&contents, strategy_part1);
    println!("Score part 1: {}", score);
    score = apply_strategy(&contents, strategy_part2);
    println!("Score part 2: {}", score);
}
