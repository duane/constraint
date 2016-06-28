use std::cmp::{Ordering, PartialEq};
use std::fmt::{Display, Error, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug,Clone)]
pub struct Var(pub String, pub bool);

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
  ///   assert!(Var(String::from("s_1"), true));
  /// }
  ///
  pub fn is_pivotable(&self) -> bool {
    match self {
      &Var(_, pivotable) => pivotable
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
  ///   assert_eq!(String::from("s_1"), Var(String::from("s_1"), true));
  /// }
  ///
  pub fn name<'s>(&'s self) -> &'s String {
    match self {
      &Var(ref name, _) => name
    }
  }
}

impl<'a> From<&'a str> for Var {
  ///
  /// Constructs a variable from a string. This variable is not pivotable.
  ///
  fn from(var: &'a str) -> Var {
    Var(String::from(var), false)
  }
}

impl<'a> From<String> for Var {
  ///
  /// Constructs a variable from a string. This variable is not pivotable.
  ///
  fn from(var: String) -> Var {
    Var(var, false)
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