use grammar::*;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Display, Formatter, Error};
use std::hash::Hash;
use std::mem::swap;
use std::string::ToString;
use std::str::FromStr;
use var::{Var, VarIndex, VarRef};

pub type Scalar = scalar::Scalar;

pub mod scalar {
  use std::f64;

  pub type Scalar = f64;
  pub const EPSILON: super::Scalar = 0.000001;
  pub const MAX: super::Scalar = f64::MAX;
}

pub fn approx_eq(a: Scalar, b: Scalar) -> bool {
  let delta = (a - b).abs();
  delta < scalar::EPSILON
}

///LinearExpression<V> contains ax+by+...+c

#[derive(Debug,Clone)]
pub struct LinearExpression<V> {
  constant: Scalar,
  terms: BTreeMap<V, Scalar>,
}

impl<V> LinearExpression<V> where V: Ord + Clone + Hash + Debug {
  ///
  /// Creates an expression equal to 0.
  ///
  /// # Examples
  ///
  /// ```
  ///   extern crate constraint;
  ///   use constraint::expr::LinearExpression;
  ///   use constraint::expr::approx_eq;
  ///   use constraint::var::VarRef;
  ///
  ///   fn main() {
  ///     let expr = LinearExpression::<VarRef>::new();
  ///     assert!(expr.terms().is_empty());
  ///     assert!(approx_eq(0.0, expr.get_constant()));
  ///   }
  /// ```
  ///
  pub fn new() -> LinearExpression<V> {
    LinearExpression{
      constant: 0.0,
      terms: BTreeMap::new()
    }
  }

  ///
  /// Construct an expression from a single term coefficient * scalar.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let expr = LinearExpression::term(index.external(String::from("x")), 42.7);
  ///   assert!(approx_eq(42.7, expr.get_coefficient(&index.external(String::from("x")))));
  /// }
  /// ```
  pub fn term(variable: V, coefficient: Scalar) -> LinearExpression<V> {
    let mut expr = LinearExpression::<V>::new();
    expr.set_coefficient(variable, coefficient);
    expr
  }

  ///
  /// Construct an expression from many terms value (coefficient) * key (variable) + constant.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use std::collections::BTreeMap;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut terms = BTreeMap::new();
  ///   let mut index = VarIndex::new();
  ///   terms.insert(index.external(String::from("x")), 1.0);
  ///   let expr = LinearExpression::from_constant_and_terms(1.2, terms);
  ///   assert!(approx_eq(expr.get_constant(), 1.2));
  ///   assert!(approx_eq(expr.get_coefficient(&index.external(String::from("x"))), 1.0));
  /// }
  /// ```
  pub fn from_constant_and_terms(constant: Scalar, terms: BTreeMap<V, Scalar>) -> LinearExpression<V> {
    LinearExpression{
      constant: constant,
      terms: terms
    }
  }
}

impl<V> LinearExpression<V> where V: Ord + Clone + Hash + Debug {

