use ast::*;

pub struct Parser {
  s: Vec<char>,
  pos: usize,
}

impl Parser {
  pub fn new(s: String) -> Parser {
    Parser {
      s: s.chars().collect(),
      pos: 0,
    }
  }

  fn peek(&self) -> Option<char> {
    self.s.get(self.pos).cloned()
  }

  fn next(&mut self) -> Option<char> {
    let val = self.peek();
    self.pos += 1;
    val
  }

  fn eof(&self) -> Option<()> {
    if self.s.len() == self.pos {
      Some(())
    } else {
      None
    }
  }

  fn char(&mut self, c: char) -> Option<char> {
    self.expect(|x| x == c)
  }

  fn expect<F>(&mut self, f: F) -> Option<char>
  where
    F: FnOnce(char) -> bool,
  {
    match self.peek() {
      Some(x) if f(x) => {
        self.next();
        Some(x)
      }
      _ => None,
    }
  }

  fn ast(&mut self) -> Option<Vec<AST>> {
    let mut res = Vec::new();
    while let Some(c) = self.peek() {
      match c {
        '>' => {
          res.push(AST::PtrInc);
          self.next();
        }
        '<' => {
          res.push(AST::PtrDec);
          self.next();
        }
        '+' => {
          res.push(AST::ValInc);
          self.next();
        }
        '-' => {
          res.push(AST::ValDec);
          self.next();
        }
        '.' => {
          res.push(AST::Out);
          self.next();
        }
        ',' => {
          res.push(AST::In);
          self.next();
        }
        '[' => {
          self.next();
          res.push(AST::Loop(self.ast()?));
          self.char(']')?;
        }
        ']' => break,
        _ => {
          self.next();
        }
      }
    }
    Some(res)
  }

  pub fn parse(&mut self) -> Option<Vec<AST>> {
    let ast = self.ast()?;
    self.eof()?;
    Some(ast)
  }
}
