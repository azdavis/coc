//! Tests.

#![cfg(test)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_code)]

fn check(a: &str, b: &str) {
  let tm_a = parse_allow_type(a);
  let ty_a = statics::go(&[], &tm_a);
  let tm_b = parse_allow_type(b);
  assert_eq!(ty_a.to_string(), tm_b.to_string());
}

fn parse_allow_type(s: &str) -> hir::Term {
  if s == "@" {
    hir::Term::Type
  } else {
    parse::parse(s)
  }
}

#[test]
fn prop() {
  check("*", "@");
}

#[test]
fn prop_with_var() {
  check("forall x: *. *", "@");
}

#[test]
fn never() {
  check("forall y: *. y", "*");
}

#[test]
fn id_type() {
  check("forall x: *. forall y: x. x", "*");
}

#[test]
fn id_term() {
  check("fn a: *. fn b: a. b", "forall a: *. forall b: a. a");
}

#[test]
fn prop_id() {
  check("fn a: *. a", "forall a: *. *");
}

#[test]
fn app() {
  check("(fn x: *. x) (forall a: *. a)", "*");
}

#[test]
fn fst() {
  check(
    "fn a: *. fn b: *. fn x: a. fn y: b. x",
    "forall a: *. forall b: *. forall x: a. forall y: b. a",
  );
}