  fn coefficient_transform<'t, F>(&'t mut self, operation: F)
    where F : Fn(Scalar) -> Scalar {
    for (_, value) in self.terms.iter_mut() {
      *value = operation(*value);
    }
    self.constant = operation(self.constant);
  }

  fn coefficient_merge<'a, 'b, F>(&'a mut self, other: &'b LinearExpression<V>, merge_fun: F)
    where F : Fn(Scalar, Scalar) -> Scalar {
    for (name, coef) in other.terms.iter() {
      let existing_coef = self.terms.get(name).map(|c|{*c}).unwrap_or(0.0);
      let new_coef = merge_fun(existing_coef, *coef);
      if approx_eq(0.0, new_coef) {
        if !approx_eq(0.0, *coef) {
          self.terms.remove(name);
        }
      } else {
        let _ = self.terms.insert(name.clone(), new_coef);
      }
    }

    self.constant = merge_fun(self.constant, other.constant);
  }

  ///
  /// Get the mutable terms. The keys are the variable names; the values are the coefficients. This does not include the constant.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let mut expr = LinearExpression::term(index.external(String::from("x")), 2.4);
  ///   expr.plus_this(&LinearExpression::term(index.external(String::from("y")), -2.0));
  ///   for (_, coef) in expr.mut_terms().iter_mut() {
  ///     *coef *= 2.0;
  ///   }
  ///   assert!(approx_eq(4.8, expr.get_coefficient(&index.external(String::from("x")))));
  ///   assert!(approx_eq(-4.0, expr.get_coefficient(&index.external(String::from("y")))));
  /// }
  /// ```
  pub fn mut_terms<'t>(&'t mut self) -> &'t mut BTreeMap<V, Scalar> {
    &mut self.terms
  }


  ///
  /// Get the terms. The keys are the variable names; the values are the coefficients. This does not include the constant.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let expr = LinearExpression::term(index.external(String::from("x")), 9.4);
  ///   let terms = expr.terms();
  ///   assert_eq!(1, terms.len());
  ///   assert!(approx_eq(9.4, *terms.get(&index.external(String::from("x"))).unwrap()));
  /// }
  /// ```
  pub fn terms<'t>(&'t self) -> &'t BTreeMap<V, Scalar> {
    &self.terms
  }

  ///
  /// Get the coefficient of a particular variable.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let expr = LinearExpression::term(index.external(String::from("x")), 1.0);
  ///   assert!(approx_eq(expr.get_coefficient(&index.external(String::from("x"))), 1.0));
  /// }
  /// ```
  pub fn get_coefficient<'s, 'v>(&'s self, v: &V) -> Scalar {
    self.terms.get(v).map(|t| *t).unwrap_or(0.0)
  }

  ///
  /// Get the coefficient of a particular variable.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::{VarIndex, VarRef};
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let mut expr = LinearExpression::<VarRef>::new();
  ///   assert!(approx_eq(0.0, expr.get_coefficient(&index.external(String::from("x")))));
  ///   expr.set_coefficient(index.external(String::from("x")), 1.5);
  ///   assert!(approx_eq(expr.get_coefficient(&index.external(String::from("x"))), 1.5));
  /// }
  /// ```
  pub fn set_coefficient(&mut self, v: V, coefficient: Scalar) {
    self.terms.insert(v, coefficient);
  }

  ///
  /// Get the constant of an expression.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarRef;
  ///
  /// fn main() {
  ///   let mut expr = LinearExpression::<VarRef>::new();
  ///   assert!(approx_eq(0.0, expr.get_constant()));
  ///   expr.set_constant(1.7);
  ///   assert!(approx_eq(1.7, expr.get_constant()));
  /// }
  /// ```
  pub fn get_constant(&self) -> Scalar {
    self.constant
  }

  ///
  /// Set the constant of an expression.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarRef;
  ///
  /// fn main() {
  ///   let mut expr = LinearExpression::<VarRef>::new();
  ///   assert!(approx_eq(0.0, expr.get_constant()));
  ///   expr.set_constant(1.7);
  ///   assert!(approx_eq(1.7, expr.get_constant()));
  /// }
  /// ```
  pub fn set_constant(&mut self, constant: Scalar) {
    self.constant = constant;
  }

  ///
  /// For an expression ax+...+c and factor f, return f*(ax+...+c).
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let expr = LinearExpression::term(index.external(String::from("x")), 2.1).plus(&LinearExpression::from(2.3));
  ///   let product = expr.times(-2.0);
  ///   assert!(approx_eq(-4.2, product.get_coefficient(&index.external(String::from("x")))));
  ///   assert!(approx_eq(-4.6, product.get_constant()));
  /// }
  /// ```
  pub fn times(&self, constant: Scalar) -> LinearExpression<V> {
    LinearExpression::<V>::from_constant_and_terms(self.constant * constant,
                                                   self.terms.iter().map(|(name, scalar)| {
                                                     (name.clone(), scalar * constant)
                                                   }).collect())
  }

  ///
  /// Mutate this expression to be the result of adding each term and constant with the argument.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let mut expr = LinearExpression::term(index.external(String::from("x")), 2.1).plus(&LinearExpression::from(2.3));
  ///   let expr2 = LinearExpression::term(index.external(String::from("x")), 1.6).plus(&LinearExpression::from(-8.7));
  ///   expr.plus_this(&expr2);
  ///   assert!(approx_eq(3.7, expr.get_coefficient(&index.external(String::from("x")))));
  ///   assert!(approx_eq(-6.4, expr.get_constant()));
  /// }
  /// ```
  pub fn plus_this(&mut self, expr: &LinearExpression<V>) {
    self.coefficient_merge(expr, |a, b| a + b);
  }

  ///
  /// Mutate this expression to be the result of multiplying each term and constant by a constant.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let mut expr = LinearExpression::term(index.external(String::from("x")), 1.6).plus(&LinearExpression::from(0.3));
  ///   expr.times_this(-1.7);
  ///   assert!(approx_eq(-2.72, expr.get_coefficient(&index.external(String::from("x")))));
  ///   assert!(approx_eq(-0.51, expr.get_constant()));
  /// }
  /// ```
  pub fn times_this(&mut self, scalar: Scalar) {
    if approx_eq(0.0, scalar) {
      self.terms.clear();
      self.constant = 0.0;
    } else {
      self.coefficient_transform(|a| a * scalar);
    }
  }

  ///
  /// Mutate this expression to be the result of dividing each term and constant by a constant.
  /// It is not valid to call this function with a scalar of 0.0.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let mut expr = LinearExpression::term(index.external(String::from("x")), 81.42).plus(&LinearExpression::from(-114.0));
  ///   expr.div_this(-2.0);
  ///   assert!(approx_eq(-40.71, expr.get_coefficient(&index.external(String::from("x")))));
  ///   assert!(approx_eq(57.0, expr.get_constant()));
  /// }
  /// ```
  pub fn div_this(&mut self, scalar: Scalar) {
    assert!(!approx_eq(0.0, scalar));
    self.coefficient_transform(|a| a / scalar);
  }

  ///
  /// Give the result of adding each term and constant with each term and constant of the given expression.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let expr = LinearExpression::term(index.external(String::from("x")), 0.7).plus(&LinearExpression::from(1.8));
  ///   assert!(approx_eq(0.7, expr.get_coefficient(&index.external(String::from("x")))));
  ///   assert!(approx_eq(1.8, expr.get_constant()));
  /// }
  /// ```
  pub fn plus(&self, expr: &LinearExpression<V>) -> LinearExpression<V> {
    let mut result = self.clone();
    result.plus_this(expr);
    result
  }

  ///
  /// Give the result of subtracting each term and constant with each term and constant of the given expression.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let expr = LinearExpression::term(index.external(String::from("x")), 1.7).plus(&LinearExpression::from(8.1));
  ///   let expr2 = expr.minus(&LinearExpression::term(index.external(String::from("y")), 42.7));
  ///   assert!(approx_eq(1.7, expr2.get_coefficient(&index.external(String::from("x")))));
  ///   assert!(approx_eq(-42.7, expr2.get_coefficient(&index.external(String::from("y")))));
  ///   assert!(approx_eq(8.1, expr2.get_constant()));
  /// }
  /// ```
  pub fn minus(&self, expr: &LinearExpression<V>) -> LinearExpression<V> {
    let mut new = expr.times(-1.0);
    new.plus_this(self);
    new
  }

  ///
  /// Mutate this expression to be the result of adding each term and constant with the argument.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let mut expr = LinearExpression::term(index.external(String::from("x")), 2.1).plus(&LinearExpression::from(2.3));
  ///   let expr2 = LinearExpression::term(index.external(String::from("x")), 1.6).plus(&LinearExpression::from(-8.7));
  ///   expr.minus_this(&expr2);
  ///   assert!(approx_eq(0.5, expr.get_coefficient(&index.external(String::from("x")))));
  ///   assert!(approx_eq(11.0, expr.get_constant()));
  /// }
  /// ```
  pub fn minus_this(&mut self, expr: &LinearExpression<V>) {
    self.coefficient_merge(expr, |a, b| a - b);
  }

  ///
  /// For an expression ax+...+c and divisor d, return (ax+...+c)/d.
  /// It is not valid to call this function with a divisor of 0.0.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let expr = LinearExpression::term(index.external(String::from("x")), 2.1).plus(&LinearExpression::from(2.3));
  ///   let product = expr.div(-2.0);
  ///   assert!(approx_eq(-1.05, product.get_coefficient(&index.external(String::from("x")))));
  ///   assert!(approx_eq(-1.15, product.get_constant()));
  /// }
  /// ```
  pub fn div(&self, constant: Scalar) -> LinearExpression<V> {
    assert!(!approx_eq(constant, 0.0));
    self.times(1.0 / constant)
  }

  ///
  /// For an expression ax+...+c, substitution variable q, and substituted expression e, return an expression with a all instances of q in expression ax+...+c replaced with e.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate constraint;
  /// use constraint::expr::LinearExpression;
  /// use constraint::expr::approx_eq;
  /// use constraint::var::VarIndex;
  ///
  /// fn main() {
  ///   let mut index = VarIndex::new();
  ///   let mut expr = LinearExpression::term(index.external(String::from("x")), 2.1).plus(&LinearExpression::from(2.3));
  ///   expr.substitute(&index.external(String::from("x")), &LinearExpression::term(index.external(String::from("y")), 1.2).plus(&LinearExpression::from(7.5)));
  ///   assert!(approx_eq(0.0, expr.get_coefficient(&index.external(String::from("x")))));
  ///   assert!(approx_eq(2.52, expr.get_coefficient(&index.external(String::from("y")))));
  ///   assert!(approx_eq(18.05, expr.get_constant()));
  /// }
  /// ```
  pub fn substitute(&mut self, var: &V, e: &LinearExpression<V>) {
    let o_coef = self.terms.remove(var);
    if o_coef.is_none() {
      return;
    }
    let coef = o_coef.unwrap();
    self.plus_this(&e.times(coef));
  }

  ///
  /// Evaluate an expression against a set of bindings.
  ///
  /// # Examples
  ///
  /// ```
  ///   extern crate constraint;
  ///   use constraint::expr::{approx_eq, Scalar};
  ///   use std::collections::HashMap;
  ///   use constraint::var::{VarIndex, VarRef};
  ///   use constraint::expr::InternedLinearExpression;
  ///
  ///   fn main() {
  ///     let mut index = VarIndex::new();
  ///     let expr = InternedLinearExpression::term(index.external(String::from("x")), 2.0);
  ///     let mut bindings: HashMap<VarRef, Scalar> = HashMap::new();
  ///     bindings.insert(index.external(String::from("x")), 1.3);
  ///     let result = expr.eval(&bindings);
  ///     assert!(approx_eq(2.6, result.get_constant()));
  ///     assert!(result.terms().is_empty());
  ///   }
  /// ```
  ///
  pub fn eval(&self, bindings: &HashMap<V, Scalar>) -> LinearExpression<V> {
    let (defined, undefined) = self.terms.iter().map(|(k,v)|(k.clone(), *v)).partition(|&(ref k,_)|bindings.contains_key(k));
    let mut result = self.constant;
    for (ref var, coef) in defined {
      result += coef * *bindings.get(var).unwrap();
    }
    LinearExpression::from_constant_and_terms(result, undefined)
  }

  ///
  /// Evaluate an expression against a set of bindings.
  ///
  /// # Examples
  ///
  /// ```
  ///   extern crate constraint;
  ///   use constraint::expr::{approx_eq, Scalar};
  ///   use std::collections::HashMap;
  ///   use constraint::var::{VarIndex, VarRef};
  ///   use constraint::expr::InternedLinearExpression;
  ///
  ///   fn main() {
  ///     let mut index = VarIndex::new();
  ///     let expr = InternedLinearExpression::term(index.external(String::from("x")), 2.0);
  ///     let mut bindings: HashMap<VarRef, Scalar> = HashMap::new();
  ///     bindings.insert(index.external(String::from("x")), 1.3);
  ///     assert!(approx_eq(2.6, expr.full_eval(&bindings).unwrap()));
  ///   }
  /// ```
  ///
  pub fn full_eval(&self, bindings: &HashMap<V, Scalar>) -> Result<Scalar, String> {
    let result = self.eval(bindings);
    if result.terms().is_empty() {
      Ok(result.get_constant())
    } else {
      Err(format!("The following terms are undefined: {:?}", result.terms().keys().collect::<Vec<&V>>()))
    }
  }
}

