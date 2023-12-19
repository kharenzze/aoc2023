use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/7.{}", extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
  HighCard,
  Pair,
  DoublePair,
  Three,
  Full,
  Four,
  Five,
}

type Cards = [char; 5];

#[derive(Debug)]
struct Hand {
  cards: Cards,
  bid: i64,
  kind: HandType,
  joker: bool,
}

fn string_to_cards(s: &str) -> Cards {
  let mut cards: Cards = ['0'; 5];
  for (i, c) in s.chars().enumerate() {
    cards[i] = c;
  }
  cards
}

fn cards_to_string(s: &Cards) -> String {
  String::from_utf8(s.iter().map(|&c| c as u8).collect::<Vec<u8>>()).unwrap()
}

impl Hand {
  fn from_line(line: &str, joker: bool) -> Self {
    let mut parts = line.split_whitespace();
    let cards_input = parts.next().unwrap();
    let mut cards: Cards = string_to_cards(&cards_input);
    let bid = parts.next().unwrap().parse().unwrap();
    let kind = if joker {
      HandType::from_cards_joker(&cards)
    } else {
      HandType::from_cards(&cards)
    };
    Self {
      cards,
      bid,
      kind,
      joker,
    }
  }
}

fn card_to_point(c: char) -> i64 {
  match c {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' => 11,
    'T' => 10,
    _ => c.to_digit(10).unwrap() as i64,
  }
}

fn card_to_point_joker(c: char) -> i64 {
  match c {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' => 1,
    'T' => 10,
    _ => c.to_digit(10).unwrap() as i64,
  }
}

impl PartialEq for Hand {
  fn eq(&self, other: &Self) -> bool {
    self.cards == other.cards
  }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    let cmp = self.kind.cmp(&other.kind);
    if cmp != Ordering::Equal {
      return cmp;
    }

    let map_to_point = if self.joker {
      card_to_point_joker
    } else {
      card_to_point
    };

    self
      .cards
      .iter()
      .zip(other.cards.iter())
      .map(|pair| (map_to_point(*pair.0), map_to_point(*pair.1)))
      .map(|pair| pair.0.cmp(&pair.1))
      .find(|&cmp| cmp != Ordering::Equal)
      .unwrap_or(Ordering::Equal)
  }
}

impl HandType {
  fn from_cards(cards: &Cards) -> Self {
    let map: HashMap<char, usize> = cards.iter().fold(Default::default(), |mut acc, c| {
      *acc.entry(*c).or_insert(0) += 1;
      acc
    });

    if map.values().any(|&c| c == 5) {
      Self::Five
    } else if map.values().any(|&c| c == 4) {
      Self::Four
    } else if map.values().any(|&c| c == 3) && map.values().any(|&c| c == 2) {
      Self::Full
    } else if map.values().any(|&c| c == 3) {
      Self::Three
    } else if map.values().filter(|&&v| v == 2).count() == 2 {
      Self::DoublePair
    } else if map.values().any(|&c| c == 2) {
      Self::Pair
    } else {
      Self::HighCard
    }
  }

  fn from_cards_joker(cards: &Cards) -> Self {
    let map: HashMap<char, usize> = cards.iter().fold(Default::default(), |mut acc, c| {
      *acc.entry(*c).or_insert(0) += 1;
      acc
    });

    let has_joker = map.get(&'J').is_some();
    if !has_joker {
      return Self::from_cards(cards);
    }

    let possible_values: Vec<char> = map
      .keys()
      .filter(|&&k| k != 'J')
      .map(char::to_owned)
      .collect();
    let cards_string: String = cards_to_string(cards);
    possible_values
      .iter()
      .map(|&c| cards_string.replace("J", c.to_string().as_str()))
      .map(|s| string_to_cards(&s))
      .map(|c| HandType::from_cards(&c))
      .max()
      .unwrap_or(HandType::Five)
  }
}

fn initial(input: Input) -> i64 {
  let mut hands: Vec<_> = input
    .iter()
    .map(String::as_ref)
    .map(|s| Hand::from_line(s, false))
    .collect();
  hands.sort();
  hands
    .iter()
    .enumerate()
    // .inspect(|p| {
    //   dbg!(p.1.cards.as_slice());
    // })
    .map(|(i, h)| (i as i64 + 1) * h.bid)
    .sum()
}

fn extra(input: Input) -> i64 {
  let mut hands: Vec<_> = input
    .iter()
    .map(String::as_ref)
    .map(|s| Hand::from_line(s, true))
    .collect();
  hands.sort();
  hands
    .iter()
    .enumerate()
    .map(|(i, h)| (i as i64 + 1) * h.bid)
    .sum()
}

pub fn solve() {
  let input = read_data(false);
  let score = extra(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hand() {
    let mut a = vec![HandType::Four, HandType::Pair, HandType::Three];
    a.sort();
    assert_eq!(a, [HandType::Pair, HandType::Three, HandType::Four]);
    let mut a = vec![1, 2];
    let mut b = vec![2, 1];
    b.sort();
    assert_eq!(a, b);
  }

  #[test]
  fn simple() {
    let input = read_data(true);
    let score = initial(input);
    assert_eq!(score, 6440)
  }

  #[test]
  fn diff() {
    let input = read_data(false);
    let mut score: Vec<_> = input
      .iter()
      .map(|s| s.split_whitespace().next().unwrap())
      .collect();
    score.dedup();
    assert_eq!(score.len(), 1000)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 5905)
  }
}
