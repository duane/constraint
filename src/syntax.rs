use std::str::FromStr;
use std::cmp::Ordering;
use std::str::Chars;

pub struct ProblemParser {
  buf: String
}

#[derive(Eq,PartialEq)]
enum FloatParseState {
  PreHyphen,
  Left,
  Right
}

use expr::*;
use std::iter::Peekable;

#[derive(Debug)]
pub struct ParserError {
  source: Option<(usize, usize, usize)>,
  message: String
}

pub struct ParserStream<'a> {
  internal_stream: Peekable<Chars<'a>>,
  col_pos: usize,
  line_pos: usize
}

impl<'s> ParserStream<'s> {
  fn new<'a>(buffer: &'a String) -> ParserStream<'a> {
    ParserStream{
      internal_stream: buffer.chars().peekable(),
      col_pos: 0,
      line_pos: 0
    }
  }
  
  fn next(&mut self) -> Option<char> {
    let o_char = self.internal_stream.next();
    match o_char {
      Some(c) if c == '\n' => {
        self.col_pos = 0;
        self.line_pos += 1;
      },
      Some(c) => {
        self.col_pos += 1;
      },
      None => return None
    }
    o_char
  }

  fn peek(&mut self) -> Option<char> {
    self.internal_stream.peek().map(|c|{*c})
  }

  fn error_here(&self, msg: &str) -> ParserError {
    ParserError{
      source: Some((self.line_pos, self.col_pos, self.col_pos)),
      message: String::from(msg)
    }
  }

  fn error_at(&self, start: usize, end: usize, msg: &str) -> ParserError {
    assert!(end >= start);
    ParserError {
      source: Some((self.line_pos, start, end)),
      message: String::from(msg)
    }
  }

  fn get_col_pos(&self) -> usize {
    self.col_pos
  }

  fn is_digit(&self, c: char) -> bool {
    c >= '0' && c <= '9'
  }

  fn is_identifier(&self, c: char, initial: bool) -> bool {
    (c >= 'a' && c <= 'z') ||
      (c >= 'A' && c <= 'Z') ||
      c == '_' ||
      (c >= '0' && c <= '9' && !initial)
  }

  fn is_white(&self, c: char, include_newline: bool) -> bool {
    c == ' ' ||
      c == '\t' ||
      ((c == '\n' || c == '\r') && include_newline)
  }

  fn is_operator(&self, c: char) -> bool {
    c == '<' ||
      c == '=' ||
      c == '!' ||
      c == '>'
  }

  fn skip_whitespace(&mut self) {
    let mut peek_char = self.peek();
    while peek_char.is_some() && self.is_white(peek_char.unwrap(), false) {
      let _ = self.next();
      peek_char = self.peek();
    }
  }
}

impl ProblemParser {
  fn unexpected_character_error(c: char, error_while: &str) -> String {
    format!("Unexpected character while parsing {}: `{}'", error_while, c)
  }

  fn parse_add<'i, 's>(input: &'i mut ParserStream<'s>) -> Result<Option<LinearExpression>, ParserError> {
    input.skip_whitespace();
    let peek_char = input.peek();
    let bookmark = input.get_col_pos();
    if peek_char == Some('+') {
      let _ = input.next();
      input.skip_whitespace();
      match ProblemParser::parse_term(input).unwrap() {
        Some(expr) => Ok(Some(expr)),
        None => Err(input.error_at(bookmark, bookmark, "Could not find a term following '+'"))
      }
    } else {
      Ok(None)
    }
  }

  fn parse_expr<'i, 's>(input: &'i mut ParserStream<'s>) -> Result<Option<LinearExpression>, ParserError> {
    input.skip_whitespace();
    let initial_term = ProblemParser::parse_term(input).unwrap();
    if initial_term.is_none() {
      return Ok(None);
    }
    let mut expr = initial_term.unwrap();
    let mut add_term = ProblemParser::parse_add(input).unwrap();
    while add_term.is_some() {
      expr = expr.plus(&add_term.unwrap());
      add_term = ProblemParser::parse_add(input).unwrap();
    }
    Ok(Some(expr))
  }
  
  fn parse_relation<'i, 's>(input: &'i mut ParserStream<'s>) -> Result<Option<Relation>, ParserError> {
    let bookmark = input.get_col_pos();
    let mut peek_char = input.peek();
    if peek_char == Some('<') {
      let _ = input.next();
      let next_char = input.peek();
      if next_char == Some('=') {
        let _ = input.next();
        Ok(Some(Relation::LEQ))
      } else {
        Ok(Some(Relation::LT))
      }
    } else if peek_char == Some('>') {
      let _ = input.next();
      let next_char = input.peek();
      if next_char == Some('=') {
        let _ = input.next();
        Ok(Some(Relation::GEQ))
      } else {
        Ok(Some(Relation::GT))
      }
    } else if peek_char == Some('=') {
      let _ = input.next();
      let next_char = input.peek();
      if next_char == Some('=') {
        let _ = input.next();
        Ok(Some(Relation::EQ))
      } else if next_char == Some('!') {
        let _ = input.next();
        let third_char = input.peek();
        if third_char == Some('=') {
          let _ = input.next();
          Ok(Some(Relation::NEQ))
        } else {
          Err(input.error_at(bookmark, bookmark, "=! is not a valid relation"))
        }
      } else {
        Err(input.error_at(bookmark, bookmark, "= is not a valid relation"))
      }
    } else {
      Ok(None)
    }
  }