impl<V> From<Scalar> for LinearExpression<V> where V: Ord + Clone + Hash + Debug {
  fn from(constant: Scalar) -> LinearExpression<V> {
    LinearExpression{
      constant: constant,
      terms: BTreeMap::new()
    }
  }
}

impl<V> Display for LinearExpression<V> where V: ToString {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let mut terms: Vec<String> = vec![];
    for (name, coeff) in self.terms.iter() {
      if approx_eq(1.0, *coeff) {
        terms.push(name.to_string().clone());
      } else {
        terms.push(format!("{}{}", coeff, name.to_string()));
      }
    }
    if !approx_eq(0.0, self.constant) || terms.is_empty() {
      terms.push(format!("{}", self.constant));
    }
    fmt.write_str(terms.join(" + ").as_ref() as &str)
  }
}

pub type RawLinearExpression = LinearExpression<Var>;
pub type InternedLinearExpression = LinearExpression<VarRef>;

impl From<VarRef> for InternedLinearExpression {
  fn from(var: VarRef) -> InternedLinearExpression {
    LinearExpression::term(var, 1.0)
  }
}

impl RawLinearExpression {
  pub fn interned(&self, index: &mut VarIndex) -> InternedLinearExpression {
    InternedLinearExpression::from_constant_and_terms(self.get_constant(), self.terms.iter().map(|(k,v)|(index.insert(k.clone()), *v)).collect())
  }
}

