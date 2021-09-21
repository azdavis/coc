use hir::Var;

#[derive(Debug, Default)]
pub(crate) struct Scope<'a> {
  vars: Vec<&'a str>,
}

impl<'a> Scope<'a> {
  pub(crate) fn push(&mut self, s: &'a str) {
    self.vars.push(s);
  }

  pub(crate) fn pop(&mut self) {
    self.vars.pop().expect("nothing to pop");
  }

  pub(crate) fn get(&self, s: &str) -> Var {
    let (i, _) = self
      .vars
      .iter()
      .enumerate()
      .rev()
      .find(|&(_, &v)| v == s)
      .expect("not in scope");
    self.vars.len() - 1 - i
  }
}
