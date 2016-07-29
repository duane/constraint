extern crate constraint;
extern crate itertools;

use constraint::grammar::*;
use constraint::var::VarIndex;
use std::io;
use std::io::BufRead;
use itertools::Itertools;

fn main() {
  let stdin = io::stdin();
  let input = stdin.lock().lines().map(|r|r.unwrap_or(String::new())).join(";");
  let mut index = VarIndex::new();
  let mut problem = parse_Problem(&mut index, input.as_ref()).unwrap();
  let mut tableau = problem.augmented_simplex().unwrap();
  tableau.simplex().unwrap();
  tableau.print();
}