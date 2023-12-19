use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

#[derive(Debug)]
struct Card {
  id: usize,
  numbers: HashSet<usize>,
  winning: HashSet<usize>,
  amount: usize,
}

impl Card {
  fn from_line(l: &str) -> Self {
    let mut parts = l.split(":");
    let id: usize = parts
      .next()
      .unwrap()
      .split_whitespace()
      .last()
      .unwrap()
      .parse()
      .unwrap();
    let mut parts = parts.next().unwrap().split("|");
    let numbers = Card::extract_number_set(parts.next().unwrap());
    let winning = Card::extract_number_set(parts.next().unwrap());
    Self {
      id,
      numbers,
      winning,
      amount: 1,
    }
  }

  fn extract_number_set(text: &str) -> HashSet<usize> {
    text
      .trim()
      .split_whitespace()
      .map(|s| s.parse::<usize>().unwrap())
      .collect()
  }

  fn get_matches(&self) -> usize {
    self.winning.intersection(&self.numbers).count()
  }

  fn get_points(&self) -> usize {
    let n = self.get_matches();
    if let Some(exp) = n.checked_sub(1) {
      2_usize.pow(exp as u32)
    } else {
      0
    }
  }
}

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/4.{}", extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

fn initial(input: Input) -> usize {
  input
    .iter()
    .map(String::as_str)
    .map(Card::from_line)
    .map(|c| c.get_points())
    .sum()
}

fn extra(input: Input) -> usize {
  let mut cards: Vec<Card> = input
    .iter()
    .map(String::as_str)
    .map(Card::from_line)
    .collect();

  let n = cards.len();

  for i in 0..n {
    let card = &cards[i];
    let matches = card.get_matches();
    if matches == 0 {
      continue;
    }

    let amount = card.amount;
    for j in (i + 1)..=(i + matches) {
      let next_card = cards.get_mut(j);
      if next_card.is_none() {
        continue;
      }
      next_card.unwrap().amount += amount;
    }
  }
  cards.iter().map(|c| c.amount).sum()
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
  fn simple() {
    let input = read_data(true);
    let score = initial(input);
    assert_eq!(score, 13)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 30)
  }
}