impl From<Var> for RawLinearExpression {
  fn from(var: Var) -> RawLinearExpression {
    LinearExpression::term(var, 1.0)
  }
}

impl FromStr for RawLinearExpression {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match parse_Expression(s) {
      Ok(result) => Ok(result),
      Err(e) => Err(format!("{:?}", e))
    }
  }
}

#[derive(Debug,Eq,PartialEq,Clone)]
pub enum Relation {
  EQ,
  NEQ,
  GT,
  LT,
  GEQ,
  LEQ
}

impl Relation {
  fn reverse(&mut self) {
    *self = match self {
      &mut Relation::EQ => Relation::NEQ,
      &mut Relation::NEQ => Relation::EQ,
      &mut Relation::GT => Relation::LT,
      &mut Relation::LEQ => Relation::GEQ,
      &mut Relation::GEQ => Relation::LEQ,
      &mut Relation::LT => Relation::GT
    };
  }
}

impl Display for Relation {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let op_str = match self {
      &Relation::EQ => "=",
      &Relation::NEQ => "!=",
      &Relation::LT => "<",
      &Relation::LEQ => "<=",
      &Relation::GT => ">",
      &Relation::GEQ => ">="
    };
    fmt.write_str(op_str)
  }
}

pub type RawLinearRelation = LinearRelation<Var>;
pub type InternedLinearRelation = LinearRelation<VarRef>;

