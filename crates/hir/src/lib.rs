//! HIR. (Or is it AST?)

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_code)]

use std::fmt;

/// A variable. Just an index.
pub type Var = usize;

/// A term in the calculus of constructions. Uses De Bruijn indices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
  /// Propositions.
  Prop,
  /// Types.
  Type,
  /// Variable.
  Var(Var),
  /// Application.
  App(Box<Term>, Box<Term>),
  /// Abstraction.
  Lam(Box<Term>, Box<Term>),
  /// Forall.
  Pi(Box<Term>, Box<Term>),
}

impl Term {
  fn show(
    &self,
    f: &mut fmt::Formatter<'_>,
    prec: Prec,
    vars: Var,
  ) -> fmt::Result {
    match self {
      Term::Prop => f.write_str("*"),
      Term::Type => f.write_str("@"),
      Term::Var(v) => {
        let n = vars
          .checked_sub(1)
          .ok_or(fmt::Error)?
          .checked_sub(*v)
          .ok_or(fmt::Error)?;
        f.write_str(&var(n))
      }
      Term::App(func, arg) => {
        if prec == Prec::C {
          f.write_str("(")?;
        }
        func.show(f, Prec::B, vars)?;
        f.write_str(" ")?;
        arg.show(f, Prec::C, vars)?;
        if prec == Prec::C {
          f.write_str(")")?;
        }
        Ok(())
      }
      Term::Lam(ann, body) => show_lam_like(ann, body, f, prec, vars, "fn "),
      Term::Pi(ann, body) => show_lam_like(ann, body, f, prec, vars, "forall "),
    }
  }
}

fn show_lam_like(
  ann: &Term,
  body: &Term,
  f: &mut fmt::Formatter<'_>,
  prec: Prec,
  vars: Var,
  s: &str,
) -> fmt::Result {
  if prec != Prec::A {
    f.write_str("(")?;
  }
  f.write_str(s)?;
  f.write_str(&var(vars))?;
  f.write_str(": ")?;
  ann.show(f, Prec::A, vars)?;
  f.write_str(". ")?;
  body.show(f, Prec::A, vars.checked_add(1).ok_or(fmt::Error)?)?;
  if prec != Prec::A {
    f.write_str(")")?;
  }
  Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Prec {
  A,
  B,
  C,
}

impl fmt::Display for Term {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.show(f, Prec::A, 0)
  }
}

const ALPHABET_SIZE: usize = 26;

const ALPHABET: [char; ALPHABET_SIZE] = [
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
  'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn var(n: Var) -> String {
  let q = n / ALPHABET_SIZE;
  let r = n % ALPHABET_SIZE;
  let mut ret = String::from(ALPHABET[r]);
  if q > 0 {
    ret.push_str(&q.to_string());
  }
  ret
}
