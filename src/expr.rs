use std::fmt::{Display, Formatter, Error};
use std::mem::swap;
use var::VarRef;
use abs::InternedLinearExpression;

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
      &Relation::EQ => "==",
      &Relation::NEQ => "=!=",
      &Relation::LT => "<",
      &Relation::LEQ => "<=",
      &Relation::GT => ">",
      &Relation::GEQ => ">="
    };
    fmt.write_str(op_str)
  }
}

#[derive(Debug,Clone)]
pub struct LinearRelation {
  pub lhs: LinearExpression,
  pub op: Relation,
  pub rhs: LinearExpression
}

impl LinearRelation {
  ///
  /// Relate two expressions
  ///
  /// # Examples
  ///
  /// ```
  ///   extern crate constraint;
  ///   use constraint::expr::{approx_eq, LinearExpression, LinearRelation, Relation};
  ///   use constraint::var::VarIndex;
  ///
  ///   fn main() {
  ///     let mut index = VarIndex::new();
  ///     let relation = LinearRelation::new(LinearExpression::term(index.external(String::from("x")), -0.5), Relation::GEQ, LinearExpression::from(3.0));
  ///     assert_eq!(Relation::GEQ, relation.op);
  ///     assert_eq!(1, relation.lhs.terms().len());
  ///     assert!(relation.rhs.terms().is_empty());
  ///     assert!(approx_eq(3.0, relation.rhs.get_constant()));
  ///   }
  /// ```
  ///
  pub fn new(lhs: LinearExpression, op: Relation, rhs: LinearExpression) -> LinearRelation {
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
  ///   use constraint::expr::{approx_eq, LinearExpression, LinearRelation, Relation};
  ///
  ///   fn main() {
  ///     let mut relation = LinearRelation::new(LinearExpression::from(4.2), Relation::GEQ, LinearExpression::from(3.1));
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
  ///   use constraint::expr::{approx_eq, LinearExpression, LinearRelation, Relation};
  ///   use constraint::var::VarIndex;
  ///
  ///   fn main() {
  ///     let mut index = VarIndex::new();
  ///     let mut relation = LinearRelation::new(LinearExpression::term(index.external(String::from("x")), 2.3), Relation::EQ, LinearExpression::from(3.7));
  ///     relation.plus_this(&LinearExpression::term(index.external(String::from("y")), 8.2).plus(&LinearExpression::from(4.1)));
  ///     assert_eq!(relation.op, Relation::EQ);
  ///     assert_eq!(relation.lhs.terms().len(), 2);
  ///     assert_eq!(relation.rhs.terms().len(), 1);
  ///     assert!(approx_eq(relation.lhs.get_constant(), 4.1));
  ///     assert!(approx_eq(relation.rhs.get_constant(), 7.8));
  ///   }
  /// ```
  pub fn plus_this(&mut self, expr: &LinearExpression) {
    self.lhs.plus_this(expr);
    self.rhs.plus_this(expr);
  }

  pub fn minus_this(&mut self, expr: &LinearExpression) {
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

  pub fn substitute(&mut self, v: &VarRef, e: &LinearExpression) {
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
  ///   use constraint::expr::{approx_eq, LinearExpression, LinearRelation, Relation};
  ///   use constraint::var::VarIndex;
  ///
  ///   fn main() {
  ///     let mut index = VarIndex::new();
  ///     let relation = LinearRelation::new(LinearExpression::term(index.external(String::from("x")), -0.5), Relation::GEQ, LinearExpression::from(3.0));
  ///     let (op, expr) = relation.solve_for(&index.external(String::from("x"))).unwrap();
  ///     assert_eq!(Relation::LEQ, op);
  ///     assert!(expr.terms().is_empty());
  ///     assert!(approx_eq(-6.0, expr.get_constant()));
  ///   }
  /// ```
  ///
  pub fn solve_for(&self, var: &VarRef) -> Result<(Relation, LinearExpression), String> {
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

impl Display for LinearRelation {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    fmt.write_str(format!("{} {} {}",
                          self.lhs,
                          self.op,
                          self.rhs).as_ref() as &str)
  }
}

pub type LinearExpression = InternedLinearExpression;

#[cfg(test)]
mod test {
  use super::*;
  use std::collections::HashMap;
  use var::{VarIndex, VarRef};

  #[test]
  fn to_string() {
    assert_eq!("0", LinearExpression::new().to_string().as_ref() as &str)
  }

  #[test]
  fn get_and_set_coefficients() {
    let mut index = VarIndex::new();
    let mut expr = LinearExpression::new();
    let var = index.external(String::from("x"));
    assert!(approx_eq(expr.get_coefficient(&var), 0.0));
    expr.set_coefficient(var.clone(), 42.0);
    assert!(approx_eq(expr.get_coefficient(&var), 42.0));
  }

  #[test]
  fn get_and_set_constant() {
    let mut expr = LinearExpression::new();
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

    let mut expr = LinearExpression::new();
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

    let mut expr1 = LinearExpression::new();
    expr1.set_coefficient(x1.clone(), p);
    expr1.set_coefficient(x2.clone(), q);
    expr1.set_constant(c1);

    let mut expr2 = LinearExpression::new();
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
    assert!(approx_eq(0.0, LinearExpression::new().full_eval(&HashMap::new()).unwrap()));
  }

  #[test]
  fn eval_constant() {
    assert!(approx_eq(10.0, LinearExpression::from(10.0).full_eval(&HashMap::new()).unwrap()));
  }

  #[test]
  fn eval_single_term() {
    let mut index = VarIndex::new();
    let x_ref = index.external(String::from("x"));
    let expr = LinearExpression::term(x_ref.clone(), -2.0);
    let mut bindings: HashMap<VarRef, Scalar> = HashMap::new();
    bindings.insert(x_ref, -21.0);
    assert!(approx_eq(42.0, expr.full_eval(&bindings).unwrap()));
  }

  #[test]
  fn eval_multiple_term() {
    let mut index = VarIndex::new();
    let expr = LinearExpression::term(index.external(String::from("x")), -2.0).plus(&LinearExpression::term(index.external(String::from("y")), 3.4));
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
    let expr = LinearExpression::term(x_ref.clone(), -2.0).plus(&LinearExpression::term(y_ref.clone(), 3.4)).plus(&LinearExpression::from(7.2));
    let mut bindings: HashMap<VarRef, Scalar> = HashMap::new();
    bindings.insert(x_ref, -21.0);
    bindings.insert(y_ref, -6.0);
    assert!(approx_eq(28.8, expr.full_eval(&bindings).unwrap()));
  }

  #[test]
  fn eval_unbound_val() {
    let mut index = VarIndex::new();
    assert!(LinearExpression::term(index.external(String::from("x")), -2.0).full_eval(&HashMap::new()).is_err());
  }

  #[test]
  fn no_substitute() {
    let mut index = VarIndex::new();
    let mut expr = LinearExpression::new();
    expr.substitute(&index.external(String::from("x")), &LinearExpression::from(2.0));
    assert!(approx_eq(0.0, expr.full_eval(&HashMap::new()).unwrap()));
    assert!(expr.terms().len() == 0);
  }

  #[test]
  fn constant_substitute() {
    let mut index = VarIndex::new();
    let x_ref = index.external(String::from("x"));
    let y_ref = index.external(String::from("y"));
    let mut expr = LinearExpression::term(x_ref.clone(), 2.0).plus(&LinearExpression::term(y_ref.clone(), -3.0));
    assert!(expr.terms().len() == 2);
    expr.substitute(&x_ref, &LinearExpression::from(1.2));
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
    let mut expr = LinearExpression::term(x_ref.clone(), 2.0).plus(&LinearExpression::term(y_ref.clone(), -3.0));
    assert!(expr.terms().len() == 2);
    expr.substitute(&x_ref, &LinearExpression::term(z_ref.clone(), -4.0));
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
    let mut expr1 = LinearExpression::term(x_ref.clone(), 2.0).plus(&LinearExpression::term(y_ref.clone(), -3.0)).plus(&LinearExpression::from(3.0));
    let expr2 = LinearExpression::term(w_ref.clone(), -2.5).plus(&LinearExpression::term(z_ref.clone(), 4.3)).plus(&LinearExpression::from(-10.0));
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
