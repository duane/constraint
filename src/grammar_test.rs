#[cfg(test)]
mod tests {
  use expr::*;
  use grammar;
  use var::Var;
  use std::str::FromStr;

  fn parse_scalar_lalr() {
    assert!(approx_eq(0.0, grammar::parse_Scalar("0.0").unwrap()));
    assert!(approx_eq(0.0, grammar::parse_Scalar("-0.0").unwrap()));
    assert!(approx_eq(0.0, grammar::parse_Scalar("0.").unwrap()));
    assert!(approx_eq(0.0, grammar::parse_Scalar("-0.").unwrap()));

    assert!(approx_eq(42.3, grammar::parse_Scalar("42.3").unwrap()));
    assert!(approx_eq(-42.3, grammar::parse_Scalar("-42.3").unwrap()));
    assert!(approx_eq(7.0, grammar::parse_Scalar("7.").unwrap()));
    assert!(approx_eq(-7.0, grammar::parse_Scalar("-7.").unwrap()));
  }

  #[test]
  fn parse_lalr_identifiers() {
    assert_eq!(&String::from("x"), grammar::parse_Variable("x").unwrap().name());
    assert_eq!(&String::from("_"), grammar::parse_Variable("_").unwrap().name());
    assert_eq!(&String::from("x2"), grammar::parse_Variable("x2").unwrap().name());
    assert!(grammar::parse_Variable("9").is_err());
  }

  #[test]
  fn parse_lalr_exprs() {
    let expr = grammar::parse_Expression("-42.3 x4 + 92 +-92.x4+0.0x2-92.3x3-82").unwrap();
    let x4_ref = Var::external(String::from("x4"));
    let x2_ref = Var::external(String::from("x2"));
    let x3_ref = Var::external(String::from("x3"));
    assert!(approx_eq(-134.3, expr.get_coefficient(&x4_ref)));
    assert!(approx_eq(0.0, expr.get_coefficient(&x2_ref)));
    assert!(approx_eq(-92.3, expr.get_coefficient(&x3_ref)));
    assert!(approx_eq(10.0, expr.get_constant()));
  }

  #[test]
  fn parse_minus() {
    RawLinearExpression::from_str("x-y").expect("Basic subtraction");
    RawLinearExpression::from_str("-x").expect("Basic unary negation");
    RawLinearExpression::from_str("-x-y").expect("Combined unary negation and subtraction");
    RawLinearExpression::from_str("x--y").expect("Combined subtraction and unary negation");
  }

  #[test]
  fn parse_lalr_relations() {
    assert_eq!(Relation::EQ, grammar::parse_Relation("=").unwrap());
    assert_eq!(Relation::NEQ, grammar::parse_Relation("!=").unwrap());
    assert_eq!(Relation::LT, grammar::parse_Relation("<").unwrap());
    assert_eq!(Relation::LEQ, grammar::parse_Relation("<=").unwrap());
    assert_eq!(Relation::GT, grammar::parse_Relation(">").unwrap());
    assert_eq!(Relation::GEQ, grammar::parse_Relation(">=").unwrap());
  }

  #[test]
  fn parse_lalr_linear_relations() {
    let gt = grammar::parse_LinearRelation("3x1>2x2").unwrap();
    assert!(gt.op == Relation::GT);
    let eq = grammar::parse_LinearRelation("-8.2x+5=8.2y+5").unwrap();
    assert!(eq.op == Relation::EQ);
    let geq = grammar::parse_LinearRelation("2x + 3x >= 5x").unwrap();
    assert!(geq.op == Relation::GEQ);
    let leq = grammar::parse_LinearRelation("2x <= 3x").unwrap();
    assert!(leq.op == Relation::LEQ);
    let lt = grammar::parse_LinearRelation("4y<5y2").unwrap();
    assert!(lt.op == Relation::LT);
    let neq = grammar::parse_LinearRelation("x != y").unwrap();
    assert!(neq.op == Relation::NEQ);
  }
}
