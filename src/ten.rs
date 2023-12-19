use crate::point::{Direction, Point, DIRECTIONS};
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

fn read_data(is_test: usize) -> Input {
  let extension = if is_test == 0 {
    "txt".to_owned()
  } else {
    format!("test{}.txt", is_test)
  };
  let filename = format!("./resources/10.{}", extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
  NE,
  NS,
  NW,
  ES,
  EW,
  SW,
  Start,
  None,
}

impl Pipe {
  fn from_char(c: char) -> Self {
    match c {
      'S' => Self::Start,
      '|' => Self::NS,
      '-' => Self::EW,
      'L' => Self::NE,
      'J' => Self::NW,
      '7' => Self::SW,
      'F' => Self::ES,
      '.' => Self::None,
      _ => unreachable!(),
    }
  }

  fn from_dir_pair(d1: Direction, d2: Direction) -> Self {
    match d1 {
      Direction::Down => match d2 {
        Direction::Right => Self::NE,
        Direction::Left => Self::NW,
        Direction::Down => Self::NS,
        _ => unreachable!(),
      },
      Direction::Up => match d2 {
        Direction::Right => Self::ES,
        Direction::Left => Self::SW,
        Direction::Up => Self::NS,
        _ => unreachable!(),
      },
      Direction::Right => match d2 {
        Direction::Up => Self::ES,
        Direction::Down => Self::NE,
        Direction::Right => Self::EW,
        _ => unreachable!(),
      },
      Direction::Left => match d2 {
        Direction::Up => Self::SW,
        Direction::Down => Self::NW,
        Direction::Left => Self::EW,
        _ => unreachable!(),
      },
    }
  }

  fn flow_direction(&self, d: Direction) -> Option<Direction> {
    match self {
      Self::NS => match d {
        Direction::Down => Some(Direction::Down),
        Direction::Up => Some(Direction::Up),
        _ => None,
      },
      Self::EW => match d {
        Direction::Right => Some(Direction::Right),
        Direction::Left => Some(Direction::Left),
        _ => None,
      },
      Self::NE => match d {
        Direction::Down => Some(Direction::Right),
        Direction::Left => Some(Direction::Up),
        _ => None,
      },
      Self::NW => match d {
        Direction::Down => Some(Direction::Left),
        Direction::Right => Some(Direction::Up),
        _ => None,
      },
      Self::SW => match d {
        Direction::Up => Some(Direction::Left),
        Direction::Right => Some(Direction::Down),
        _ => None,
      },
      Self::ES => match d {
        Direction::Up => Some(Direction::Right),
        Direction::Left => Some(Direction::Down),
        _ => None,
      },
      _ => None,
    }
  }

  fn inner_directions(&self, d: Direction) -> Vec<Direction> {
    match self {
      Self::NS => match d {
        Direction::Down => vec![Direction::Left],
        Direction::Up => vec![Direction::Right],
        _ => vec![],
      },
      Self::EW => match d {
        Direction::Right => vec![Direction::Down],
        Direction::Left => vec![Direction::Up],
        _ => vec![],
      },
      Self::NE => match d {
        Direction::Down => vec![Direction::Down, Direction::Left],
        Direction::Left => vec![],
        _ => vec![],
      },
      Self::NW => match d {
        Direction::Down => vec![],
        Direction::Right => vec![Direction::Down, Direction::Right],
        _ => vec![],
      },
      Self::SW => match d {
        Direction::Up => vec![Direction::Up, Direction::Right],
        Direction::Right => vec![],
        _ => vec![],
      },
      Self::ES => match d {
        Direction::Up => vec![],
        Direction::Left => vec![Direction::Up, Direction::Left],
        _ => vec![],
      },
      _ => vec![],
    }
  }
}

struct Game {
  data: Vec<Vec<Pipe>>,
  dim: Point,
}

struct Analysis {
  upper_left: Point,
  map: HashSet<Point>,
  start: Pipe,
}

impl Game {
  fn from_input(input: Input) -> Self {
    let data: Vec<Vec<Pipe>> = input
      .into_iter()
      .map(|s| s.chars().map(Pipe::from_char).collect())
      .collect();
    let dim = Point::new(data.len(), data[0].len());
    Self { data, dim }
  }

  fn find_start(&self) -> Point {
    for x in 0..self.dim.x {
      for y in 0..self.dim.y {
        if self.data[x][y] == Pipe::Start {
          return Point::new(x, y);
        }
      }
    }
    unreachable!()
  }

  fn get_score(&self) -> usize {
    let start_point = self.find_start();
    for &dir in DIRECTIONS {
      let mut len: usize = 0;
      let mut current_dir = dir;
      let mut current_point = start_point;
      let mut loop_closed = false;
      loop {
        let p = current_point.get_next(current_dir, &self.dim);
        if p.is_none() {
          break;
        }
        let p = p.unwrap();
        let pipe = &self.data[p.x][p.y];
        if pipe == &Pipe::Start {
          loop_closed = true;
          break;
        }
        if let Some(next_dir) = pipe.flow_direction(current_dir) {
          current_dir = next_dir;
          current_point = p;
          len += 1;
        } else {
          break;
        }
      }
      if loop_closed {
        let mid = len / 2;
        return if len % 2 == 0 { mid } else { mid + 1 };
      }
    }
    unreachable!()
  }

  fn get_upper_left_corner(&self) -> Analysis {
    let start_point = self.find_start();
    let mut best = start_point;
    let mut point_map = HashSet::new();
    point_map.insert(best);
    let mut start_dir = Direction::Right;
    let mut end_dir = Direction::Right;
    for &dir in DIRECTIONS {
      let mut len: usize = 0;
      start_dir = dir;
      let mut current_dir = dir;
      let mut current_point = start_point;
      let mut loop_closed = false;
      loop {
        let p = current_point.get_next(current_dir, &self.dim);
        if p.is_none() {
          break;
        }
        let p = p.unwrap();
        let pipe = &self.data[p.x][p.y];
        if pipe == &Pipe::Start {
          loop_closed = true;
          end_dir = current_dir;
          break;
        }
        if let Some(next_dir) = pipe.flow_direction(current_dir) {
          current_dir = next_dir;
          current_point = p;
          len += 1;
          point_map.insert(p);
          if p.x < best.x {
            best = p;
          } else if p.x == best.x && p.y < best.y {
            best = p;
          }
        } else {
          break;
        }
      }
      if loop_closed {
        break;
      }
    }
    Analysis {
      upper_left: best,
      map: point_map,
      start: Pipe::from_dir_pair(start_dir, end_dir),
    }
  }

  fn get_extra_score(&self) -> usize {
    let analysis = self.get_upper_left_corner();
    let mut current_dir = Direction::Right;
    let mut current_point = analysis.upper_left;
    let mut explorable: HashSet<Point> = HashSet::new();
    loop {
      let p = current_point.get_next(current_dir, &self.dim).unwrap();
      let pipe = &self.data[p.x][p.y];
      if p == analysis.upper_left {
        break;
      }
      let pipe = if *pipe == Pipe::Start {
        analysis.start
      } else {
        *pipe
      };
      let inner = pipe.inner_directions(current_dir);
      for d in inner {
        let to_explore_opt = p.get_next(d, &self.dim);
        if let Some(to_explore) = to_explore_opt {
          explorable.insert(to_explore);
        }
      }
      if let Some(next_dir) = pipe.flow_direction(current_dir) {
        current_dir = next_dir;
        current_point = p;
      } else {
        break;
      }
    }

    let mut solution: HashSet<Point> = HashSet::new();
    let mut explored: HashSet<Point> = HashSet::new();
    while explorable.len() > 0 {
      let point = explorable.iter().next().cloned().unwrap();
      let point = explorable.take(&point).unwrap();

      if !analysis.map.contains(&point) {
        solution.insert(point);
        let next_points = point.get_points_around().into_iter().filter_map(|p| p);
        for next_point in next_points {
          if !explored.contains(&next_point) {
            explorable.insert(next_point);
          }
        }
      }
      explored.insert(point);
    }
    solution.iter().count()
  }
}

fn initial(input: Input) -> usize {
  let game = Game::from_input(input);
  let score = game.get_score();
  score
}

fn extra(input: Input) -> usize {
  let game = Game::from_input(input);
  let score = game.get_extra_score();
  score
}

pub fn solve() {
  let input = read_data(0);
  let score = extra(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    let input = read_data(1);
    let score = initial(input);
    assert_eq!(score, 8)
  }

  #[test]
  fn two() {
    let input = read_data(1);
    let score = extra(input);
    assert_eq!(score, 1)
  }

  #[test]
  fn three() {
    let input = read_data(2);
    let score = extra(input);
    assert_eq!(score, 8)
  }
}
