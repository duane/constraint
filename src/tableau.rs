use expr::*;
use problem::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::collections::hash_map;

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

use var::Var;

pub const TABLEAU_OBJECTIVE_VARIABLE: &'static str = "z";

pub enum ProblemDirection {
  Maximize,
  Minimize
}

pub struct Tableau {
  direction: ProblemDirection,
  rows: HashMap<Var, LinearExpression>,
  columns: HashMap<Var, HashSet<Var>>,
  restricted: HashSet<Var>
}

impl Tableau {
  ///
  /// Create a new tableau to store linear problems.
  /// `new` creates a problem equivalent to `minimize(0)`.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::approx_eq;
  /// use constraint::tableau::Tableau;
  /// use std::collections::HashMap;
  ///
  /// fn main() {
  ///   let tableau = Tableau::new();
  ///   assert!(approx_eq(0.0, tableau.get_objective().get_expr().full_eval(&HashMap::new()).unwrap()));
  /// }
  /// ```
  pub fn new() -> Tableau {
    let mut t = Tableau{
      direction: ProblemDirection::Minimize,
      rows: HashMap::new(),
      columns: HashMap::new(),
      restricted: HashSet::new(),
    };
    t.rows.insert(Var::from(TABLEAU_OBJECTIVE_VARIABLE), LinearExpression::new());
    t
  }

