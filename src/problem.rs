use expr;

use constraint::*;
use expr::*;
use std::fmt::{Display, Error, Formatter};
use std::collections::{HashSet, LinkedList};
use state::*;
use std::collections::BTreeSet;

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

impl Problem {
  pub fn new(objective: ProblemObjective, subject_to: Vec<LinearRelation>) -> Problem {
    Problem{
      objective: objective,
      subject_to: subject_to
    }
  }

  fn convert_to_leq_and_eq(&self, lr: &mut LinearRelation) {
    match lr.op {
      Relation::EQ | Relation::NEQ | Relation::LEQ | Relation::LT => (),
      Relation::GEQ | Relation::GT => {
        lr.reverse();
      }
    }
    if lr.op == Relation::LT {
      lr.rhs.plus_this(&LinearExpression::from(SCALAR_EPSILON * 2.0));
      lr.op = Relation::LEQ;
    }
  }

  // Returns true iff lr is (0 <= VAR)
  fn get_restrained_var<'s, 'r>(&'s self, lr: &'r LinearRelation) -> Option<Variable> {
    let lhs_valid = lr.lhs.terms().is_empty() && approx_eq(0.0, lr.lhs.get_constant());
    let op_valid = lr.op == Relation::LEQ;
    let rhs_valid = lr.rhs.terms().len() == 1 && *lr.rhs.terms().values().next().unwrap() >= 0.0;
    if lhs_valid && op_valid && rhs_valid {
      Some(lr.rhs.terms().keys().next().unwrap().clone())
    } else {
      None
    }
  }

  // lhs + c_1 <= rhs + c_2
  // (lhs - rhs) + (c_1 - c_2) <= 0
  // lhs - rhs <= c_2 - c_1
  // lhs - rhs + s_n == c_2 - c_1, 0 <= s_n
  fn convert_leq_to_eq<'s, 'r>(&'s self, lr: &'r mut LinearRelation, namer: &mut Namer) -> Option<Variable> {
    if (lr.op == Relation::LEQ) {
      let slack = Variable::from(namer.vend());
      lr.lhs.plus_this(&lr.rhs);
      lr.rhs = LinearExpression::from(-lr.lhs.get_constant());
      lr.lhs.set_constant(0.0);
      lr.lhs.plus_this(&LinearExpression::from(slack.clone()));
      lr.op = Relation::EQ;
      Some(slack)
    } else {
      None
    }
  }

  // 1: convert to LEQs and NEQs. Err on NEQs.
  // 2: Note if this forms a `restrained` constraint for a specific variable: 0 <= var. Special case. Else, treat like normal slack: -var + s_n <= 0.
  // 3: Convert LEQs to equations, generating a slack variable. Add to restricted variables.
  // 4: Return: <equations, slack variables>
  pub fn augmented_simplex(&self) {
    let raw_constraints: LinkedList<LinearRelation> = self.subject_to.iter().map(|c|{c.clone()}).collect();
    let mut normalized_constraints = LinkedList::<LinearRelation>::new();
    let mut restrained_vars = BTreeSet::<Variable>::new();
    let mut slack_namer = Namer::init("s_");

    for mut constraint in raw_constraints.into_iter() {
      match constraint.op {
        Relation::NEQ => {
          panic!("I don't know what to do!");
          unreachable!()
        }
        Relation::EQ => {
          normalized_constraints.push_back(constraint);
          continue;
        }
        Relation::LEQ => (),
        _ => self.convert_to_leq_and_eq(&mut constraint)
      }
      match self.get_restrained_var(&constraint) {
        Some(var) => {
          restrained_vars.insert(var);
          continue;
        }
        _ => ()
      }
      match self.convert_leq_to_eq(&mut constraint, &mut slack_namer) {
        Some(var) => {
          restrained_vars.insert(var);
        }
        _ => ()
      }
      normalized_constraints.push_back(constraint);
    }
    println!("{}", self.objective);
    println!("subject to:");
    for constraint in normalized_constraints {
      println!("{}", constraint);
    }
    println!("and");
    let s: Vec<&str> = restrained_vars.iter().map(|s|{s.name.as_ref() as &str}).collect();
    println!("0 <= {}", s.join(", "));
  }
}

impl Display for Problem {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let mut lines = vec![self.objective.to_string(),
                            String::from("subject to:")];
    lines.extend(self.subject_to.iter().map(|c|{c.to_string()}));
    fmt.write_str(lines.join("\n").as_ref() as &str)
  }
}

mod test {
  use grammar::*;
  use expr::*;
  use super::*;
  
  #[test]
  fn test_problem_parse() {
    let buf = r#"minimize(x_m+-1.0x_t);2x_m==x_t+x_r;x_l+10<=x_r;x_l>=0;x_r<=100"#;
    let problem = parse_Problem(buf).unwrap();
    println!("before problem: {}", problem);
    println!("");
    problem.augmented_simplex();
  }
}
