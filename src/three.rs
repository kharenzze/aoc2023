use crate::point::Point;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/3.{}", extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

struct NumberMatch {
  pos: Point,
  text: String,
}

fn initial(input: Input) -> usize {
  let dim = Point::new(input.len(), input[0].len());
  let mut n_list: Vec<usize> = Default::default();
  for x in 0..dim.x {
    let mut acc_val: Option<NumberMatch> = None;
    let acc = &mut acc_val;
    let mut match_list: Vec<NumberMatch> = Default::default();
    for (y, c) in input[x].char_indices() {
      if c.is_digit(10) {
        if acc.is_none() {
          *acc = Some(NumberMatch {
            pos: Point::new(x, y),
            text: c.to_string(),
          });
        } else {
          let number_match = acc.as_mut().unwrap();
          number_match.text.push(c);
        }
      } else {
        if acc.is_some() {
          match_list.push(acc.take().unwrap());
        }
      }

      let is_last = y == dim.y - 1;
      if is_last && acc.is_some() {
        match_list.push(acc.take().unwrap());
      }
    }

    for m in match_list {
      let len = m.text.len();
      let mut points_to_check: Vec<Point> = Vec::with_capacity(len);
      for j in 0..len {
        points_to_check.push(Point::new(m.pos.x, m.pos.y + j));
      }

      let is_valid: bool = points_to_check.iter().any(|point| {
        point
          .get_points_around_indirect()
          .iter()
          .filter_map(|v| *v)
          .filter(|v| v.is_contained_in(&dim))
          .find(|p| is_special_char(input[p.x].chars().nth(p.y).unwrap()))
          .is_some()
      });
      if is_valid {
        let n = m.text.parse::<usize>().unwrap();
        n_list.push(n);
      }
    }
  }
  n_list.iter().sum()
}

fn extra(input: Input) -> usize {
  let dim = Point::new(input.len(), input[0].len());
  let mut gear_map: HashMap<Point, HashSet<usize>> = Default::default();

  for x in 0..dim.x {
    let mut acc_val: Option<NumberMatch> = None;
    let acc = &mut acc_val;
    let mut match_list: Vec<NumberMatch> = Default::default();
    for (y, c) in input[x].char_indices() {
      if c.is_digit(10) {
        if acc.is_none() {
          *acc = Some(NumberMatch {
            pos: Point::new(x, y),
            text: c.to_string(),
          });
        } else {
          let number_match = acc.as_mut().unwrap();
          number_match.text.push(c);
        }
      } else {
        if acc.is_some() {
          match_list.push(acc.take().unwrap());
        }
      }

      let is_last = y == dim.y - 1;
      if is_last && acc.is_some() {
        match_list.push(acc.take().unwrap());
      }
    }

    for m in match_list {
      let len = m.text.len();
      let mut points_to_check: Vec<Point> = Vec::with_capacity(len);
      for j in 0..len {
        points_to_check.push(Point::new(m.pos.x, m.pos.y + j));
      }

      points_to_check
        .iter()
        .map(|point| {
          point
            .get_points_around_indirect()
            .iter()
            .filter_map(|v| *v)
            .filter(|v| v.is_contained_in(&dim))
            .filter(|p| is_gear(input[p.x].chars().nth(p.y).unwrap()))
            .collect::<HashSet<_>>()
        })
        .reduce(|a, b| a.union(&b).cloned().collect::<HashSet<_>>())
        .unwrap_or_default()
        .iter()
        .for_each(|p| {
          let n = m.text.parse::<usize>().unwrap();
          gear_map.entry(*p).or_insert_with(HashSet::new).insert(n);
        });
    }
  }

  gear_map
    .iter()
    .map(|(_, v)| v)
    .filter(|map| map.len() == 2)
    .map(|map| map.iter().cloned().reduce(|a, b| a * b).unwrap())
    .sum()
}

fn is_special_char(c: char) -> bool {
  !c.is_digit(10) && c != '.'
}

fn is_gear(c: char) -> bool {
  c == '*'
}

pub fn solve() {
  let input = read_data(false);
  let score = extra(input);
  println!("{score}");
}

#[cfg(test)]
mod tests {

  use super::*;
  #[test]
  fn simple() {
    let input = read_data(true);
    let score = initial(input);
    assert_eq!(score, 4361);
  }

  #[test]
  fn part_two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 467835);
  }
}
