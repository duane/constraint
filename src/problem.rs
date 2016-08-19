use expr::*;
use std::fmt::{Display, Error, Formatter};
use std::collections::{HashMap, HashSet, LinkedList};
use state::*;
use tableau::*;
use var::{VarIndex, VarRef};

#[derive(Clone)]
pub enum ProblemObjective {
  Minimize(LinearExpression),
  Maximize(LinearExpression)
}

impl ProblemObjective {
  ///
  /// Get the expression to either maximize or minimize.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::{approx_eq, LinearExpression};
  /// use constraint::problem::ProblemObjective;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let objective = ProblemObjective::Minimize(LinearExpression::from(index.external(String::from("x"))));
  ///   assert!(approx_eq(1.0, objective.get_expr().get_coefficient(&index.external(String::from("x")))));
  ///   assert!(approx_eq(0.0, objective.get_expr().get_constant()));
  /// }
  /// ```
  pub fn get_expr<'s>(&'s self) -> &'s LinearExpression {
    match self {
      &ProblemObjective::Maximize(ref expr) => expr,
      &ProblemObjective::Minimize(ref expr) => expr
    }
  }

  ///
  /// Get the expression to either maximize or minimize.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::{approx_eq, LinearExpression};
  /// use constraint::problem::ProblemObjective;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let mut objective = ProblemObjective::Minimize(LinearExpression::new());
  ///   assert!(approx_eq(0.0, objective.get_expr().get_coefficient(&index.external(String::from("x")))));
  ///   objective.set_expr(LinearExpression::from(index.external(String::from("x"))));
  ///   assert!(approx_eq(1.0, objective.get_expr().get_coefficient(&index.external(String::from("x")))));
  /// }
  /// ```
  pub fn set_expr(&mut self, expr: LinearExpression) {
    match self {
      &mut ProblemObjective::Maximize(ref mut self_expr) => *self_expr = expr,
      &mut ProblemObjective::Minimize(ref mut self_expr) => *self_expr = expr
    }
  }

  fn substitute(&mut self, v: &VarRef, e: &LinearExpression) {
    let mut f_e = self.get_mut_expr();
    f_e.substitute(v, e);
  }

  fn get_mut_expr<'s>(&'s mut self) -> &'s mut LinearExpression {
    match self {
      &mut ProblemObjective::Maximize(ref mut expr) => expr,
      &mut ProblemObjective::Minimize(ref mut expr) => expr
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
  subject_to: Vec<LinearRelation>,
  index: VarIndex
}

impl Problem {
  ///
  /// Create a new problem based on an objective and constraints.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::{LinearExpression, LinearRelation, Relation};
  /// use constraint::problem::{Problem, ProblemObjective};
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let objective = ProblemObjective::Minimize(LinearExpression::from(index.external(String::from("x"))));
  ///   let constraints = vec!(LinearRelation::new(LinearExpression::from(index.external(String::from("x"))), Relation::EQ, LinearExpression::from(5.0)));
  ///   let _ = Problem::new(objective, constraints);
  /// }
  /// ```
  pub fn new(objective: ProblemObjective, subject_to: Vec<LinearRelation>, index: VarIndex) -> Problem {
    Problem{
      objective: objective,
      subject_to: subject_to,
      index: index
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
      lr.rhs.plus_this(&LinearExpression::from(scalar::EPSILON * 2.0));
      lr.op = Relation::LEQ;
    }
  }

  // lhs <= rhs
  // lhs + s_n == rhs, 0 <= s_n
  // 0 == rhs - lhs - s_n, 0 <= s_n
  // 0 >= (rhs - lhs - + s_n), 0 <= s_n
  // 0 == (rhs - lhs), 0 <= s_n
  fn convert_leq_to_eq<'s, 'r>(&'s mut self, lr: &'r mut LinearRelation, namer: &mut Namer) -> Option<VarRef> {
    if lr.op == Relation::LEQ {
      let slack = self.index.internal(namer.vend());
      lr.lhs.plus_this(&LinearExpression::from(slack.clone()));
      lr.lhs.times_this(-1.0);
      lr.rhs.plus_this(&lr.lhs);
      lr.lhs = LinearExpression::new();

      lr.op = Relation::EQ;
      Some(slack)
    } else {
      None
    }
  }

