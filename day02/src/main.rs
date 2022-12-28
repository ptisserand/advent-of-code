use std::env;
use std::fmt;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum OutCome {
    Win,
    Draw,
    Loss,
}
#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

// 'Move' parsing
impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
        }
    }
}

// 'Round' parsing
impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some (theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected <theirs>SP<ours>EOF, got {s:?}"));
        };

        Ok(Self {
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }
}

impl Move {
    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn outcome(self, theirs: Move) -> OutCome {
        match (self, theirs) {
            (Move::Rock, Move::Rock) => OutCome::Draw,
            (Move::Rock, Move::Paper) => OutCome::Loss,
            (Move::Rock, Move::Scissors) => OutCome::Win,
            (Move::Paper, Move::Rock) => OutCome::Win,
            (Move::Paper, Move::Paper) => OutCome::Draw,
            (Move::Paper, Move::Scissors) => OutCome::Loss,
            (Move::Scissors, Move::Rock) => OutCome::Loss,
            (Move::Scissors, Move::Paper) => OutCome::Win,
            (Move::Scissors, Move::Scissors) => OutCome::Draw,
        }
    }
}
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for round in contents.lines().map(|line| line.parse::<Round>()) {
        let round = round?;
        println!("{round:?}");
    };

    Ok(())
}
