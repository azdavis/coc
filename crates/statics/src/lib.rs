//! Static analysis.

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_code)]

use hir::{Term, Var};
use std::cmp::Ordering;

/// Returns the type of the term under the environment.
pub fn go(env: &[Term], term: &Term) -> Term {
  match term {
    Term::Prop => Term::Type,
    Term::Type => unreachable!("cannot write Type in concrete syntax"),
    Term::Var(v) => {
      let mut ret = env.get(env.len() - 1 - *v).expect("not in scope").clone();
      lift(*v + 1, 0, &mut ret);
      ret
    }
    Term::App(func, arg) => {
      let func_ty = wh(go(env, func));
      let arg_ty = go(env, arg);
      let (ann, mut body_ty) = match func_ty {
        Term::Pi(a, b) => (*a, *b),
        _ => panic!("App func ty not Pi"),
      };
      // NOTE: alpha-equivalence is just `==`
      assert_eq!(wh(ann), wh(arg_ty), "not alpha-equivalent");
      subst(0, arg, &mut body_ty);
      body_ty
    }
    Term::Lam(ann, body) => {
      let ann = wh((**ann).clone());
      go(env, &ann);
      let new_env = env_ins(env.to_vec(), ann.clone());
      let body_ty = go(&new_env, body);
      let ret = Term::Pi(Box::new(ann), Box::new(body_ty));
      // TODO do we really *need* to check for valid pi?
      go(env, &ret);
      ret
    }
    Term::Pi(ann, body) => {
      let ann = wh((**ann).clone());
      let ann_ty = wh(go(env, &ann));
      assert!(is_sort(&ann_ty), "failed: {:?}", ann_ty);
      let new_env = env_ins(env.to_vec(), ann);
      let ret = wh(go(&new_env, body));
      assert!(is_sort(&ret), "failed: {:?} for {:?}", ret, body);
      ret
    }
  }
}

/// this should only be used on the 'body' of an expression that had 1 variable
/// binder like Lam or Pi.
fn subst(var: Var, var_term: &Term, term: &mut Term) {
  match term {
    Term::Prop | Term::Type => {}
    Term::Var(v) => match (*v).cmp(&var) {
      Ordering::Less => {}
      Ordering::Equal => *term = var_term.clone(),
      // see the test `apply` for why this is necessary
      Ordering::Greater => *v -= 1,
    },
    Term::App(func, arg) => {
      subst(var, var_term, func);
      subst(var, var_term, arg);
    }
    Term::Lam(ann, body) | Term::Pi(ann, body) => {
      subst(var, var_term, ann);
      let mut var_term = var_term.clone();
      lift(1, 0, &mut var_term);
      subst(var + 1, &var_term, body);
    }
  }
}

/// lifts up free variables in `term`, so that it may be inserted into the body
/// of a single additional binder.
fn lift(by: usize, free: Var, term: &mut Term) {
  match term {
    Term::Prop | Term::Type => {}
    Term::Var(v) => {
      if *v >= free {
        *v += by;
      }
    }
    Term::App(func, arg) => {
      lift(by, free, func);
      lift(by, free, arg);
    }
    Term::Lam(ann, body) | Term::Pi(ann, body) => {
      lift(by, free, ann);
      lift(by, free + 1, body);
    }
  }
}

/// returns the weak head normal form of `tm`.
fn wh(term: Term) -> Term {
  match term {
    Term::App(func, arg) => match wh(*func) {
      Term::Lam(_, mut body) => {
        subst(0, &arg, &mut body);
        wh(*body)
      }
      func => Term::App(Box::new(func), arg),
    },
    _ => term,
  }
}

fn is_sort(term: &Term) -> bool {
  matches!(*term, Term::Prop | Term::Type)
}

/// `tm` should be in WHNF.
fn env_ins(mut env: Vec<Term>, tm: Term) -> Vec<Term> {
  env.push(tm);
  env
}
