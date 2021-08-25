use crate::lex::Token;
use crate::scope::Scope;
use hir::Var;

pub(crate) struct Parser<'a> {
  tokens: Vec<Token<'a>>,
  i: usize,
  scope: Scope<'a>,
}

impl<'a> Parser<'a> {
  pub(crate) fn new(tokens: Vec<Token<'a>>) -> Self {
    Self {
      tokens,
      i: 0,
      scope: Scope::default(),
    }
  }

  pub(crate) fn peek(&self) -> Option<Token<'a>> {
    self.tokens.get(self.i).copied()
  }

  pub(crate) fn at(&self, token: Token<'_>) -> bool {
    self.peek().map_or(false, |t| t == token)
  }

  pub(crate) fn bump(&mut self) {
    self.i += 1;
  }

  pub(crate) fn eat(&mut self, token: Token<'_>) {
    if self.at(token) {
      self.bump();
    } else {
      panic!("expected {:?}", token);
    }
  }

  pub(crate) fn var(&self) -> Option<&'a str> {
    self.peek().and_then(|tok| match tok {
      Token::Var(v) => Some(v),
      _ => None,
    })
  }

  pub(crate) fn push(&mut self, s: &'a str) {
    self.scope.push(s)
  }

  pub(crate) fn pop(&mut self) {
    self.scope.pop()
  }

  pub(crate) fn get(&mut self, s: &str) -> Var {
    self.scope.get(s)
  }
}
