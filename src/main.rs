mod eight;
mod eleven;
mod five;
mod four;
mod nine;
mod one;
mod point;
mod seven;
mod six;
mod ten;
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
    "4" => crate::four::solve(),
    "5" => crate::five::solve(),
    "6" => crate::six::solve(),
    "7" => crate::seven::solve(),
    "8" => crate::eight::solve(),
    "9" => crate::nine::solve(),
    "10" => crate::ten::solve(),
    "11" => crate::eleven::solve(),
    _ => unreachable!(),
  }
}
