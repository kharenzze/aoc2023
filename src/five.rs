use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;
type Tag = usize;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/5.{}", extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

#[derive(Debug)]
struct Game {
  input: Vec<Tag>,
  mappers: Vec<Mapper>,
}

#[derive(Debug)]
struct RangeGame {
  input: Vec<Tag>,
  mappers: Vec<Mapper>,
}

struct Rng {
  from: Tag,
  to: Tag,
}

#[derive(Debug)]
struct Mapper {
  title: String,
  ranges: Vec<RangeMapper>,
}

#[derive(Debug)]
struct RangeMapper {
  from: Tag,
  to: Tag,
  len: usize,
}

impl RangeGame {
  fn from_game(g: Game) -> Self {
    unimplemented!()
    /*
    let input: Vec<Rng> = g
      .input
      .chunks(2)
      .map(|c| Rng {
        from: c[0],
        to: c[1],
      })
      .collect();
    Self {
      input,
      mappers: g.mappers,
    };
     */
  }
}

impl Game {
  fn from_input(input: Input) -> Self {
    let mut it = input.into_iter();
    let input = it.next().unwrap();
    let input = Self::parse_seed(input.as_str());
    it.next(); //empty line

    let mut mappers: Vec<Mapper> = Vec::new();
    loop {
      let block_it = it.by_ref().take_while(|l| !l.is_empty());
      let slice = block_it.collect::<Vec<_>>();
      if slice.is_empty() {
        break;
      }
      let mapper = Mapper::from_input(&slice);
      mappers.push(mapper);
    }

    Self { input, mappers }
  }

  fn parse_seed(line: &str) -> Vec<Tag> {
    let list_string = line.split(":").last().unwrap().trim();
    list_string
      .split_whitespace()
      .map(|s| s.parse::<Tag>().unwrap())
      .collect()
  }

  fn play(&self) -> Vec<Tag> {
    self
      .input
      .iter()
      .map(|x| self.mappers.iter().fold(*x, |acc, m| m.map_input(acc)))
      .collect()
  }
}

impl Mapper {
  fn from_input(input: &[String]) -> Self {
    let mut it = input.iter();
    let title = it.next().unwrap().clone();
    let ranges = it.map(|s| RangeMapper::from_line(s)).collect::<Vec<_>>();
    Self { title, ranges }
  }

  fn map_input(&self, input: Tag) -> Tag {
    self
      .ranges
      .iter()
      .find_map(|r| r.map_value(input))
      .unwrap_or(input)
  }
}

impl RangeMapper {
  fn from_line(input: &str) -> Self {
    let mut it = input.split_whitespace();
    let local_parse = |s: &str| s.parse::<Tag>().unwrap();
    let to = local_parse(it.next().unwrap());
    let from = local_parse(it.next().unwrap());
    let len = local_parse(it.next().unwrap());
    Self { from, to, len }
  }

  fn map_value(&self, x: Tag) -> Option<Tag> {
    if self.input_is_in_range(x) {
      let res = x as isize + self.diff();
      Some(res as Tag)
    } else {
      None
    }
  }

  fn input_is_in_range(&self, x: Tag) -> bool {
    self.from <= x && x < self.max_input()
  }

  #[inline]
  fn max_input(&self) -> Tag {
    self.from + self.len
  }

  #[inline]
  fn diff(&self) -> isize {
    self.to as isize - self.from as isize
  }
}

fn initial(input: Input) -> usize {
  let game = Game::from_input(input);
  let mut result = game.play();
  result.sort();

  let first = result.first().unwrap();
  *first
}

fn extra(input: Input) -> usize {
  unimplemented!()
}

pub fn solve() {
  let input = read_data(false);
  let score = initial(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parsing() {
    let input = read_data(true);
    let game = Game::from_input(input);

    assert_eq!(game.mappers.len(), 7)
  }

  #[test]
  fn mapping() {
    let mapper = Mapper {
      title: "Mapper".to_string(),
      ranges: vec![
        RangeMapper {
          to: 50,
          from: 98,
          len: 2,
        },
        RangeMapper {
          to: 52,
          from: 50,
          len: 48,
        },
      ],
    };

    assert_eq!(mapper.map_input(98), 50);
    assert_eq!(mapper.map_input(1), 1);
    assert_eq!(mapper.map_input(53), 55);
  }

  #[test]
  fn simple() {
    let input = read_data(true);
    let score = initial(input);
    assert_eq!(score, 35)
  }
}
