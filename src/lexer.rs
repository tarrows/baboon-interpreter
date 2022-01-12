use crate::token::Token;

pub struct Lexer<'a> {
  input: std::str::Chars<'a>,
  curr: char,  // current character
  peek: char,  // next (peeked) character
}


impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    let mut lexer = Lexer {
      input: input.chars(),
      curr: '\u{0}',  // null
      peek: '\u{0}',
    };

    // set first input character to peek
    lexer.read_char();

    // set first input character to curr
    lexer.read_char();

    lexer
  }

  fn read_char(&mut self) -> char {
    let c = self.curr;

    self.curr = self.peek;
    self.peek = self.input.next().unwrap_or('\u{0}');

    c
  }

  pub fn next_token(&mut self) -> Token {
    let token = match self.curr {
      '=' => Token::ASSIGN,
      ';' => Token::SEMICOLON,
      '\u{0}' => Token::EOF,
      _ => Token::ILLEGAL,
    };

    self.read_char();

    token
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::token::Token;

  #[test]
  fn generate_tokens() {
    let input = "=3;";

    let expected = [
      Token::ASSIGN,
      Token::ILLEGAL,
      Token::SEMICOLON,
    ];

    let mut lexer = Lexer::new(input);

    for (i, token) in expected.iter().enumerate() {
      let actual = &lexer.next_token();

      assert_eq!(actual, token, "Failed at token {}", i + 1);
    }
  }
}
