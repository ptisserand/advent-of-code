use std::{error::Error, str::FromStr, collections::HashMap, cmp::Ordering, env, fs};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
enum Card {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAkind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}


impl Card {
    fn index(self) -> u32 {
        match self {
            Card::One => 0,
            Card::Two => 1,
            Card::Three => 2,
            Card::Four => 3,
            Card::Five => 4, 
            Card::Six => 5,
            Card::Seven => 6,
            Card::Eight => 7,
            Card::Nine => 8,
            Card::Ten => 9,
            Card::Jack => 10,
            Card::Queen => 11,
            Card::King => 12,
            Card::Ace => 13,
        }
    }
}

impl FromStr for Card {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Card::One),
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "J" => Ok(Card::Jack),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            _ => Err(format!("'{}' is not a card", s).into()),
        }
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        match self {
            Card::One => "1".to_string(),
            Card::Two => "2".to_string(),
            Card::Three => "3".to_string(),
            Card::Four => "4".to_string(),
            Card::Five => "5".to_string(),
            Card::Six => "6".to_string(),
            Card::Seven => "7".to_string(),
            Card::Eight => "8".to_string(),
            Card::Nine => "9".to_string(),
            Card::Ten => "T".to_string(),
            Card::Jack => "J".to_string(),
            Card::Queen => "Q".to_string(),
            Card::King => "K".to_string(),
            Card::Ace => "A".to_string(),
        }
    }
}

impl FromStr for Hand {
    type Err = Box<dyn Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut cards: [Card; 5] = [Card::One; 5];
        let mut splitted = line.split(' ');
        let it = splitted.next();
        for (i, c) in it.unwrap().chars().enumerate() {
            cards[i] = Card::from_str(c.to_string().as_str()).unwrap();
        }
        let bid = splitted.next().unwrap().parse().unwrap();
        Ok(Self { cards, bid })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = self.result();
        let b = other.result();
        if a == b {
            for i in 0..5 {
                if self.cards[i] != other.cards[i] {
                    return self.cards[i].partial_cmp(&other.cards[i]);
                }
            }
            Some(Ordering::Equal)
        } else {
            a.partial_cmp(&b)
        }
    }
}


impl Hand {
    fn result(&self) -> HandType {
        let mut m: HashMap<Card, usize> = HashMap::new();
        for c in self.cards {
            *m.entry(c).or_default() += 1;
        }
        let mut m_vec: Vec<_> = m.iter().collect();
        m_vec.sort_by(|a, b| a.1.cmp(b.1).reverse());
        match m_vec[0] {
            (_, 5) => HandType::FiveOfAKind,
            (_, 4) => HandType::FourOfAKind,
            (_, 3) => match m_vec[1] {
                (_, 2) => HandType::FullHouse,
                (_, _) => HandType::ThreeOfAkind,
            },
            (_, 2) => match m_vec[1] {
                (_, 2) => HandType::TwoPair,
                (_, _) => HandType::OnePair,
            },
            (_, _) => HandType::HighCard,
        }
    }
}

fn part1(contents: &str) -> u32 {
    let mut result = 0;
    let mut hands: Vec<Hand> = Vec::new();
    for line in contents.lines() {
        hands.push(Hand::from_str(line).unwrap());
    }
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for (i, h) in hands.iter().enumerate() {
        result += h.bid * (i + 1) as u32;
    }
    result
}

fn part2(contents: &str) -> u32 {
    0
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let value_part1 = part1(&contents);
    println!("Part1: {}", value_part1);
    let value_part2 = part2(&contents);
    println!("Part2: {}", value_part2);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_order() {
        assert!(Card::One < Card::Two);
        assert!(Card::Ace > Card::Queen);
    }

    #[test]
    fn test_card_parsing() {
        assert_eq!(Card::from_str("A").unwrap(), Card::Ace);
        assert_eq!(Card::from_str("5").unwrap(), Card::Five);
        assert_eq!(Card::from_str("8").unwrap(), Card::Eight);
    }

    #[test]
    fn test_hand_parsing() {
        assert_eq!(
            Hand {
                cards: [Card::One, Card::Ten, Card::Jack, Card::Queen, Card::Ace],
                bid: 324
            },
            Hand::from_str("1TJQA 324").unwrap()
        );
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(
            Hand::from_str("32T3K 765").unwrap().result(),
            HandType::OnePair
        );
        assert_eq!(
            Hand::from_str("T55J5 684").unwrap().result(),
            HandType::ThreeOfAkind
        );
        assert_eq!(
            Hand::from_str("KK677 28").unwrap().result(),
            HandType::TwoPair
        );
        assert_eq!(
            Hand::from_str("KTJJT 220").unwrap().result(),
            HandType::TwoPair
        );
        assert_eq!(
            Hand::from_str("QQQJA 483").unwrap().result(),
            HandType::ThreeOfAkind
        );
    }

    #[test]
    fn test_hand_compare() {
        let a = Hand::from_str("KK677 28").unwrap();
        let b = Hand::from_str("KTJJT 220").unwrap();
        assert!(a > b);
        let a = Hand::from_str("32T3K 765").unwrap();
        let b = Hand::from_str("T55J5 684").unwrap();
        assert!(a < b);
    }

    #[test]
    fn test_part1() {
        let contents = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part1(contents), 6440);
    }
}
