use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

fn read_data() -> Input {
  let filename = format!("./resources/1.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

pub fn solve() {
  let input = read_data();
  let all = input.join("\n");
  main(all.as_str());
}

const STR_DIGITS: &[&[u8]] = &[
  b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn digit_sum(w: &[u8], part_two: bool) -> usize {
  let mut digits = (0..w.len()).filter_map(|i| match w[i] {
    b'0'..=b'9' => Some((w[i] - b'0') as usize),
    _ if part_two => STR_DIGITS
      .iter()
      .enumerate()
      .find_map(|(di, d)| w[i..].starts_with(d).then_some(di + 1)),
    _ => None,
  });
  let a = digits.next().unwrap();
  let b = digits.last().unwrap_or(a);
  a * 10 + b
}

fn main(input: &str) -> (usize, usize) {
  let lines = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
  let p1 = lines.iter().map(|line| digit_sum(line, false)).sum();
  let p2 = lines.iter().map(|line| digit_sum(line, true)).sum();
  dbg!(&p2);
  (p1, p2)
}
