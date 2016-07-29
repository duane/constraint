#[cfg(test)]
mod tests {
  use expr::*;
  use grammar;
  use var::{VarIndex, VarRef};

  fn parse_scalar_lalr() {
    assert!(approx_eq(0.0, grammar::parse_Scalar(&mut VarIndex::new(), "0.0").unwrap()));
    assert!(approx_eq(0.0, grammar::parse_Scalar(&mut VarIndex::new(), "-0.0").unwrap()));
    assert!(approx_eq(0.0, grammar::parse_Scalar(&mut VarIndex::new(), "0.").unwrap()));
    assert!(approx_eq(0.0, grammar::parse_Scalar(&mut VarIndex::new(), "-0.").unwrap()));

    assert!(approx_eq(42.3, grammar::parse_Scalar(&mut VarIndex::new(), "42.3").unwrap()));
    assert!(approx_eq(-42.3, grammar::parse_Scalar(&mut VarIndex::new(), "-42.3").unwrap()));
    assert!(approx_eq(7.0, grammar::parse_Scalar(&mut VarIndex::new(), "7.").unwrap()));
    assert!(approx_eq(-7.0, grammar::parse_Scalar(&mut VarIndex::new(), "-7.").unwrap()));
  }

  #[test]
  fn parse_lalr_identifiers() {
    let mut index = VarIndex::new();
    assert_eq!(&String::from("x"), grammar::parse_Variable(&mut index, "x").unwrap().name());
    assert_eq!(&String::from("_"), grammar::parse_Variable(&mut index, "_").unwrap().name());
    assert_eq!(&String::from("x2"), grammar::parse_Variable(&mut index, "x2").unwrap().name());
    assert!(grammar::parse_Variable(&mut index, "9").is_err());
  }

  #[test]
  fn parse_lalr_terms() {
    let mut index = VarIndex::new();
    let term1 = grammar::parse_Term(&mut index, "2x").unwrap();
    assert!(approx_eq(2.0, term1.0.unwrap()));
    assert_eq!(&String::from("x"), term1.1.unwrap().name());

    let term2 = grammar::parse_Term(&mut index, "-43.x2").unwrap();
    assert!(approx_eq(-43.0, term2.0.unwrap()));
    assert_eq!(&String::from("x2"), term2.1.unwrap().name());

    let term3 = grammar::parse_Term(&mut index, "y").unwrap();
    assert!(term3.0.is_none());
    assert_eq!(&String::from("y"), term3.1.unwrap().name());

    let term4 = grammar::parse_Term(&mut index, "0.4").unwrap();
    assert!(approx_eq(0.4, term4.0.unwrap()));
    assert!(term4.1.is_none());

    let term5 = grammar::parse_Term(&mut index, "-72.3 x3").unwrap();
    assert!(approx_eq(-72.3, term5.0.unwrap()));
    assert_eq!(&String::from("x3"), term5.1.unwrap().name());

    let term6 = grammar::parse_Term(&mut index, "-72.3*x3").unwrap();
    assert!(approx_eq(-72.3, term6.0.unwrap()));
    assert_eq!(&String::from("x3"), term6.1.unwrap().name());
  }

  #[test]
  fn parse_lalr_exprs() {
    let mut index = VarIndex::new();
    let expr = grammar::parse_Expression(&mut index, "-42.3 x4 + 92 +-92.x4+0.0x2+-92.3x3+-82").unwrap();
    let x4_var: VarRef = expr.terms().keys().find(|s|s.name() == &String::from("x4")).unwrap().clone();
    println!("{:?}", x4_var);
    println!("{:?}", expr.get_coefficient(&(&mut index).get(x4_var.name()).unwrap()));
    assert!(approx_eq(-134.3, expr.get_coefficient(&(&mut index).get(&String::from("x4")).expect("x4 must exist in index"))));
    assert!(approx_eq(0.0, expr.get_coefficient(&(&mut index).get(&String::from("x2")).expect("x2 must exist in index"))));
    assert!(approx_eq(-92.3, expr.get_coefficient(&(&mut index).get(&String::from("x3")).expect("x3 must exist in index"))));
    assert!(approx_eq(10.0, expr.get_constant()));
  }

  #[test]
  fn parse_lalr_relations() {
    let mut index = VarIndex::new();
    assert_eq!(Relation::EQ, grammar::parse_Relation(&mut index, "==").unwrap());
    assert_eq!(Relation::NEQ, grammar::parse_Relation(&mut index, "=!=").unwrap());
    assert_eq!(Relation::LT, grammar::parse_Relation(&mut index, "<").unwrap());
    assert_eq!(Relation::LEQ, grammar::parse_Relation(&mut index, "<=").unwrap());
    assert_eq!(Relation::GT, grammar::parse_Relation(&mut index, ">").unwrap());
    assert_eq!(Relation::GEQ, grammar::parse_Relation(&mut index, ">=").unwrap());
  }

  #[test]
  fn parse_lalr_linear_relations() {
    let mut index = VarIndex::new();
    let gt = grammar::parse_LinearRelation(&mut index, "3x1>2x2").unwrap();
    assert!(gt.op == Relation::GT);
    let eq = grammar::parse_LinearRelation(&mut index, "-8.2x+5==8.2y+5").unwrap();
    assert!(eq.op == Relation::EQ);
    let geq = grammar::parse_LinearRelation(&mut index, "2x + 3x >= 5x").unwrap();
    assert!(geq.op == Relation::GEQ);
    let leq = grammar::parse_LinearRelation(&mut index, "2x <= 3x").unwrap();
    assert!(leq.op == Relation::LEQ);
    let lt = grammar::parse_LinearRelation(&mut index, "4y<5y2").unwrap();
    assert!(lt.op == Relation::LT);
    let neq = grammar::parse_LinearRelation(&mut index, "x =!= y").unwrap();
    assert!(neq.op == Relation::NEQ);
  }
}