  ///
  /// Use the augmented simplex algorithm to convert the problem into a tableau adapted for the simplex algorithm.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::{approx_eq, LinearExpression, LinearRelation, Relation};
  /// use constraint::problem::{Problem, ProblemObjective};
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let objective = ProblemObjective::Minimize(LinearExpression::from(index.external(String::from("x"))));
  ///   let constraints = vec!(LinearRelation::new(LinearExpression::from(index.external(String::from("x"))), Relation::EQ, LinearExpression::from(5.0)));
  ///   let problem = Problem::new(objective, constraints);
  ///   let tableau = problem.augmented_simplex().unwrap();
  ///   let basic_feasible_solution = tableau.get_basic_feasible_solution();
  ///   assert!(approx_eq(5.0, *basic_feasible_solution.get(&index.external(String::from("x"))).unwrap()));
  /// }
  pub fn augmented_simplex(&mut self) -> Result<Tableau, String> {
    let mut slack_namer = Namer::new("s_");
    let (constraints, restrained_vars) = self.augmented_simplex_phase_one(&mut slack_namer);
    self.augmented_simplex_phase_two(&constraints, &restrained_vars)
  }

  // 1: convert to LEQs and NEQs. Err on NEQs.
  // 2: Note if this forms a `restrained` constraint for a specific variable: 0 <= var. Special case. Else, treat like normal slack: -var + s_n <= 0.
  // 3: Convert LEQs to equations, generating a slack variable. Add to restricted variables.
  // 4: Return: <equations, slack variables>
  fn augmented_simplex_phase_one(&mut self, slack_namer: &mut Namer) -> (LinkedList<LinearRelation>, HashSet<VarRef>) {
    let raw_constraints: LinkedList<LinearRelation> = self.subject_to.iter().map(|c|{c.clone()}).collect();
    let mut normalized_constraints = LinkedList::<LinearRelation>::new();
    let mut restricted_vars = HashSet::<VarRef>::new();

    for mut constraint in raw_constraints.into_iter() {
      match constraint.op {
        Relation::NEQ => panic!("I don't know what to do!"),
        Relation::EQ => {
          constraint.rhs.minus_this(&constraint.lhs);
          constraint.lhs = LinearExpression::new();
          normalized_constraints.push_back(constraint);
          continue;
        }
        Relation::LEQ => (),
        _ => self.convert_to_leq_and_eq(&mut constraint)
      }
      match self.convert_leq_to_eq(&mut constraint, slack_namer) {
        Some(var) => {
          restricted_vars.insert(var);
        }
        _ => ()
      }
      normalized_constraints.push_back(constraint);
    }

    (normalized_constraints, restricted_vars)
  }

