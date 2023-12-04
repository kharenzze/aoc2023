use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

#[derive(Debug, Default)]
struct GameInfo {
  id: usize,
  game: Vec<BallSet>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct BallSet {
  r: usize,
  b: usize,
  g: usize,
}

impl BallSet {
  fn is_contained_in(&self, other: &Self) -> bool {
    self.r <= other.r && self.b <= other.b && self.g <= other.g
  }

  fn get_power(&self) -> usize {
    self.r * self.b * self.g
  }

  fn get_union(&self, other: &Self) -> Self {
    Self {
      r: self.r.max(other.r),
      b: self.b.max(other.b),
      g: self.g.max(other.g),
    }
  }
}

impl GameInfo {
  fn from_line(l: &str) -> Self {
    let mut parts = l.split(":");
    let game_part = parts.next().unwrap();
    let id: usize = game_part.split(" ").last().unwrap().parse().unwrap();
    let sets = parts.next().unwrap().split(";");
    let game = sets.map(GameInfo::parse_ball_set).collect();
    Self { id, game }
  }

  fn parse_ball_set(bs: &str) -> BallSet {
    let parts = bs.split(",").map(|p| p.trim());
    let mut ball_set: BallSet = Default::default();
    for part in parts {
      let mut part = part.split(" ");
      let n: usize = part.next().unwrap().parse().unwrap();
      let t: &str = part.last().unwrap();
      match t {
        "red" => ball_set.r = n,
        "blue" => ball_set.b = n,
        "green" => ball_set.g = n,
        _ => unreachable!(),
      }
    }
    ball_set
  }
}

fn read_data() -> Input {
  let filename = format!("./resources/2.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

fn initial(input: Input) -> usize {
  let game_infos: Vec<GameInfo> = input
    .iter()
    .map(|a| a.as_str())
    .map(GameInfo::from_line)
    .collect();
  let max = BallSet {
    r: 12,
    g: 13,
    b: 14,
  };
  let score: usize = game_infos
    .iter()
    .filter(|&info| {
      info
        .game
        .iter()
        .all(|ball_set| ball_set.is_contained_in(&max))
    })
    .map(|info| info.id)
    .sum();
  score
}

fn extra(input: Input) -> usize {
  let game_infos: Vec<GameInfo> = input
    .iter()
    .map(|a| a.as_str())
    .map(GameInfo::from_line)
    .collect();
  let score: usize = game_infos
    .iter()
    .map(|info| {
      info
        .game
        .iter()
        .fold(BallSet::default(), |a, b| a.get_union(&b))
        .get_power()
    })
    .sum();
  score
}

pub fn solve() {
  let input = read_data();
  let score = extra(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let info: GameInfo = GameInfo::from_line(line);
    assert_eq!(info.id, 1);
    assert_eq!(info.game.len(), 3);
    assert_eq!(info.game[0], BallSet { r: 4, b: 3, g: 0 });
    assert_eq!(info.game[1], BallSet { r: 1, b: 6, g: 2 });
    assert_eq!(info.game[2], BallSet { r: 0, b: 0, g: 2 });
    assert!(true);
  }

  static SAMPLE: &'static str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

  #[test]
  fn sample_data() {
    let score = initial(SAMPLE.lines().map(String::from).collect());
    assert_eq!(score, 8);
  }

  #[test]
  fn sample_data_extra() {
    let score = extra(SAMPLE.lines().map(String::from).collect());
    assert_eq!(score, 2286);
  }
}
