use expr::*;
use problem::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::collections::hash_map;
use std::str::FromStr;
use grammar::parse_Problem;

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use abs::{InternedLinearExpression, RawLinearExpression};

use state::Namer;
use var::{Var, VarIndex, VarRef};

pub const TABLEAU_OBJECTIVE_VARIABLE: &'static str = "OMGIAMANOBJECTIVE";

#[derive(Debug)]
pub enum ProblemDirection {
  Maximize,
  Minimize
}

#[derive(Debug)]
pub struct Tableau {
  direction: ProblemDirection,
  rows: HashMap<VarRef, InternedLinearExpression>,
  columns: HashMap<VarRef, HashSet<VarRef>>,
  restricted: HashSet<VarRef>,
  pub index: VarIndex,
  slack_namer: Namer
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
    Tableau::from_index(VarIndex::new())
  }

  pub fn from_index(index: VarIndex) -> Tableau {
    let mut i = index;
    let objective_ref = {i.external(String::from(TABLEAU_OBJECTIVE_VARIABLE))};
    let mut t = Tableau {
      direction: ProblemDirection::Minimize,
      rows: HashMap::new(),
      columns: HashMap::new(),
      restricted: HashSet::new(),
      index: i,
      slack_namer: Namer::new("s_")
    };
    t.rows.insert(objective_ref, InternedLinearExpression::new());
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
  pub fn get_objective<'s>(&'s self) -> InternedProblemObjective {
    let objective_expr = self.rows.get(&self.index.get(&String::from(TABLEAU_OBJECTIVE_VARIABLE)).expect("tableau objective variable not in index")).unwrap().clone();
    match self.direction {
      ProblemDirection::Maximize => ProblemObjective::Maximize(objective_expr),
      ProblemDirection::Minimize => ProblemObjective::Minimize(objective_expr)
    }
  }

  fn set_interned_objective(&mut self, f: InternedProblemObjective) {
    let objective_var_ref = self.index.get(&String::from(TABLEAU_OBJECTIVE_VARIABLE)).expect("Could not find objective variable in index");
    match f {
      ProblemObjective::Maximize(e) => {
        self.direction = ProblemDirection::Maximize;
        self.rows.insert(objective_var_ref, e);
      }
      ProblemObjective::Minimize(e) => {
        self.direction = ProblemDirection::Minimize;
        self.rows.insert(objective_var_ref, e);
      }
    }
  }

  ///
  /// Set the objective of a tableau.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::approx_eq;
  /// use constraint::tableau::Tableau;
  /// use constraint::problem::RawProblemObjective;
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   tableau.set_objective(RawProblemObjective::from_str("minimize(x)").unwrap());
  ///   let x_ref = {tableau.index.external(String::from("x"))};
  ///   assert!(approx_eq(1.0, tableau.get_objective().get_expr().get_coefficient(&x_ref)));
  /// }
  /// ```
  pub fn set_objective(&mut self, f: RawProblemObjective) {
    let interned = {f.interned(&mut self.index)};
    self.set_interned_objective(interned);
  }

  ///
  /// Determine whether a given variable is basic (as opposed to parametric).
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::abs::RawLinearExpression;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::external(String::from("x")), RawLinearExpression::from_str("s1+10").unwrap(), false).is_ok());
  ///   let x_ref = {tableau.index.external(String::from("x"))};
  ///   assert!(tableau.is_basic(&x_ref));
  ///   let s1_ref = {tableau.index.external(String::from("s1"))};
  ///   assert!(!tableau.is_basic(&s1_ref));
  /// }
  /// ```
  pub fn is_basic(&self, var: &VarRef) -> bool {
    self.rows.contains_key(var)
  }

  ///
  /// Determine whether a given variable is basic (as opposed to parametric).
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::abs::RawLinearExpression;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::external(String::from("x")), RawLinearExpression::from_str("s1+10").unwrap(), false).is_ok());
  ///   let x_ref = {tableau.index.external(String::from("x"))};
  ///   let s1_ref = {tableau.index.external(String::from("s1"))};
  ///   let c_ref = {tableau.index.external(String::from("c"))};
  ///   assert!(!tableau.is_parametric(&x_ref));
  ///   assert!(tableau.is_parametric(&s1_ref));
  ///   assert!(!tableau.is_parametric(&c_ref));
  /// }
  /// ```
  pub fn is_parametric(&self, var: &VarRef) -> bool {
    self.columns.contains_key(var)
  }

  ///
  /// Get a given basic variable
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::abs::RawLinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::external(String::from("x")), RawLinearExpression::from_str("s1+10").unwrap(), false).is_ok());
  ///   let x_ref = {tableau.index.external(String::from("x"))};
  ///   let s1_ref = {tableau.index.external(String::from("s1"))};
  ///   let basic = tableau.get_basic(&x_ref).unwrap();
  ///   assert!(approx_eq(1.0, basic.get_coefficient(&s1_ref)));
  ///   assert!(approx_eq(10.0, basic.get_constant()));
  /// }
  /// ```
  pub fn get_basic<'s>(&'s self, var: &VarRef) -> Option<&'s InternedLinearExpression> {
    self.rows.get(var)
  }

  ///
  /// Get an iterator to the parametric vars.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::abs::RawLinearExpression;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::external(String::from("x")), RawLinearExpression::from_str("s1+10").unwrap(), false).is_ok());
  ///   let s1_ref = {tableau.index.external(String::from("s1"))};
  ///   let mut keys = tableau.parametric_vars();
  ///   assert_eq!(1, keys.len());
  ///   assert_eq!(keys.next().unwrap(), &s1_ref);
  /// }
  /// ```
  pub fn parametric_vars<'s>(&'s self) -> hash_map::Keys<'s, VarRef, HashSet<VarRef>> {
    self.columns.keys()
  }

  ///
  /// Get an iterator to the basic vars.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::abs::RawLinearExpression;
  /// use constraint::tableau::{TABLEAU_OBJECTIVE_VARIABLE, Tableau};
  /// use constraint::var::{Var, VarRef};
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::external(String::from("x")), RawLinearExpression::from_str("s1+10").unwrap(), false).is_ok());
  ///   let keys: Vec<VarRef> = tableau.basic_vars().map(|s|s.clone()).collect();
  ///   assert_eq!(2, keys.len());
  ///   let x_ref = {tableau.index.external(String::from("x"))};
  ///   let objective_ref = {tableau.index.external(String::from(TABLEAU_OBJECTIVE_VARIABLE))};
  ///   assert!(keys.contains(&x_ref));
  ///   assert!(keys.contains(&objective_ref));
  /// }
  /// ```
  pub fn basic_vars<'s>(&'s self) -> hash_map::Keys<'s, VarRef, InternedLinearExpression> {
    self.rows.keys()
  }

  ///
  /// Get an the parametric vars in a set.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::abs::RawLinearExpression;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::external(String::from("x")), RawLinearExpression::from_str("s1+10").unwrap(), false).is_ok());
  ///   let s1_ref = {tableau.index.external(String::from("s1"))};
  ///   let keys = tableau.get_parametric_vars();
  ///   assert_eq!(1, keys.len());
  ///   assert!(keys.contains(&s1_ref));
  /// }
  /// ```
  pub fn get_parametric_vars(&self) -> HashSet<VarRef> {
    self.parametric_vars().map(|s|s.clone()).collect()
  }

  ///
  /// Get all the basic variable names that are defined in terms of the given parametric variable.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::abs::RawLinearExpression;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::external(String::from("x")), RawLinearExpression::from_str("s1+10").unwrap(), false).is_ok());
  ///   let s1_ref = {tableau.index.external(String::from("s1"))};
  ///   let x_ref = {tableau.index.external(String::from("x"))};
  ///   let vars = tableau.get_basic_vars_for_param(&s1_ref);
  ///   assert_eq!(1, vars.len());
  ///   assert!(vars.contains(&x_ref));
  /// }
  /// ```
  pub fn get_basic_vars_for_param<'s>(&'s self, var: &VarRef) -> HashSet<VarRef> {
    self.columns.get(var).map(|c|c.clone()).unwrap_or(HashSet::new())
  }

  fn note_added_variable(&mut self, basic: &VarRef, parametric: &VarRef) -> Result<(), VarRef> {
    if self.columns.contains_key(parametric) {
      self.columns.get_mut(parametric).unwrap().insert(basic.clone());
    } else {
      let mut index = HashSet::new();
      index.insert(basic.clone());
      self.columns.insert(parametric.clone(), index);
    }
    Ok(())
  }

  fn note_removed_variable(&mut self, basic: &VarRef, parametric: &VarRef) -> Result<(), VarRef> {
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

  fn add_interned_row(&mut self, var: VarRef, e: InternedLinearExpression, restricted: bool) -> Result<(), String> {
    assert!(!self.rows.contains_key(&var));
    let vars: HashSet<VarRef> = e.terms().keys().map(|s| s.clone()).collect();
    var.set_value(e.get_constant());
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
  /// Add a row using a basic variable and a linear expression
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::abs::RawLinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let x = Var::external(String::from("x"));
  ///   let s1 = Var::internal(String::from("s1"));
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(x.clone(), RawLinearExpression::from_str("s1+10").unwrap(), false).is_ok());
  ///   let basic = tableau.get_basic(&tableau.index.get(&String::from("x")).unwrap()).unwrap();
  ///   assert!(approx_eq(1.0, basic.get_coefficient(&tableau.index.get(&String::from("s1")).unwrap())));
  ///   assert!(approx_eq(10.0, basic.get_constant()));
  /// }
  /// ```
  pub fn add_row(&mut self, var: Var, e: RawLinearExpression, restricted: bool) -> Result<(), String> {
    let interned_var = {self.index.insert(var)};
    let interned_terms = {e.interned(&mut self.index)};
    self.add_interned_row(interned_var, interned_terms, restricted)
  }

  ///
  /// Vends a new external variable reference. Does not add to the tableau; just the index. The returned reference is the only strong reference.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::var::Var;
  /// use constraint::tableau::Tableau;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert_eq!(*tableau.external(String::from("x")), Var::external(String::from("x")));
  /// }
  /// ```
  pub fn external(&mut self, name: String) -> VarRef {
    match self.index.get(&name) {
      Some(vr) => vr,
      None => self.index.external(name)
    }
  }

  ///
  /// Vends a new external variable reference. Does not add to the tableau; just the index. The returned reference is the only strong reference.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::var::Var;
  /// use constraint::tableau::Tableau;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert_eq!(*tableau.slack(), Var::internal(String::from("s_1")));
  /// }
  /// ```
  pub fn slack(&mut self) -> VarRef {
    self.index.internal(self.slack_namer.vend())
  }

  pub fn get_var(&self, name: &String) -> Option<VarRef> {
    self.index.get(name)
  }

  ///
  /// Remove a row assigned to a basic variable
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::abs::RawLinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::external(String::from("x")), RawLinearExpression::from_str("s1+10").unwrap(), false).is_ok());
  ///   let x_ref = {tableau.index.external(String::from("x"))};
  ///   let s1_ref = {tableau.index.external(String::from("s1"))};
  ///   {
  ///     let basic = tableau.get_basic(&x_ref).unwrap();
  ///     assert!(approx_eq(1.0, basic.get_coefficient(&s1_ref)));
  ///     assert!(approx_eq(10.0, basic.get_constant()));
  ///   }
  ///   assert!(tableau.remove_row(&x_ref).is_ok());
  ///   assert!(tableau.get_basic(&x_ref).is_none());
  /// }
  /// ```
  pub fn remove_row(&mut self, var: &VarRef) -> Result<InternedLinearExpression, String> {
    var.set_value(0.0);
    let removed = match self.rows.remove(var) {
      Some(expr) => expr,
      None => return Err(String::from("Cannot remove variable from tableau that does not exist"))
    };
    {
      let vars: HashSet<&VarRef> = removed.terms().keys().collect();
      for parameter in vars {
        let _ = self.note_removed_variable(var, parameter);
      }
    }
    self.restricted.remove(var);
    Ok(removed)
  }

  pub fn remove_column(&mut self, var: &VarRef) -> Result<(), String> {
    match self.columns.get(var) {
      Some(basic_vars) => {
        if !basic_vars.iter().all(|e| approx_eq(0.0, self.get_basic(e).unwrap().get_coefficient(var))) {
          return Err(String::from("Parametric var still has weight"));
        }
      }
      None => return Err(String::from("column does not exist"))
    };
    let _ = self.columns.remove(var);
    Ok(())
  }

  ///
  /// Substitute a variable for an expression throughout a tableau.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::abs::{InternedLinearExpression, RawLinearExpression};
  /// use constraint::expr::approx_eq;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::external(String::from("x")), RawLinearExpression::from_str("s1+10").unwrap(), false).is_ok());
  ///   let s1_ref = {tableau.index.external(String::from("s1"))};
  ///   let x_ref = {tableau.index.external(String::from("x"))};
  ///   assert!(tableau.substitute(&s1_ref, &InternedLinearExpression::from(-8.0)).is_ok());
  ///   let basic = tableau.get_basic(&x_ref).unwrap();
  ///   assert!(approx_eq(0.0, basic.get_coefficient(&s1_ref)));
  ///   assert!(approx_eq(2.0, basic.get_constant()));
  /// }
  /// ```
  pub fn substitute(&mut self, var: &VarRef, e: &InternedLinearExpression) -> Result<(), String> {
    for (this_var, row_expr) in self.rows.iter_mut() {
      row_expr.substitute(var, e);
      this_var.set_value(row_expr.get_constant());
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
  /// use constraint::abs::RawLinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::tableau::Tableau;
  /// use constraint::var::Var;
  /// use std::str::FromStr;
  ///
  /// fn main() {
  ///   let mut tableau = Tableau::new();
  ///   assert!(tableau.add_row(Var::external(String::from("x")), RawLinearExpression::from_str("s1+10").unwrap(), false).is_ok());
  ///   assert!(tableau.add_row(Var::external(String::from("y")), RawLinearExpression::from_str("s1+5.2s2+-72.3").unwrap(), false).is_ok());
  ///   let solution = tableau.get_basic_feasible_solution();
  ///   let x_ref = {tableau.index.external(String::from("x"))};
  ///   let y_ref = {tableau.index.external(String::from("y"))};
  ///   assert_eq!(solution.len(), 5); // two basics + two parameters + the objective variable.
  ///   assert!(approx_eq(10.0, *solution.get(&x_ref).unwrap()));
  ///   assert!(approx_eq(-72.3, *solution.get(&y_ref).unwrap()));
  /// }
  /// ```
  pub fn get_basic_feasible_solution(&self) -> HashMap<VarRef, Scalar> {
    let basic_bindings = self.rows.iter().map(|(k,v)|(k.clone(), v.get_constant()));
    let parametric_bindings = self.columns.keys().map(|k|(k.clone(), 0.0));
    basic_bindings.chain(parametric_bindings).collect()
  }

  pub fn pivot(&mut self, entry_var: &VarRef, exit_var: &VarRef) -> Result<(), String> {
    let expr = match self.remove_row(exit_var) {
      Ok(expr) => expr,
      Err(s) => return Err(s)
    };
    let relation = InternedLinearRelation::new(InternedLinearExpression::from(exit_var.clone()), Relation::EQ, expr);
    let solved_for_entry = match relation.solve_for(entry_var) {
      Ok((_, expr)) => expr,
      Err(s) => return Err(s)
    };
    self.substitute(entry_var, &solved_for_entry).unwrap();
    self.add_interned_row(entry_var.clone(), solved_for_entry, false).unwrap();
    self.remove_column(entry_var).unwrap();
    Ok(())
  }

  pub fn simplex(&mut self) -> Result<(), String> {
    loop {
      let initial: Option<(VarRef, Scalar)> = None;
      let o_entry_var = self.get_objective().
        get_expr().
        terms().
        iter().
        filter(|&(k, _)| k.is_pivotable()).
        fold(initial, |found, (k, v)| {
        if *v < found.clone().map(|t| t.1).unwrap_or(0.0) {
          Some((k.clone(), *v))
        } else {
          found
        }
      });
      let (entry_var, _) = match o_entry_var {
        Some(tuple) => tuple,
        None => return Ok(())
      };
      assert!(entry_var.is_pivotable());
      let exit_candidates_exprs: HashMap<VarRef, InternedLinearExpression> = self.get_basic_vars_for_param(&entry_var).
        iter().
        filter(|v| v.is_pivotable()).
        map(|v| (v.clone(), self.get_basic(v).unwrap().clone())).
        collect();
      if !exit_candidates_exprs.values().any(|e| e.get_coefficient(&entry_var) < 0.0) {
        return Err(String::from("Unbounded"));
      }
      let initial_state: Option<(VarRef, Scalar)> = None;
      let o_exit_var = exit_candidates_exprs.iter().fold(initial_state, |state, (k, e)| {
        let a_ij = e.get_coefficient(&entry_var);
        let ratio = -e.get_constant() / a_ij;
        if state.is_none() || ratio < state.clone().unwrap().1 {
          Some((k.clone(), ratio))
        } else {
          state
        }
      });
      let exit_var = match o_exit_var {
        Some((v, _)) => v,
        None => return Ok(())
      };
      self.pivot(&entry_var, &exit_var).unwrap();
    }
  }

  ///
  /// Print the tableau to stdout prettily.
  ///
  pub fn print(&self) {
    let parametric_vars: BTreeSet<VarRef> = self.columns.keys().map(|c|c.clone()).collect();
    let header_row: Vec<Cell> = vec![String::new(), String::from("c")].into_iter().chain(parametric_vars.iter().map(|v|(*v).name().clone())).map(|s|Cell::from(&s)).collect();
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

impl FromStr for Tableau {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match parse_Problem(s) {
      Ok(result) => result.augmented_simplex(),
      Err(e) => Err(format!("{:?}", e))
    }
  }
}

#[cfg(test)]
mod test {
  extern crate itertools;
  use expr::approx_eq;
  use std::io::BufReader;
  use std::io::prelude::*;
  use std::fs::File;
  use self::itertools::Itertools;
  use super::*;
  fn from_file(file: &str) -> Tableau {
    let f = BufReader::new(File::open(file).unwrap());
    let buf = f.lines().map(|r|r.unwrap_or(String::new())).join(";");
    parse_Problem(buf.as_ref()).unwrap().augmented_simplex().unwrap()
  }

  use grammar::*;
  #[test]
  fn cassowary_tochi() {
    let mut  tableau = from_file("test/problems/cassowary-tochi");
    tableau.simplex().unwrap();
    let solution = tableau.get_basic_feasible_solution();
    let x_l_ref = tableau.index.external(String::from("x_l"));
    let x_m_ref = tableau.index.external(String::from("x_m"));
    let x_r_ref = tableau.index.external(String::from("x_r"));
    assert!(approx_eq(-10.0, x_l_ref.get_value()));
    assert!(approx_eq(-5.0, x_m_ref.get_value()));
    assert!(approx_eq(0.0, x_r_ref.get_value()));
    assert!(approx_eq(-10.0, *solution.get(&x_l_ref).unwrap()));
    assert!(approx_eq(-5.0, *solution.get(&x_m_ref).unwrap()));
    assert!(approx_eq(0.0, *solution.get(&x_r_ref).unwrap()));
    assert!(approx_eq(5.0, *solution.get(&tableau.index.external(String::from(super::TABLEAU_OBJECTIVE_VARIABLE))).unwrap()));
  }

  #[test]
  fn equate() {
    let mut tableau = from_file("test/problems/equate");
    tableau.simplex().unwrap();
    let solution = tableau.get_basic_feasible_solution();
    let x_ref = tableau.index.external(String::from("x"));
    assert!(approx_eq(4.0, x_ref.get_value()));
    assert!(approx_eq(4.0, *solution.get(&x_ref).unwrap()));
    assert!(approx_eq(4.0, *solution.get(&tableau.index.external(String::from(super::TABLEAU_OBJECTIVE_VARIABLE))).unwrap()));
  }

  #[test]
  fn one_slack() {
    let mut tableau = from_file("test/problems/one_slack");
    tableau.simplex().unwrap();
    let solution = tableau.get_basic_feasible_solution();
    let x_ref = tableau.index.external(String::from("x"));
    assert!(approx_eq(0.0, x_ref.get_value()));
    assert!(approx_eq(0.0, *solution.get(&x_ref).unwrap()));
    assert!(approx_eq(0.0, *solution.get(&tableau.index.external(String::from(super::TABLEAU_OBJECTIVE_VARIABLE))).unwrap()));
  }

  #[test]
  fn two_slack() {
    let mut tableau = from_file("test/problems/two_slack");
    tableau.simplex().unwrap();
    let solution = tableau.get_basic_feasible_solution();
    let x_ref = tableau.index.external(String::from("x"));
    let y_ref = tableau.index.external(String::from("y"));
    assert!(approx_eq(0.0, x_ref.get_value()));
    assert!(approx_eq(0.0, y_ref.get_value()));
    assert!(approx_eq(0.0, *solution.get(&x_ref).unwrap()));
    assert!(approx_eq(0.0, *solution.get(&y_ref).unwrap()));
    assert!(approx_eq(0.0, *solution.get(&tableau.index.external(String::from(super::TABLEAU_OBJECTIVE_VARIABLE))).unwrap()));
  }

  #[test]
  fn one_pivot() {
    let mut tableau = from_file("test/problems/one_pivot");
    tableau.simplex().unwrap();
    let solution = tableau.get_basic_feasible_solution();
    let x_ref = tableau.index.external(String::from("x"));
    let o_ref = tableau.index.external(String::from("o"));
    let y_ref = tableau.index.external(String::from("y"));
    assert!(approx_eq(-80.0, x_ref.get_value()));
    assert!(approx_eq(-80.0, o_ref.get_value()));
    assert!(approx_eq(0.0, y_ref.get_value()));
    assert!(approx_eq(-80.0, *solution.get(&x_ref).unwrap()));
    assert!(approx_eq(-80.0, *solution.get(&o_ref).unwrap()));
    assert!(approx_eq(0.0, *solution.get(&y_ref).unwrap()));
    assert!(approx_eq(-40.0, *solution.get(&tableau.index.external(String::from(super::TABLEAU_OBJECTIVE_VARIABLE))).unwrap()));
  }

  #[test]
  fn unbounded() {
    let mut tableau = from_file("test/problems/unbounded");
    assert_eq!(tableau.simplex().unwrap_err(), String::from("Unbounded"));
  }
}
