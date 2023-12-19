use num::integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

type Input = Vec<String>;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/8.{}", extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
  id: String,
  left: String,
  right: String,
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
  Left,
  Right,
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "L" => Ok(Instruction::Left),
      "R" => Ok(Instruction::Right),
      _ => Err(()),
    }
  }
}

impl FromStr for Node {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      id: s[0..3].to_string(),
      left: s[7..10].to_string(),
      right: s[12..15].to_string(),
    })
  }
}

impl Node {
  fn next(&self, i: &Instruction) -> &str {
    match i {
      Instruction::Left => &self.left,
      Instruction::Right => &self.right,
    }
  }
}

static START: &str = "AAA";
static TARGET: &str = "ZZZ";

fn initial(input: Input) -> usize {
  let mut iter_input = input.iter();
  let instructions: Vec<Instruction> = iter_input
    .next()
    .unwrap()
    .chars()
    .map(|c| c.to_string())
    .map(|c| Instruction::from_str(&c).unwrap())
    .collect();
  iter_input.next(); // left blank
  let nodes: Vec<Node> = iter_input.map(|s| Node::from_str(s).unwrap()).collect();
  let map: HashMap<String, Node> = nodes.into_iter().fold(Default::default(), |mut acc, node| {
    acc.insert(node.id.clone(), node);
    acc
  });
  let mut current_pos = START.to_string();
  let mut steps: usize = 0;
  for i in instructions.iter().cycle() {
    steps += 1;
    let node = map.get(&current_pos).unwrap();
    let next = node.next(i);
    if next == TARGET {
      break;
    }
    current_pos = next.to_string();
  }

  steps
}

fn extra(input: Input) -> usize {
  let mut iter_input = input.iter();
  let instructions: Vec<Instruction> = iter_input
    .next()
    .unwrap()
    .chars()
    .map(|c| c.to_string())
    .map(|c| Instruction::from_str(&c).unwrap())
    .collect();
  iter_input.next(); // left blank
  let nodes: Vec<Node> = iter_input.map(|s| Node::from_str(s).unwrap()).collect();
  let mut current_positions: Vec<String> = nodes
    .iter()
    .map(|n| &n.id)
    .filter(|s| s.ends_with("A"))
    .cloned()
    .collect();
  let map: HashMap<String, Node> = nodes.into_iter().fold(Default::default(), |mut acc, node| {
    acc.insert(node.id.clone(), node);
    acc
  });

  let min_steps = current_positions
    .iter()
    .map(|s| {
      let mut current_pos = s.clone();
      let mut steps: usize = 0;
      for i in instructions.iter().cycle() {
        steps += 1;
        let node = map.get(&current_pos).unwrap();
        let next = node.next(i);
        if next.ends_with("Z") {
          break;
        }
        current_pos = next.to_string();
      }
      steps
    })
    .fold(1, lcm);

  min_steps
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
  fn parse() {
    let input = "AAA = (BBB, BBB)";
    let n = Node::from_str(input).unwrap();
    let expected = Node {
      id: "AAA".to_string(),
      left: "BBB".to_string(),
      right: "BBB".to_string(),
    };
    assert_eq!(n, expected)
  }

  #[test]
  fn simple() {
    let input = read_data(true);
    let score = initial(input);
    assert_eq!(score, 6)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 13)
  }
}
