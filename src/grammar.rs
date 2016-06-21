#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use expr::{Scalar, LinearExpression, LinearRelation, Relation};
use problem::{Problem, ProblemObjective};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expression {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expr::{Scalar, LinearExpression, LinearRelation, Relation};
    use problem::{Problem, ProblemObjective};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Expression<
        'input,
    >(
        input: &'input str,
    ) -> Result<LinearExpression, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Expression(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _22_2a_22_3f(::std::option::Option<&'input str>),
        _28LINE__SEP_2b_20LinearRelation_29((::std::vec::Vec<&'input str>, LinearRelation)),
        _28LINE__SEP_2b_20LinearRelation_29_2a(::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>),
        _28Term_20_22_2b_22_29(((Option<Scalar>, Option<String>), &'input str)),
        _28Term_20_22_2b_22_29_2a(::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>),
        Expression(LinearExpression),
        LINE__SEP(&'input str),
        LINE__SEP_2b(::std::vec::Vec<&'input str>),
        LinearRelation(LinearRelation),
        Problem(Problem),
        ProblemObjective(ProblemObjective),
        Relation(Relation),
        Scalar(Scalar),
        Term((Option<Scalar>, Option<String>)),
        Variable(String),
        ____Expression(LinearExpression),
        ____LINE__SEP(&'input str),
        ____LinearRelation(LinearRelation),
        ____Problem(Problem),
        ____ProblemObjective(ProblemObjective),
        ____Relation(Relation),
        ____Scalar(Scalar),
        ____Term((Option<Scalar>, Option<String>)),
        ____Variable(String),
    }

    // State 0
    //   (Term "+")* = (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (*) (Term "+")* Term [EOF]
    //   __Expression = (*) Expression [EOF]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //
    //   (Term "+")* -> S1
    //   Expression -> S2
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Expression(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   (Term "+") = (*) Term "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = (*) Term "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* (*) Term [EOF]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [EOF]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["*"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["+"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = (*) Scalar [EOF]
    //   Term = (*) Scalar ["+"]
    //   Term = (*) Scalar "*"? Variable [EOF]
    //   Term = (*) Scalar "*"? Variable ["+"]
    //   Term = (*) Variable [EOF]
    //   Term = (*) Variable ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Shift(S7)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S8)
    //
    //   (Term "+") -> S3
    //   Scalar -> S4
    //   Term -> S5
    //   Variable -> S6
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Scalar(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 2
    //   __Expression = Expression (*) [EOF]
    //
    //   EOF -> Reduce(__Expression = Expression => Call(ActionFn(3));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<LinearExpression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 3
    //   (Term "+")* = (Term "+")* (Term "+") (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (Term "+")* (Term "+") (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* = (Term "+")*, (Term "+") => Call(ActionFn(32));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* = (Term "+")*, (Term "+") => Call(ActionFn(32));)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
        __sym1: &mut Option<((Option<Scalar>, Option<String>), &'input str)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action32(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   "*"? = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   "*"? = (*) "*" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = Scalar (*) [EOF]
    //   Term = Scalar (*) ["+"]
    //   Term = Scalar (*) "*"? Variable [EOF]
    //   Term = Scalar (*) "*"? Variable ["+"]
    //
    //   EOF -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "*" -> Shift(S10)
    //   "+" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? =  => Call(ActionFn(35));)
    //
    //   "*"? -> S9
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, _), _)) => {
                let __nt = super::__action35(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt));
            }
            None |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_22_2a_22_3f(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 5
    //   (Term "+") = Term (*) "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = Term (*) "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* Term (*) [EOF]
    //
    //   EOF -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   "+" -> Shift(S11)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
        __sym1: &mut Option<(Option<Scalar>, Option<String>)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 6
    //   Term = Variable (*) [EOF]
    //   Term = Variable (*) ["+"]
    //
    //   EOF -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "+" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 7
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [EOF]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["*"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["+"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   EOF -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "*" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "+" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Scalar(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 8
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [EOF]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["+"]
    //
    //   EOF -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "+" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 9
    //   Term = Scalar "*"? (*) Variable [EOF]
    //   Term = Scalar "*"? (*) Variable ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S8)
    //
    //   Variable -> S12
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 10
    //   "*"? = "*" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? = "*" => Call(ActionFn(34));)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action34(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 11
    //   (Term "+") = Term "+" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = Term "+" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+") = Term, "+" => Call(ActionFn(33));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+") = Term, "+" => Call(ActionFn(33));)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(Option<Scalar>, Option<String>)>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action33(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //   Term = Scalar "*"? Variable (*) [EOF]
    //   Term = Scalar "*"? Variable (*) ["+"]
    //
    //   EOF -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "+" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
        __sym2: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Expression::parse_Expression;

mod __parse__LINE_SEP {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expr::{Scalar, LinearExpression, LinearRelation, Relation};
    use problem::{Problem, ProblemObjective};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_LINE_SEP<
        'input,
    >(
        input: &'input str,
    ) -> Result<&'input str, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____LINE__SEP(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _22_2a_22_3f(::std::option::Option<&'input str>),
        _28LINE__SEP_2b_20LinearRelation_29((::std::vec::Vec<&'input str>, LinearRelation)),
        _28LINE__SEP_2b_20LinearRelation_29_2a(::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>),
        _28Term_20_22_2b_22_29(((Option<Scalar>, Option<String>), &'input str)),
        _28Term_20_22_2b_22_29_2a(::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>),
        Expression(LinearExpression),
        LINE__SEP(&'input str),
        LINE__SEP_2b(::std::vec::Vec<&'input str>),
        LinearRelation(LinearRelation),
        Problem(Problem),
        ProblemObjective(ProblemObjective),
        Relation(Relation),
        Scalar(Scalar),
        Term((Option<Scalar>, Option<String>)),
        Variable(String),
        ____Expression(LinearExpression),
        ____LINE__SEP(&'input str),
        ____LinearRelation(LinearRelation),
        ____Problem(Problem),
        ____ProblemObjective(ProblemObjective),
        ____Relation(Relation),
        ____Scalar(Scalar),
        ____Term((Option<Scalar>, Option<String>)),
        ____Variable(String),
    }

    // State 0
    //   LINE_SEP = (*) ";" [EOF]
    //   __LINE_SEP = (*) LINE_SEP [EOF]
    //
    //   ";" -> Shift(S2)
    //
    //   LINE_SEP -> S1
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state2(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::LINE__SEP(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   __LINE_SEP = LINE_SEP (*) [EOF]
    //
    //   EOF -> Reduce(__LINE_SEP = LINE_SEP => Call(ActionFn(7));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____LINE__SEP(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   LINE_SEP = ";" (*) [EOF]
    //
    //   EOF -> Reduce(LINE_SEP = ";" => Call(ActionFn(24));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action24(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::LINE__SEP(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__LINE_SEP::parse_LINE_SEP;

mod __parse__LinearRelation {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expr::{Scalar, LinearExpression, LinearRelation, Relation};
    use problem::{Problem, ProblemObjective};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_LinearRelation<
        'input,
    >(
        input: &'input str,
    ) -> Result<LinearRelation, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____LinearRelation(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _22_2a_22_3f(::std::option::Option<&'input str>),
        _28LINE__SEP_2b_20LinearRelation_29((::std::vec::Vec<&'input str>, LinearRelation)),
        _28LINE__SEP_2b_20LinearRelation_29_2a(::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>),
        _28Term_20_22_2b_22_29(((Option<Scalar>, Option<String>), &'input str)),
        _28Term_20_22_2b_22_29_2a(::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>),
        Expression(LinearExpression),
        LINE__SEP(&'input str),
        LINE__SEP_2b(::std::vec::Vec<&'input str>),
        LinearRelation(LinearRelation),
        Problem(Problem),
        ProblemObjective(ProblemObjective),
        Relation(Relation),
        Scalar(Scalar),
        Term((Option<Scalar>, Option<String>)),
        Variable(String),
        ____Expression(LinearExpression),
        ____LINE__SEP(&'input str),
        ____LinearRelation(LinearRelation),
        ____Problem(Problem),
        ____ProblemObjective(ProblemObjective),
        ____Relation(Relation),
        ____Scalar(Scalar),
        ____Term((Option<Scalar>, Option<String>)),
        ____Variable(String),
    }

    // State 0
    //   (Term "+")* = (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (*) (Term "+")* Term ["<"]
    //   Expression = (*) (Term "+")* Term ["<="]
    //   Expression = (*) (Term "+")* Term ["=!="]
    //   Expression = (*) (Term "+")* Term ["=="]
    //   Expression = (*) (Term "+")* Term [">"]
    //   Expression = (*) (Term "+")* Term [">="]
    //   LinearRelation = (*) Expression Relation Expression [EOF]
    //   __LinearRelation = (*) LinearRelation [EOF]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //
    //   (Term "+")* -> S1
    //   Expression -> S2
    //   LinearRelation -> S3
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Expression(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::LinearRelation(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   (Term "+") = (*) Term "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = (*) Term "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* (*) Term ["<"]
    //   Expression = (Term "+")* (*) Term ["<="]
    //   Expression = (Term "+")* (*) Term ["=!="]
    //   Expression = (Term "+")* (*) Term ["=="]
    //   Expression = (Term "+")* (*) Term [">"]
    //   Expression = (Term "+")* (*) Term [">="]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["*"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["+"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["<"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["<="]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["=!="]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["=="]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [">"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [">="]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = (*) Scalar ["+"]
    //   Term = (*) Scalar ["<"]
    //   Term = (*) Scalar ["<="]
    //   Term = (*) Scalar ["=!="]
    //   Term = (*) Scalar ["=="]
    //   Term = (*) Scalar [">"]
    //   Term = (*) Scalar [">="]
    //   Term = (*) Scalar "*"? Variable ["+"]
    //   Term = (*) Scalar "*"? Variable ["<"]
    //   Term = (*) Scalar "*"? Variable ["<="]
    //   Term = (*) Scalar "*"? Variable ["=!="]
    //   Term = (*) Scalar "*"? Variable ["=="]
    //   Term = (*) Scalar "*"? Variable [">"]
    //   Term = (*) Scalar "*"? Variable [">="]
    //   Term = (*) Variable ["+"]
    //   Term = (*) Variable ["<"]
    //   Term = (*) Variable ["<="]
    //   Term = (*) Variable ["=!="]
    //   Term = (*) Variable ["=="]
    //   Term = (*) Variable [">"]
    //   Term = (*) Variable [">="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["<"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["<="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["=!="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["=="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [">"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [">="]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Shift(S8)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S9)
    //
    //   (Term "+") -> S4
    //   Scalar -> S5
    //   Term -> S6
    //   Variable -> S7
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Scalar(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 2
    //   LinearRelation = Expression (*) Relation Expression [EOF]
    //   Relation = (*) "<" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = (*) "<" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Relation = (*) "<=" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = (*) "<=" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Relation = (*) "=!=" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = (*) "=!=" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Relation = (*) "==" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = (*) "==" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Relation = (*) ">" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = (*) ">" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Relation = (*) ">=" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = (*) ">=" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   "<" -> Shift(S11)
    //   "<=" -> Shift(S12)
    //   "=!=" -> Shift(S13)
    //   "==" -> Shift(S14)
    //   ">" -> Shift(S15)
    //   ">=" -> Shift(S16)
    //
    //   Relation -> S10
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<LinearExpression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Relation(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 3
    //   __LinearRelation = LinearRelation (*) [EOF]
    //
    //   EOF -> Reduce(__LinearRelation = LinearRelation => Call(ActionFn(5));)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<LinearRelation>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____LinearRelation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   (Term "+")* = (Term "+")* (Term "+") (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (Term "+")* (Term "+") (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* = (Term "+")*, (Term "+") => Call(ActionFn(32));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* = (Term "+")*, (Term "+") => Call(ActionFn(32));)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
        __sym1: &mut Option<((Option<Scalar>, Option<String>), &'input str)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action32(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   "*"? = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   "*"? = (*) "*" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = Scalar (*) ["+"]
    //   Term = Scalar (*) ["<"]
    //   Term = Scalar (*) ["<="]
    //   Term = Scalar (*) ["=!="]
    //   Term = Scalar (*) ["=="]
    //   Term = Scalar (*) [">"]
    //   Term = Scalar (*) [">="]
    //   Term = Scalar (*) "*"? Variable ["+"]
    //   Term = Scalar (*) "*"? Variable ["<"]
    //   Term = Scalar (*) "*"? Variable ["<="]
    //   Term = Scalar (*) "*"? Variable ["=!="]
    //   Term = Scalar (*) "*"? Variable ["=="]
    //   Term = Scalar (*) "*"? Variable [">"]
    //   Term = Scalar (*) "*"? Variable [">="]
    //
    //   "*" -> Shift(S18)
    //   "+" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "<" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "<=" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "=!=" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "==" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   ">" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   ">=" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? =  => Call(ActionFn(35));)
    //
    //   "*"? -> S17
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, _), _)) => {
                let __nt = super::__action35(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt));
            }
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_22_2a_22_3f(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 6
    //   (Term "+") = Term (*) "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = Term (*) "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* Term (*) ["<"]
    //   Expression = (Term "+")* Term (*) ["<="]
    //   Expression = (Term "+")* Term (*) ["=!="]
    //   Expression = (Term "+")* Term (*) ["=="]
    //   Expression = (Term "+")* Term (*) [">"]
    //   Expression = (Term "+")* Term (*) [">="]
    //
    //   "+" -> Shift(S19)
    //   "<" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   "<=" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   "=!=" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   "==" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   ">" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   ">=" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
        __sym1: &mut Option<(Option<Scalar>, Option<String>)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 7
    //   Term = Variable (*) ["+"]
    //   Term = Variable (*) ["<"]
    //   Term = Variable (*) ["<="]
    //   Term = Variable (*) ["=!="]
    //   Term = Variable (*) ["=="]
    //   Term = Variable (*) [">"]
    //   Term = Variable (*) [">="]
    //
    //   "+" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "<" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "<=" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "=!=" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "==" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   ">" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   ">=" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 8
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["*"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["+"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["<"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["<="]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["=!="]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["=="]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [">"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [">="]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   "*" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "+" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "<" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "<=" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "=!=" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "==" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   ">" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   ">=" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Scalar(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 9
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["+"]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["<"]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["<="]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["=!="]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["=="]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [">"]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [">="]
    //
    //   "+" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "<" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "<=" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "=!=" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "==" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   ">" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   ">=" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 10
    //   (Term "+")* = (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (*) (Term "+")* Term [EOF]
    //   LinearRelation = Expression Relation (*) Expression [EOF]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //
    //   (Term "+")* -> S20
    //   Expression -> S21
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<LinearExpression>,
        __sym1: &mut Option<Relation>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Expression(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 11
    //   Relation = "<" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = "<" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(Relation = "<" => Call(ActionFn(15));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Relation = "<" => Call(ActionFn(15));)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //   Relation = "<=" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = "<=" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(Relation = "<=" => Call(ActionFn(16));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Relation = "<=" => Call(ActionFn(16));)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action16(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 13
    //   Relation = "=!=" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = "=!=" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(Relation = "=!=" => Call(ActionFn(20));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Relation = "=!=" => Call(ActionFn(20));)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action20(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 14
    //   Relation = "==" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = "==" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(Relation = "==" => Call(ActionFn(19));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Relation = "==" => Call(ActionFn(19));)
    //
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action19(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 15
    //   Relation = ">" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = ">" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(Relation = ">" => Call(ActionFn(17));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Relation = ">" => Call(ActionFn(17));)
    //
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action17(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 16
    //   Relation = ">=" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = ">=" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(Relation = ">=" => Call(ActionFn(18));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Relation = ">=" => Call(ActionFn(18));)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action18(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 17
    //   Term = Scalar "*"? (*) Variable ["+"]
    //   Term = Scalar "*"? (*) Variable ["<"]
    //   Term = Scalar "*"? (*) Variable ["<="]
    //   Term = Scalar "*"? (*) Variable ["=!="]
    //   Term = Scalar "*"? (*) Variable ["=="]
    //   Term = Scalar "*"? (*) Variable [">"]
    //   Term = Scalar "*"? (*) Variable [">="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["<"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["<="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["=!="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["=="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [">"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [">="]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S9)
    //
    //   Variable -> S22
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
    //   "*"? = "*" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? = "*" => Call(ActionFn(34));)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action34(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 19
    //   (Term "+") = Term "+" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = Term "+" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+") = Term, "+" => Call(ActionFn(33));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+") = Term, "+" => Call(ActionFn(33));)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(Option<Scalar>, Option<String>)>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action33(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 20
    //   (Term "+") = (*) Term "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = (*) Term "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* (*) Term [EOF]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [EOF]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["*"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["+"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = (*) Scalar [EOF]
    //   Term = (*) Scalar ["+"]
    //   Term = (*) Scalar "*"? Variable [EOF]
    //   Term = (*) Scalar "*"? Variable ["+"]
    //   Term = (*) Variable [EOF]
    //   Term = (*) Variable ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Shift(S26)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S27)
    //
    //   (Term "+") -> S4
    //   Scalar -> S23
    //   Term -> S24
    //   Variable -> S25
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state26(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Scalar(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 21
    //   LinearRelation = Expression Relation Expression (*) [EOF]
    //
    //   EOF -> Reduce(LinearRelation = Expression, Relation, Expression => Call(ActionFn(21));)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<LinearExpression>,
        __sym1: &mut Option<Relation>,
        __sym2: &mut Option<LinearExpression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action21(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::LinearRelation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 22
    //   Term = Scalar "*"? Variable (*) ["+"]
    //   Term = Scalar "*"? Variable (*) ["<"]
    //   Term = Scalar "*"? Variable (*) ["<="]
    //   Term = Scalar "*"? Variable (*) ["=!="]
    //   Term = Scalar "*"? Variable (*) ["=="]
    //   Term = Scalar "*"? Variable (*) [">"]
    //   Term = Scalar "*"? Variable (*) [">="]
    //
    //   "+" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "<" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "<=" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "=!=" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "==" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   ">" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   ">=" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
        __sym2: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 23
    //   "*"? = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   "*"? = (*) "*" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = Scalar (*) [EOF]
    //   Term = Scalar (*) ["+"]
    //   Term = Scalar (*) "*"? Variable [EOF]
    //   Term = Scalar (*) "*"? Variable ["+"]
    //
    //   EOF -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "*" -> Shift(S18)
    //   "+" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? =  => Call(ActionFn(35));)
    //
    //   "*"? -> S28
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, _), _)) => {
                let __nt = super::__action35(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt));
            }
            None |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_22_2a_22_3f(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 24
    //   (Term "+") = Term (*) "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = Term (*) "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* Term (*) [EOF]
    //
    //   EOF -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   "+" -> Shift(S19)
    //
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
        __sym1: &mut Option<(Option<Scalar>, Option<String>)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 25
    //   Term = Variable (*) [EOF]
    //   Term = Variable (*) ["+"]
    //
    //   EOF -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "+" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 26
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [EOF]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["*"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["+"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   EOF -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "*" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "+" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Scalar(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 27
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [EOF]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["+"]
    //
    //   EOF -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "+" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 28
    //   Term = Scalar "*"? (*) Variable [EOF]
    //   Term = Scalar "*"? (*) Variable ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S27)
    //
    //   Variable -> S29
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 29
    //   Term = Scalar "*"? Variable (*) [EOF]
    //   Term = Scalar "*"? Variable (*) ["+"]
    //
    //   EOF -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "+" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //
    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
        __sym2: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__LinearRelation::parse_LinearRelation;

mod __parse__Problem {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expr::{Scalar, LinearExpression, LinearRelation, Relation};
    use problem::{Problem, ProblemObjective};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Problem<
        'input,
    >(
        input: &'input str,
    ) -> Result<Problem, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Problem(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _22_2a_22_3f(::std::option::Option<&'input str>),
        _28LINE__SEP_2b_20LinearRelation_29((::std::vec::Vec<&'input str>, LinearRelation)),
        _28LINE__SEP_2b_20LinearRelation_29_2a(::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>),
        _28Term_20_22_2b_22_29(((Option<Scalar>, Option<String>), &'input str)),
        _28Term_20_22_2b_22_29_2a(::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>),
        Expression(LinearExpression),
        LINE__SEP(&'input str),
        LINE__SEP_2b(::std::vec::Vec<&'input str>),
        LinearRelation(LinearRelation),
        Problem(Problem),
        ProblemObjective(ProblemObjective),
        Relation(Relation),
        Scalar(Scalar),
        Term((Option<Scalar>, Option<String>)),
        Variable(String),
        ____Expression(LinearExpression),
        ____LINE__SEP(&'input str),
        ____LinearRelation(LinearRelation),
        ____Problem(Problem),
        ____ProblemObjective(ProblemObjective),
        ____Relation(Relation),
        ____Scalar(Scalar),
        ____Term((Option<Scalar>, Option<String>)),
        ____Variable(String),
    }

    // State 0
    //   Problem = (*) ProblemObjective LINE_SEP+ LinearRelation (LINE_SEP+ LinearRelation)* [EOF]
    //   ProblemObjective = (*) "maximize(" Expression ")" [";"]
    //   ProblemObjective = (*) "minimize(" Expression ")" [";"]
    //   __Problem = (*) Problem [EOF]
    //
    //   "maximize(" -> Shift(S3)
    //   "minimize(" -> Shift(S4)
    //
    //   Problem -> S1
    //   ProblemObjective -> S2
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state3(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Problem(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::ProblemObjective(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   __Problem = Problem (*) [EOF]
    //
    //   EOF -> Reduce(__Problem = Problem => Call(ActionFn(8));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Problem>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Problem(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   LINE_SEP = (*) ";" [";"]
    //   LINE_SEP = (*) ";" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP = (*) ";" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   LINE_SEP+ = (*) LINE_SEP [";"]
    //   LINE_SEP+ = (*) LINE_SEP [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP+ = (*) LINE_SEP [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   LINE_SEP+ = (*) LINE_SEP+ LINE_SEP [";"]
    //   LINE_SEP+ = (*) LINE_SEP+ LINE_SEP [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP+ = (*) LINE_SEP+ LINE_SEP [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Problem = ProblemObjective (*) LINE_SEP+ LinearRelation (LINE_SEP+ LinearRelation)* [EOF]
    //
    //   ";" -> Shift(S7)
    //
    //   LINE_SEP -> S5
    //   LINE_SEP+ -> S6
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<ProblemObjective>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::LINE__SEP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::LINE__SEP_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 3
    //   (Term "+")* = (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (*) (Term "+")* Term [")"]
    //   ProblemObjective = "maximize(" (*) Expression ")" [";"]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //
    //   (Term "+")* -> S8
    //   Expression -> S9
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expression(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 4
    //   (Term "+")* = (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (*) (Term "+")* Term [")"]
    //   ProblemObjective = "minimize(" (*) Expression ")" [";"]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //
    //   (Term "+")* -> S8
    //   Expression -> S10
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expression(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 5
    //   LINE_SEP+ = LINE_SEP (*) [";"]
    //   LINE_SEP+ = LINE_SEP (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP+ = LINE_SEP (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   ";" -> Reduce(LINE_SEP+ = LINE_SEP => Call(ActionFn(29));)
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(LINE_SEP+ = LINE_SEP => Call(ActionFn(29));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(LINE_SEP+ = LINE_SEP => Call(ActionFn(29));)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (3, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action29(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::LINE__SEP_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 6
    //   (Term "+")* = (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (*) (Term "+")* Term ["<"]
    //   Expression = (*) (Term "+")* Term ["<="]
    //   Expression = (*) (Term "+")* Term ["=!="]
    //   Expression = (*) (Term "+")* Term ["=="]
    //   Expression = (*) (Term "+")* Term [">"]
    //   Expression = (*) (Term "+")* Term [">="]
    //   LINE_SEP = (*) ";" [";"]
    //   LINE_SEP = (*) ";" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP = (*) ";" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   LINE_SEP+ = LINE_SEP+ (*) LINE_SEP [";"]
    //   LINE_SEP+ = LINE_SEP+ (*) LINE_SEP [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP+ = LINE_SEP+ (*) LINE_SEP [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   LinearRelation = (*) Expression Relation Expression [EOF]
    //   LinearRelation = (*) Expression Relation Expression [";"]
    //   Problem = ProblemObjective LINE_SEP+ (*) LinearRelation (LINE_SEP+ LinearRelation)* [EOF]
    //
    //   ";" -> Shift(S7)
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //
    //   (Term "+")* -> S11
    //   Expression -> S12
    //   LINE_SEP -> S13
    //   LinearRelation -> S14
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<ProblemObjective>,
        __sym1: &mut Option<::std::vec::Vec<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Expression(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::LINE__SEP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym1, __sym2));
                }
                __Nonterminal::LinearRelation(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state14(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   LINE_SEP = ";" (*) [";"]
    //   LINE_SEP = ";" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP = ";" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   ";" -> Reduce(LINE_SEP = ";" => Call(ActionFn(24));)
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(LINE_SEP = ";" => Call(ActionFn(24));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(LINE_SEP = ";" => Call(ActionFn(24));)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (3, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action24(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::LINE__SEP(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 8
    //   (Term "+") = (*) Term "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = (*) Term "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* (*) Term [")"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [")"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["*"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["+"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = (*) Scalar [")"]
    //   Term = (*) Scalar ["+"]
    //   Term = (*) Scalar "*"? Variable [")"]
    //   Term = (*) Scalar "*"? Variable ["+"]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Shift(S19)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S20)
    //
    //   (Term "+") -> S15
    //   Scalar -> S16
    //   Term -> S17
    //   Variable -> S18
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Scalar(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 9
    //   ProblemObjective = "maximize(" Expression (*) ")" [";"]
    //
    //   ")" -> Shift(S21)
    //
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<LinearExpression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 10
    //   ProblemObjective = "minimize(" Expression (*) ")" [";"]
    //
    //   ")" -> Shift(S22)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<LinearExpression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 11
    //   (Term "+") = (*) Term "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = (*) Term "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* (*) Term ["<"]
    //   Expression = (Term "+")* (*) Term ["<="]
    //   Expression = (Term "+")* (*) Term ["=!="]
    //   Expression = (Term "+")* (*) Term ["=="]
    //   Expression = (Term "+")* (*) Term [">"]
    //   Expression = (Term "+")* (*) Term [">="]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["*"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["+"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["<"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["<="]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["=!="]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["=="]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [">"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [">="]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = (*) Scalar ["+"]
    //   Term = (*) Scalar ["<"]
    //   Term = (*) Scalar ["<="]
    //   Term = (*) Scalar ["=!="]
    //   Term = (*) Scalar ["=="]
    //   Term = (*) Scalar [">"]
    //   Term = (*) Scalar [">="]
    //   Term = (*) Scalar "*"? Variable ["+"]
    //   Term = (*) Scalar "*"? Variable ["<"]
    //   Term = (*) Scalar "*"? Variable ["<="]
    //   Term = (*) Scalar "*"? Variable ["=!="]
    //   Term = (*) Scalar "*"? Variable ["=="]
    //   Term = (*) Scalar "*"? Variable [">"]
    //   Term = (*) Scalar "*"? Variable [">="]
    //   Term = (*) Variable ["+"]
    //   Term = (*) Variable ["<"]
    //   Term = (*) Variable ["<="]
    //   Term = (*) Variable ["=!="]
    //   Term = (*) Variable ["=="]
    //   Term = (*) Variable [">"]
    //   Term = (*) Variable [">="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["<"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["<="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["=!="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["=="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [">"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [">="]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Shift(S26)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S27)
    //
    //   (Term "+") -> S15
    //   Scalar -> S23
    //   Term -> S24
    //   Variable -> S25
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state26(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Scalar(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 12
    //   LinearRelation = Expression (*) Relation Expression [EOF]
    //   LinearRelation = Expression (*) Relation Expression [";"]
    //   Relation = (*) "<" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = (*) "<" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Relation = (*) "<=" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = (*) "<=" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Relation = (*) "=!=" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = (*) "=!=" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Relation = (*) "==" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = (*) "==" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Relation = (*) ">" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = (*) ">" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Relation = (*) ">=" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = (*) ">=" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   "<" -> Shift(S29)
    //   "<=" -> Shift(S30)
    //   "=!=" -> Shift(S31)
    //   "==" -> Shift(S32)
    //   ">" -> Shift(S33)
    //   ">=" -> Shift(S34)
    //
    //   Relation -> S28
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<LinearExpression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state31(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state32(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state33(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state34(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Relation(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 13
    //   LINE_SEP+ = LINE_SEP+ LINE_SEP (*) [";"]
    //   LINE_SEP+ = LINE_SEP+ LINE_SEP (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP+ = LINE_SEP+ LINE_SEP (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   ";" -> Reduce(LINE_SEP+ = LINE_SEP+, LINE_SEP => Call(ActionFn(30));)
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(LINE_SEP+ = LINE_SEP+, LINE_SEP => Call(ActionFn(30));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(LINE_SEP+ = LINE_SEP+, LINE_SEP => Call(ActionFn(30));)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<&'input str>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (3, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action30(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::LINE__SEP_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 14
    //   (LINE_SEP+ LinearRelation)* = (*) [EOF]
    //   (LINE_SEP+ LinearRelation)* = (*) [";"]
    //   (LINE_SEP+ LinearRelation)* = (*) (LINE_SEP+ LinearRelation)* (LINE_SEP+ LinearRelation) [EOF]
    //   (LINE_SEP+ LinearRelation)* = (*) (LINE_SEP+ LinearRelation)* (LINE_SEP+ LinearRelation) [";"]
    //   Problem = ProblemObjective LINE_SEP+ LinearRelation (*) (LINE_SEP+ LinearRelation)* [EOF]
    //
    //   EOF -> Reduce((LINE_SEP+ LinearRelation)* =  => Call(ActionFn(26));)
    //   ";" -> Reduce((LINE_SEP+ LinearRelation)* =  => Call(ActionFn(26));)
    //
    //   (LINE_SEP+ LinearRelation)* -> S35
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<ProblemObjective>,
        __sym1: &mut Option<::std::vec::Vec<&'input str>>,
        __sym2: &mut Option<LinearRelation>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) => {
                let __nt = super::__action26(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28LINE__SEP_2b_20LinearRelation_29_2a(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28LINE__SEP_2b_20LinearRelation_29_2a(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state35(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 15
    //   (Term "+")* = (Term "+")* (Term "+") (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (Term "+")* (Term "+") (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* = (Term "+")*, (Term "+") => Call(ActionFn(32));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* = (Term "+")*, (Term "+") => Call(ActionFn(32));)
    //
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
        __sym1: &mut Option<((Option<Scalar>, Option<String>), &'input str)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action32(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 16
    //   "*"? = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   "*"? = (*) "*" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = Scalar (*) [")"]
    //   Term = Scalar (*) ["+"]
    //   Term = Scalar (*) "*"? Variable [")"]
    //   Term = Scalar (*) "*"? Variable ["+"]
    //
    //   ")" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "*" -> Shift(S37)
    //   "+" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? =  => Call(ActionFn(35));)
    //
    //   "*"? -> S36
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state37(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, _), _)) => {
                let __nt = super::__action35(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt));
            }
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_22_2a_22_3f(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 17
    //   (Term "+") = Term (*) "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = Term (*) "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* Term (*) [")"]
    //
    //   ")" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   "+" -> Shift(S38)
    //
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
        __sym1: &mut Option<(Option<Scalar>, Option<String>)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 18
    //   Term = Variable (*) [")"]
    //   Term = Variable (*) ["+"]
    //
    //   ")" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "+" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 19
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [")"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["*"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["+"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   ")" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "*" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "+" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Scalar(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 20
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [")"]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["+"]
    //
    //   ")" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "+" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 21
    //   ProblemObjective = "maximize(" Expression ")" (*) [";"]
    //
    //   ";" -> Reduce(ProblemObjective = "maximize(", Expression, ")" => Call(ActionFn(22));)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<LinearExpression>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ProblemObjective(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 22
    //   ProblemObjective = "minimize(" Expression ")" (*) [";"]
    //
    //   ";" -> Reduce(ProblemObjective = "minimize(", Expression, ")" => Call(ActionFn(23));)
    //
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<LinearExpression>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ProblemObjective(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 23
    //   "*"? = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   "*"? = (*) "*" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = Scalar (*) ["+"]
    //   Term = Scalar (*) ["<"]
    //   Term = Scalar (*) ["<="]
    //   Term = Scalar (*) ["=!="]
    //   Term = Scalar (*) ["=="]
    //   Term = Scalar (*) [">"]
    //   Term = Scalar (*) [">="]
    //   Term = Scalar (*) "*"? Variable ["+"]
    //   Term = Scalar (*) "*"? Variable ["<"]
    //   Term = Scalar (*) "*"? Variable ["<="]
    //   Term = Scalar (*) "*"? Variable ["=!="]
    //   Term = Scalar (*) "*"? Variable ["=="]
    //   Term = Scalar (*) "*"? Variable [">"]
    //   Term = Scalar (*) "*"? Variable [">="]
    //
    //   "*" -> Shift(S37)
    //   "+" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "<" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "<=" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "=!=" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "==" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   ">" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   ">=" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? =  => Call(ActionFn(35));)
    //
    //   "*"? -> S39
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state37(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, _), _)) => {
                let __nt = super::__action35(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt));
            }
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_22_2a_22_3f(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state39(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 24
    //   (Term "+") = Term (*) "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = Term (*) "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* Term (*) ["<"]
    //   Expression = (Term "+")* Term (*) ["<="]
    //   Expression = (Term "+")* Term (*) ["=!="]
    //   Expression = (Term "+")* Term (*) ["=="]
    //   Expression = (Term "+")* Term (*) [">"]
    //   Expression = (Term "+")* Term (*) [">="]
    //
    //   "+" -> Shift(S38)
    //   "<" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   "<=" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   "=!=" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   "==" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   ">" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   ">=" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
        __sym1: &mut Option<(Option<Scalar>, Option<String>)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 25
    //   Term = Variable (*) ["+"]
    //   Term = Variable (*) ["<"]
    //   Term = Variable (*) ["<="]
    //   Term = Variable (*) ["=!="]
    //   Term = Variable (*) ["=="]
    //   Term = Variable (*) [">"]
    //   Term = Variable (*) [">="]
    //
    //   "+" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "<" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "<=" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "=!=" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "==" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   ">" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   ">=" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 26
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["*"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["+"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["<"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["<="]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["=!="]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["=="]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [">"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [">="]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   "*" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "+" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "<" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "<=" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "=!=" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "==" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   ">" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   ">=" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Scalar(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 27
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["+"]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["<"]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["<="]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["=!="]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["=="]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [">"]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [">="]
    //
    //   "+" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "<" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "<=" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "=!=" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "==" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   ">" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   ">=" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 28
    //   (Term "+")* = (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (*) (Term "+")* Term [EOF]
    //   Expression = (*) (Term "+")* Term [";"]
    //   LinearRelation = Expression Relation (*) Expression [EOF]
    //   LinearRelation = Expression Relation (*) Expression [";"]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //
    //   (Term "+")* -> S40
    //   Expression -> S41
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<LinearExpression>,
        __sym1: &mut Option<Relation>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state40(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Expression(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state41(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 29
    //   Relation = "<" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = "<" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(Relation = "<" => Call(ActionFn(15));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Relation = "<" => Call(ActionFn(15));)
    //
    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 30
    //   Relation = "<=" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = "<=" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(Relation = "<=" => Call(ActionFn(16));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Relation = "<=" => Call(ActionFn(16));)
    //
    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action16(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 31
    //   Relation = "=!=" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = "=!=" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(Relation = "=!=" => Call(ActionFn(20));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Relation = "=!=" => Call(ActionFn(20));)
    //
    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action20(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 32
    //   Relation = "==" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = "==" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(Relation = "==" => Call(ActionFn(19));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Relation = "==" => Call(ActionFn(19));)
    //
    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action19(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 33
    //   Relation = ">" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = ">" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(Relation = ">" => Call(ActionFn(17));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Relation = ">" => Call(ActionFn(17));)
    //
    pub fn __state33<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action17(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 34
    //   Relation = ">=" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   Relation = ">=" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce(Relation = ">=" => Call(ActionFn(18));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Relation = ">=" => Call(ActionFn(18));)
    //
    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action18(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 35
    //   (LINE_SEP+ LinearRelation) = (*) LINE_SEP+ LinearRelation [EOF]
    //   (LINE_SEP+ LinearRelation) = (*) LINE_SEP+ LinearRelation [";"]
    //   (LINE_SEP+ LinearRelation)* = (LINE_SEP+ LinearRelation)* (*) (LINE_SEP+ LinearRelation) [EOF]
    //   (LINE_SEP+ LinearRelation)* = (LINE_SEP+ LinearRelation)* (*) (LINE_SEP+ LinearRelation) [";"]
    //   LINE_SEP = (*) ";" [";"]
    //   LINE_SEP = (*) ";" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP = (*) ";" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   LINE_SEP+ = (*) LINE_SEP [";"]
    //   LINE_SEP+ = (*) LINE_SEP [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP+ = (*) LINE_SEP [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   LINE_SEP+ = (*) LINE_SEP+ LINE_SEP [";"]
    //   LINE_SEP+ = (*) LINE_SEP+ LINE_SEP [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP+ = (*) LINE_SEP+ LINE_SEP [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Problem = ProblemObjective LINE_SEP+ LinearRelation (LINE_SEP+ LinearRelation)* (*) [EOF]
    //
    //   EOF -> Reduce(Problem = ProblemObjective, LINE_SEP+, LinearRelation, (LINE_SEP+ LinearRelation)* => Call(ActionFn(25));)
    //   ";" -> Shift(S7)
    //
    //   (LINE_SEP+ LinearRelation) -> S42
    //   LINE_SEP -> S5
    //   LINE_SEP+ -> S43
    pub fn __state35<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<ProblemObjective>,
        __sym1: &mut Option<::std::vec::Vec<&'input str>>,
        __sym2: &mut Option<LinearRelation>,
        __sym3: &mut Option<::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym4));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Problem(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28LINE__SEP_2b_20LinearRelation_29(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state42(input, __lookbehind, __tokens, __lookahead, __sym3, __sym4));
                }
                __Nonterminal::LINE__SEP(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::LINE__SEP_2b(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state43(input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 36
    //   Term = Scalar "*"? (*) Variable [")"]
    //   Term = Scalar "*"? (*) Variable ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S20)
    //
    //   Variable -> S44
    pub fn __state36<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state44(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 37
    //   "*"? = "*" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? = "*" => Call(ActionFn(34));)
    //
    pub fn __state37<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action34(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 38
    //   (Term "+") = Term "+" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = Term "+" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+") = Term, "+" => Call(ActionFn(33));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+") = Term, "+" => Call(ActionFn(33));)
    //
    pub fn __state38<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(Option<Scalar>, Option<String>)>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action33(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 39
    //   Term = Scalar "*"? (*) Variable ["+"]
    //   Term = Scalar "*"? (*) Variable ["<"]
    //   Term = Scalar "*"? (*) Variable ["<="]
    //   Term = Scalar "*"? (*) Variable ["=!="]
    //   Term = Scalar "*"? (*) Variable ["=="]
    //   Term = Scalar "*"? (*) Variable [">"]
    //   Term = Scalar "*"? (*) Variable [">="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["<"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["<="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["=!="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["=="]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [">"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [">="]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S27)
    //
    //   Variable -> S45
    pub fn __state39<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state45(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 40
    //   (Term "+") = (*) Term "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = (*) Term "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* (*) Term [EOF]
    //   Expression = (Term "+")* (*) Term [";"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [EOF]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["*"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["+"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [";"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = (*) Scalar [EOF]
    //   Term = (*) Scalar ["+"]
    //   Term = (*) Scalar [";"]
    //   Term = (*) Scalar "*"? Variable [EOF]
    //   Term = (*) Scalar "*"? Variable ["+"]
    //   Term = (*) Scalar "*"? Variable [";"]
    //   Term = (*) Variable [EOF]
    //   Term = (*) Variable ["+"]
    //   Term = (*) Variable [";"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [";"]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Shift(S49)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S50)
    //
    //   (Term "+") -> S15
    //   Scalar -> S46
    //   Term -> S47
    //   Variable -> S48
    pub fn __state40<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state49(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state50(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Scalar(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state46(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state47(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state48(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 41
    //   LinearRelation = Expression Relation Expression (*) [EOF]
    //   LinearRelation = Expression Relation Expression (*) [";"]
    //
    //   EOF -> Reduce(LinearRelation = Expression, Relation, Expression => Call(ActionFn(21));)
    //   ";" -> Reduce(LinearRelation = Expression, Relation, Expression => Call(ActionFn(21));)
    //
    pub fn __state41<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<LinearExpression>,
        __sym1: &mut Option<Relation>,
        __sym2: &mut Option<LinearExpression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action21(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::LinearRelation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 42
    //   (LINE_SEP+ LinearRelation)* = (LINE_SEP+ LinearRelation)* (LINE_SEP+ LinearRelation) (*) [EOF]
    //   (LINE_SEP+ LinearRelation)* = (LINE_SEP+ LinearRelation)* (LINE_SEP+ LinearRelation) (*) [";"]
    //
    //   EOF -> Reduce((LINE_SEP+ LinearRelation)* = (LINE_SEP+ LinearRelation)*, (LINE_SEP+ LinearRelation) => Call(ActionFn(27));)
    //   ";" -> Reduce((LINE_SEP+ LinearRelation)* = (LINE_SEP+ LinearRelation)*, (LINE_SEP+ LinearRelation) => Call(ActionFn(27));)
    //
    pub fn __state42<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>>,
        __sym1: &mut Option<(::std::vec::Vec<&'input str>, LinearRelation)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action27(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28LINE__SEP_2b_20LinearRelation_29_2a(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 43
    //   (LINE_SEP+ LinearRelation) = LINE_SEP+ (*) LinearRelation [EOF]
    //   (LINE_SEP+ LinearRelation) = LINE_SEP+ (*) LinearRelation [";"]
    //   (Term "+")* = (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (*) (Term "+")* Term ["<"]
    //   Expression = (*) (Term "+")* Term ["<="]
    //   Expression = (*) (Term "+")* Term ["=!="]
    //   Expression = (*) (Term "+")* Term ["=="]
    //   Expression = (*) (Term "+")* Term [">"]
    //   Expression = (*) (Term "+")* Term [">="]
    //   LINE_SEP = (*) ";" [";"]
    //   LINE_SEP = (*) ";" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP = (*) ";" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   LINE_SEP+ = LINE_SEP+ (*) LINE_SEP [";"]
    //   LINE_SEP+ = LINE_SEP+ (*) LINE_SEP [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   LINE_SEP+ = LINE_SEP+ (*) LINE_SEP [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   LinearRelation = (*) Expression Relation Expression [EOF]
    //   LinearRelation = (*) Expression Relation Expression [";"]
    //
    //   ";" -> Shift(S7)
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //
    //   (Term "+")* -> S11
    //   Expression -> S12
    //   LINE_SEP -> S13
    //   LinearRelation -> S51
    pub fn __state43<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expression(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::LINE__SEP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::LinearRelation(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state51(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 44
    //   Term = Scalar "*"? Variable (*) [")"]
    //   Term = Scalar "*"? Variable (*) ["+"]
    //
    //   ")" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "+" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //
    pub fn __state44<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
        __sym2: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 45
    //   Term = Scalar "*"? Variable (*) ["+"]
    //   Term = Scalar "*"? Variable (*) ["<"]
    //   Term = Scalar "*"? Variable (*) ["<="]
    //   Term = Scalar "*"? Variable (*) ["=!="]
    //   Term = Scalar "*"? Variable (*) ["=="]
    //   Term = Scalar "*"? Variable (*) [">"]
    //   Term = Scalar "*"? Variable (*) [">="]
    //
    //   "+" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "<" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "<=" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "=!=" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "==" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   ">" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   ">=" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //
    pub fn __state45<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
        __sym2: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 46
    //   "*"? = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   "*"? = (*) "*" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = Scalar (*) [EOF]
    //   Term = Scalar (*) ["+"]
    //   Term = Scalar (*) [";"]
    //   Term = Scalar (*) "*"? Variable [EOF]
    //   Term = Scalar (*) "*"? Variable ["+"]
    //   Term = Scalar (*) "*"? Variable [";"]
    //
    //   EOF -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "*" -> Shift(S37)
    //   "+" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   ";" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? =  => Call(ActionFn(35));)
    //
    //   "*"? -> S52
    pub fn __state46<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state37(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, _), _)) => {
                let __nt = super::__action35(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt));
            }
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_22_2a_22_3f(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state52(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 47
    //   (Term "+") = Term (*) "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = Term (*) "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* Term (*) [EOF]
    //   Expression = (Term "+")* Term (*) [";"]
    //
    //   EOF -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   "+" -> Shift(S38)
    //   ";" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //
    pub fn __state47<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
        __sym1: &mut Option<(Option<Scalar>, Option<String>)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            None |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 48
    //   Term = Variable (*) [EOF]
    //   Term = Variable (*) ["+"]
    //   Term = Variable (*) [";"]
    //
    //   EOF -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "+" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   ";" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //
    pub fn __state48<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 49
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [EOF]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["*"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["+"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [";"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   EOF -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "*" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "+" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   ";" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //
    pub fn __state49<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Scalar(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 50
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [EOF]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["+"]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [";"]
    //
    //   EOF -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "+" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   ";" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //
    pub fn __state50<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 51
    //   (LINE_SEP+ LinearRelation) = LINE_SEP+ LinearRelation (*) [EOF]
    //   (LINE_SEP+ LinearRelation) = LINE_SEP+ LinearRelation (*) [";"]
    //
    //   EOF -> Reduce((LINE_SEP+ LinearRelation) = LINE_SEP+, LinearRelation => Call(ActionFn(28));)
    //   ";" -> Reduce((LINE_SEP+ LinearRelation) = LINE_SEP+, LinearRelation => Call(ActionFn(28));)
    //
    pub fn __state51<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<&'input str>>,
        __sym1: &mut Option<LinearRelation>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action28(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28LINE__SEP_2b_20LinearRelation_29(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 52
    //   Term = Scalar "*"? (*) Variable [EOF]
    //   Term = Scalar "*"? (*) Variable ["+"]
    //   Term = Scalar "*"? (*) Variable [";"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [";"]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S50)
    //
    //   Variable -> S53
    pub fn __state52<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state50(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state53(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 53
    //   Term = Scalar "*"? Variable (*) [EOF]
    //   Term = Scalar "*"? Variable (*) ["+"]
    //   Term = Scalar "*"? Variable (*) [";"]
    //
    //   EOF -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "+" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   ";" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //
    pub fn __state53<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
        __sym2: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Problem::parse_Problem;

mod __parse__ProblemObjective {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expr::{Scalar, LinearExpression, LinearRelation, Relation};
    use problem::{Problem, ProblemObjective};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_ProblemObjective<
        'input,
    >(
        input: &'input str,
    ) -> Result<ProblemObjective, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____ProblemObjective(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _22_2a_22_3f(::std::option::Option<&'input str>),
        _28LINE__SEP_2b_20LinearRelation_29((::std::vec::Vec<&'input str>, LinearRelation)),
        _28LINE__SEP_2b_20LinearRelation_29_2a(::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>),
        _28Term_20_22_2b_22_29(((Option<Scalar>, Option<String>), &'input str)),
        _28Term_20_22_2b_22_29_2a(::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>),
        Expression(LinearExpression),
        LINE__SEP(&'input str),
        LINE__SEP_2b(::std::vec::Vec<&'input str>),
        LinearRelation(LinearRelation),
        Problem(Problem),
        ProblemObjective(ProblemObjective),
        Relation(Relation),
        Scalar(Scalar),
        Term((Option<Scalar>, Option<String>)),
        Variable(String),
        ____Expression(LinearExpression),
        ____LINE__SEP(&'input str),
        ____LinearRelation(LinearRelation),
        ____Problem(Problem),
        ____ProblemObjective(ProblemObjective),
        ____Relation(Relation),
        ____Scalar(Scalar),
        ____Term((Option<Scalar>, Option<String>)),
        ____Variable(String),
    }

    // State 0
    //   ProblemObjective = (*) "maximize(" Expression ")" [EOF]
    //   ProblemObjective = (*) "minimize(" Expression ")" [EOF]
    //   __ProblemObjective = (*) ProblemObjective [EOF]
    //
    //   "maximize(" -> Shift(S2)
    //   "minimize(" -> Shift(S3)
    //
    //   ProblemObjective -> S1
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state2(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state3(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ProblemObjective(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   __ProblemObjective = ProblemObjective (*) [EOF]
    //
    //   EOF -> Reduce(__ProblemObjective = ProblemObjective => Call(ActionFn(6));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<ProblemObjective>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____ProblemObjective(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   (Term "+")* = (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (*) (Term "+")* Term [")"]
    //   ProblemObjective = "maximize(" (*) Expression ")" [EOF]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //
    //   (Term "+")* -> S4
    //   Expression -> S5
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expression(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 3
    //   (Term "+")* = (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (*) (Term "+")* (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (*) (Term "+")* Term [")"]
    //   ProblemObjective = "minimize(" (*) Expression ")" [EOF]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* =  => Call(ActionFn(31));)
    //
    //   (Term "+")* -> S4
    //   Expression -> S6
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expression(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 4
    //   (Term "+") = (*) Term "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = (*) Term "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (Term "+")* (*) (Term "+") [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* (*) Term [")"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [")"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["*"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["+"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = (*) Scalar [")"]
    //   Term = (*) Scalar ["+"]
    //   Term = (*) Scalar "*"? Variable [")"]
    //   Term = (*) Scalar "*"? Variable ["+"]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Shift(S11)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S12)
    //
    //   (Term "+") -> S7
    //   Scalar -> S8
    //   Term -> S9
    //   Variable -> S10
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Term_20_22_2b_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Scalar(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 5
    //   ProblemObjective = "maximize(" Expression (*) ")" [EOF]
    //
    //   ")" -> Shift(S13)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<LinearExpression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 6
    //   ProblemObjective = "minimize(" Expression (*) ")" [EOF]
    //
    //   ")" -> Shift(S14)
    //
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<LinearExpression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 7
    //   (Term "+")* = (Term "+")* (Term "+") (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+")* = (Term "+")* (Term "+") (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+")* = (Term "+")*, (Term "+") => Call(ActionFn(32));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+")* = (Term "+")*, (Term "+") => Call(ActionFn(32));)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
        __sym1: &mut Option<((Option<Scalar>, Option<String>), &'input str)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action32(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29_2a(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 8
    //   "*"? = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   "*"? = (*) "*" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = Scalar (*) [")"]
    //   Term = Scalar (*) ["+"]
    //   Term = Scalar (*) "*"? Variable [")"]
    //   Term = Scalar (*) "*"? Variable ["+"]
    //
    //   ")" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "*" -> Shift(S16)
    //   "+" -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? =  => Call(ActionFn(35));)
    //
    //   "*"? -> S15
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, _), _)) => {
                let __nt = super::__action35(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt));
            }
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_22_2a_22_3f(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 9
    //   (Term "+") = Term (*) "+" [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = Term (*) "+" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Expression = (Term "+")* Term (*) [")"]
    //
    //   ")" -> Reduce(Expression = (Term "+")*, Term => Call(ActionFn(14));)
    //   "+" -> Shift(S17)
    //
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>>,
        __sym1: &mut Option<(Option<Scalar>, Option<String>)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 10
    //   Term = Variable (*) [")"]
    //   Term = Variable (*) ["+"]
    //
    //   ")" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //   "+" -> Reduce(Term = Variable => Call(ActionFn(12));)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 11
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [")"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["*"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["+"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   ")" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "*" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "+" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Scalar(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [")"]
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["+"]
    //
    //   ")" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //   "+" -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 13
    //   ProblemObjective = "maximize(" Expression ")" (*) [EOF]
    //
    //   EOF -> Reduce(ProblemObjective = "maximize(", Expression, ")" => Call(ActionFn(22));)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<LinearExpression>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ProblemObjective(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 14
    //   ProblemObjective = "minimize(" Expression ")" (*) [EOF]
    //
    //   EOF -> Reduce(ProblemObjective = "minimize(", Expression, ")" => Call(ActionFn(23));)
    //
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<LinearExpression>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ProblemObjective(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 15
    //   Term = Scalar "*"? (*) Variable [")"]
    //   Term = Scalar "*"? (*) Variable ["+"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S12)
    //
    //   Variable -> S18
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 16
    //   "*"? = "*" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? = "*" => Call(ActionFn(34));)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action34(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 17
    //   (Term "+") = Term "+" (*) [r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"#]
    //   (Term "+") = Term "+" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Reduce((Term "+") = Term, "+" => Call(ActionFn(33));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce((Term "+") = Term, "+" => Call(ActionFn(33));)
    //
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(Option<Scalar>, Option<String>)>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action33(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28Term_20_22_2b_22_29(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 18
    //   Term = Scalar "*"? Variable (*) [")"]
    //   Term = Scalar "*"? Variable (*) ["+"]
    //
    //   ")" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //   "+" -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
        __sym2: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__ProblemObjective::parse_ProblemObjective;

mod __parse__Relation {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expr::{Scalar, LinearExpression, LinearRelation, Relation};
    use problem::{Problem, ProblemObjective};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Relation<
        'input,
    >(
        input: &'input str,
    ) -> Result<Relation, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Relation(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _22_2a_22_3f(::std::option::Option<&'input str>),
        _28LINE__SEP_2b_20LinearRelation_29((::std::vec::Vec<&'input str>, LinearRelation)),
        _28LINE__SEP_2b_20LinearRelation_29_2a(::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>),
        _28Term_20_22_2b_22_29(((Option<Scalar>, Option<String>), &'input str)),
        _28Term_20_22_2b_22_29_2a(::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>),
        Expression(LinearExpression),
        LINE__SEP(&'input str),
        LINE__SEP_2b(::std::vec::Vec<&'input str>),
        LinearRelation(LinearRelation),
        Problem(Problem),
        ProblemObjective(ProblemObjective),
        Relation(Relation),
        Scalar(Scalar),
        Term((Option<Scalar>, Option<String>)),
        Variable(String),
        ____Expression(LinearExpression),
        ____LINE__SEP(&'input str),
        ____LinearRelation(LinearRelation),
        ____Problem(Problem),
        ____ProblemObjective(ProblemObjective),
        ____Relation(Relation),
        ____Scalar(Scalar),
        ____Term((Option<Scalar>, Option<String>)),
        ____Variable(String),
    }

    // State 0
    //   Relation = (*) "<" [EOF]
    //   Relation = (*) "<=" [EOF]
    //   Relation = (*) "=!=" [EOF]
    //   Relation = (*) "==" [EOF]
    //   Relation = (*) ">" [EOF]
    //   Relation = (*) ">=" [EOF]
    //   __Relation = (*) Relation [EOF]
    //
    //   "<" -> Shift(S2)
    //   "<=" -> Shift(S3)
    //   "=!=" -> Shift(S4)
    //   "==" -> Shift(S5)
    //   ">" -> Shift(S6)
    //   ">=" -> Shift(S7)
    //
    //   Relation -> S1
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state2(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state3(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Relation(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   __Relation = Relation (*) [EOF]
    //
    //   EOF -> Reduce(__Relation = Relation => Call(ActionFn(4));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Relation>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   Relation = "<" (*) [EOF]
    //
    //   EOF -> Reduce(Relation = "<" => Call(ActionFn(15));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 3
    //   Relation = "<=" (*) [EOF]
    //
    //   EOF -> Reduce(Relation = "<=" => Call(ActionFn(16));)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action16(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   Relation = "=!=" (*) [EOF]
    //
    //   EOF -> Reduce(Relation = "=!=" => Call(ActionFn(20));)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action20(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   Relation = "==" (*) [EOF]
    //
    //   EOF -> Reduce(Relation = "==" => Call(ActionFn(19));)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action19(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 6
    //   Relation = ">" (*) [EOF]
    //
    //   EOF -> Reduce(Relation = ">" => Call(ActionFn(17));)
    //
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action17(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 7
    //   Relation = ">=" (*) [EOF]
    //
    //   EOF -> Reduce(Relation = ">=" => Call(ActionFn(18));)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action18(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Relation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Relation::parse_Relation;

mod __parse__Scalar {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expr::{Scalar, LinearExpression, LinearRelation, Relation};
    use problem::{Problem, ProblemObjective};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Scalar<
        'input,
    >(
        input: &'input str,
    ) -> Result<Scalar, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Scalar(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _22_2a_22_3f(::std::option::Option<&'input str>),
        _28LINE__SEP_2b_20LinearRelation_29((::std::vec::Vec<&'input str>, LinearRelation)),
        _28LINE__SEP_2b_20LinearRelation_29_2a(::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>),
        _28Term_20_22_2b_22_29(((Option<Scalar>, Option<String>), &'input str)),
        _28Term_20_22_2b_22_29_2a(::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>),
        Expression(LinearExpression),
        LINE__SEP(&'input str),
        LINE__SEP_2b(::std::vec::Vec<&'input str>),
        LinearRelation(LinearRelation),
        Problem(Problem),
        ProblemObjective(ProblemObjective),
        Relation(Relation),
        Scalar(Scalar),
        Term((Option<Scalar>, Option<String>)),
        Variable(String),
        ____Expression(LinearExpression),
        ____LINE__SEP(&'input str),
        ____LinearRelation(LinearRelation),
        ____Problem(Problem),
        ____ProblemObjective(ProblemObjective),
        ____Relation(Relation),
        ____Scalar(Scalar),
        ____Term((Option<Scalar>, Option<String>)),
        ____Variable(String),
    }

    // State 0
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [EOF]
    //   __Scalar = (*) Scalar [EOF]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Shift(S2)
    //
    //   Scalar -> S1
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state2(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Scalar(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   __Scalar = Scalar (*) [EOF]
    //
    //   EOF -> Reduce(__Scalar = Scalar => Call(ActionFn(0));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Scalar(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [EOF]
    //
    //   EOF -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Scalar(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Scalar::parse_Scalar;

mod __parse__Term {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expr::{Scalar, LinearExpression, LinearRelation, Relation};
    use problem::{Problem, ProblemObjective};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Term<
        'input,
    >(
        input: &'input str,
    ) -> Result<(Option<Scalar>, Option<String>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Term(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _22_2a_22_3f(::std::option::Option<&'input str>),
        _28LINE__SEP_2b_20LinearRelation_29((::std::vec::Vec<&'input str>, LinearRelation)),
        _28LINE__SEP_2b_20LinearRelation_29_2a(::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>),
        _28Term_20_22_2b_22_29(((Option<Scalar>, Option<String>), &'input str)),
        _28Term_20_22_2b_22_29_2a(::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>),
        Expression(LinearExpression),
        LINE__SEP(&'input str),
        LINE__SEP_2b(::std::vec::Vec<&'input str>),
        LinearRelation(LinearRelation),
        Problem(Problem),
        ProblemObjective(ProblemObjective),
        Relation(Relation),
        Scalar(Scalar),
        Term((Option<Scalar>, Option<String>)),
        Variable(String),
        ____Expression(LinearExpression),
        ____LINE__SEP(&'input str),
        ____LinearRelation(LinearRelation),
        ____Problem(Problem),
        ____ProblemObjective(ProblemObjective),
        ____Relation(Relation),
        ____Scalar(Scalar),
        ____Term((Option<Scalar>, Option<String>)),
        ____Variable(String),
    }

    // State 0
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [EOF]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# ["*"]
    //   Scalar = (*) r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = (*) Scalar [EOF]
    //   Term = (*) Scalar "*"? Variable [EOF]
    //   Term = (*) Variable [EOF]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   __Term = (*) Term [EOF]
    //
    //   r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# -> Shift(S4)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S5)
    //
    //   Scalar -> S1
    //   Term -> S2
    //   Variable -> S3
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Scalar(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   "*"? = (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   "*"? = (*) "*" [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //   Term = Scalar (*) [EOF]
    //   Term = Scalar (*) "*"? Variable [EOF]
    //
    //   EOF -> Reduce(Term = Scalar => Call(ActionFn(11));)
    //   "*" -> Shift(S7)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? =  => Call(ActionFn(35));)
    //
    //   "*"? -> S6
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, _), _)) => {
                let __nt = super::__action35(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_22_2a_22_3f(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 2
    //   __Term = Term (*) [EOF]
    //
    //   EOF -> Reduce(__Term = Term => Call(ActionFn(2));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(Option<Scalar>, Option<String>)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action2(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 3
    //   Term = Variable (*) [EOF]
    //
    //   EOF -> Reduce(Term = Variable => Call(ActionFn(12));)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [EOF]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) ["*"]
    //   Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   EOF -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   "*" -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce(Scalar = r#"-?[0-9]+\\.?[0-9]*([eE]-?[0-9]+)?"# => Call(ActionFn(9));)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Scalar(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [EOF]
    //
    //   EOF -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 6
    //   Term = Scalar "*"? (*) Variable [EOF]
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S5)
    //
    //   Variable -> S8
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   "*"? = "*" (*) [r#"[_a-zA-Z][_a-zA-Z0-9]*"#]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Reduce("*"? = "*" => Call(ActionFn(34));)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action34(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_22_2a_22_3f(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 8
    //   Term = Scalar "*"? Variable (*) [EOF]
    //
    //   EOF -> Reduce(Term = Scalar, "*"?, Variable => Call(ActionFn(13));)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Scalar>,
        __sym1: &mut Option<::std::option::Option<&'input str>>,
        __sym2: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Term::parse_Term;

mod __parse__Variable {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expr::{Scalar, LinearExpression, LinearRelation, Relation};
    use problem::{Problem, ProblemObjective};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Variable<
        'input,
    >(
        input: &'input str,
    ) -> Result<String, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Variable(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _22_2a_22_3f(::std::option::Option<&'input str>),
        _28LINE__SEP_2b_20LinearRelation_29((::std::vec::Vec<&'input str>, LinearRelation)),
        _28LINE__SEP_2b_20LinearRelation_29_2a(::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>),
        _28Term_20_22_2b_22_29(((Option<Scalar>, Option<String>), &'input str)),
        _28Term_20_22_2b_22_29_2a(::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>),
        Expression(LinearExpression),
        LINE__SEP(&'input str),
        LINE__SEP_2b(::std::vec::Vec<&'input str>),
        LinearRelation(LinearRelation),
        Problem(Problem),
        ProblemObjective(ProblemObjective),
        Relation(Relation),
        Scalar(Scalar),
        Term((Option<Scalar>, Option<String>)),
        Variable(String),
        ____Expression(LinearExpression),
        ____LINE__SEP(&'input str),
        ____LinearRelation(LinearRelation),
        ____Problem(Problem),
        ____ProblemObjective(ProblemObjective),
        ____Relation(Relation),
        ____Scalar(Scalar),
        ____Term((Option<Scalar>, Option<String>)),
        ____Variable(String),
    }

    // State 0
    //   Variable = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   __Variable = (*) Variable [EOF]
    //
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S2)
    //
    //   Variable -> S1
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state2(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Variable(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   __Variable = Variable (*) [EOF]
    //
    //   EOF -> Reduce(__Variable = Variable => Call(ActionFn(1));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action1(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [EOF]
    //
    //   EOF -> Reduce(Variable = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => Call(ActionFn(10));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Variable::parse_Variable;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ')' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '*' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '+' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        '-' => {
                            __current_state = 4;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        ';' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '<' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '=' => {
                            __current_state = 8;
                            continue;
                        }
                        '>' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '.' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_state = 14;
                            continue;
                        }
                        'e' => {
                            __current_state = 14;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '!' => {
                            __current_state = 16;
                            continue;
                        }
                        '=' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        'E' => {
                            __current_state = 14;
                            continue;
                        }
                        'e' => {
                            __current_state = 14;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '-' => {
                            __current_state = 21;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 33;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '(' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 36;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '(' => {
                            __current_match = Some((11, __index + 1));
                            __current_state = 37;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    __0: Scalar,
) -> Scalar
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    __0: String,
) -> String
{
    (__0)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    __0: (Option<Scalar>, Option<String>),
) -> (Option<Scalar>, Option<String>)
{
    (__0)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    __0: LinearExpression,
) -> LinearExpression
{
    (__0)
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    __0: Relation,
) -> Relation
{
    (__0)
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    __0: LinearRelation,
) -> LinearRelation
{
    (__0)
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __0: ProblemObjective,
) -> ProblemObjective
{
    (__0)
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: Problem,
) -> Problem
{
    (__0)
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    s: &'input str,
) -> Scalar
{
    f64::from_str(s).unwrap()
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    s: &'input str,
) -> String
{
    String::from(s)
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    s: Scalar,
) -> (Option<Scalar>, Option<String>)
{
    (Some(s), None)
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    v: String,
) -> (Option<Scalar>, Option<String>)
{
    (None, Some(v))
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    s: Scalar,
    _: ::std::option::Option<&'input str>,
    v: String,
) -> (Option<Scalar>, Option<String>)
{
    (Some(s), Some(v))
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    e: ::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>,
    u: (Option<Scalar>, Option<String>),
) -> LinearExpression
{
    {
    let mut expr = LinearExpression::new();
    let mut terms: Vec<(Option<Scalar>, Option<String>)> = e.into_iter().map(|t| t.0).collect();
    terms.push(u);
    for (scalar, var) in terms.into_iter() {
      let term = match (scalar, var) {
        (Some(scalar), Some(var)) => LinearExpression::term(var, scalar),
        (None, Some(var)) => LinearExpression::from(var),
        (Some(scalar), None) => LinearExpression::from(scalar),
        (None, None) => LinearExpression::new()
      };
      expr.plus_this(&term);
    }
    expr
  }
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Relation
{
    Relation::LT
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Relation
{
    Relation::LEQ
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Relation
{
    Relation::GT
}

pub fn __action18<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Relation
{
    Relation::GEQ
}

pub fn __action19<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Relation
{
    Relation::EQ
}

pub fn __action20<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Relation
{
    Relation::NEQ
}

pub fn __action21<
    'input,
>(
    input: &'input str,
    lhs: LinearExpression,
    o: Relation,
    rhs: LinearExpression,
) -> LinearRelation
{
    LinearRelation::new(lhs, o, rhs)
}

pub fn __action22<
    'input,
>(
    input: &'input str,
    _: &'input str,
    e: LinearExpression,
    _: &'input str,
) -> ProblemObjective
{
    ProblemObjective::Maximize(e)
}

pub fn __action23<
    'input,
>(
    input: &'input str,
    _: &'input str,
    e: LinearExpression,
    _: &'input str,
) -> ProblemObjective
{
    ProblemObjective::Minimize(e)
}

pub fn __action24<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> &'input str
{
    (__0)
}

pub fn __action25<
    'input,
>(
    input: &'input str,
    o: ProblemObjective,
    _: ::std::vec::Vec<&'input str>,
    i: LinearRelation,
    r: ::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>,
) -> Problem
{
    {
    let mut exprs: Vec<LinearRelation> = r.into_iter().map(|t|{t.1}).collect();
    exprs.push(i);
    Problem::new(o, exprs)
  }
}

pub fn __action26<
    'input,
>(
    input: &'input str,
) -> ::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>
{
    vec![]
}

pub fn __action27<
    'input,
>(
    input: &'input str,
    v: ::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>,
    e: (::std::vec::Vec<&'input str>, LinearRelation),
) -> ::std::vec::Vec<(::std::vec::Vec<&'input str>, LinearRelation)>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action28<
    'input,
>(
    input: &'input str,
    __0: ::std::vec::Vec<&'input str>,
    __1: LinearRelation,
) -> (::std::vec::Vec<&'input str>, LinearRelation)
{
    (__0, __1)
}

pub fn __action29<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

pub fn __action30<
    'input,
>(
    input: &'input str,
    v: ::std::vec::Vec<&'input str>,
    e: &'input str,
) -> ::std::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action31<
    'input,
>(
    input: &'input str,
) -> ::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>
{
    vec![]
}

pub fn __action32<
    'input,
>(
    input: &'input str,
    v: ::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>,
    e: ((Option<Scalar>, Option<String>), &'input str),
) -> ::std::vec::Vec<((Option<Scalar>, Option<String>), &'input str)>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action33<
    'input,
>(
    input: &'input str,
    __0: (Option<Scalar>, Option<String>),
    __1: &'input str,
) -> ((Option<Scalar>, Option<String>), &'input str)
{
    (__0, __1)
}

pub fn __action34<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> ::std::option::Option<&'input str>
{
    Some(__0)
}

pub fn __action35<
    'input,
>(
    input: &'input str,
) -> ::std::option::Option<&'input str>
{
    None
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
