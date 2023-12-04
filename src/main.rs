mod one;
mod point;
mod three;
mod two;

use std::env;
fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Invalid number of arguments")
  }

  let day: &str = args.get(1).unwrap();

  match day {
    "1" => crate::one::solve(),
    "2" => crate::two::solve(),
    "3" => crate::three::solve(),
    _ => unreachable!(),
  }
}