  ///
  /// Access the objective of a tableau.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::approx_eq;
  /// use constraint::tableau::Tableau;
  /// use std::collections::HashMap;
  ///
  /// fn main() {
  ///   let tableau = Tableau::new();
  ///   assert!(approx_eq(0.0, tableau.get_objective().get_expr().full_eval(&HashMap::new()).unwrap()));
  /// }
  /// ```
  pub fn get_objective<'s>(&'s self) -> ProblemObjective {
    match self.direction {
      ProblemDirection::Maximize => ProblemObjective::Maximize(self.rows.get(&Var::from(TABLEAU_OBJECTIVE_VARIABLE)).unwrap().clone()),
      ProblemDirection::Minimize => ProblemObjective::Minimize(self.rows.get(&Var::from(TABLEAU_OBJECTIVE_VARIABLE)).unwrap().clone())
    }
  }

  ///
  /// Set the objective of a tableau.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::{approx_eq, LinearExpression};
  /// use constraint::tableau::Tableau;
  /// use constraint::problem::ProblemObjective;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   tableau.set_objective(ProblemObjective::Maximize(LinearExpression::from(Var::from("x"))));
  ///   assert!(approx_eq(1.0, tableau.get_objective().get_expr().get_coefficient(&Var::from("x"))));
  /// }
  /// ```
  pub fn set_objective(&mut self, f: ProblemObjective) {
    match f {
      ProblemObjective::Maximize(e) => {
        self.direction = ProblemDirection::Maximize;
        self.rows.insert(Var::from(TABLEAU_OBJECTIVE_VARIABLE), e);
      }
      ProblemObjective::Minimize(e) => {
        self.direction = ProblemDirection::Minimize;
        self.rows.insert(Var::from(TABLEAU_OBJECTIVE_VARIABLE), e);
      }
    }
  }

  ///
  /// Determine whether a given variable is basic (as opposed to parametric).
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::from("x"), LinearExpression::from(Var::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   assert!(tableau.is_basic(&Var::from("x")));
  ///   assert!(!tableau.is_basic(&Var::from("s1")));
  /// }
  /// ```
  pub fn is_basic(&self, var: &Var) -> bool {
    self.rows.contains_key(var)
  }

  ///
  /// Determine whether a given variable is basic (as opposed to parametric).
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::from("x"), LinearExpression::from(Var::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   assert!(!tableau.is_parametric(&Var::from("x")));
  ///   assert!(tableau.is_parametric(&Var::from("s1")));
  ///   assert!(!tableau.is_parametric(&Var::from("c")));
  /// }
  /// ```
  pub fn is_parametric(&self, var: &Var) -> bool {
    self.columns.contains_key(var)
  }

  ///
  /// Get a given basic variable
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::{approx_eq, LinearExpression};
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::from("x"), LinearExpression::from(Var::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   let basic = tableau.get_basic(&Var::from("x")).unwrap();
  ///   assert!(approx_eq(1.0, basic.get_coefficient(&Var::from("s1"))));
  ///   assert!(approx_eq(10.0, basic.get_constant()));
  /// }
  /// ```
  pub fn get_basic<'s>(&'s self, var: &Var) -> Option<&'s LinearExpression> {
    self.rows.get(var)
  }

  ///
  /// Get an iterator to the parametric vars.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::from("x"), LinearExpression::from(Var::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   let keys: Vec<Var> = tableau.parametric_vars_iter().map(|s|s.clone()).collect();
  ///   assert_eq!(1, keys.len());
  ///   assert_eq!(keys[0], Var::from("s1"));
  /// }
  /// ```
  pub fn parametric_vars_iter<'s>(&'s self) -> hash_map::Keys<'s, Var, HashSet<Var>> {
    self.columns.keys()
  }

  ///
  /// Get an the parametric vars in a set.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::from("x"), LinearExpression::from(Var::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   let keys: Vec<Var> = tableau.parametric_vars_iter().map(|s|s.clone()).collect();
  ///   assert_eq!(1, keys.len());
  ///   assert_eq!(keys[0], Var::from("s1"));
  /// }
  /// ```
  pub fn get_parametric_vars(&self) -> HashSet<Var> {
    self.parametric_vars_iter().map(|s|s.clone()).collect()
  }

  ///
  /// Get all the basic variable names that are defined in terms of the given parametric variable.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::from("x"), LinearExpression::from(Var::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   let vars = tableau.get_basic_vars_for_param(&Var::from("s1")).unwrap();
  ///   assert_eq!(1, vars.len());
  ///   for var in vars.iter() {
  ///     assert_eq!(var, &Var::from("x"));
  ///   }
  /// }
  /// ```
  pub fn get_basic_vars_for_param<'s>(&'s self, var: &Var) -> Option<&'s HashSet<Var>> {
    self.columns.get(var)
  }

  fn note_added_variable(&mut self, basic: &Var, parametric: &Var) -> Result<(), Var> {
    if self.columns.contains_key(parametric) {
      self.columns.get_mut(parametric).unwrap().insert(basic.clone());
    } else {
      let mut index = HashSet::new();
      index.insert(basic.clone());
      self.columns.insert(parametric.clone(), index);
    }
    Ok(())
  }

  fn note_removed_variable(&mut self, basic: &Var, parametric: &Var) -> Result<(), Var> {
    if !self.columns.contains_key(parametric) {
      return Ok(());
    }
    let needs_removal = {
      let mut index = self.columns.get_mut(parametric).unwrap();
      index.remove(basic);
      index.is_empty()
    };
    if needs_removal {
      self.columns.remove(parametric);
    }
    Ok(())
  }

  ///
  /// Add a row using a basic variable and a linear expression
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::{approx_eq, LinearExpression};
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::from("x"), LinearExpression::from(Var::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   let basic = tableau.get_basic(&Var::from("x")).unwrap();
  ///   assert!(approx_eq(1.0, basic.get_coefficient(&Var::from("s1"))));
  ///   assert!(approx_eq(10.0, basic.get_constant()));
  /// }
  /// ```
  pub fn add_row(&mut self, var: Var, e: LinearExpression, restricted: bool) -> Result<(), String> {
    assert!(!self.rows.contains_key(&var));
    let vars: HashSet<Var> = e.terms().keys().map(|s| s.clone()).collect();
    self.rows.insert(var.clone(), e);
    for parameter in vars.iter() {
      self.note_added_variable(&var, parameter).unwrap();
    }
    if restricted {
      self.restricted.insert(var.clone());
    }
    Ok(())
  }

  ///
  /// Remove a row assigned to a basic variable
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::{approx_eq, LinearExpression};
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::from("x"), LinearExpression::from(Var::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   {
  ///     let basic = tableau.get_basic(&Var::from("x")).unwrap();
  ///     assert!(approx_eq(1.0, basic.get_coefficient(&Var::from("s1"))));
  ///     assert!(approx_eq(10.0, basic.get_constant()));
  ///   }
  ///   assert!(tableau.remove_row(&Var::from("x")).is_ok());
  ///   assert!(tableau.get_basic(&Var::from("x")).is_none());
  /// }
  /// ```
  pub fn remove_row(&mut self, var: &Var) -> Result<(), String> {
    for e in self.rows.remove(var) {
      let vars: HashSet<&Var> = e.terms().keys().collect();
      for parameter in vars {
        let _ = self.note_removed_variable(var, parameter);
      }
    }
    self.restricted.remove(var);
    Ok(())
  }

  fn remove_parameter(&mut self, var: &Var) -> Result<(), String> {
    Err(String::from("Dunno what to do"))
  }

  ///
  /// Substitute a variable for an expression throughout a tableau.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::{approx_eq, LinearExpression};
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::from("x"), LinearExpression::from(Var::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   assert!(tableau.substitute(&Var::from("s1"), &LinearExpression::from(-8.0)).is_ok());
  ///   let basic = tableau.get_basic(&Var::from("x")).unwrap();
  ///   assert!(approx_eq(0.0, basic.get_coefficient(&Var::from("s1"))));
  ///   assert!(approx_eq(2.0, basic.get_constant()));
  /// }
  /// ```
  pub fn substitute(&mut self, var: &Var, e: &LinearExpression) -> Result<(), String> {
    for (_, row_expr) in self.rows.iter_mut() {
      row_expr.substitute(var, e);
    }
    Ok(())
  }

  ///
  /// Find a minimum viable solution by binding all basics to their constants and all parameters to 0.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::{approx_eq, LinearExpression};
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::from("x"),
  ///           LinearExpression::from(Var::from("s1")).
  ///             plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   assert!(tableau.add_row(Var::from("y"),
  ///           LinearExpression::from(Var::from("s1")).
  ///             plus(&LinearExpression::term(Var::from("s2"), 5.2)).
  ///             plus(&LinearExpression::from(-72.3)), false).is_ok());
  ///   let solution = tableau.get_basic_feasible_solution();
  ///   assert_eq!(solution.len(), 5); // two basics + two parameters + the objective variable.
  ///   assert!(approx_eq(10.0, *solution.get(&Var::from("x")).unwrap()));
  ///   assert!(approx_eq(-72.3, *solution.get(&Var::from("y")).unwrap()));
  /// }
  /// ```
  pub fn get_basic_feasible_solution(&self) -> HashMap<Var, Scalar> {
    let basic_bindings = self.rows.iter().map(|(k,v)|(k.clone(), v.get_constant()));
    let parametric_bindings = self.columns.keys().map(|k|(k.clone(), 0.0));
    basic_bindings.chain(parametric_bindings).collect()
  }

  ///
  /// Print the tableau to stdout prettily.
  ///
  pub fn print(&self) {
    let basic_vars: BTreeSet<Var> = self.rows.keys().map(|c|c.clone()).collect();
    let parametric_vars: BTreeSet<Var> = self.columns.keys().map(|c|c.clone()).collect();
    let header_row: Vec<Cell> = vec![Var::from(""), Var::from("c")].iter().chain(parametric_vars.iter()).map(|s|Cell::from(s)).collect();
    let mut table = Table::new();
    table.add_row(Row::new(header_row));

    for (basic, expr) in self.rows.iter() {
      let mut row: Vec<Cell> = vec![Cell::from(basic), Cell::from(&expr.get_constant().to_string())];
      for parametric_var in parametric_vars.iter() {
        let coef = expr.get_coefficient(parametric_var);
        row.push(Cell::from(&coef.to_string()));
      }
      table.add_row(Row::new(row));
    }
    println!("{}", table);
    println!("Basic feasible solution: {:?}", self.get_basic_feasible_solution());
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn it_works() {
    let tableau = super::Tableau::new();
  }
}