#[derive(Debug,Clone)]
pub struct LinearRelation<V: Ord + Clone + Hash + Debug> {
  pub lhs: LinearExpression<V>,
  pub op: Relation,
  pub rhs: LinearExpression<V>
}

impl<V> LinearRelation<V> where V: Ord + Clone + Hash + Debug + Display {
  ///
  /// Relate two expressions
  ///
  /// # Examples
  ///
  /// ```
  ///   extern crate constraint;
  ///   use constraint::expr::InternedLinearExpression;
  ///   use constraint::expr::{approx_eq, LinearRelation, Relation};
  ///   use constraint::var::VarIndex;
  ///
  ///   fn main() {
  ///     let mut index = VarIndex::new();
  ///     let relation = LinearRelation::new(InternedLinearExpression::term(index.external(String::from("x")), -0.5), Relation::GEQ, InternedLinearExpression::from(3.0));
  ///     assert_eq!(Relation::GEQ, relation.op);
  ///     assert_eq!(1, relation.lhs.terms().len());
  ///     assert!(relation.rhs.terms().is_empty());
  ///     assert!(approx_eq(3.0, relation.rhs.get_constant()));
  ///   }
  /// ```
  ///
  pub fn new(lhs: LinearExpression<V>, op: Relation, rhs: LinearExpression<V>) -> LinearRelation<V> {
    LinearRelation{
      lhs: lhs,
      op: op,
      rhs: rhs
    }
  }

  ///
  /// Reverses both the lhs and the rhs and the relation itself
  ///
  /// # Examples
  ///
  /// ```
  ///   extern crate constraint;
  ///   use constraint::expr::InternedLinearExpression;
  ///   use constraint::expr::{approx_eq, LinearRelation, Relation};
  ///
  ///   fn main() {
  ///     let mut relation = LinearRelation::new(InternedLinearExpression::from(4.2), Relation::GEQ, InternedLinearExpression::from(3.1));
  ///     relation.reverse();
  ///     assert_eq!(relation.op, Relation::LEQ);
  ///     assert!(approx_eq(relation.lhs.get_constant(), 3.1));
  ///     assert!(approx_eq(relation.rhs.get_constant(), 4.2));
  ///   }
  /// ```
  pub fn reverse(&mut self) {
    swap(&mut self.lhs, &mut self.rhs);
    self.op.reverse();
  }