  fn parse_linear_relation<'i, 's>(input: &'i mut ParserStream<'s>) -> Result<Option<LinearRelation>, ParserError> {
    input.skip_whitespace();
    let lhs = ProblemParser::parse_expr(input).unwrap();
    if lhs.is_none() {
      return Ok(None);
    }
    let relation_bookmark = input.get_col_pos();
    let relation = ProblemParser::parse_relation(input).unwrap();
    if relation.is_none() {
      return Err(input.error_at(relation_bookmark, relation_bookmark, "Expected a relation, found an expression"));
    }
    let rhs_bookmark = input.get_col_pos();
    let rhs = ProblemParser::parse_expr(input).unwrap();
    if rhs.is_none() {
      return Err(input.error_at(rhs_bookmark, rhs_bookmark, "Expected the right hand side of the expression"))
    }
    Ok(Some(LinearRelation::new(lhs.unwrap(), relation.unwrap(), rhs.unwrap())))
  }
  
  fn parse_scalar<'i, 's>(input: &'i mut ParserStream<'s>) -> Result<Option<Scalar>, ParserError> {
    let mut state = FloatParseState::PreHyphen;
    let mut buf = String::new();
    let mut peek_char = input.peek();
    while peek_char.is_some() {
      match peek_char {
        Some(c @ '-') => {
          if state != FloatParseState::PreHyphen {
            break
          }
          state = FloatParseState::Left;
          buf.push(input.next().unwrap());
        }
        Some(d) if input.is_digit(d) => {
          if state == FloatParseState::PreHyphen {
            state = FloatParseState::Left;
          }
          buf.push(input.next().unwrap());
        }
        Some(c @ '.') => {
          if state == FloatParseState::PreHyphen || state == FloatParseState::Left {
            buf.push(input.next().unwrap());
            state = FloatParseState::Right;
          } else {
            break
          }
        }
        _ => break
      }
      peek_char = input.peek();
    }
    if buf.is_empty() {
      Ok(None)
    } else {
      Ok(Some(f64::from_str(buf.as_ref() as &str).unwrap()))
    }
  }

  fn parse_identifier<'i, 's>(input: &'i mut ParserStream<'s>) -> Result<Option<Variable>, ParserError> {
    let mut buf = String::new();
    let mut peek_char = input.peek();
    
    loop {
      match peek_char {
        Some(c) if input.is_identifier(c, buf.is_empty()) => buf.push(input.next().unwrap()),
        _ => break
      }
      peek_char = input.peek();
    }

    if buf.is_empty() {
      Ok(None)
    } else {
      Ok(Some(Variable::from(buf)))
    }
  }

  fn parse_term<'i, 's>(input: &'i mut ParserStream<'s>) -> Result<Option<LinearExpression>, ParserError> {
    let coef = ProblemParser::parse_scalar(input).unwrap();
    input.skip_whitespace();
    let var = ProblemParser::parse_identifier(input).unwrap();
    match (coef, var) {
      (Some(scalar), Some(var)) => Ok(Some(LinearExpression::term(var, scalar))),
      (Some(scalar), None) => Ok(Some(LinearExpression::from(scalar))),
      (None, Some(var)) => Ok(Some(LinearExpression::from(var))),
      (None, None) => Ok(None)
    }
  }
}


mod test {
  use expr::{approx_eq, LinearExpression, LinearRelation, Relation, Scalar, Variable};
  use super::{ParserError, ParserStream, ProblemParser};

  fn parse_scalar(s: &str) -> Result<Option<Scalar>, ParserError> {
    ProblemParser::parse_scalar(&mut ParserStream::new(&String::from(s)))
  }

  fn parse_identifier(s: &str) -> Result<Option<Variable>, ParserError> {
    ProblemParser::parse_identifier(&mut ParserStream::new(&String::from(s)))
  }

  fn parse_term(s: &str) -> Result<Option<LinearExpression>, ParserError> {
    ProblemParser::parse_term(&mut ParserStream::new(&String::from(s)))
  }

  fn parse_expr(s: &str) -> Result<Option<LinearExpression>, ParserError> {
    ProblemParser::parse_expr(&mut ParserStream::new(&String::from(s)))
  }

  fn parse_relation(s: &str) -> Result<Option<Relation>, ParserError> {
    ProblemParser::parse_relation(&mut ParserStream::new(&String::from(s)))
  }

