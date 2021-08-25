//! Parsing, lexing, scope resolution, and lowering into HIR.

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_code)]

mod lex;
mod parser;
mod scope;

use hir::Term;
use lex::Token;
use parser::Parser;

/// Do the parsing.
pub fn parse(s: &str) -> Term {
  let ts = lex::lex(s);
  let mut p = Parser::new(ts);
  let ret = term(&mut p);
  assert!(p.peek().is_none());
  ret
}

fn term(p: &mut Parser<'_>) -> Term {
  if p.at(Token::Fn) {
    let (ann, body) = lam_like(p);
    Term::Lam(ann, body)
  } else if p.at(Token::Forall) {
    let (ann, body) = lam_like(p);
    Term::Pi(ann, body)
  } else {
    let mut ret = at_term(p).expect("expected a term");
    while let Some(arg) = at_term(p) {
      ret = Term::App(Box::new(ret), Box::new(arg));
    }
    ret
  }
}

fn lam_like(p: &mut Parser<'_>) -> (Box<Term>, Box<Term>) {
  p.bump();
  let var = p.var().expect("expected a var");
  p.bump();
  p.eat(Token::Colon);
  let ann = term(p);
  p.eat(Token::Dot);
  p.push(var);
  let body = term(p);
  p.pop();
  (Box::new(ann), Box::new(body))
}

fn at_term(p: &mut Parser<'_>) -> Option<Term> {
  let ret = if p.at(Token::LRound) {
    p.bump();
    let ret = term(p);
    p.eat(Token::RRound);
    ret
  } else if let Some(v) = p.var() {
    p.bump();
    Term::Var(p.get(v))
  } else if p.at(Token::Star) {
    p.bump();
    Term::Prop
  } else {
    return None;
  };
  Some(ret)
}
