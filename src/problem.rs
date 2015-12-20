use expr;

use constraint::*;
use expr::*;
use std::fmt::{Display, Error, Formatter};

pub enum ProblemObjective {
  Minimize(LinearExpression),
  Maximize(LinearExpression)
}

impl ProblemObjective {
  fn get_expr<'s>(&'s self) -> &'s LinearExpression {
    match self {
      &ProblemObjective::Maximize(ref expr) => expr,
      &ProblemObjective::Minimize(ref expr) => expr
      }
  }
  
  fn get_mut_expr<'s>(&'s mut self) -> &'s mut LinearExpression {
    match self {
      &mut ProblemObjective::Maximize(ref mut expr) => expr,
      &mut ProblemObjective::Minimize(ref mut expr) => expr
    }
  }
  
  fn set_expr(&mut self, expr: LinearExpression) {
    match self {
      &mut ProblemObjective::Maximize(ref mut self_expr) => *self_expr = expr,
      &mut ProblemObjective::Minimize(ref mut self_expr) => *self_expr = expr
    }
  }
}

impl Display for ProblemObjective {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let buf = match self {
      &ProblemObjective::Maximize(ref expr) => format!("maximize({})", expr),
      &ProblemObjective::Minimize(ref expr) => format!("minimize({})", expr)
    };
    fmt.write_str(buf.as_ref() as &str)
  }
}

pub struct Problem {
  objective: ProblemObjective,
  subject_to: Vec<LinearRelation>
}

impl Display for Problem {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let mut lines = vec![self.objective.to_string(),
                            String::from("subject to:")];
    lines.extend(self.subject_to.iter().map(|c|{c.to_string()}));
    fmt.write_str(lines.join("\n").as_ref() as &str)
  }
}