  fn parse_linear_relation(s: &str) -> Result<Option<LinearRelation>, ParserError> {
    ProblemParser::parse_linear_relation(&mut ParserStream::new(&String::from(s)))
  }

  #[test]
  fn parse_zeroes() {
    assert!(approx_eq(0.0, parse_scalar("0.0").unwrap().unwrap()));
    assert!(approx_eq(0.0, parse_scalar("-0.0").unwrap().unwrap()));
    assert!(approx_eq(0.0, parse_scalar("0.").unwrap().unwrap()));
    assert!(approx_eq(0.0, parse_scalar("-0.").unwrap().unwrap()));
    assert!(approx_eq(0.0, parse_scalar(".0").unwrap().unwrap()));
    assert!(approx_eq(0.0, parse_scalar("-.0").unwrap().unwrap()));
  }

  #[test]
  fn parse_scalars() {
    assert!(approx_eq(42.3, parse_scalar("42.3").unwrap().unwrap()));
    assert!(approx_eq(-42.3, parse_scalar("-42.3").unwrap().unwrap()));
    assert!(approx_eq(8.0, parse_scalar("8.").unwrap().unwrap()));
    assert!(approx_eq(-8.0, parse_scalar("-8.").unwrap().unwrap()));
    assert!(approx_eq(0.9241, parse_scalar(".9241").unwrap().unwrap()));
    assert!(approx_eq(-0.9241, parse_scalar("-.9241").unwrap().unwrap()));

    assert!(parse_scalar("").unwrap().is_none());
    assert!(parse_scalar("y").unwrap().is_none());
  }

  #[test]
  fn parse_identifiers() {
    assert_eq!(Variable::from("x"), parse_identifier("x").unwrap().unwrap());
    assert_eq!(Variable::from("_"), parse_identifier("_").unwrap().unwrap());
    assert!(parse_identifier("9").unwrap().is_none());
    assert_eq!(Variable::from("x2"), parse_identifier("x2").unwrap().unwrap());
  }

  #[test]
  fn parse_terms() {
    let term1 = parse_term("2x").unwrap().unwrap();
    assert!(approx_eq(2.0, term1.get_coefficient(&Variable::from("x"))));
    assert!(approx_eq(0.0, term1.get_constant()));

    let term2 = parse_term("-43.x2").unwrap().unwrap();
    assert!(approx_eq(-43.0, term2.get_coefficient(&Variable::from("x2"))));
    assert!(approx_eq(0.0, term2.get_constant()));

    let term3 = parse_term("y").unwrap().unwrap();
    assert!(approx_eq(1.0, term3.get_coefficient(&Variable::from("y"))));
    assert!(approx_eq(0.0, term3.get_constant()));

    let term4 = parse_term(".4").unwrap().unwrap();
    assert!(approx_eq(0.4, term4.get_constant()));

    let term5 = parse_term("-72.3 x3").unwrap().unwrap();
    assert!(approx_eq(-72.3, term5.get_coefficient(&Variable::from("x3"))));
  }

  #[test]
  fn parse_exprs() {
    let expr = parse_expr("-42.3 x4 + 92 +-92.x4+0.0x2+-92.3x3+-82").unwrap().unwrap();
    assert!(approx_eq(-134.3, expr.get_coefficient(&Variable::from("x4"))));
    assert!(approx_eq(0.0, expr.get_coefficient(&Variable::from("x2"))));
    assert!(approx_eq(-92.3, expr.get_coefficient(&Variable::from("x3"))));
    assert!(approx_eq(10.0, expr.get_constant()));
  }

  #[test]
  fn parse_relations() {
    assert_eq!(Some(Relation::EQ), parse_relation("==").unwrap());
    assert_eq!(Some(Relation::NEQ), parse_relation("=!=").unwrap());
    assert_eq!(Some(Relation::LT), parse_relation("<").unwrap());
    assert_eq!(Some(Relation::LEQ), parse_relation("<=").unwrap());
    assert_eq!(Some(Relation::GT), parse_relation(">").unwrap());
    assert_eq!(Some(Relation::GEQ), parse_relation(">=").unwrap());
    assert!(parse_relation("foo").unwrap().is_none());
  }

  #[test]
  fn parse_linear_relations() {
    let gt = parse_linear_relation("3x1>2x2").unwrap().unwrap();
    assert!(gt.op == Relation::GT);
    let eq = parse_linear_relation("-8.2x+5==8.2y+5").unwrap().unwrap();
    assert!(eq.op == Relation::EQ);
    let geq = parse_linear_relation("2x + 3x >= 5x").unwrap().unwrap();
    assert!(geq.op == Relation::GEQ);
    let leq = parse_linear_relation("2x <= 3x").unwrap().unwrap();
    assert!(leq.op == Relation::LEQ);
    let lt = parse_linear_relation("4y<5y2").unwrap().unwrap();
    assert!(lt.op == Relation::LT);
    let neq = parse_linear_relation("x =!= y").unwrap().unwrap();
    assert!(neq.op == Relation::NEQ);
  }
}
