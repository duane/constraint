use std::cmp::{Ordering, PartialEq};
use std::fmt::{Display, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::cell::Cell;
//use std::rc::Rc;
use expr::Scalar;

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