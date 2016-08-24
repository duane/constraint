use expr::*;
use std::fmt::{Debug, Display, Error, Formatter};
use std::collections::{HashMap, HashSet, LinkedList};
use state::*;
use tableau::*;
use var::{Var, VarIndex, VarRef};
use abs::{LinearExpression, RawLinearExpression};
use std::hash::Hash;

pub type RawProblemObjective = ProblemObjective<Var>;

impl RawProblemObjective {
  pub fn interned(&self, index: &mut VarIndex) -> InternedProblemObjective {
    match self {
      &ProblemObjective::Minimize(ref expr) => ProblemObjective::Minimize(expr.interned(index)),
      &ProblemObjective::Maximize(ref expr) => ProblemObjective::Maximize(expr.interned(index))
    }
  }
}

pub type InternedProblemObjective = ProblemObjective<VarRef>;

#[derive(Clone,Debug)]
pub enum ProblemObjective<V: Ord + Clone + Hash + Debug> {
  Minimize(LinearExpression<V>),
  Maximize(LinearExpression<V>)
}

impl<V> ProblemObjective<V> where V: Ord + Clone + Hash + Debug {

  ///
  /// Get the expression to either maximize or minimize.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::abs::InternedLinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::problem::ProblemObjective;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let objective = ProblemObjective::Minimize(InternedLinearExpression::from(index.external(String::from("x"))));
  ///   assert!(approx_eq(1.0, objective.get_expr().get_coefficient(&index.external(String::from("x")))));
  ///   assert!(approx_eq(0.0, objective.get_expr().get_constant()));
  /// }
  /// ```
  pub fn get_expr<'s>(&'s self) -> &'s LinearExpression<V> {
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
  /// use constraint::abs::InternedLinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::problem::ProblemObjective;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let mut objective = ProblemObjective::Minimize(InternedLinearExpression::new());
  ///   assert!(approx_eq(0.0, objective.get_expr().get_coefficient(&index.external(String::from("x")))));
  ///   objective.set_expr(InternedLinearExpression::from(index.external(String::from("x"))));
  ///   assert!(approx_eq(1.0, objective.get_expr().get_coefficient(&index.external(String::from("x")))));
  /// }
  /// ```
  pub fn set_expr(&mut self, expr: LinearExpression<V>) {
    match self {
      &mut ProblemObjective::Maximize(ref mut self_expr) => *self_expr = expr,
      &mut ProblemObjective::Minimize(ref mut self_expr) => *self_expr = expr
    }
  }

  fn substitute(&mut self, v: &V, e: &LinearExpression<V>) {
    let mut f_e = self.get_mut_expr();
    f_e.substitute(v, e);
  }

  fn get_mut_expr<'s>(&'s mut self) -> &'s mut LinearExpression<V> {
    match self {
      &mut ProblemObjective::Maximize(ref mut expr) => expr,
      &mut ProblemObjective::Minimize(ref mut expr) => expr
    }
  }

}

impl<V> Display for ProblemObjective<V> where V: Ord + Clone + Hash + Debug + Display {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let buf = match self {
      &ProblemObjective::Maximize(ref expr) => format!("maximize({})", expr),
      &ProblemObjective::Minimize(ref expr) => format!("minimize({})", expr)
    };
    fmt.write_str(buf.as_ref() as &str)
  }
}

pub struct Problem {
  objective: RawProblemObjective,
  subject_to: Vec<RawLinearRelation>
}

impl Problem {
  ///
  /// Create a new problem based on an objective and constraints.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::RawLinearRelation;
  /// use constraint::problem::{Problem, RawProblemObjective};
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let objective = RawProblemObjective::from_str("minimize(x)").unwrap();
  ///   let constraints = vec!(RawLinearRelation::from_str("x == 5").unwrap());
  ///   let _ = Problem::new(objective, constraints);
  /// }
  /// ```
  pub fn new(objective: RawProblemObjective, subject_to: Vec<RawLinearRelation>) -> Problem {
    Problem {
      objective: objective,
      subject_to: subject_to
    }
  }

  fn convert_to_leq_and_eq(&self, lr: &mut RawLinearRelation) {
    match lr.op {
      Relation::EQ | Relation::NEQ | Relation::LEQ | Relation::LT => (),
      Relation::GEQ | Relation::GT => {
        lr.reverse();
      }
    }
    if lr.op == Relation::LT {
      lr.rhs.plus_this(&RawLinearExpression::from(scalar::EPSILON * 2.0));
      lr.op = Relation::LEQ;
    }
  }