  ///
  /// Mutates the relation in place by adding the expression to both sides.
  ///
  /// # Examples
  ///
  /// ```
  ///   extern crate constraint;
  ///   use constraint::expr::InternedLinearExpression;
  ///   use constraint::expr::{approx_eq, LinearRelation, Relation};
  ///   use constraint::var::VarIndex;
  ///
  ///   fn main() {
  ///     let mut index = VarIndex::new();
  ///     let mut relation = LinearRelation::new(InternedLinearExpression::term(index.external(String::from("x")), 2.3), Relation::EQ, InternedLinearExpression::from(3.7));
  ///     relation.plus_this(&InternedLinearExpression::term(index.external(String::from("y")), 8.2).plus(&InternedLinearExpression::from(4.1)));
  ///     assert_eq!(relation.op, Relation::EQ);
  ///     assert_eq!(relation.lhs.terms().len(), 2);
  ///     assert_eq!(relation.rhs.terms().len(), 1);
  ///     assert!(approx_eq(relation.lhs.get_constant(), 4.1));
  ///     assert!(approx_eq(relation.rhs.get_constant(), 7.8));
  ///   }
  /// ```
  pub fn plus_this(&mut self, expr: &LinearExpression<V>) {
    self.lhs.plus_this(expr);
    self.rhs.plus_this(expr);
  }

  pub fn minus_this(&mut self, expr: &LinearExpression<V>) {
    self.lhs.minus_this(expr);
    self.rhs.minus_this(expr);
  }

  pub fn times_this(&mut self, constant: Scalar) {
    self.lhs.times_this(constant);
    self.rhs.times_this(constant);
  }

  pub fn div_this(&mut self, constant: Scalar) {
    self.lhs.div_this(constant);
    self.rhs.div_this(constant);
  }

  pub fn substitute(&mut self, v: &V, e: &LinearExpression<V>) {
    self.lhs.substitute(v, e);
    self.rhs.substitute(v, e);
  }

  ///
  /// Examples
  ///
  /// # Examples
  ///
  /// ```
  ///   extern crate constraint;
  ///   use constraint::expr::InternedLinearExpression;
  ///   use constraint::expr::{approx_eq, LinearRelation, Relation};
  ///   use constraint::var::VarIndex;
  ///
  ///   fn main() {
  ///     let mut index = VarIndex::new();
  ///     let relation = LinearRelation::new(InternedLinearExpression::term(index.external(String::from("x")), -0.5), Relation::GEQ, InternedLinearExpression::from(3.0));
  ///     let (op, expr) = relation.solve_for(&index.external(String::from("x"))).unwrap();
  ///     assert_eq!(Relation::LEQ, op);
  ///     assert!(expr.terms().is_empty());
  ///     assert!(approx_eq(-6.0, expr.get_constant()));
  ///   }
  /// ```
  ///
  pub fn solve_for(&self, var: &V) -> Result<(Relation, LinearExpression<V>), String> {
    let mut lhs = self.lhs.clone();
    let mut rhs = self.rhs.clone();
    let a = lhs.mut_terms().remove(var).unwrap_or(0.0);
    let b = rhs.mut_terms().remove(var).unwrap_or(0.0);
    let numerator = rhs.minus(&lhs);
    let denominator = a - b;
    if approx_eq(0.0, denominator) {
      return Err(format!("Solving for {} results in {} / 0", var, numerator));
    }
    let mut op = self.op.clone();
    if denominator < 0.0 && op != Relation::EQ {
      op.reverse()
    }
    Ok((op, numerator.div(denominator)))
  }
}

impl FromStr for RawLinearRelation {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match parse_LinearRelation(s) {
      Ok(result) => Ok(result),
      Err(e) => Err(format!("{:?}", e))
    }
  }
}

impl<V> Display for LinearRelation<V> where V: Ord + Clone + Hash + Debug + Display {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    fmt.write_str(format!("{} {} {}",
                          self.lhs,
                          self.op,
                          self.rhs).as_ref() as &str)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::collections::HashMap;
  use var::{VarIndex, VarRef};

  #[test]
  fn to_string() {
    assert_eq!("0", InternedLinearExpression::new().to_string().as_ref() as &str)
  }

  #[test]
  fn get_and_set_coefficients() {
    let mut index = VarIndex::new();
    let mut expr = InternedLinearExpression::new();
    let var = index.external(String::from("x"));
    assert!(approx_eq(expr.get_coefficient(&var), 0.0));
    expr.set_coefficient(var.clone(), 42.0);
    assert!(approx_eq(expr.get_coefficient(&var), 42.0));
  }