  // for every constraint in c_s:
  //   for every unrestrained variable:
  //      pull constraint out of c_s.
  //      solve for unrestrained variable.
  //      replace every instance of variable with rhs of expresion in c_u, c_s, and f.
  //      Move resulting equation into c_u.
  fn augmented_simplex_phase_two(&self,
                                 constraints: &LinkedList<LinearRelation>,
                                 restrained_variables: &HashSet<VarRef>) -> Result<Tableau, String> {
    let mut c_e = constraints.clone();
    let mut c_s = Vec::<LinearRelation>::new();
    let mut c_u = HashMap::<VarRef, LinearExpression>::new();
    let mut new_f = self.objective.clone();

    for ref mut constraint in c_e.iter_mut() {
      let mut vars: HashSet<VarRef> = constraint.lhs.terms().keys().chain(constraint.rhs.terms().keys()).map(|s| s.clone()).collect();

      // first substitute any pending changes.
      for ref var in vars.iter() {
        for replacement in c_u.get(var.clone()) {
          constraint.substitute(var, replacement);
        }
      }

      vars = constraint.lhs.terms().keys().chain(constraint.rhs.terms().keys()).map(|s| s.clone()).collect();

      // then look for unrestrained variables.
      let unrestrained = vars.iter().find(|v| !restrained_variables.contains(v.clone()));
      match unrestrained {
        Some(v) => {
          let (op, e) = constraint.solve_for(v).unwrap();
          assert_eq!(op, Relation::EQ);
          assert!(!c_u.contains_key(v));

          for s in c_s.iter_mut() {
            s.substitute(v, &e);
          }

          for (_, u) in c_u.iter_mut() {
            u.substitute(v, &e);
          }

          new_f.substitute(v, &e);

          c_u.insert(v.clone(), e);
        }
        None => c_s.push(constraint.clone())
      }
    }

    let mut tableau = Tableau::from_index(self.index.clone());
    tableau.set_objective(new_f);
    for (var, e) in c_u.into_iter() {
      let _ = tableau.add_row(var, e, false);
    }

    for constraint in c_s.into_iter() {
      let p_vars: HashSet<VarRef> = tableau.parametric_vars().map(|s| s.clone()).collect();
      let o_var = constraint.lhs.terms().keys().chain(constraint.rhs.terms().keys()).find(|s| !p_vars.contains(s.clone()));
      if o_var.is_none() {
        return Err(format!("{} is not a viable simplex equation", constraint));
      }
      let var = o_var.unwrap();
      let (op, expr) = try!(constraint.solve_for(&var));
      assert_eq!(op, Relation::EQ);
      match tableau.add_row(var.clone(), expr, true) {
        Err(e) => return Err(e),
        Ok(_) => ()
      }
    }
    Ok(tableau)
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

#[cfg(test)]
mod test {
  use grammar::*;
  use expr::*;
  use problem::*;
  use var::VarIndex;

  #[test]
  fn const_equation() {
    let mut index = VarIndex::new();
    let x_ref = index.external(String::from("x"));
    let objective = ProblemObjective::Minimize(LinearExpression::from(x_ref.clone()));
    let constraints = vec!(LinearRelation::new(LinearExpression::from(x_ref.clone()), Relation::EQ, LinearExpression::from(5.0)));
    let mut problem = Problem::new(objective, constraints, index);
    let tableau = problem.augmented_simplex().unwrap();
    let basic_feasible_solution = tableau.get_basic_feasible_solution();
    assert!(approx_eq(5.0, *basic_feasible_solution.get(&x_ref).unwrap()));
  }

  #[test]
  fn one_slack() {
    let mut index = VarIndex::new();
    let x_ref = index.external(String::from("x"));
    let objective = ProblemObjective::Minimize(LinearExpression::from(x_ref.clone()));
    let constraints = vec!(LinearRelation::new(LinearExpression::from(x_ref.clone()), Relation::LEQ, LinearExpression::from(-5.0)));
    let mut problem = Problem::new(objective, constraints, index);
    let tableau = problem.augmented_simplex().unwrap();
    assert_eq!(1, tableau.get_parametric_vars().len());
    assert!(tableau.is_parametric(&tableau.get_var(&String::from("s_1")).expect("slack variable does not exist!")));
  }

  #[test]
  fn test_problem_parse() {
    let buf = r#"minimize(x_m+-1.0x_l);2x_m==x_l+x_r;x_l+10<=x_r;x_l>=-10;x_r<=100"#;
    let mut index = VarIndex::new();
    let mut problem = parse_Problem(&mut index, buf).unwrap();
    let mut tableau = problem.augmented_simplex().unwrap();
    tableau.simplex().unwrap();
  }
}
