use std::collections::HashMap;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::iter::Iterator;
use std::cmp::{Ord, Ordering};
use std::fmt::{Display, Formatter, Error};

pub type Scalar = f64;
const SCALAR_EPSILON: Scalar = 0.000001;

pub fn approx_eq(a: Scalar, b: Scalar) -> bool {
  let delta = (a - b).abs();
  delta < SCALAR_EPSILON
}

#[derive(Debug,Eq,PartialEq,Clone,Copy)]
pub enum Relation {
  EQ,
  NEQ,
  GT,
  LT,
  GEQ,
  LEQ
}

impl Relation {
  fn reverse(self) -> Relation {
    match self {
      Relation::EQ => Relation::NEQ,
      Relation::NEQ => Relation::EQ,
      Relation::GT => Relation::LEQ,
      Relation::LEQ => Relation::GT,
      Relation::LT => Relation::GEQ,
      Relation::GEQ => Relation::LT
    }
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
}

impl Display for LinearRelation {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    fmt.write_str(format!("{} {} {}",
                          self.lhs,
                          self.op,
                          self.rhs).as_ref() as &str)
  }
}

#[derive(Debug,Hash,Eq,PartialEq,PartialOrd,Clone)]
pub struct Variable {
  pub name: String
}

impl<'a> From<&'a str> for Variable {
  fn from(name: &'a str) -> Variable {
    Variable{
      name: String::from(name)
    }
  }
}

impl From<String> for Variable {
  fn from(name: String) -> Variable {
    Variable{
      name: name
    }
  }
}

impl Display for Variable {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    fmt.write_str(self.name.as_ref() as &str)
  }
}

impl Ord for Variable {
  fn cmp(&self, other: &Self) -> Ordering {
    self.name.cmp(&(other.name))
  }
}

///LinearExpression contains ax+by+...+c

#[derive(Clone)]
pub struct LinearExpression {
  constant: Scalar,
  terms: BTreeMap<Variable, Scalar>,
}

impl LinearExpression {
  pub fn new() -> LinearExpression {
    LinearExpression{
      constant: 0.0,
      terms: BTreeMap::new()
    }
  }

  pub fn term(variable: Variable, coefficient: Scalar) -> LinearExpression {
    let mut expr = LinearExpression::new();
    expr.set_coefficient(variable, coefficient);
    expr
  }

  pub fn from_constant_and_terms(constant: Scalar, terms: BTreeMap<Variable, Scalar>) -> LinearExpression {
    LinearExpression{
      constant: constant,
      terms: terms
    }
  }

  pub fn get_coefficient(&self, v: &Variable) -> Scalar {
    self.terms.get(v).map(|t|{*t}).unwrap_or(0.0)
  }

  pub fn set_coefficient(&mut self, v: Variable, coefficient: Scalar) {
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

  pub fn plus(&self, expr: &LinearExpression) -> LinearExpression {
    let mut new_terms = self.terms.clone();

    for (name, coef) in expr.terms.iter() {
      let existing_coef = new_terms.get(name).map(|c|{*c}).unwrap_or(0.0);
      let _ = new_terms.insert(name.clone(), existing_coef + coef);
    }

    LinearExpression::from_constant_and_terms(self.constant + expr.constant,
                                              new_terms)
  }

  pub fn minus(&self, expr: &LinearExpression) -> LinearExpression {
    self.plus(&expr.times(-1.0))
  }

  pub fn div(&self, constant: Scalar) -> LinearExpression {
    assert!(!approx_eq(constant, 0.0));
    self.times(1.0 / constant)
  }
}

impl From<Variable> for LinearExpression {
  fn from(var: Variable) -> LinearExpression {
    LinearExpression::term(var, 1.0)
  }
}

impl<'a> From<&'a str> for LinearExpression {
  fn from(var: &'a str) -> LinearExpression {
    LinearExpression::from(Variable::from(String::from(var)))
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
    let mut terms = vec![];
    for (ref name, coeff) in self.terms.iter() {
      if approx_eq(1.0, *coeff) {
        terms.push(name.name.clone());
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
      let var = Variable::from(String::from("x"));
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
      let x1 = Variable::from(String::from("x1"));
      let x2 = Variable::from(String::from("x2"));
      
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

      let x1 = Variable::from(String::from("x1"));
      let x2 = Variable::from(String::from("x2"));
      let x3 = Variable::from(String::from("x3"));

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
