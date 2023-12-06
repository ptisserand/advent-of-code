use std::{env, fs};

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: usize,
    best_distance: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Competition {
    races: Vec<Race>,
}

impl Race {
    fn distance(&self, hold: usize) -> usize {
        if hold >= self.time {
            0
        } else {
            (self.time - hold) * hold
        }
    }

    fn limits(&self) -> (Option<usize>, Option<usize>) {
        let det = (self.time * self.time) as i32 - 4 * self.best_distance as i32;
        if det < 0 {
            return (None, None);
        }
        if det == 0 {
            (Some(self.time / 2), None)
        } else {
            let sqdet = (det as f64).sqrt();
            let a = (self.time as f64 - sqdet) / 2_f64;
            let b = (self.time as f64 + sqdet) / 2_f64;
            // println!("Results: {} {}", a, b);
            let a = if a == a.ceil() {
                a as usize + 1
            } else {
                a.ceil() as usize
            };
            let b = if b == b.floor() {
                b as usize - 1
            } else {
                b.floor() as usize
            };
            (Some(a), Some(b))
        }
    }

    fn nb_wins(&self) -> usize {
        match self.limits() {
            (None, None) => 0,
            (Some(_), None) => 1,
            (None, Some(_)) => 1,
            (Some(a), Some(b)) => (a..b + 1).len(),
        }
    }
}

impl Competition {
    fn parse(contents: &str) -> Self {
        let mut times: Vec<usize> = Vec::new();
        let mut distances: Vec<usize> = Vec::new();
        let mut races: Vec<Race> = Vec::new();
        let mut it = contents.lines();
        let re = Regex::new(r"(\d+)").unwrap();
        for d in re.captures_iter(it.next().unwrap()) {
            times.push(d[1].parse().unwrap());
        }
        for d in re.captures_iter(it.next().unwrap()) {
            distances.push(d[1].parse().unwrap());
        }
        for (t, d) in times.iter().zip(distances.iter()) {
            races.push(Race{time: *t, best_distance: *d})
        }
        Competition {
            races,
        }
    }
}

fn part1(contents: &str) -> usize {
    let competition = Competition::parse(contents);
    competition.races.iter().map(|r| r.nb_wins()).product()
}


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let value_part1 = part1(&contents);
    println!("Almanac part1: {}", value_part1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_distance() {
        let race = Race {
            time: 7,
            best_distance: 9,
        };
        assert_eq!(race.distance(0), 0);
        assert_eq!(race.distance(1), 6);
        assert_eq!(race.distance(2), 10);
        assert_eq!(race.distance(3), 12);
        assert_eq!(race.distance(4), 12);
        assert_eq!(race.distance(5), 10);
        assert_eq!(race.distance(6), 6);
        assert_eq!(race.distance(7), 0);
    }

    #[test]
    fn test_race_limits() {
        let race = Race {
            time: 7,
            best_distance: 9,
        };
        assert_eq!(race.limits(), (Some(2), Some(5)));
        let race = Race {
            time: 15,
            best_distance: 40,
        };
        assert_eq!(race.limits(), (Some(4), Some(11)));
        let race = Race {
            time: 30,
            best_distance: 200,
        };
        assert_eq!(race.limits(), (Some(11), Some(19)));
    }

    #[test]
    fn test_race_wins() {
        let race = Race {
            time: 7,
            best_distance: 9,
        };
        assert_eq!(race.nb_wins(), 4);
        let race = Race {
            time: 15,
            best_distance: 40,
        };
        assert_eq!(race.nb_wins(), 8);
        let race = Race {
            time: 30,
            best_distance: 200,
        };
        assert_eq!(race.nb_wins(), 9);
    }

    #[test]
    fn test_competition_parse() {
        let contents = r"Time:      7  15   30
Distance:  9  40  200";
        let competition = Competition::parse(contents);
        assert_eq!(competition.races.len(), 3);
        assert_eq!(
            competition.races[0],
            Race {
                time: 7,
                best_distance: 9
            }
        );
        assert_eq!(
            competition.races[1],
            Race {
                time: 15,
                best_distance: 40
            }
        );
        assert_eq!(
            competition.races[2],
            Race {
                time: 30,
                best_distance: 200
            }
        );
    }

    #[test]
    fn test_part1() {
        let contents = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(contents), 288)
    }
}
