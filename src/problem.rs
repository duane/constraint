use expr::*;
use std::fmt::{Display, Error, Formatter};
use std::collections::{HashMap, HashSet, LinkedList};
use state::*;
use tableau::*;

#[derive(Clone)]
pub enum ProblemObjective {
  Minimize(LinearExpression),
  Maximize(LinearExpression)
}

impl ProblemObjective {
  #[allow(dead_code)]
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

  #[allow(dead_code)]
  fn set_expr(&mut self, expr: LinearExpression) {
    match self {
      &mut ProblemObjective::Maximize(ref mut self_expr) => *self_expr = expr,
      &mut ProblemObjective::Minimize(ref mut self_expr) => *self_expr = expr
    }
  }

  fn substitute(&mut self, v: &String, e: &LinearExpression) {
    let mut f_e = self.get_mut_expr();
    f_e.substitute(v, e);
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
  fn get_restrained_var<'s, 'r>(&'s self, lr: &'r LinearRelation) -> Option<String> {
    let lhs_valid = lr.lhs.terms().is_empty() && approx_eq(0.0, lr.lhs.get_constant());
    let op_valid = lr.op == Relation::LEQ;
    let rhs_valid = lr.rhs.terms().len() == 1 && *lr.rhs.terms().values().next().unwrap() >= 0.0;
    if lhs_valid && op_valid && rhs_valid {
      Some(lr.rhs.terms().keys().next().unwrap().clone())
    } else {
      None
    }
  }

  // lhs <= rhs
  // lhs + s_n == rhs, 0 <= s_n
  // 0 == rhs - lhs - s_n, 0 <= s_n
  // 0 >= (rhs - lhs - + s_n), 0 <= s_n
  // 0 == (rhs - lhs), 0 <= s_n
  fn convert_leq_to_eq<'s, 'r>(&'s self, lr: &'r mut LinearRelation, namer: &mut Namer) -> Option<String> {
    if lr.op == Relation::LEQ {
      let slack = namer.vend();
      lr.lhs.plus_this(&LinearExpression::from(slack.as_ref()));
      lr.lhs.times_this(-1.0);
      lr.rhs.plus_this(&lr.lhs);
      lr.lhs = LinearExpression::new(); 

      lr.op = Relation::EQ;
      Some(slack)
    } else {
      None
    }
  }

  // for every constraint in c_s:
  //   for every unrestrained variable:
  //      pull constraint out of c_s.
  //      solve for unrestrained variable. 
  //      replace every instance of variable with rhs of expresion in c_u, c_s, and f.
  //      Move resulting equation into c_u.
  pub fn augmented_simplex_phase_two(&self,
                                     constraints: &LinkedList<LinearRelation>,
                                     restrained_variables: &HashSet<String>) -> Result<(), String> {
    let mut c_e = constraints.clone();
    let mut c_s = Vec::<LinearRelation>::new();
    let mut c_u = HashMap::<String, LinearExpression>::new();
    let mut new_f = self.objective.clone();

    for ref mut constraint in c_e.iter_mut() {
      let mut vars: HashSet<String> = constraint.lhs.terms().keys().chain(constraint.rhs.terms().keys()).map(|s| s.clone()).collect();

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

    println!("{}", new_f);
    println!("phase two: {}", new_f);
    println!("subject to:");
    for (v, e) in c_u.iter() {
      println!("{} == {}", v, e);
    }
    println!("------------------------------------------------------------");
    for r in c_s.iter() {
      println!("{}", r);
    }
    println!("with restrained variables:");
    let vars: Vec<&str> = restrained_variables.iter().map(|s|s.as_ref()).collect();
    println!("0 <= {}", vars.join(", "));

    let mut tableau = Tableau::new();
    tableau.set_objective(new_f);
    for (var, e) in c_u.into_iter() {
      let _ = tableau.add_row(var, e, false);
    }

    for constraint in c_s.into_iter() {
      let p_vars: HashSet<String> = tableau.parametric_vars().map(|s| s.clone()).collect();
      let o_var = constraint.lhs.terms().keys().chain(constraint.rhs.terms().keys()).find(|s| !p_vars.contains(s.clone()));
      if o_var.is_none() {
        return Err(format!("{} is not a viable simplex equation", constraint));
      }
      let var = o_var.unwrap();
      let (op, expr) = try!(constraint.solve_for(&var));
      assert_eq!(op, Relation::EQ);
      match tableau.add_row(var.clone(), expr, true) {
        e @ Err(_) => return e,
        Ok(_) => ()
      }
    }
    tableau.print();
    Ok(())
  }

  pub fn augmented_simplex(&self) {
    let mut slack_namer = Namer::init("s_");
    let (constraints, restrained_vars) = self.augmented_simplex_phase_one(&mut slack_namer);
    let _ = self.augmented_simplex_phase_two(&constraints, &restrained_vars);
  }

  // 1: convert to LEQs and NEQs. Err on NEQs.
  // 2: Note if this forms a `restrained` constraint for a specific variable: 0 <= var. Special case. Else, treat like normal slack: -var + s_n <= 0.
  // 3: Convert LEQs to equations, generating a slack variable. Add to restricted variables.
  // 4: Return: <equations, slack variables>
  pub fn augmented_simplex_phase_one(&self, slack_namer: &mut Namer) -> (LinkedList<LinearRelation>, HashSet<String>) {
    let raw_constraints: LinkedList<LinearRelation> = self.subject_to.iter().map(|c|{c.clone()}).collect();
    let mut normalized_constraints = LinkedList::<LinearRelation>::new();
    let mut restrained_vars = HashSet::<String>::new();

    for mut constraint in raw_constraints.into_iter() {
      match constraint.op {
        Relation::NEQ => panic!("I don't know what to do!"),
        Relation::EQ => {
          constraint.rhs.plus_this(&constraint.lhs);
          constraint.lhs = LinearExpression::new();
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
      match self.convert_leq_to_eq(&mut constraint, slack_namer) {
        Some(var) => {
          restrained_vars.insert(var);
        }
        _ => ()
      }
      normalized_constraints.push_back(constraint);
    }

    println!("phase one: {}", self.objective);
    println!("subject to:");
    for constraint in normalized_constraints.iter() {
      println!("{}", constraint);
    }
    println!("and restrained variables:");
    print!("0 <= ");
    let mut first = true;
    for variable in restrained_vars.iter() {
      if !first {
        print!(", ");
      } else {
        first = false;
      }
      print!("{}", variable);
    }
    println!("");

    (normalized_constraints, restrained_vars)
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

  #[test]
  fn test_problem_parse() {
    let buf = r#"minimize(x_m+-1.0x_l);2x_m==x_l+x_r;x_l+10<=x_r;x_l>=-10;x_r<=100"#;
    let problem = parse_Problem(buf).unwrap();
    println!("before problem: {}", problem);
    println!("");
    problem.augmented_simplex();
  }
}
