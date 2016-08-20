use std::cmp::{Ordering, PartialEq};
use std::fmt::{Display, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use expr::Scalar;

pub type VarRef = Rc<Var>;

#[derive(Debug,Clone)]
pub enum Var {
  Internal(String),
  External(String, Cell<Scalar>)
}

impl Var {
  ///
  /// Returns if the variable may be used as a pivot
  ///
  /// # Examples
  ///
  /// extern crate constraint;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   assert!(Var::internal(String::from("s_1")));
  /// }
  ///
  pub fn is_pivotable(&self) -> bool {
    match self {
      &Var::Internal(_) => true,
      &Var::External(_, _) => false
    }
  }

  ///
  /// Returns the variable's name.
  ///
  /// # Examples
  ///
  /// extern crate constraint;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   assert_eq!(String::from("s_1"), Var::internal(String::from("s_1")));
  /// }
  ///
  pub fn name<'s>(&'s self) -> &'s String {
    match self {
      &Var::Internal(ref name) => name,
      &Var::External(ref name, _) => name
    }
  }

  pub fn internal(name: String) -> Var {
    Var::Internal(name)
  }

  pub fn external(name: String) -> Var {
    Var::External(name, Cell::new(0.0))
  }
}

impl Hash for Var {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.name().hash(state);
  }
}

impl Ord for Var {
  fn cmp(&self, other: &Self) -> Ordering {
    self.name().cmp(other.name())
  }
}

impl PartialEq for Var {
  fn eq(&self, other: &Self) -> bool {
    self.name().eq(other.name())
  }
}

impl PartialOrd for Var {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.name().partial_cmp(other.name())
  }
}

impl Eq for Var { }

impl Display for Var {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    fmt.write_str(self.name())
  }
}

#[derive(Debug,Clone,Default)]
pub struct VarIndex {
  variables: HashMap<String, Weak<Var>>
}

impl VarIndex {
  ///
  /// Initialize a new VarIndex
  ///
  /// # Examples
  ///
  /// extern crate constraint;
  /// use constraint::var::Var;
  ///
  /// fn main() {
  ///   let _ = VarIndex::new();
  /// }
  ///
  pub fn new() -> VarIndex {
    VarIndex{
      variables: HashMap::new()
    }
  }

  ///
  /// Inserting a var into a index
  ///
  /// # Examples
  ///
  /// extern crate constraint;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   index.insert(Var::external("x"))
  /// }
  ///
  pub fn insert<'s>(&'s mut self, var: Var) -> VarRef {
    let name = {var.name().clone()};
    let var_ref = Rc::new(var);
    self.variables.entry(name).or_insert_with(||Rc::downgrade(&var_ref));
    var_ref
  }

  ///
  /// Checking whether a variable is already bound.
  ///
  /// # Examples
  ///
  /// extern crate constraint;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   index.insert(Var::external("x"));
  ///   assert!(index.contains(&String::from("x")));
  /// }
  ///
  pub fn contains<'s>(&'s self, by_name: &String) -> bool {
    self.variables.get(by_name).map(|c|c.upgrade().is_some()).unwrap_or(false)
  }

  ///
  /// Getting a variable.
  ///
  /// # Examples
  ///
  /// extern crate constraint;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   index.insert(Var::external("x"));
  ///   assert_eq!(index.get(&String::from("x")).unwrap().name(), &String::from("x"));
  /// }
  ///
  pub fn get<'s>(&'s self, by_name: &String) -> Option<VarRef> {
    self.variables.get(by_name).and_then(|c|c.upgrade())
  }

  ///
  /// Generate and insert a new internal variable.
  ///
  /// # Examples
  ///
  /// extern crate constraint;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let var = index.internal(String::from("x"));
  ///   assert_eq!(var.name(), &String::from("x"));
  /// }
  ///
  pub fn internal<'s>(&'s mut self, name: String) -> VarRef {
    self.insert(Var::internal(name))
  }

  ///
  /// Generate and insert a new external variable.
  ///
  /// # Examples
  ///
  /// extern crate constraint;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let var = index.external(String::from("x"));
  ///   assert_eq!(var.name(), &String::from("x"));
  /// }
  ///
  pub fn external<'s>(&'s mut self, name: String) -> VarRef {
    self.insert(Var::external(name))
  }
}
