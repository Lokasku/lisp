use crate::parser::Position;
use crate::parser::parser::{
    Sexp,
    SexpT,
    Atom, independant_sexp
};
use crate::errors::{
    EvalError,
    Error
};
use crate::builtins;

const PRIMITIVES: [&str; 8] = ["car", "cdr", "cons", "atom", "quote", "lambda", "cond", "eq"];

fn args_checker(name: String, expected: usize, args: Vec<Sexp>, pos: Position) -> Result<(), Error> {
    let amount = args.len() - 1; // operator is not included
    match amount == expected {
        true => Ok(()),
        false => Err(Error::EvalError(EvalError::ArityMismatch(name, amount, expected, pos)))
    }
}

pub fn eval(ast: Result<Sexp, Error>) -> Result<Sexp, Error> {
    match ast {
        Ok(ref sexp @ Sexp { ref sexpt, pos }) => match sexpt {
            SexpT::Atom(atom) => match atom {
                Atom::Symbol(s) => {
                    if PRIMITIVES.contains(&s.as_str()) {
                        return Ok(sexp.clone())
                    }
                    if s == &"t".to_owned() {
                        return Ok(independant_sexp(SexpT::List(
                            vec![
                                independant_sexp(SexpT::Atom(Atom::Symbol("quote".to_owned()))),
                                independant_sexp(SexpT::Atom(Atom::Symbol("t".to_owned())))
                            ]
                        )));
                    }
                    Err(Error::EvalError(EvalError::UnboundSymbol(s.to_owned(), pos)))
                }
                _ => Ok(sexp.clone()) }
            SexpT::List(v) => match v.get(0) {
                Some(Sexp { sexpt, pos }) => match sexpt {
                    SexpT::Atom(atom) => match atom {
                        Atom::Symbol(sn) => match sn.as_str() {
                            "eval" => {
                                if let Err(e) = args_checker(sn.to_owned(), 1, v.clone(), *pos) {
                                    return Err(e)
                                }
                                eval(eval(Ok(v.get(1).unwrap().clone())))
                            }
                            "quote" => {
                                if let Err(e) = args_checker(sn.to_owned(), 1, v.clone(), *pos) {
                                    return Err(e)
                                }
                                Ok(builtins::quote(v.get(1).unwrap().clone()))
                            }
                            "car" => {
                                if let Err(e) = args_checker(sn.to_owned(), 1, v.clone(), *pos) {
                                    return Err(e)
                                }
                                let list = eval(Ok(v.get(1).unwrap().clone()))?;
                                builtins::car(list)
                            }
                            "cdr" => {
                                if let Err(e) = args_checker(sn.to_owned(), 1, v.clone(), *pos) {
                                    return Err(e)
                                }
                                let list = eval(Ok(v.get(1).unwrap().clone()))?;
                                builtins::cdr(list)
                             }
                             "cons" => {
                                if let Err(e) = args_checker(sn.to_owned(), 2, v.clone(), *pos) {
                                    return Err(e)
                                }
                                let item = eval(Ok(v.get(1).unwrap().clone()))?;
                                let list = eval(Ok(v.get(2).unwrap().clone()))?;
                                builtins::cons(item, list)
                             }
                             "atom" => {
                                if let Err(e) = args_checker(sn.to_owned(), 1, v.clone(), *pos) {
                                    return Err(e)
                                }
                                let sexp = eval(Ok(v.get(1).unwrap().clone()))?;
                                Ok(builtins::atom(sexp))
                              }
                              "eq" => {
                                  if let Err(e) = args_checker(sn.to_owned(), 2, v.clone(), *pos) {
                                      return Err(e)
                                   }
                                   let sexp1 = eval(Ok(v.get(1).unwrap().clone()))?;
                                   let sexp2 = eval(Ok(v.get(2).unwrap().clone()))?;
                                   Ok(builtins::eq(sexp1, sexp2))
                              }
                              "cond" => {
                                  if (v.len() - 1) < 1 {
                                      return Err(Error::EvalError(EvalError::ArityMismatch(sn.to_owned(), 0, 2, *pos)));
                                  }
                                  builtins::cond(v.get(1..).unwrap().to_vec())
                              }
                            _ => Err(Error::EvalError(EvalError::UnboundSymbol(sn.to_owned(), *pos)))
                        }
                        _ => Err(Error::EvalError(EvalError::IllegalFunctionCall(*pos)))

                    }
                    _ => todo!()
                }
                None => Ok(sexp.clone()),
            }
        }
        Err(e) => {
            Err(e)
        }
    }
}
