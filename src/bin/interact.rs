extern crate constraint;
extern crate itertools;

use constraint::grammar::*;
use std::io;
use std::io::BufRead;
use itertools::Itertools;

fn main() {
  let stdin = io::stdin();
  let input = stdin.lock().lines().map(|r|r.unwrap_or(String::new())).join(";");
  let problem = parse_Problem(input.as_ref()).unwrap();
  let mut tableau = problem.augmented_simplex().unwrap();
  tableau.simplex().unwrap();
  tableau.print();
}