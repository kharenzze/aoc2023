use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::isize;

type Input = Vec<String>;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/6.{}", extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

#[derive(Debug)]
struct Game {
  time: i64,
  distance: i64,
}

impl Game {
  fn from_pair(pairs: (&i64, &i64)) -> Self {
    Self {
      time: *pairs.0,
      distance: *pairs.1,
    }
  }

  fn play(&self) -> i64 {
    let range = (1..self.time);
    let reversed = range.clone().rev();
    range
      .zip(reversed)
      .map(|p| p.0 * p.1)
      .filter(|n| n.gt(&self.distance))
      .count() as i64
  }
}

fn parse_line(l: &str) -> Vec<i64> {
  l.split(":")
    .last()
    .unwrap()
    .trim()
    .split_whitespace()
    .map(str::parse::<i64>)
    .map(Result::unwrap)
    .collect()
}

fn parse_line_extra(l: &str) -> i64 {
  l.split(":")
    .last()
    .unwrap()
    .trim()
    .split_whitespace()
    .collect::<Vec<&str>>()
    .join("")
    .parse()
    .unwrap()
}

fn initial(input: Input) -> i64 {
  let mut input_iter = input.iter();
  let times = parse_line(input_iter.next().unwrap());
  let distances = parse_line(input_iter.next().unwrap());
  let games: Vec<Game> = times
    .iter()
    .zip(distances.iter())
    .map(Game::from_pair)
    .collect();

  games
    .iter()
    .map(Game::play)
    .fold(1, |acc, value| acc * value)
}

fn extra(input: Input) -> i64 {
  let mut input_iter = input.iter();
  let time = parse_line_extra(input_iter.next().unwrap());
  let distance = parse_line_extra(input_iter.next().unwrap());
  let mut t: i64 = 1;
  loop {
    let remaining = time - t;
    let d = t * remaining;
    if d > distance {
      break;
    }
    t += 1;
  }
  let half = (time / 2) - t;
  if time % 2 == 0 {
    2 * half + 1
  } else {
    2 * half
  }
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
    assert_eq!(score, 288)
  }

  #[test]
  fn part2() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 71503)
  }
}
