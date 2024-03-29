use crate::parser::parser::{
    Sexp,
    Atom,
    SexpT,
    independant_sexp
};
use crate::errors::{
    Error,
    EvalError
};
use crate::eval::eval;


pub fn print(sexp: Result<Sexp, Error>) {
    match sexp {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{e}")
    }
}

pub fn quote(sexp: Sexp) -> Sexp { sexp }

pub fn car(list: Sexp) -> Result<Sexp, Error> {
    if let SexpT::List(v) = list.sexpt {
        match v.get(0) {
            Some(sexp) => Ok(sexp.clone()),
            None => Ok(independant_sexp(SexpT::List(Vec::new())))
        }
    } else {
        Err(Error::EvalError(EvalError::TypeMismatch(list.sexpt, String::from("list"), list.pos)))
    }
}

pub fn cdr(list: Sexp) -> Result<Sexp, Error> {
    if let SexpT::List(v) = list.sexpt {
        match v.get(1..) {
            Some(v) => Ok(independant_sexp(SexpT::List(v.to_vec()))),
            None => Ok(independant_sexp(SexpT::List(Vec::new())))
        }
    } else {
        Err(Error::EvalError(EvalError::TypeMismatch(list.sexpt, String::from("list"), list.pos)))
    }
}

pub fn cons(item: Sexp, list: Sexp) -> Result<Sexp, Error> {
    if let SexpT::List(mut v) = list.sexpt {
        v.insert(0, item);
        Ok(independant_sexp(SexpT::List(v)))
    } else {
        Err(Error::EvalError(EvalError::TypeMismatch(list.sexpt, String::from("list"), list.pos)))
    }
}

pub fn atom(sexp: Sexp) -> Sexp {
    match sexp.sexpt {
        SexpT::Atom(_) => independant_sexp(SexpT::Atom(Atom::Symbol("t".to_owned()))),
        SexpT::List(_) => independant_sexp(SexpT::List(Vec::new()))
    }
}

pub fn eq(left: Sexp, right: Sexp) -> Sexp {
    match left == right {
        true => independant_sexp(SexpT::Atom(Atom::Symbol("t".to_owned()))),
        false => independant_sexp(SexpT::List(Vec::new()))
    }
}

pub fn cond(conditions: Vec<Sexp>) -> Result<Sexp, Error> {
    for c in conditions {
        match c.sexpt {
            SexpT::Atom(_) => return Err(Error::EvalError(EvalError::TypeMismatch(c.sexpt, String::from("list"), c.pos))),
            SexpT::List(v) => {
                if v.len() != 2 {
                    return Err(Error::EvalError(EvalError::ArityMismatch("condition".to_owned(), v.len(), 2, c.pos)))
                }
                let cond = eval(Ok(v.get(0).unwrap().clone()))?;
                if cond != independant_sexp(SexpT::List(Vec::new())) {
                    return Ok(v.get(1).unwrap().clone())
                }
            }
        }
    }
    Ok(independant_sexp(SexpT::List(Vec::new())))
}