  #[test]
  fn get_and_set_constant() {
    let mut expr = InternedLinearExpression::new();
    assert!(approx_eq(expr.get_constant(), 0.0));
    expr.set_constant(-34.0);
    assert!(approx_eq(expr.get_constant(), -34.0));
  }

  #[test]
  fn times_and_div() {
    let a = 1.0;
    let b = -2.0;
    let c = 7.0;
    let mut index = VarIndex::new();
    let x1 = index.external(String::from("x1"));
    let x2 = index.external(String::from("x2"));

    let mut expr = InternedLinearExpression::new();
    expr.set_coefficient(x1.clone(), a);
    expr.set_coefficient(x2.clone(), b);
    expr.set_constant(c);

    let times_expr = expr.times(2.0);
    assert!(approx_eq(times_expr.get_coefficient(&x1), 2.0));
    assert!(approx_eq(times_expr.get_coefficient(&x2), -4.0));
    assert!(approx_eq(times_expr.get_constant(), 14.0));

    let div_expr = expr.div(2.0);
    assert!(approx_eq(div_expr.get_coefficient(&x1), 0.5));
    assert!(approx_eq(div_expr.get_coefficient(&x2), -1.0));
    assert!(approx_eq(div_expr.get_constant(), 3.5));
  }

  #[test]
  fn plus_and_minus() {
    let p = 1.0;
    let q = -2.0;
    let r = 3.4;
    let s = 10.0;
    let c1 = 3.0;
    let c2 = -4.5;

    let mut index = VarIndex::new();
    let x1 = index.external(String::from("x1"));
    let x2 = index.external(String::from("x2"));
    let x3 = index.external(String::from("x3"));

    let mut expr1 = InternedLinearExpression::new();
    expr1.set_coefficient(x1.clone(), p);
    expr1.set_coefficient(x2.clone(), q);
    expr1.set_constant(c1);

    let mut expr2 = InternedLinearExpression::new();
    expr2.set_coefficient(x2.clone(), r);
    expr2.set_coefficient(x3.clone(), s);
    expr2.set_constant(c2);

    let add_expr = expr1.plus(&expr2);
    assert!(approx_eq(add_expr.get_coefficient(&x1), p));
    assert!(approx_eq(add_expr.get_coefficient(&x2), q + r));
    assert!(approx_eq(add_expr.get_coefficient(&x3), s));
    assert!(approx_eq(add_expr.get_constant(), c1 + c2));

    let minus_expr = expr1.minus(&expr2);
    assert!(approx_eq(minus_expr.get_coefficient(&x1), p));
    assert!(approx_eq(minus_expr.get_coefficient(&x2), q - r));
    assert!(approx_eq(minus_expr.get_coefficient(&x3), -s));
    assert!(approx_eq(minus_expr.get_constant(), c1 - c2));
  }

  #[test]
  fn eval_zero() {
    assert!(approx_eq(0.0, InternedLinearExpression::new().full_eval(&HashMap::new()).unwrap()));
  }

  #[test]
  fn eval_constant() {
    assert!(approx_eq(10.0, InternedLinearExpression::from(10.0).full_eval(&HashMap::new()).unwrap()));
  }

  #[test]
  fn eval_single_term() {
    let mut index = VarIndex::new();
    let x_ref = index.external(String::from("x"));
    let expr = InternedLinearExpression::term(x_ref.clone(), -2.0);
    let mut bindings: HashMap<VarRef, Scalar> = HashMap::new();
    bindings.insert(x_ref, -21.0);
    assert!(approx_eq(42.0, expr.full_eval(&bindings).unwrap()));
  }

  #[test]
  fn eval_multiple_term() {
    let mut index = VarIndex::new();
    let expr = InternedLinearExpression::term(index.external(String::from("x")), -2.0).plus(&InternedLinearExpression::term(index.external(String::from("y")), 3.4));
    let mut bindings: HashMap<VarRef, Scalar> = HashMap::new();
    bindings.insert(index.external(String::from("x")), -21.0);
    bindings.insert(index.external(String::from("y")), -6.0);
    assert!(approx_eq(21.6, expr.full_eval(&bindings).unwrap()));
  }

