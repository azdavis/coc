#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Token<'a> {
  Var(&'a str),
  Fn,
  Forall,
  Colon,
  Dot,
  LRound,
  RRound,
  Star,
}

pub(crate) fn lex(s: &str) -> Vec<Token<'_>> {
  let bs = s.as_bytes();
  let mut i = 0;
  let mut tokens = Vec::new();
  while let Some(&b) = bs.get(i) {
    // whitespace
    if b.is_ascii_whitespace() {
      i += 1;
      continue;
    }
    // comments
    if b == b'#' {
      i += 1;
      i = advance_while(bs, i, |b| b != b'\n');
      continue;
    }
    // variables and keywords
    if is_var_byte(b) {
      let start = i;
      i += 1;
      i = advance_while(bs, i, is_var_byte);
      let tok = match &bs[start..i] {
        b"fn" => Token::Fn,
        b"forall" => Token::Forall,
        s => Token::Var(std::str::from_utf8(s).unwrap()),
      };
      tokens.push(tok);
      continue;
    }
    // punctuation
    if let Some(&(_, t)) = PUNCTUATION.iter().find(|&&(tb, _)| tb == b) {
      i += 1;
      tokens.push(t);
      continue;
    }
    panic!("unknown byte: {}", b);
  }
  tokens
}

const PUNCTUATION: [(u8, Token<'_>); 5] = [
  (b':', Token::Colon),
  (b'.', Token::Dot),
  (b'(', Token::LRound),
  (b')', Token::RRound),
  (b'*', Token::Star),
];

fn is_var_byte(b: u8) -> bool {
  b.is_ascii_alphabetic() || b == b'_'
}

fn advance_while(bs: &[u8], mut i: usize, f: fn(u8) -> bool) -> usize {
  while let Some(&b) = bs.get(i) {
    if f(b) {
      i += 1;
    } else {
      break;
    }
  }
  i
}
