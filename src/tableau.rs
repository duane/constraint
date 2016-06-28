use expr::*;
use problem::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::collections::hash_map;

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

pub const TABLEAU_OBJECTIVE_VARIABLE: &'static str = "z";

pub enum ProblemDirection {
  Maximize,
  Minimize
}

pub struct Tableau {
  direction: ProblemDirection,
  rows: HashMap<String, LinearExpression>,
  columns: HashMap<String, HashSet<String>>,
  restricted: HashSet<String>
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
    t.rows.insert(String::from(TABLEAU_OBJECTIVE_VARIABLE), LinearExpression::new());
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
      ProblemDirection::Maximize => ProblemObjective::Maximize(self.rows.get(TABLEAU_OBJECTIVE_VARIABLE).unwrap().clone()),
      ProblemDirection::Minimize => ProblemObjective::Minimize(self.rows.get(TABLEAU_OBJECTIVE_VARIABLE).unwrap().clone())
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
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   tableau.set_objective(ProblemObjective::Maximize(LinearExpression::from(String::from("x"))));
  ///   assert!(approx_eq(1.0, tableau.get_objective().get_expr().get_coefficient(&String::from("x"))));
  /// }
  /// ```
  pub fn set_objective(&mut self, f: ProblemObjective) {
    match f {
      ProblemObjective::Maximize(e) => {
        self.direction = ProblemDirection::Maximize;
        self.rows.insert(String::from(TABLEAU_OBJECTIVE_VARIABLE), e);
      }
      ProblemObjective::Minimize(e) => {
        self.direction = ProblemDirection::Minimize;
        self.rows.insert(String::from(TABLEAU_OBJECTIVE_VARIABLE), e);
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
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(String::from("x"), LinearExpression::from(String::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   assert!(tableau.is_basic(&String::from("x")));
  ///   assert!(!tableau.is_basic(&String::from("s1")));
  /// }
  /// ```
  pub fn is_basic(&self, var: &String) -> bool {
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
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(String::from("x"), LinearExpression::from(String::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   assert!(!tableau.is_parametric(&String::from("x")));
  ///   assert!(tableau.is_parametric(&String::from("s1")));
  /// }
  /// ```
  pub fn is_parametric(&self, var: &String) -> bool {
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
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(String::from("x"), LinearExpression::from(String::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   let basic = tableau.get_basic(&String::from("x")).unwrap();
  ///   assert!(approx_eq(1.0, basic.get_coefficient(&String::from("s1"))));
  ///   assert!(approx_eq(10.0, basic.get_constant()));
  /// }
  /// ```
  pub fn get_basic<'s>(&'s self, var: &String) -> Option<&'s LinearExpression> {
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
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(String::from("x"), LinearExpression::from(String::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   let keys: Vec<String> = tableau.parametric_vars_iter().map(|s|s.clone()).collect();
  ///   assert_eq!(1, keys.len());
  ///   assert_eq!(keys[0], String::from("s1"));
  /// }
  /// ```
  pub fn parametric_vars_iter<'s>(&'s self) -> hash_map::Keys<'s, String, HashSet<String>> {
    self.columns.keys()
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
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(String::from("x"), LinearExpression::from(String::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   let vars = tableau.get_basic_vars_for_param(&String::from("s1")).unwrap();
  ///   assert_eq!(1, vars.len());
  ///   for var in vars.iter() {
  ///     assert_eq!(var, &String::from("x"));
  ///   }
  /// }
  /// ```
  pub fn get_basic_vars_for_param<'s>(&'s self, var: &String) -> Option<&'s HashSet<String>> {
    self.columns.get(var)
  }

  fn note_added_variable(&mut self, basic: &String, parametric: &String) -> Result<(), String> {
    if self.columns.contains_key(parametric) {
      self.columns.get_mut(parametric).unwrap().insert(basic.clone());
    } else {
      let mut index = HashSet::new();
      index.insert(basic.clone());
      self.columns.insert(parametric.clone(), index);
    }
    Ok(())
  }

  fn note_removed_variable(&mut self, basic: &String, parametric: &String) -> Result<(), String> {
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
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(String::from("x"), LinearExpression::from(String::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   let basic = tableau.get_basic(&String::from("x")).unwrap();
  ///   assert!(approx_eq(1.0, basic.get_coefficient(&String::from("s1"))));
  ///   assert!(approx_eq(10.0, basic.get_constant()));
  /// }
  /// ```
  pub fn add_row(&mut self, var: String, e: LinearExpression, restricted: bool) -> Result<(), String> {
    assert!(!self.rows.contains_key(&var));
    let vars: HashSet<String> = e.terms().keys().map(|s| s.clone()).collect();
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
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(String::from("x"), LinearExpression::from(String::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   {
  ///     let basic = tableau.get_basic(&String::from("x")).unwrap();
  ///     assert!(approx_eq(1.0, basic.get_coefficient(&String::from("s1"))));
  ///     assert!(approx_eq(10.0, basic.get_constant()));
  ///   }
  ///   assert!(tableau.remove_row(&String::from("x")).is_ok());
  ///   assert!(tableau.get_basic(&String::from("x")).is_none());
  /// }
  /// ```
  pub fn remove_row(&mut self, var: &String) -> Result<(), String> {
    for e in self.rows.remove(var) {
      let vars: HashSet<&String> = e.terms().keys().collect();
      for parameter in vars {
        let _ = self.note_removed_variable(var, parameter);
      }
    }
    self.restricted.remove(var);
    Ok(())
  }

  fn remove_parameter(&mut self, var: &String) -> Result<(), String> {
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
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(String::from("x"), LinearExpression::from(String::from("s1")).plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   assert!(tableau.substitute(&String::from("s1"), &LinearExpression::from(-8.0)).is_ok());
  ///   let basic = tableau.get_basic(&String::from("x")).unwrap();
  ///   assert!(approx_eq(0.0, basic.get_coefficient(&String::from("s1"))));
  ///   assert!(approx_eq(2.0, basic.get_constant()));
  /// }
  /// ```
  pub fn substitute(&mut self, var: &String, e: &LinearExpression) -> Result<(), String> {
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
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(String::from("x"),
  ///           LinearExpression::from(String::from("s1")).
  ///             plus(&LinearExpression::from(10.0)), false).is_ok());
  ///   assert!(tableau.add_row(String::from("y"),
  ///           LinearExpression::from(String::from("s1")).
  ///             plus(&LinearExpression::term(String::from("s2"), 5.2)).
  ///             plus(&LinearExpression::from(-72.3)), false).is_ok());
  ///   let solution = tableau.get_basic_feasible_solution();
  ///   assert_eq!(solution.len(), 5); // two basics + two parameters + the objective variable.
  ///   assert!(approx_eq(10.0, *solution.get(&String::from("x")).unwrap()));
  ///   assert!(approx_eq(-72.3, *solution.get(&String::from("y")).unwrap()));
  /// }
  /// ```
  pub fn get_basic_feasible_solution(&self) -> HashMap<String, Scalar> {
    let basic_bindings = self.rows.iter().map(|(k,v)|(k.clone(), v.get_constant()));
    let parametric_bindings = self.columns.keys().map(|k|(k.clone(), 0.0));
    basic_bindings.chain(parametric_bindings).collect()
  }

  ///
  /// Print the tableau to stdout prettily.
  ///
  pub fn print(&self) {
    let basic_vars: BTreeSet<String> = self.rows.keys().map(|c|c.clone()).collect();
    let parametric_vars: BTreeSet<String> = self.columns.keys().map(|c|c.clone()).collect();
    let header_row: Vec<Cell> = vec![String::from(""), String::from("c")].iter().chain(parametric_vars.iter()).map(|s|Cell::from(s)).collect();
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
