use crate::point::Point;
use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/11.{}", extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

struct Game {
  map: Vec<Vec<char>>,
  points: Vec<Point>,
}

impl Game {
  fn new(input: Input, expansion_factor: usize) -> Self {
    let map: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();
    let mut empty_rows: Vec<usize> = vec![];
    for (i, row) in map.iter().enumerate() {
      if row.iter().all(|&c| c == '.') {
        empty_rows.push(i);
      }
    }

    let mut empty_cols: Vec<usize> = vec![];
    for col in 0..map.len() {
      let is_empty: bool = map.iter().map(|row| row[col]).all(|c| c == '.');
      if is_empty {
        empty_cols.push(col);
      }
    }

    let mut points = vec![];
    for i in 0..map.len() {
      for j in 0..map[i].len() {
        if map[i][j] == '#' {
          points.push(Point::new(i, j));
        }
      }
    }

    //Apply expansion to points
    for point in points.iter_mut() {
      let expanded_rows = empty_rows.iter().take_while(|x| **x < point.x).count();
      let expanded_cols = empty_cols.iter().take_while(|y| **y < point.y).count();
      let extra = expanded_rows * (expansion_factor - 2);
      point.x += expanded_rows + extra;
      let extra = expanded_cols * (expansion_factor - 2);
      point.y += expanded_cols + extra;
    }

    Self { map, points }
  }

  fn get_score(&self) -> usize {
    let pairs: Vec<(&Point, &Point)> = self.points.iter().tuple_combinations().collect();
    pairs
      .iter()
      .map(|&(p1, p2)| p1.abs_diff(*p2))
      .map(|p| p.x + p.y)
      .sum()
  }
}

fn initial(input: Input) -> usize {
  let game = Game::new(input, 2);
  game.get_score()
}

fn extra(input: Input) -> usize {
  let game = Game::new(input, 1000000);
  game.get_score()
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
    assert_eq!(score, 374)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let game = Game::new(input, 100);
    let score = game.get_score();
    assert_eq!(score, 8410)
  }
}