  // lhs <= rhs
  // lhs + s_n == rhs, 0 <= s_n
  // 0 == rhs - lhs - s_n, 0 <= s_n
  // 0 >= (rhs - lhs - + s_n), 0 <= s_n
  // 0 == (rhs - lhs), 0 <= s_n
  fn convert_leq_to_eq<'s, 'r>(&'s self, lr: &'r mut RawLinearRelation, namer: &mut Namer) -> Option<Var> {
    if lr.op == Relation::LEQ {
      let slack = Var::internal(namer.vend());
      lr.lhs.plus_this(&RawLinearExpression::from(slack.clone()));
      lr.lhs.times_this(-1.0);
      lr.rhs.plus_this(&lr.lhs);
      lr.lhs = RawLinearExpression::new();

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
  /// use constraint::expr::approx_eq;
  /// use constraint::tableau::Tableau;
  /// use std::str::FromStr;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let tableau = Tableau::from_str("minimize(x);x==5").unwrap();
  ///   let basic_feasible_solution = tableau.get_basic_feasible_solution();
  ///   assert!(approx_eq(5.0, *basic_feasible_solution.get(&Var::external(String::from("x"))).unwrap()));
  /// }
  pub fn augmented_simplex(&self) -> Result<Tableau, String> {
    let mut slack_namer = Namer::new("s_");
    let (constraints, restrained_vars) = self.augmented_simplex_phase_one(&mut slack_namer);
    self.augmented_simplex_phase_two(&constraints, &restrained_vars)
  }

  // 1: convert to LEQs and NEQs. Err on NEQs.
  // 2: Note if this forms a `restrained` constraint for a specific variable: 0 <= var. Special case. Else, treat like normal slack: -var + s_n <= 0.
  // 3: Convert LEQs to equations, generating a slack variable. Add to restricted variables.
  // 4: Return: <equations, slack variables>
  fn augmented_simplex_phase_one(&self, slack_namer: &mut Namer) -> (LinkedList<RawLinearRelation>, HashSet<Var>) {
    let raw_constraints: LinkedList<RawLinearRelation> = self.subject_to.iter().map(|c|{c.clone()}).collect();
    let mut normalized_constraints = LinkedList::<RawLinearRelation>::new();
    let mut restricted_vars = HashSet::<Var>::new();

    for mut constraint in raw_constraints.into_iter() {
      match constraint.op {
        Relation::NEQ => panic!("I don't know what to do!"),
        Relation::EQ => {
          constraint.rhs.minus_this(&constraint.lhs);
          constraint.lhs = RawLinearExpression::new();
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
                                 constraints: &LinkedList<RawLinearRelation>,
                                 restrained_variables: &HashSet<Var>) -> Result<Tableau, String> {
    let mut c_e = constraints.clone();
    let mut c_s = Vec::<RawLinearRelation>::new();
    let mut c_u = HashMap::<Var, RawLinearExpression>::new();
    let mut new_f = self.objective.clone();

    for ref mut constraint in c_e.iter_mut() {
      let mut vars: HashSet<Var> = constraint.lhs.terms().keys().chain(constraint.rhs.terms().keys()).map(|s| s.clone()).collect();

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

    let mut tableau = Tableau::new();
    tableau.set_objective(new_f);
    for (var, e) in c_u.into_iter() {
      let _ = tableau.add_row(var, e, false);
    }

    for constraint in c_s.into_iter() {
      let p_vars: HashSet<Var> = tableau.parametric_vars().map(|s| (**s).clone()).collect();
      let o_var = constraint.lhs.terms().keys().chain(constraint.rhs.terms().keys()).find(|s| !p_vars.contains(s));
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

use std::str::FromStr;
use grammar::*;

impl FromStr for RawProblemObjective {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match parse_ProblemObjective(s) {
      Ok(result) => Ok(result),
      Err(e) => Err(format!("{:?}", e))
    }
  }
}

impl FromStr for Problem {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match parse_Problem(s) {
      Ok(result) => Ok(result),
      Err(e) => Err(format!("{:?}", e))
    }
  }
}

#[cfg(test)]
mod test {
  use grammar::*;
  use expr::*;
  use problem::*;
  use var::Var;
  use abs::RawLinearExpression;

  #[test]
  fn const_equation() {
    let x_ref = Var::external(String::from("x"));
    let objective = ProblemObjective::Minimize(RawLinearExpression::from(x_ref.clone()));
    let constraints = vec!(LinearRelation::new(RawLinearExpression::from(x_ref.clone()), Relation::EQ, RawLinearExpression::from(5.0)));
    let problem = Problem::new(objective, constraints);
    let tableau = problem.augmented_simplex().unwrap();
    let basic_feasible_solution = tableau.get_basic_feasible_solution();
    assert!(approx_eq(5.0, *basic_feasible_solution.get(&x_ref).unwrap()));
  }

  #[test]
  fn one_slack() {
    let x_ref = Var::external(String::from("x"));
    let objective = ProblemObjective::Minimize(RawLinearExpression::from(x_ref.clone()));
    let constraints = vec!(LinearRelation::new(RawLinearExpression::from(x_ref.clone()), Relation::LEQ, RawLinearExpression::from(-5.0)));
    let problem = Problem::new(objective, constraints);
    let tableau = problem.augmented_simplex().unwrap();
    assert_eq!(1, tableau.get_parametric_vars().len());
    assert!(tableau.is_parametric(&tableau.get_var(&String::from("s_1")).expect("slack variable does not exist!")));
  }

  #[test]
  fn test_problem_parse() {
    let buf = r#"minimize(x_m+-1.0x_l);2x_m==x_l+x_r;x_l+10<=x_r;x_l>=-10;x_r<=100"#;
    let problem = parse_Problem(buf).unwrap();
    let mut tableau = problem.augmented_simplex().unwrap();
    tableau.simplex().unwrap();
  }
}