  #[test]
  fn eval_terms_and_constant() {
    let mut index = VarIndex::new();
    let x_ref = index.external(String::from("x"));
    let y_ref = index.external(String::from("y"));
    let expr = InternedLinearExpression::term(x_ref.clone(), -2.0).plus(&InternedLinearExpression::term(y_ref.clone(), 3.4)).plus(&InternedLinearExpression::from(7.2));
    let mut bindings: HashMap<VarRef, Scalar> = HashMap::new();
    bindings.insert(x_ref, -21.0);
    bindings.insert(y_ref, -6.0);
    assert!(approx_eq(28.8, expr.full_eval(&bindings).unwrap()));
  }

  #[test]
  fn eval_unbound_val() {
    let mut index = VarIndex::new();
    assert!(InternedLinearExpression::term(index.external(String::from("x")), -2.0).full_eval(&HashMap::new()).is_err());
  }

  #[test]
  fn no_substitute() {
    let mut index = VarIndex::new();
    let mut expr = InternedLinearExpression::new();
    expr.substitute(&index.external(String::from("x")), &InternedLinearExpression::from(2.0));
    assert!(approx_eq(0.0, expr.full_eval(&HashMap::new()).unwrap()));
    assert!(expr.terms().len() == 0);
  }

  #[test]
  fn constant_substitute() {
    let mut index = VarIndex::new();
    let x_ref = index.external(String::from("x"));
    let y_ref = index.external(String::from("y"));
    let mut expr = InternedLinearExpression::term(x_ref.clone(), 2.0).plus(&InternedLinearExpression::term(y_ref.clone(), -3.0));
    assert!(expr.terms().len() == 2);
    expr.substitute(&x_ref, &InternedLinearExpression::from(1.2));
    assert!(expr.terms().len() == 1);
    let mut bindings: HashMap<VarRef, Scalar> = HashMap::new();
    bindings.insert(y_ref, 1.6);
    assert!(approx_eq(-2.4, expr.full_eval(&bindings).unwrap()));
  }

  #[test]
  fn replace_substitute() {
    let mut index = VarIndex::new();
    let x_ref = index.external(String::from("x"));
    let y_ref = index.external(String::from("y"));
    let z_ref = index.external(String::from("z"));
    let mut expr = InternedLinearExpression::term(x_ref.clone(), 2.0).plus(&InternedLinearExpression::term(y_ref.clone(), -3.0));
    assert!(expr.terms().len() == 2);
    expr.substitute(&x_ref, &InternedLinearExpression::term(z_ref.clone(), -4.0));
    assert!(expr.terms().len() == 2);
    assert!(approx_eq(-8.0, expr.get_coefficient(&z_ref)));
    let mut bindings: HashMap<VarRef, Scalar> = HashMap::new();
    bindings.insert(z_ref, 1.3);
    bindings.insert(y_ref, 1.6);
    assert!(approx_eq(-15.2, expr.full_eval(&bindings).unwrap()));
  }

  #[test]
  fn complicated_substitute() {
    let mut index = VarIndex::new();
    let x_ref = index.external(String::from("x"));
    let y_ref = index.external(String::from("y"));
    let z_ref = index.external(String::from("z"));
    let w_ref = index.external(String::from("w"));
    let mut expr1 = InternedLinearExpression::term(x_ref.clone(), 2.0).plus(&InternedLinearExpression::term(y_ref.clone(), -3.0)).plus(&InternedLinearExpression::from(3.0));
    let expr2 = InternedLinearExpression::term(w_ref.clone(), -2.5).plus(&InternedLinearExpression::term(z_ref.clone(), 4.3)).plus(&InternedLinearExpression::from(-10.0));
    assert!(expr1.terms().len() == 2);
    expr1.substitute(&x_ref, &expr2);
    assert!(expr1.terms().len() == 3);
    assert!(approx_eq(8.6, expr1.get_coefficient(&z_ref)));
    assert!(approx_eq(-5.0, expr1.get_coefficient(&w_ref)));
    assert!(approx_eq(-3.0, expr1.get_coefficient(&y_ref)));
    assert!(approx_eq(-17.0, expr1.get_constant()));
    let mut bindings: HashMap<VarRef, Scalar> = HashMap::new();
    bindings.insert(z_ref, 1.3);
    bindings.insert(y_ref, 1.6);
    bindings.insert(w_ref, -2.7);
    assert!(approx_eq(2.88, expr1.full_eval(&bindings).unwrap()));
  }
}
