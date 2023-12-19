use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

type Input = Vec<String>;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/9.{}", extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

fn initial(input: Input) -> i64 {
  input
    .iter()
    .map(|s| Game::from_str(&s).unwrap().get_score())
    .sum()
}

fn extra(input: Input) -> i64 {
  input
    .iter()
    .map(|s| Game::from_str(&s).unwrap().get_score_backwards())
    .sum()
}

struct Game {
  data: Vec<i64>,
}

impl FromStr for Game {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let data: Vec<i64> = s
      .split_whitespace()
      .map(|text| text.parse().unwrap())
      .collect();
    Ok(Self { data })
  }
}

impl Game {
  fn compute_diffs(a: &Vec<i64>) -> Vec<i64> {
    let l = a.len();
    assert!(l > 1);
    let l = l - 1;
    let mut b = vec![0; l];

    for i in 0..l {
      b[i] = a[i + 1] - a[i];
    }

    b
  }

  fn get_score(&self) -> i64 {
    let mut data = vec![self.data.clone()];
    loop {
      let next = Game::compute_diffs(&data[data.len() - 1]);
      let is_all_zero = next.iter().all(|&n| n == 0);
      data.push(next);
      if is_all_zero {
        break;
      }
    }

    let mut extra: Vec<i64> = vec![0; data.len()];
    let l = data.len();
    for i in 1..data.len() {
      extra[i] = extra[i - 1] + data[l - i - 1].iter().last().unwrap();
    }

    extra.iter().last().unwrap().to_owned()
  }

  fn get_score_backwards(&self) -> i64 {
    let mut data = vec![self.data.clone()];
    loop {
      let next = Game::compute_diffs(&data[data.len() - 1]);
      let is_all_zero = next.iter().all(|&n| n == 0);
      data.push(next);
      if is_all_zero {
        break;
      }
    }

    let mut extra: Vec<i64> = vec![0; data.len()];
    let l = data.len();
    for i in 1..data.len() {
      extra[i] = data[l - i - 1].iter().next().unwrap() - extra[i - 1];
    }

    extra.iter().last().unwrap().to_owned()
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
    assert_eq!(score, 114)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 2)
  }
}
