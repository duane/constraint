use expr::*;

use std::fmt::{Display, Error, Formatter};

///LinearInequality, LinearEquality combine LinearExpression with an operation.

pub struct LinearEquality {
  // normalized to equal zero
  expr: LinearExpression
}

impl LinearEquality {
  pub fn new() -> LinearEquality {
    LinearEquality{
      expr: LinearExpression::new()
    }
  }

  pub fn zero(expr: &LinearExpression) -> LinearEquality {
    LinearEquality{
      expr: expr.clone()
    }
  }

  pub fn equals(lhs: &LinearExpression, rhs: &LinearExpression) -> LinearEquality {
    LinearEquality{
      expr: rhs.minus(lhs)
    }
  }
}

impl Display for LinearEquality {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    fmt.write_str(format!("0 {} {}", Relation::EQ, self.expr).as_ref() as &str)
  }
}

pub struct LinearInequality {
  op: Relation,
  expr: LinearExpression
}

impl LinearInequality {
  pub fn new(expr1: &LinearExpression, op: Relation, expr2: &LinearExpression) -> LinearInequality {
    match op {
      Relation::GEQ | Relation::GT => {
        LinearInequality{
          op: op,
          expr: expr1.minus(expr2)
        }
      }
      Relation::LEQ | Relation::LT => {
        LinearInequality{
          op: op,
          expr: expr2.minus(expr1)
        }
      }
      _ => panic!("not a valid inequality: {:?}", op)
    }
  }
}

impl Display for LinearInequality {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    fmt.write_str(format!("0 {} {}", self.op, self.expr).as_ref() as &str)
  }
}

pub struct LinearBounds {
  variable: Variable,
  lower: Option<Scalar>,
  upper: Option<Scalar>
}

impl LinearBounds {
  pub fn free(var: Variable) -> LinearBounds {
    LinearBounds{
      variable: var,
      lower: None,
      upper: None
    }
  }

  pub fn lower(var: Variable, lower: Scalar) -> LinearBounds {
    LinearBounds{
      variable: var,
      lower: Some(lower),
      upper: None
    }
  }

  pub fn upper(var: Variable, upper: Scalar) -> LinearBounds {
    LinearBounds{
      variable: var,
      lower: None,
      upper: Some(upper)
    }
  }

  pub fn double(var: Variable, lower: Scalar, upper: Scalar) -> LinearBounds {
    assert!(lower <= upper);
    LinearBounds{
      variable: var,
      lower: Some(lower),
      upper: Some(upper)
    }
  }

  pub fn fixed(var: Variable, constant: Scalar) -> LinearBounds {
    LinearBounds{
      variable: var,
      lower: Some(constant),
      upper: Some(constant)
    }
  }
}

impl Display for LinearBounds {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let inf = String::from("\u{221E}");
    match (self.lower, self.upper) {
      (Some(lower), Some(upper)) => {
        if approx_eq(lower, upper) {
          fmt.write_str(format!("{} = {}", self.variable, lower).as_ref() as &str)
        } else {
          fmt.write_str(format!("{} {} {} {} {}",
                                lower,
                                Relation::LEQ,
                                self.variable,
                                Relation::LEQ,
                                upper).as_ref() as &str)
        }
      }
      (None, Some(upper)) => {
        fmt.write_str(format!("-{} {} {} {} {}",
                              inf,
                              Relation::LT,
                              self.variable,
                              Relation::LEQ,
                              upper).as_ref() as &str)
      }
      (Some(lower), None) => {
        fmt.write_str(format!("{} {} {} {} {}",
                              lower,
                              Relation::LEQ,
                              self.variable,
                              Relation::LT,
                              inf).as_ref() as &str)
      }
      (None, None) => {
        fmt.write_str(format!("-{} {} {} {} {}",
                              inf,
                              Relation::LT,
                              self.variable,
                              Relation::LT,
                              inf).as_ref() as &str)
      }
    }
  }
}
