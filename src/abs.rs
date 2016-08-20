use expr::*;
use var::*;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Display, Error, Formatter};
use std::string::ToString;
use std::hash::Hash;

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
  ///   use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  /// use constraint::abs::LinearExpression;
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
  ///   use constraint::abs::InternedLinearExpression;
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
    let (defined, undefined) = self.terms.iter().map(|(k,v)|(k.clone(), *v)).partition(|&(ref k,v)|bindings.contains_key(k));
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
  ///   use constraint::abs::InternedLinearExpression;
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

pub type InternedLinearExpression = LinearExpression<VarRef>;

impl From<VarRef> for InternedLinearExpression {
  fn from(var: VarRef) -> InternedLinearExpression {
    LinearExpression::term(var, 1.0)
  }
}

pub type RawLinearExpression = LinearExpression<Var>;

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