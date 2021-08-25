//! Static analysis.

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_code)]

use hir::{Term, Var};
use std::cmp::Ordering;

/// TODO return the term's normal form and its type, not just its type? doesn't
/// seem easy to do without HOAS? is a variable in normal form? what about
/// reducing to normal form to handle applications?
pub fn go(env: &[Term], term: &Term) -> Term {
  match term {
    Term::Prop => Term::Type,
    Term::Type => unreachable!("cannot write Type in concrete syntax"),
    Term::Var(v) => env.get(env.len() - 1 - *v).expect("not in scope").clone(),
    Term::App(func, arg) => {
      let func_ty = go(env, func);
      let arg_ty = go(env, arg);
      let (ann, mut body_ty) = match func_ty {
        Term::Pi(a, b) => (*a, *b),
        _ => panic!("App func ty not Pi"),
      };
      // NOTE: alpha-equivalence is just `==`
      assert_eq!(ann, arg_ty, "not alpha-equivalent");
      subst(0, arg, &mut body_ty);
      body_ty
    }
    Term::Lam(ann, body) => {
      go(env, ann);
      let new_env = env_ins(env.to_vec(), (**ann).clone());
      let body_ty = go(&new_env, body);
      // TODO do we need to check for valid pi?
      let ret = Term::Pi(ann.clone(), Box::new(body_ty));
      go(env, &ret);
      ret
    }
    Term::Pi(ann, body) => {
      let ann_ty = go(env, ann);
      assert!(is_sort(&ann_ty), "failed: {:?}", ann_ty);
      let new_env = env_ins(env.to_vec(), (**ann).clone());
      let ret = go(&new_env, body);
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
      lift(0, &mut var_term);
      subst(var + 1, &var_term, body);
    }
  }
}

fn lift(free: Var, term: &mut Term) {
  match term {
    Term::Prop | Term::Type => {}
    Term::Var(v) => {
      if *v >= free {
        *v += 1;
      }
    }
    Term::App(func, arg) => {
      lift(free, func);
      lift(free, arg);
    }
    Term::Lam(ann, body) | Term::Pi(ann, body) => {
      lift(free, ann);
      lift(free + 1, body);
    }
  }
}

fn is_sort(term: &Term) -> bool {
  matches!(*term, Term::Prop | Term::Type)
}

fn env_ins(mut env: Vec<Term>, tm: Term) -> Vec<Term> {
  env.push(tm);
  for tm in env.iter_mut() {
    lift(0, tm);
  }
  env
}
