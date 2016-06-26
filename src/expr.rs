use std::collections::{BTreeMap, HashMap, HashSet};
use std::iter::Iterator;
use std::fmt::{Display, Formatter, Error};
use std::mem::swap;

pub type Scalar = f64;
pub const SCALAR_EPSILON: Scalar = 0.000001;

pub fn approx_eq(a: Scalar, b: Scalar) -> bool {
  let delta = (a - b).abs();
  delta < SCALAR_EPSILON
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
  pub fn new(lhs: LinearExpression, op: Relation, rhs: LinearExpression) -> LinearRelation {
    LinearRelation{
      lhs: lhs,
      op: op,
      rhs: rhs
    }
  }

  pub fn reverse(&mut self) {
    swap(&mut self.lhs, &mut self.rhs);
    self.op.reverse();
  }

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

  pub fn substitute(&mut self, v: &String, e: &LinearExpression) {
    self.lhs.substitute(v, e);
    self.rhs.substitute(v, e);
  }

  ///
  /// # Examples
  ///
  /// ```
  ///   extern crate constraint;
  ///   use constraint::expr::{approx_eq, LinearExpression, LinearRelation, Relation};
  ///
  ///   fn main() {
  ///     let relation = LinearRelation::new(LinearExpression::term(String::from("x"), -0.5), Relation::GEQ, LinearExpression::from(3.0));
  ///     let (op, expr) = relation.solve_for(&String::from("x")).unwrap();
  ///     assert_eq!(Relation::LEQ, op);
  ///     assert!(expr.terms().is_empty());
  ///     assert!(approx_eq(-6.0, expr.get_constant()));
  ///   }
  /// ```
  ///
  pub fn solve_for(&self, var: &String) -> Result<(Relation, LinearExpression), String> {
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

///LinearExpression contains ax+by+...+c

#[derive(Debug,Clone)]
pub struct LinearExpression {
  constant: Scalar,
  terms: BTreeMap<String, Scalar>,
}

impl LinearExpression {
  pub fn new() -> LinearExpression {
    LinearExpression{
      constant: 0.0,
      terms: BTreeMap::new()
    }
  }

  pub fn eval(&self, bindings: &HashMap<String, Scalar>) -> Result<Scalar, String> {
    let undefined_bindings: HashSet<String> = self.terms.keys().filter(|v|!bindings.contains_key(v.clone())).map(|v|v.clone()).collect();
    if undefined_bindings.is_empty() {
      let mut sum = 0.0f64;
      for (var, coef) in self.terms.iter() {
        sum += *bindings.get(var).unwrap();
      }
      Ok(sum + self.constant)
    } else {
      Err(format!("cannot evaluate undefined variables: {:?}", undefined_bindings))
    }
  }

  pub fn coefficient_transform<'t, F>(&'t mut self, operation: F)
    where F : Fn(Scalar) -> Scalar {
    for (_, value) in self.terms.iter_mut() {
      *value = operation(*value);
    }
    self.constant = operation(self.constant);
  }

  pub fn coefficient_merge<'a, 'b, F>(&'a mut self, other: &'b LinearExpression, merge_fun: F)
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

  pub fn mut_terms<'t>(&'t mut self) -> &'t mut BTreeMap<String, Scalar> {
    &mut self.terms
  }
  
  pub fn terms<'t>(&'t self) -> &'t BTreeMap<String, Scalar> {
    &self.terms
  }

  pub fn term(variable: String, coefficient: Scalar) -> LinearExpression {
    let mut expr = LinearExpression::new();
    expr.set_coefficient(variable, coefficient);
    expr
  }

  pub fn from_constant_and_terms(constant: Scalar, terms: BTreeMap<String, Scalar>) -> LinearExpression {
    LinearExpression{
      constant: constant,
      terms: terms
    }
  }

  pub fn get_coefficient<'s, 'v>(&'s self, v: &String) -> Scalar {
    self.terms.get(v).map(|t| *t).unwrap_or(0.0)
  }

  pub fn set_coefficient(&mut self, v: String, coefficient: Scalar) {
    self.terms.insert(v, coefficient);
  }

  pub fn get_constant(&self) -> Scalar {
    self.constant
  }

  pub fn set_constant(&mut self, constant: Scalar) {
    self.constant = constant;
  }

  pub fn times(&self, constant: Scalar) -> LinearExpression {
    LinearExpression::from_constant_and_terms(self.constant * constant,
                                              self.terms.iter().map(|(name, scalar)| {
                                                (name.clone(), scalar * constant)
                                              }).collect())
  }

  pub fn plus_this(&mut self, expr: &LinearExpression) {
    self.coefficient_merge(expr, |a, b| a + b);
  }

  pub fn times_this(&mut self, scalar: Scalar) {
    if approx_eq(0.0, scalar) {
      self.terms.clear();
      self.constant = 0.0;
    } else {
      self.coefficient_transform(|a| a * scalar);
    }
  }

  pub fn div_this(&mut self, scalar: Scalar) {
    assert!(!approx_eq(0.0, scalar));
    self.coefficient_transform(|a| a / scalar);
  }
  
  pub fn plus(&self, expr: &LinearExpression) -> LinearExpression {
    let mut result = self.clone();
    result.plus_this(expr);
    result
  }

  pub fn minus(&self, expr: &LinearExpression) -> LinearExpression {
    let mut new = expr.times(-1.0);
    new.plus_this(self);
    new
  }

  pub fn minus_this(&mut self, expr: &LinearExpression) {
    self.coefficient_merge(expr, |a, b| a - b);
  }

  pub fn div(&self, constant: Scalar) -> LinearExpression {
    assert!(!approx_eq(constant, 0.0));
    self.times(1.0 / constant)
  }

  pub fn substitute(&mut self, var: &String, e: &LinearExpression) {
    let o_coef = self.terms.remove(var);
    if o_coef.is_none() {
      return;
    }
    let coef = o_coef.unwrap();
    self.plus_this(&e.times(coef));
  }
}

impl<'a> From<&'a str> for LinearExpression {
  fn from(var: &'a str) -> LinearExpression {
    LinearExpression::from(String::from(var))
  }
}

impl<'a> From<String> for LinearExpression {
  fn from(var: String) -> LinearExpression {
    LinearExpression::term(var, 1.0)
  }
}

impl From<Scalar> for LinearExpression {
  fn from(constant: Scalar) -> LinearExpression {
    LinearExpression{
      constant: constant,
      terms: BTreeMap::new()
    }
  }
}

impl Display for LinearExpression {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let mut terms: Vec<String> = vec![];
    for (name, coeff) in self.terms.iter() {
      if approx_eq(1.0, *coeff) {
        terms.push(name.clone());
      } else {
        terms.push(format!("{}{}", coeff, name));
      }
    }
    if !approx_eq(0.0, self.constant) || terms.is_empty() {
      terms.push(format!("{}", self.constant));
    }
    fmt.write_str(terms.join(" + ").as_ref() as &str)
  }
}

#[cfg(test)]
mod test {
  mod linear_expression {
    use super::super::*;
    #[test]
    fn to_string() {
      assert_eq!("0", LinearExpression::new().to_string().as_ref() as &str)
    }

    #[test]
    fn get_and_set_coefficients() {
      let mut expr = LinearExpression::new();
      let var = String::from("x");
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
      let x1 = String::from("x1");
      let x2 = String::from("x2");
      
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

      let x1 = String::from("x1");
      let x2 = String::from("x2");
      let x3 = String::from("x3");

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
  }
}
